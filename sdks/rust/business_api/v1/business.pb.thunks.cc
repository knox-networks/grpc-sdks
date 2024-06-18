#include "business_api/v1/business.proto.h"
#include "google/protobuf/rust/cpp_kernel/cpp_api.h"
// business_api.v1.ListRequest
        // clang-format off
extern "C" {
void* __rust_proto_thunk__business_api_v1_ListRequest_new() { return new ::business_api::v1::ListRequest(); }
void __rust_proto_thunk__business_api_v1_ListRequest_delete(void* ptr) { delete static_cast<::business_api::v1::ListRequest*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__business_api_v1_ListRequest_serialize(::business_api::v1::ListRequest* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__business_api_v1_ListRequest_parse(::business_api::v1::ListRequest* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__business_api_v1_ListRequest_copy_from(::business_api::v1::ListRequest* dst, const ::business_api::v1::ListRequest* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__business_api_v1_ListRequest_repeated_len(google::protobuf::RepeatedPtrField<::business_api::v1::ListRequest>* field) {
  return field->size();
}
const ::business_api::v1::ListRequest& __rust_proto_thunk__business_api_v1_ListRequest_repeated_get(
  google::protobuf::RepeatedPtrField<::business_api::v1::ListRequest>* field,
  size_t index) {
  return field->Get(index);
}
::business_api::v1::ListRequest* __rust_proto_thunk__business_api_v1_ListRequest_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::business_api::v1::ListRequest>* field,
  size_t index) {
  return field->Mutable(index);
}
::business_api::v1::ListRequest* __rust_proto_thunk__business_api_v1_ListRequest_repeated_add(google::protobuf::RepeatedPtrField<::business_api::v1::ListRequest>* field) {
  return field->Add();
}
void __rust_proto_thunk__business_api_v1_ListRequest_repeated_clear(google::protobuf::RepeatedPtrField<::business_api::v1::ListRequest>* field) {
  field->Clear();
}
void __rust_proto_thunk__business_api_v1_ListRequest_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::business_api::v1::ListRequest>& dst,
  const google::protobuf::RepeatedPtrField<::business_api::v1::ListRequest>& src) {
  dst = src;
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::business_api::v1::ListRequest>* __rust_proto_thunk__Map_i32_business_api_v1_ListRequest_new() {
    return new google::protobuf::Map<int32_t, ::business_api::v1::ListRequest>();
  }
  void __rust_proto_thunk__Map_i32_business_api_v1_ListRequest_free(const google::protobuf::Map<int32_t, ::business_api::v1::ListRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_business_api_v1_ListRequest_clear(google::protobuf::Map<int32_t, ::business_api::v1::ListRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_business_api_v1_ListRequest_size(const google::protobuf::Map<int32_t, ::business_api::v1::ListRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_business_api_v1_ListRequest_insert(google::protobuf::Map<int32_t, ::business_api::v1::ListRequest> * m,
                          int32_t key, ::business_api::v1::ListRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_business_api_v1_ListRequest_get(const google::protobuf::Map<int32_t, ::business_api::v1::ListRequest>* m,
                       int32_t key, const ::business_api::v1::ListRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::ListRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_business_api_v1_ListRequest_remove(google::protobuf::Map<int32_t, ::business_api::v1::ListRequest> * m,
                          int32_t key, ::business_api::v1::ListRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_business_api_v1_ListRequest_iter(
      const google::protobuf::Map<int32_t, ::business_api::v1::ListRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_business_api_v1_ListRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::business_api::v1::ListRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::business_api::v1::ListRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::business_api::v1::ListRequest>* __rust_proto_thunk__Map_u32_business_api_v1_ListRequest_new() {
    return new google::protobuf::Map<uint32_t, ::business_api::v1::ListRequest>();
  }
  void __rust_proto_thunk__Map_u32_business_api_v1_ListRequest_free(const google::protobuf::Map<uint32_t, ::business_api::v1::ListRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_business_api_v1_ListRequest_clear(google::protobuf::Map<uint32_t, ::business_api::v1::ListRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_business_api_v1_ListRequest_size(const google::protobuf::Map<uint32_t, ::business_api::v1::ListRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_business_api_v1_ListRequest_insert(google::protobuf::Map<uint32_t, ::business_api::v1::ListRequest> * m,
                          uint32_t key, ::business_api::v1::ListRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_business_api_v1_ListRequest_get(const google::protobuf::Map<uint32_t, ::business_api::v1::ListRequest>* m,
                       uint32_t key, const ::business_api::v1::ListRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::ListRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_business_api_v1_ListRequest_remove(google::protobuf::Map<uint32_t, ::business_api::v1::ListRequest> * m,
                          uint32_t key, ::business_api::v1::ListRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_business_api_v1_ListRequest_iter(
      const google::protobuf::Map<uint32_t, ::business_api::v1::ListRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_business_api_v1_ListRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::business_api::v1::ListRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::business_api::v1::ListRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::business_api::v1::ListRequest>* __rust_proto_thunk__Map_i64_business_api_v1_ListRequest_new() {
    return new google::protobuf::Map<int64_t, ::business_api::v1::ListRequest>();
  }
  void __rust_proto_thunk__Map_i64_business_api_v1_ListRequest_free(const google::protobuf::Map<int64_t, ::business_api::v1::ListRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_business_api_v1_ListRequest_clear(google::protobuf::Map<int64_t, ::business_api::v1::ListRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_business_api_v1_ListRequest_size(const google::protobuf::Map<int64_t, ::business_api::v1::ListRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_business_api_v1_ListRequest_insert(google::protobuf::Map<int64_t, ::business_api::v1::ListRequest> * m,
                          int64_t key, ::business_api::v1::ListRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_business_api_v1_ListRequest_get(const google::protobuf::Map<int64_t, ::business_api::v1::ListRequest>* m,
                       int64_t key, const ::business_api::v1::ListRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::ListRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_business_api_v1_ListRequest_remove(google::protobuf::Map<int64_t, ::business_api::v1::ListRequest> * m,
                          int64_t key, ::business_api::v1::ListRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_business_api_v1_ListRequest_iter(
      const google::protobuf::Map<int64_t, ::business_api::v1::ListRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_business_api_v1_ListRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::business_api::v1::ListRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::business_api::v1::ListRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::business_api::v1::ListRequest>* __rust_proto_thunk__Map_u64_business_api_v1_ListRequest_new() {
    return new google::protobuf::Map<uint64_t, ::business_api::v1::ListRequest>();
  }
  void __rust_proto_thunk__Map_u64_business_api_v1_ListRequest_free(const google::protobuf::Map<uint64_t, ::business_api::v1::ListRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_business_api_v1_ListRequest_clear(google::protobuf::Map<uint64_t, ::business_api::v1::ListRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_business_api_v1_ListRequest_size(const google::protobuf::Map<uint64_t, ::business_api::v1::ListRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_business_api_v1_ListRequest_insert(google::protobuf::Map<uint64_t, ::business_api::v1::ListRequest> * m,
                          uint64_t key, ::business_api::v1::ListRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_business_api_v1_ListRequest_get(const google::protobuf::Map<uint64_t, ::business_api::v1::ListRequest>* m,
                       uint64_t key, const ::business_api::v1::ListRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::ListRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_business_api_v1_ListRequest_remove(google::protobuf::Map<uint64_t, ::business_api::v1::ListRequest> * m,
                          uint64_t key, ::business_api::v1::ListRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_business_api_v1_ListRequest_iter(
      const google::protobuf::Map<uint64_t, ::business_api::v1::ListRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_business_api_v1_ListRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::business_api::v1::ListRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::business_api::v1::ListRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::business_api::v1::ListRequest>* __rust_proto_thunk__Map_bool_business_api_v1_ListRequest_new() {
    return new google::protobuf::Map<bool, ::business_api::v1::ListRequest>();
  }
  void __rust_proto_thunk__Map_bool_business_api_v1_ListRequest_free(const google::protobuf::Map<bool, ::business_api::v1::ListRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_business_api_v1_ListRequest_clear(google::protobuf::Map<bool, ::business_api::v1::ListRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_business_api_v1_ListRequest_size(const google::protobuf::Map<bool, ::business_api::v1::ListRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_business_api_v1_ListRequest_insert(google::protobuf::Map<bool, ::business_api::v1::ListRequest> * m,
                          bool key, ::business_api::v1::ListRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_business_api_v1_ListRequest_get(const google::protobuf::Map<bool, ::business_api::v1::ListRequest>* m,
                       bool key, const ::business_api::v1::ListRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::ListRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_business_api_v1_ListRequest_remove(google::protobuf::Map<bool, ::business_api::v1::ListRequest> * m,
                          bool key, ::business_api::v1::ListRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_business_api_v1_ListRequest_iter(
      const google::protobuf::Map<bool, ::business_api::v1::ListRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_business_api_v1_ListRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::business_api::v1::ListRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::business_api::v1::ListRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::business_api::v1::ListRequest>* __rust_proto_thunk__Map_ProtoStr_business_api_v1_ListRequest_new() {
    return new google::protobuf::Map<std::string, ::business_api::v1::ListRequest>();
  }
  void __rust_proto_thunk__Map_ProtoStr_business_api_v1_ListRequest_free(const google::protobuf::Map<std::string, ::business_api::v1::ListRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_business_api_v1_ListRequest_clear(google::protobuf::Map<std::string, ::business_api::v1::ListRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_business_api_v1_ListRequest_size(const google::protobuf::Map<std::string, ::business_api::v1::ListRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_business_api_v1_ListRequest_insert(google::protobuf::Map<std::string, ::business_api::v1::ListRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::business_api::v1::ListRequest value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_business_api_v1_ListRequest_get(const google::protobuf::Map<std::string, ::business_api::v1::ListRequest>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::business_api::v1::ListRequest** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::ListRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_business_api_v1_ListRequest_remove(google::protobuf::Map<std::string, ::business_api::v1::ListRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::business_api::v1::ListRequest * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_business_api_v1_ListRequest_iter(
      const google::protobuf::Map<std::string, ::business_api::v1::ListRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_business_api_v1_ListRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::business_api::v1::ListRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::business_api::v1::ListRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// business_api.v1.ListResponse
        // clang-format off
extern "C" {
void* __rust_proto_thunk__business_api_v1_ListResponse_new() { return new ::business_api::v1::ListResponse(); }
void __rust_proto_thunk__business_api_v1_ListResponse_delete(void* ptr) { delete static_cast<::business_api::v1::ListResponse*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__business_api_v1_ListResponse_serialize(::business_api::v1::ListResponse* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__business_api_v1_ListResponse_parse(::business_api::v1::ListResponse* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__business_api_v1_ListResponse_copy_from(::business_api::v1::ListResponse* dst, const ::business_api::v1::ListResponse* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__business_api_v1_ListResponse_repeated_len(google::protobuf::RepeatedPtrField<::business_api::v1::ListResponse>* field) {
  return field->size();
}
const ::business_api::v1::ListResponse& __rust_proto_thunk__business_api_v1_ListResponse_repeated_get(
  google::protobuf::RepeatedPtrField<::business_api::v1::ListResponse>* field,
  size_t index) {
  return field->Get(index);
}
::business_api::v1::ListResponse* __rust_proto_thunk__business_api_v1_ListResponse_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::business_api::v1::ListResponse>* field,
  size_t index) {
  return field->Mutable(index);
}
::business_api::v1::ListResponse* __rust_proto_thunk__business_api_v1_ListResponse_repeated_add(google::protobuf::RepeatedPtrField<::business_api::v1::ListResponse>* field) {
  return field->Add();
}
void __rust_proto_thunk__business_api_v1_ListResponse_repeated_clear(google::protobuf::RepeatedPtrField<::business_api::v1::ListResponse>* field) {
  field->Clear();
}
void __rust_proto_thunk__business_api_v1_ListResponse_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::business_api::v1::ListResponse>& dst,
  const google::protobuf::RepeatedPtrField<::business_api::v1::ListResponse>& src) {
  dst = src;
}

void __rust_proto_thunk__business_api_v1_ListResponse_clear_list(::business_api::v1::ListResponse* msg) {
  msg->clear_list();
}
google::protobuf::RepeatedPtrField<std::string>* __rust_proto_thunk__business_api_v1_ListResponse_get_mut_list(::business_api::v1::ListResponse* msg) {
  return msg->mutable_list();
}
const google::protobuf::RepeatedPtrField<std::string>* __rust_proto_thunk__business_api_v1_ListResponse_get_list(
    const ::business_api::v1::ListResponse* msg) {
  return &msg->list();
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::business_api::v1::ListResponse>* __rust_proto_thunk__Map_i32_business_api_v1_ListResponse_new() {
    return new google::protobuf::Map<int32_t, ::business_api::v1::ListResponse>();
  }
  void __rust_proto_thunk__Map_i32_business_api_v1_ListResponse_free(const google::protobuf::Map<int32_t, ::business_api::v1::ListResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_business_api_v1_ListResponse_clear(google::protobuf::Map<int32_t, ::business_api::v1::ListResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_business_api_v1_ListResponse_size(const google::protobuf::Map<int32_t, ::business_api::v1::ListResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_business_api_v1_ListResponse_insert(google::protobuf::Map<int32_t, ::business_api::v1::ListResponse> * m,
                          int32_t key, ::business_api::v1::ListResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_business_api_v1_ListResponse_get(const google::protobuf::Map<int32_t, ::business_api::v1::ListResponse>* m,
                       int32_t key, const ::business_api::v1::ListResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::ListResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_business_api_v1_ListResponse_remove(google::protobuf::Map<int32_t, ::business_api::v1::ListResponse> * m,
                          int32_t key, ::business_api::v1::ListResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_business_api_v1_ListResponse_iter(
      const google::protobuf::Map<int32_t, ::business_api::v1::ListResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_business_api_v1_ListResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::business_api::v1::ListResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::business_api::v1::ListResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::business_api::v1::ListResponse>* __rust_proto_thunk__Map_u32_business_api_v1_ListResponse_new() {
    return new google::protobuf::Map<uint32_t, ::business_api::v1::ListResponse>();
  }
  void __rust_proto_thunk__Map_u32_business_api_v1_ListResponse_free(const google::protobuf::Map<uint32_t, ::business_api::v1::ListResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_business_api_v1_ListResponse_clear(google::protobuf::Map<uint32_t, ::business_api::v1::ListResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_business_api_v1_ListResponse_size(const google::protobuf::Map<uint32_t, ::business_api::v1::ListResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_business_api_v1_ListResponse_insert(google::protobuf::Map<uint32_t, ::business_api::v1::ListResponse> * m,
                          uint32_t key, ::business_api::v1::ListResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_business_api_v1_ListResponse_get(const google::protobuf::Map<uint32_t, ::business_api::v1::ListResponse>* m,
                       uint32_t key, const ::business_api::v1::ListResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::ListResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_business_api_v1_ListResponse_remove(google::protobuf::Map<uint32_t, ::business_api::v1::ListResponse> * m,
                          uint32_t key, ::business_api::v1::ListResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_business_api_v1_ListResponse_iter(
      const google::protobuf::Map<uint32_t, ::business_api::v1::ListResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_business_api_v1_ListResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::business_api::v1::ListResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::business_api::v1::ListResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::business_api::v1::ListResponse>* __rust_proto_thunk__Map_i64_business_api_v1_ListResponse_new() {
    return new google::protobuf::Map<int64_t, ::business_api::v1::ListResponse>();
  }
  void __rust_proto_thunk__Map_i64_business_api_v1_ListResponse_free(const google::protobuf::Map<int64_t, ::business_api::v1::ListResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_business_api_v1_ListResponse_clear(google::protobuf::Map<int64_t, ::business_api::v1::ListResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_business_api_v1_ListResponse_size(const google::protobuf::Map<int64_t, ::business_api::v1::ListResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_business_api_v1_ListResponse_insert(google::protobuf::Map<int64_t, ::business_api::v1::ListResponse> * m,
                          int64_t key, ::business_api::v1::ListResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_business_api_v1_ListResponse_get(const google::protobuf::Map<int64_t, ::business_api::v1::ListResponse>* m,
                       int64_t key, const ::business_api::v1::ListResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::ListResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_business_api_v1_ListResponse_remove(google::protobuf::Map<int64_t, ::business_api::v1::ListResponse> * m,
                          int64_t key, ::business_api::v1::ListResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_business_api_v1_ListResponse_iter(
      const google::protobuf::Map<int64_t, ::business_api::v1::ListResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_business_api_v1_ListResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::business_api::v1::ListResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::business_api::v1::ListResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::business_api::v1::ListResponse>* __rust_proto_thunk__Map_u64_business_api_v1_ListResponse_new() {
    return new google::protobuf::Map<uint64_t, ::business_api::v1::ListResponse>();
  }
  void __rust_proto_thunk__Map_u64_business_api_v1_ListResponse_free(const google::protobuf::Map<uint64_t, ::business_api::v1::ListResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_business_api_v1_ListResponse_clear(google::protobuf::Map<uint64_t, ::business_api::v1::ListResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_business_api_v1_ListResponse_size(const google::protobuf::Map<uint64_t, ::business_api::v1::ListResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_business_api_v1_ListResponse_insert(google::protobuf::Map<uint64_t, ::business_api::v1::ListResponse> * m,
                          uint64_t key, ::business_api::v1::ListResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_business_api_v1_ListResponse_get(const google::protobuf::Map<uint64_t, ::business_api::v1::ListResponse>* m,
                       uint64_t key, const ::business_api::v1::ListResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::ListResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_business_api_v1_ListResponse_remove(google::protobuf::Map<uint64_t, ::business_api::v1::ListResponse> * m,
                          uint64_t key, ::business_api::v1::ListResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_business_api_v1_ListResponse_iter(
      const google::protobuf::Map<uint64_t, ::business_api::v1::ListResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_business_api_v1_ListResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::business_api::v1::ListResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::business_api::v1::ListResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::business_api::v1::ListResponse>* __rust_proto_thunk__Map_bool_business_api_v1_ListResponse_new() {
    return new google::protobuf::Map<bool, ::business_api::v1::ListResponse>();
  }
  void __rust_proto_thunk__Map_bool_business_api_v1_ListResponse_free(const google::protobuf::Map<bool, ::business_api::v1::ListResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_business_api_v1_ListResponse_clear(google::protobuf::Map<bool, ::business_api::v1::ListResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_business_api_v1_ListResponse_size(const google::protobuf::Map<bool, ::business_api::v1::ListResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_business_api_v1_ListResponse_insert(google::protobuf::Map<bool, ::business_api::v1::ListResponse> * m,
                          bool key, ::business_api::v1::ListResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_business_api_v1_ListResponse_get(const google::protobuf::Map<bool, ::business_api::v1::ListResponse>* m,
                       bool key, const ::business_api::v1::ListResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::ListResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_business_api_v1_ListResponse_remove(google::protobuf::Map<bool, ::business_api::v1::ListResponse> * m,
                          bool key, ::business_api::v1::ListResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_business_api_v1_ListResponse_iter(
      const google::protobuf::Map<bool, ::business_api::v1::ListResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_business_api_v1_ListResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::business_api::v1::ListResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::business_api::v1::ListResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::business_api::v1::ListResponse>* __rust_proto_thunk__Map_ProtoStr_business_api_v1_ListResponse_new() {
    return new google::protobuf::Map<std::string, ::business_api::v1::ListResponse>();
  }
  void __rust_proto_thunk__Map_ProtoStr_business_api_v1_ListResponse_free(const google::protobuf::Map<std::string, ::business_api::v1::ListResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_business_api_v1_ListResponse_clear(google::protobuf::Map<std::string, ::business_api::v1::ListResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_business_api_v1_ListResponse_size(const google::protobuf::Map<std::string, ::business_api::v1::ListResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_business_api_v1_ListResponse_insert(google::protobuf::Map<std::string, ::business_api::v1::ListResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::business_api::v1::ListResponse value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_business_api_v1_ListResponse_get(const google::protobuf::Map<std::string, ::business_api::v1::ListResponse>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::business_api::v1::ListResponse** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::ListResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_business_api_v1_ListResponse_remove(google::protobuf::Map<std::string, ::business_api::v1::ListResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::business_api::v1::ListResponse * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_business_api_v1_ListResponse_iter(
      const google::protobuf::Map<std::string, ::business_api::v1::ListResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_business_api_v1_ListResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::business_api::v1::ListResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::business_api::v1::ListResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// business_api.v1.ClearRequest
        // clang-format off
extern "C" {
void* __rust_proto_thunk__business_api_v1_ClearRequest_new() { return new ::business_api::v1::ClearRequest(); }
void __rust_proto_thunk__business_api_v1_ClearRequest_delete(void* ptr) { delete static_cast<::business_api::v1::ClearRequest*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__business_api_v1_ClearRequest_serialize(::business_api::v1::ClearRequest* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__business_api_v1_ClearRequest_parse(::business_api::v1::ClearRequest* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__business_api_v1_ClearRequest_copy_from(::business_api::v1::ClearRequest* dst, const ::business_api::v1::ClearRequest* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__business_api_v1_ClearRequest_repeated_len(google::protobuf::RepeatedPtrField<::business_api::v1::ClearRequest>* field) {
  return field->size();
}
const ::business_api::v1::ClearRequest& __rust_proto_thunk__business_api_v1_ClearRequest_repeated_get(
  google::protobuf::RepeatedPtrField<::business_api::v1::ClearRequest>* field,
  size_t index) {
  return field->Get(index);
}
::business_api::v1::ClearRequest* __rust_proto_thunk__business_api_v1_ClearRequest_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::business_api::v1::ClearRequest>* field,
  size_t index) {
  return field->Mutable(index);
}
::business_api::v1::ClearRequest* __rust_proto_thunk__business_api_v1_ClearRequest_repeated_add(google::protobuf::RepeatedPtrField<::business_api::v1::ClearRequest>* field) {
  return field->Add();
}
void __rust_proto_thunk__business_api_v1_ClearRequest_repeated_clear(google::protobuf::RepeatedPtrField<::business_api::v1::ClearRequest>* field) {
  field->Clear();
}
void __rust_proto_thunk__business_api_v1_ClearRequest_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::business_api::v1::ClearRequest>& dst,
  const google::protobuf::RepeatedPtrField<::business_api::v1::ClearRequest>& src) {
  dst = src;
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::business_api::v1::ClearRequest>* __rust_proto_thunk__Map_i32_business_api_v1_ClearRequest_new() {
    return new google::protobuf::Map<int32_t, ::business_api::v1::ClearRequest>();
  }
  void __rust_proto_thunk__Map_i32_business_api_v1_ClearRequest_free(const google::protobuf::Map<int32_t, ::business_api::v1::ClearRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_business_api_v1_ClearRequest_clear(google::protobuf::Map<int32_t, ::business_api::v1::ClearRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_business_api_v1_ClearRequest_size(const google::protobuf::Map<int32_t, ::business_api::v1::ClearRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_business_api_v1_ClearRequest_insert(google::protobuf::Map<int32_t, ::business_api::v1::ClearRequest> * m,
                          int32_t key, ::business_api::v1::ClearRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_business_api_v1_ClearRequest_get(const google::protobuf::Map<int32_t, ::business_api::v1::ClearRequest>* m,
                       int32_t key, const ::business_api::v1::ClearRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::ClearRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_business_api_v1_ClearRequest_remove(google::protobuf::Map<int32_t, ::business_api::v1::ClearRequest> * m,
                          int32_t key, ::business_api::v1::ClearRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_business_api_v1_ClearRequest_iter(
      const google::protobuf::Map<int32_t, ::business_api::v1::ClearRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_business_api_v1_ClearRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::business_api::v1::ClearRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::business_api::v1::ClearRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::business_api::v1::ClearRequest>* __rust_proto_thunk__Map_u32_business_api_v1_ClearRequest_new() {
    return new google::protobuf::Map<uint32_t, ::business_api::v1::ClearRequest>();
  }
  void __rust_proto_thunk__Map_u32_business_api_v1_ClearRequest_free(const google::protobuf::Map<uint32_t, ::business_api::v1::ClearRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_business_api_v1_ClearRequest_clear(google::protobuf::Map<uint32_t, ::business_api::v1::ClearRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_business_api_v1_ClearRequest_size(const google::protobuf::Map<uint32_t, ::business_api::v1::ClearRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_business_api_v1_ClearRequest_insert(google::protobuf::Map<uint32_t, ::business_api::v1::ClearRequest> * m,
                          uint32_t key, ::business_api::v1::ClearRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_business_api_v1_ClearRequest_get(const google::protobuf::Map<uint32_t, ::business_api::v1::ClearRequest>* m,
                       uint32_t key, const ::business_api::v1::ClearRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::ClearRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_business_api_v1_ClearRequest_remove(google::protobuf::Map<uint32_t, ::business_api::v1::ClearRequest> * m,
                          uint32_t key, ::business_api::v1::ClearRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_business_api_v1_ClearRequest_iter(
      const google::protobuf::Map<uint32_t, ::business_api::v1::ClearRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_business_api_v1_ClearRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::business_api::v1::ClearRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::business_api::v1::ClearRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::business_api::v1::ClearRequest>* __rust_proto_thunk__Map_i64_business_api_v1_ClearRequest_new() {
    return new google::protobuf::Map<int64_t, ::business_api::v1::ClearRequest>();
  }
  void __rust_proto_thunk__Map_i64_business_api_v1_ClearRequest_free(const google::protobuf::Map<int64_t, ::business_api::v1::ClearRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_business_api_v1_ClearRequest_clear(google::protobuf::Map<int64_t, ::business_api::v1::ClearRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_business_api_v1_ClearRequest_size(const google::protobuf::Map<int64_t, ::business_api::v1::ClearRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_business_api_v1_ClearRequest_insert(google::protobuf::Map<int64_t, ::business_api::v1::ClearRequest> * m,
                          int64_t key, ::business_api::v1::ClearRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_business_api_v1_ClearRequest_get(const google::protobuf::Map<int64_t, ::business_api::v1::ClearRequest>* m,
                       int64_t key, const ::business_api::v1::ClearRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::ClearRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_business_api_v1_ClearRequest_remove(google::protobuf::Map<int64_t, ::business_api::v1::ClearRequest> * m,
                          int64_t key, ::business_api::v1::ClearRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_business_api_v1_ClearRequest_iter(
      const google::protobuf::Map<int64_t, ::business_api::v1::ClearRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_business_api_v1_ClearRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::business_api::v1::ClearRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::business_api::v1::ClearRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::business_api::v1::ClearRequest>* __rust_proto_thunk__Map_u64_business_api_v1_ClearRequest_new() {
    return new google::protobuf::Map<uint64_t, ::business_api::v1::ClearRequest>();
  }
  void __rust_proto_thunk__Map_u64_business_api_v1_ClearRequest_free(const google::protobuf::Map<uint64_t, ::business_api::v1::ClearRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_business_api_v1_ClearRequest_clear(google::protobuf::Map<uint64_t, ::business_api::v1::ClearRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_business_api_v1_ClearRequest_size(const google::protobuf::Map<uint64_t, ::business_api::v1::ClearRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_business_api_v1_ClearRequest_insert(google::protobuf::Map<uint64_t, ::business_api::v1::ClearRequest> * m,
                          uint64_t key, ::business_api::v1::ClearRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_business_api_v1_ClearRequest_get(const google::protobuf::Map<uint64_t, ::business_api::v1::ClearRequest>* m,
                       uint64_t key, const ::business_api::v1::ClearRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::ClearRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_business_api_v1_ClearRequest_remove(google::protobuf::Map<uint64_t, ::business_api::v1::ClearRequest> * m,
                          uint64_t key, ::business_api::v1::ClearRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_business_api_v1_ClearRequest_iter(
      const google::protobuf::Map<uint64_t, ::business_api::v1::ClearRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_business_api_v1_ClearRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::business_api::v1::ClearRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::business_api::v1::ClearRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::business_api::v1::ClearRequest>* __rust_proto_thunk__Map_bool_business_api_v1_ClearRequest_new() {
    return new google::protobuf::Map<bool, ::business_api::v1::ClearRequest>();
  }
  void __rust_proto_thunk__Map_bool_business_api_v1_ClearRequest_free(const google::protobuf::Map<bool, ::business_api::v1::ClearRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_business_api_v1_ClearRequest_clear(google::protobuf::Map<bool, ::business_api::v1::ClearRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_business_api_v1_ClearRequest_size(const google::protobuf::Map<bool, ::business_api::v1::ClearRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_business_api_v1_ClearRequest_insert(google::protobuf::Map<bool, ::business_api::v1::ClearRequest> * m,
                          bool key, ::business_api::v1::ClearRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_business_api_v1_ClearRequest_get(const google::protobuf::Map<bool, ::business_api::v1::ClearRequest>* m,
                       bool key, const ::business_api::v1::ClearRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::ClearRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_business_api_v1_ClearRequest_remove(google::protobuf::Map<bool, ::business_api::v1::ClearRequest> * m,
                          bool key, ::business_api::v1::ClearRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_business_api_v1_ClearRequest_iter(
      const google::protobuf::Map<bool, ::business_api::v1::ClearRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_business_api_v1_ClearRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::business_api::v1::ClearRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::business_api::v1::ClearRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::business_api::v1::ClearRequest>* __rust_proto_thunk__Map_ProtoStr_business_api_v1_ClearRequest_new() {
    return new google::protobuf::Map<std::string, ::business_api::v1::ClearRequest>();
  }
  void __rust_proto_thunk__Map_ProtoStr_business_api_v1_ClearRequest_free(const google::protobuf::Map<std::string, ::business_api::v1::ClearRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_business_api_v1_ClearRequest_clear(google::protobuf::Map<std::string, ::business_api::v1::ClearRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_business_api_v1_ClearRequest_size(const google::protobuf::Map<std::string, ::business_api::v1::ClearRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_business_api_v1_ClearRequest_insert(google::protobuf::Map<std::string, ::business_api::v1::ClearRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::business_api::v1::ClearRequest value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_business_api_v1_ClearRequest_get(const google::protobuf::Map<std::string, ::business_api::v1::ClearRequest>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::business_api::v1::ClearRequest** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::ClearRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_business_api_v1_ClearRequest_remove(google::protobuf::Map<std::string, ::business_api::v1::ClearRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::business_api::v1::ClearRequest * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_business_api_v1_ClearRequest_iter(
      const google::protobuf::Map<std::string, ::business_api::v1::ClearRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_business_api_v1_ClearRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::business_api::v1::ClearRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::business_api::v1::ClearRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// business_api.v1.ClearResponse
        // clang-format off
extern "C" {
void* __rust_proto_thunk__business_api_v1_ClearResponse_new() { return new ::business_api::v1::ClearResponse(); }
void __rust_proto_thunk__business_api_v1_ClearResponse_delete(void* ptr) { delete static_cast<::business_api::v1::ClearResponse*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__business_api_v1_ClearResponse_serialize(::business_api::v1::ClearResponse* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__business_api_v1_ClearResponse_parse(::business_api::v1::ClearResponse* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__business_api_v1_ClearResponse_copy_from(::business_api::v1::ClearResponse* dst, const ::business_api::v1::ClearResponse* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__business_api_v1_ClearResponse_repeated_len(google::protobuf::RepeatedPtrField<::business_api::v1::ClearResponse>* field) {
  return field->size();
}
const ::business_api::v1::ClearResponse& __rust_proto_thunk__business_api_v1_ClearResponse_repeated_get(
  google::protobuf::RepeatedPtrField<::business_api::v1::ClearResponse>* field,
  size_t index) {
  return field->Get(index);
}
::business_api::v1::ClearResponse* __rust_proto_thunk__business_api_v1_ClearResponse_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::business_api::v1::ClearResponse>* field,
  size_t index) {
  return field->Mutable(index);
}
::business_api::v1::ClearResponse* __rust_proto_thunk__business_api_v1_ClearResponse_repeated_add(google::protobuf::RepeatedPtrField<::business_api::v1::ClearResponse>* field) {
  return field->Add();
}
void __rust_proto_thunk__business_api_v1_ClearResponse_repeated_clear(google::protobuf::RepeatedPtrField<::business_api::v1::ClearResponse>* field) {
  field->Clear();
}
void __rust_proto_thunk__business_api_v1_ClearResponse_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::business_api::v1::ClearResponse>& dst,
  const google::protobuf::RepeatedPtrField<::business_api::v1::ClearResponse>& src) {
  dst = src;
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::business_api::v1::ClearResponse>* __rust_proto_thunk__Map_i32_business_api_v1_ClearResponse_new() {
    return new google::protobuf::Map<int32_t, ::business_api::v1::ClearResponse>();
  }
  void __rust_proto_thunk__Map_i32_business_api_v1_ClearResponse_free(const google::protobuf::Map<int32_t, ::business_api::v1::ClearResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_business_api_v1_ClearResponse_clear(google::protobuf::Map<int32_t, ::business_api::v1::ClearResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_business_api_v1_ClearResponse_size(const google::protobuf::Map<int32_t, ::business_api::v1::ClearResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_business_api_v1_ClearResponse_insert(google::protobuf::Map<int32_t, ::business_api::v1::ClearResponse> * m,
                          int32_t key, ::business_api::v1::ClearResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_business_api_v1_ClearResponse_get(const google::protobuf::Map<int32_t, ::business_api::v1::ClearResponse>* m,
                       int32_t key, const ::business_api::v1::ClearResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::ClearResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_business_api_v1_ClearResponse_remove(google::protobuf::Map<int32_t, ::business_api::v1::ClearResponse> * m,
                          int32_t key, ::business_api::v1::ClearResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_business_api_v1_ClearResponse_iter(
      const google::protobuf::Map<int32_t, ::business_api::v1::ClearResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_business_api_v1_ClearResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::business_api::v1::ClearResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::business_api::v1::ClearResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::business_api::v1::ClearResponse>* __rust_proto_thunk__Map_u32_business_api_v1_ClearResponse_new() {
    return new google::protobuf::Map<uint32_t, ::business_api::v1::ClearResponse>();
  }
  void __rust_proto_thunk__Map_u32_business_api_v1_ClearResponse_free(const google::protobuf::Map<uint32_t, ::business_api::v1::ClearResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_business_api_v1_ClearResponse_clear(google::protobuf::Map<uint32_t, ::business_api::v1::ClearResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_business_api_v1_ClearResponse_size(const google::protobuf::Map<uint32_t, ::business_api::v1::ClearResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_business_api_v1_ClearResponse_insert(google::protobuf::Map<uint32_t, ::business_api::v1::ClearResponse> * m,
                          uint32_t key, ::business_api::v1::ClearResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_business_api_v1_ClearResponse_get(const google::protobuf::Map<uint32_t, ::business_api::v1::ClearResponse>* m,
                       uint32_t key, const ::business_api::v1::ClearResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::ClearResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_business_api_v1_ClearResponse_remove(google::protobuf::Map<uint32_t, ::business_api::v1::ClearResponse> * m,
                          uint32_t key, ::business_api::v1::ClearResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_business_api_v1_ClearResponse_iter(
      const google::protobuf::Map<uint32_t, ::business_api::v1::ClearResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_business_api_v1_ClearResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::business_api::v1::ClearResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::business_api::v1::ClearResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::business_api::v1::ClearResponse>* __rust_proto_thunk__Map_i64_business_api_v1_ClearResponse_new() {
    return new google::protobuf::Map<int64_t, ::business_api::v1::ClearResponse>();
  }
  void __rust_proto_thunk__Map_i64_business_api_v1_ClearResponse_free(const google::protobuf::Map<int64_t, ::business_api::v1::ClearResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_business_api_v1_ClearResponse_clear(google::protobuf::Map<int64_t, ::business_api::v1::ClearResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_business_api_v1_ClearResponse_size(const google::protobuf::Map<int64_t, ::business_api::v1::ClearResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_business_api_v1_ClearResponse_insert(google::protobuf::Map<int64_t, ::business_api::v1::ClearResponse> * m,
                          int64_t key, ::business_api::v1::ClearResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_business_api_v1_ClearResponse_get(const google::protobuf::Map<int64_t, ::business_api::v1::ClearResponse>* m,
                       int64_t key, const ::business_api::v1::ClearResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::ClearResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_business_api_v1_ClearResponse_remove(google::protobuf::Map<int64_t, ::business_api::v1::ClearResponse> * m,
                          int64_t key, ::business_api::v1::ClearResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_business_api_v1_ClearResponse_iter(
      const google::protobuf::Map<int64_t, ::business_api::v1::ClearResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_business_api_v1_ClearResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::business_api::v1::ClearResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::business_api::v1::ClearResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::business_api::v1::ClearResponse>* __rust_proto_thunk__Map_u64_business_api_v1_ClearResponse_new() {
    return new google::protobuf::Map<uint64_t, ::business_api::v1::ClearResponse>();
  }
  void __rust_proto_thunk__Map_u64_business_api_v1_ClearResponse_free(const google::protobuf::Map<uint64_t, ::business_api::v1::ClearResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_business_api_v1_ClearResponse_clear(google::protobuf::Map<uint64_t, ::business_api::v1::ClearResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_business_api_v1_ClearResponse_size(const google::protobuf::Map<uint64_t, ::business_api::v1::ClearResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_business_api_v1_ClearResponse_insert(google::protobuf::Map<uint64_t, ::business_api::v1::ClearResponse> * m,
                          uint64_t key, ::business_api::v1::ClearResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_business_api_v1_ClearResponse_get(const google::protobuf::Map<uint64_t, ::business_api::v1::ClearResponse>* m,
                       uint64_t key, const ::business_api::v1::ClearResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::ClearResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_business_api_v1_ClearResponse_remove(google::protobuf::Map<uint64_t, ::business_api::v1::ClearResponse> * m,
                          uint64_t key, ::business_api::v1::ClearResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_business_api_v1_ClearResponse_iter(
      const google::protobuf::Map<uint64_t, ::business_api::v1::ClearResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_business_api_v1_ClearResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::business_api::v1::ClearResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::business_api::v1::ClearResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::business_api::v1::ClearResponse>* __rust_proto_thunk__Map_bool_business_api_v1_ClearResponse_new() {
    return new google::protobuf::Map<bool, ::business_api::v1::ClearResponse>();
  }
  void __rust_proto_thunk__Map_bool_business_api_v1_ClearResponse_free(const google::protobuf::Map<bool, ::business_api::v1::ClearResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_business_api_v1_ClearResponse_clear(google::protobuf::Map<bool, ::business_api::v1::ClearResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_business_api_v1_ClearResponse_size(const google::protobuf::Map<bool, ::business_api::v1::ClearResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_business_api_v1_ClearResponse_insert(google::protobuf::Map<bool, ::business_api::v1::ClearResponse> * m,
                          bool key, ::business_api::v1::ClearResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_business_api_v1_ClearResponse_get(const google::protobuf::Map<bool, ::business_api::v1::ClearResponse>* m,
                       bool key, const ::business_api::v1::ClearResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::ClearResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_business_api_v1_ClearResponse_remove(google::protobuf::Map<bool, ::business_api::v1::ClearResponse> * m,
                          bool key, ::business_api::v1::ClearResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_business_api_v1_ClearResponse_iter(
      const google::protobuf::Map<bool, ::business_api::v1::ClearResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_business_api_v1_ClearResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::business_api::v1::ClearResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::business_api::v1::ClearResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::business_api::v1::ClearResponse>* __rust_proto_thunk__Map_ProtoStr_business_api_v1_ClearResponse_new() {
    return new google::protobuf::Map<std::string, ::business_api::v1::ClearResponse>();
  }
  void __rust_proto_thunk__Map_ProtoStr_business_api_v1_ClearResponse_free(const google::protobuf::Map<std::string, ::business_api::v1::ClearResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_business_api_v1_ClearResponse_clear(google::protobuf::Map<std::string, ::business_api::v1::ClearResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_business_api_v1_ClearResponse_size(const google::protobuf::Map<std::string, ::business_api::v1::ClearResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_business_api_v1_ClearResponse_insert(google::protobuf::Map<std::string, ::business_api::v1::ClearResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::business_api::v1::ClearResponse value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_business_api_v1_ClearResponse_get(const google::protobuf::Map<std::string, ::business_api::v1::ClearResponse>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::business_api::v1::ClearResponse** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::ClearResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_business_api_v1_ClearResponse_remove(google::protobuf::Map<std::string, ::business_api::v1::ClearResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::business_api::v1::ClearResponse * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_business_api_v1_ClearResponse_iter(
      const google::protobuf::Map<std::string, ::business_api::v1::ClearResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_business_api_v1_ClearResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::business_api::v1::ClearResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::business_api::v1::ClearResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// business_api.v1.SendPayloadRequest
        // clang-format off
extern "C" {
void* __rust_proto_thunk__business_api_v1_SendPayloadRequest_new() { return new ::business_api::v1::SendPayloadRequest(); }
void __rust_proto_thunk__business_api_v1_SendPayloadRequest_delete(void* ptr) { delete static_cast<::business_api::v1::SendPayloadRequest*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__business_api_v1_SendPayloadRequest_serialize(::business_api::v1::SendPayloadRequest* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__business_api_v1_SendPayloadRequest_parse(::business_api::v1::SendPayloadRequest* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__business_api_v1_SendPayloadRequest_copy_from(::business_api::v1::SendPayloadRequest* dst, const ::business_api::v1::SendPayloadRequest* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__business_api_v1_SendPayloadRequest_repeated_len(google::protobuf::RepeatedPtrField<::business_api::v1::SendPayloadRequest>* field) {
  return field->size();
}
const ::business_api::v1::SendPayloadRequest& __rust_proto_thunk__business_api_v1_SendPayloadRequest_repeated_get(
  google::protobuf::RepeatedPtrField<::business_api::v1::SendPayloadRequest>* field,
  size_t index) {
  return field->Get(index);
}
::business_api::v1::SendPayloadRequest* __rust_proto_thunk__business_api_v1_SendPayloadRequest_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::business_api::v1::SendPayloadRequest>* field,
  size_t index) {
  return field->Mutable(index);
}
::business_api::v1::SendPayloadRequest* __rust_proto_thunk__business_api_v1_SendPayloadRequest_repeated_add(google::protobuf::RepeatedPtrField<::business_api::v1::SendPayloadRequest>* field) {
  return field->Add();
}
void __rust_proto_thunk__business_api_v1_SendPayloadRequest_repeated_clear(google::protobuf::RepeatedPtrField<::business_api::v1::SendPayloadRequest>* field) {
  field->Clear();
}
void __rust_proto_thunk__business_api_v1_SendPayloadRequest_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::business_api::v1::SendPayloadRequest>& dst,
  const google::protobuf::RepeatedPtrField<::business_api::v1::SendPayloadRequest>& src) {
  dst = src;
}

const void* __rust_proto_thunk__business_api_v1_SendPayloadRequest_get_payload(::business_api::v1::SendPayloadRequest* msg) {
  return static_cast<const void*>(&msg->payload());
}
void* __rust_proto_thunk__business_api_v1_SendPayloadRequest_get_mut_payload(::business_api::v1::SendPayloadRequest* msg) {
  return static_cast<void*>(msg->mutable_payload());
}
void __rust_proto_thunk__business_api_v1_SendPayloadRequest_clear_payload(::business_api::v1::SendPayloadRequest* msg) { msg->clear_payload(); }
bool __rust_proto_thunk__business_api_v1_SendPayloadRequest_has_payload(::business_api::v1::SendPayloadRequest* msg) { return msg->has_payload(); }


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::business_api::v1::SendPayloadRequest>* __rust_proto_thunk__Map_i32_business_api_v1_SendPayloadRequest_new() {
    return new google::protobuf::Map<int32_t, ::business_api::v1::SendPayloadRequest>();
  }
  void __rust_proto_thunk__Map_i32_business_api_v1_SendPayloadRequest_free(const google::protobuf::Map<int32_t, ::business_api::v1::SendPayloadRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_business_api_v1_SendPayloadRequest_clear(google::protobuf::Map<int32_t, ::business_api::v1::SendPayloadRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_business_api_v1_SendPayloadRequest_size(const google::protobuf::Map<int32_t, ::business_api::v1::SendPayloadRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_business_api_v1_SendPayloadRequest_insert(google::protobuf::Map<int32_t, ::business_api::v1::SendPayloadRequest> * m,
                          int32_t key, ::business_api::v1::SendPayloadRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_business_api_v1_SendPayloadRequest_get(const google::protobuf::Map<int32_t, ::business_api::v1::SendPayloadRequest>* m,
                       int32_t key, const ::business_api::v1::SendPayloadRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::SendPayloadRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_business_api_v1_SendPayloadRequest_remove(google::protobuf::Map<int32_t, ::business_api::v1::SendPayloadRequest> * m,
                          int32_t key, ::business_api::v1::SendPayloadRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_business_api_v1_SendPayloadRequest_iter(
      const google::protobuf::Map<int32_t, ::business_api::v1::SendPayloadRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_business_api_v1_SendPayloadRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::business_api::v1::SendPayloadRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::business_api::v1::SendPayloadRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::business_api::v1::SendPayloadRequest>* __rust_proto_thunk__Map_u32_business_api_v1_SendPayloadRequest_new() {
    return new google::protobuf::Map<uint32_t, ::business_api::v1::SendPayloadRequest>();
  }
  void __rust_proto_thunk__Map_u32_business_api_v1_SendPayloadRequest_free(const google::protobuf::Map<uint32_t, ::business_api::v1::SendPayloadRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_business_api_v1_SendPayloadRequest_clear(google::protobuf::Map<uint32_t, ::business_api::v1::SendPayloadRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_business_api_v1_SendPayloadRequest_size(const google::protobuf::Map<uint32_t, ::business_api::v1::SendPayloadRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_business_api_v1_SendPayloadRequest_insert(google::protobuf::Map<uint32_t, ::business_api::v1::SendPayloadRequest> * m,
                          uint32_t key, ::business_api::v1::SendPayloadRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_business_api_v1_SendPayloadRequest_get(const google::protobuf::Map<uint32_t, ::business_api::v1::SendPayloadRequest>* m,
                       uint32_t key, const ::business_api::v1::SendPayloadRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::SendPayloadRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_business_api_v1_SendPayloadRequest_remove(google::protobuf::Map<uint32_t, ::business_api::v1::SendPayloadRequest> * m,
                          uint32_t key, ::business_api::v1::SendPayloadRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_business_api_v1_SendPayloadRequest_iter(
      const google::protobuf::Map<uint32_t, ::business_api::v1::SendPayloadRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_business_api_v1_SendPayloadRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::business_api::v1::SendPayloadRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::business_api::v1::SendPayloadRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::business_api::v1::SendPayloadRequest>* __rust_proto_thunk__Map_i64_business_api_v1_SendPayloadRequest_new() {
    return new google::protobuf::Map<int64_t, ::business_api::v1::SendPayloadRequest>();
  }
  void __rust_proto_thunk__Map_i64_business_api_v1_SendPayloadRequest_free(const google::protobuf::Map<int64_t, ::business_api::v1::SendPayloadRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_business_api_v1_SendPayloadRequest_clear(google::protobuf::Map<int64_t, ::business_api::v1::SendPayloadRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_business_api_v1_SendPayloadRequest_size(const google::protobuf::Map<int64_t, ::business_api::v1::SendPayloadRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_business_api_v1_SendPayloadRequest_insert(google::protobuf::Map<int64_t, ::business_api::v1::SendPayloadRequest> * m,
                          int64_t key, ::business_api::v1::SendPayloadRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_business_api_v1_SendPayloadRequest_get(const google::protobuf::Map<int64_t, ::business_api::v1::SendPayloadRequest>* m,
                       int64_t key, const ::business_api::v1::SendPayloadRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::SendPayloadRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_business_api_v1_SendPayloadRequest_remove(google::protobuf::Map<int64_t, ::business_api::v1::SendPayloadRequest> * m,
                          int64_t key, ::business_api::v1::SendPayloadRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_business_api_v1_SendPayloadRequest_iter(
      const google::protobuf::Map<int64_t, ::business_api::v1::SendPayloadRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_business_api_v1_SendPayloadRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::business_api::v1::SendPayloadRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::business_api::v1::SendPayloadRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::business_api::v1::SendPayloadRequest>* __rust_proto_thunk__Map_u64_business_api_v1_SendPayloadRequest_new() {
    return new google::protobuf::Map<uint64_t, ::business_api::v1::SendPayloadRequest>();
  }
  void __rust_proto_thunk__Map_u64_business_api_v1_SendPayloadRequest_free(const google::protobuf::Map<uint64_t, ::business_api::v1::SendPayloadRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_business_api_v1_SendPayloadRequest_clear(google::protobuf::Map<uint64_t, ::business_api::v1::SendPayloadRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_business_api_v1_SendPayloadRequest_size(const google::protobuf::Map<uint64_t, ::business_api::v1::SendPayloadRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_business_api_v1_SendPayloadRequest_insert(google::protobuf::Map<uint64_t, ::business_api::v1::SendPayloadRequest> * m,
                          uint64_t key, ::business_api::v1::SendPayloadRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_business_api_v1_SendPayloadRequest_get(const google::protobuf::Map<uint64_t, ::business_api::v1::SendPayloadRequest>* m,
                       uint64_t key, const ::business_api::v1::SendPayloadRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::SendPayloadRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_business_api_v1_SendPayloadRequest_remove(google::protobuf::Map<uint64_t, ::business_api::v1::SendPayloadRequest> * m,
                          uint64_t key, ::business_api::v1::SendPayloadRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_business_api_v1_SendPayloadRequest_iter(
      const google::protobuf::Map<uint64_t, ::business_api::v1::SendPayloadRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_business_api_v1_SendPayloadRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::business_api::v1::SendPayloadRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::business_api::v1::SendPayloadRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::business_api::v1::SendPayloadRequest>* __rust_proto_thunk__Map_bool_business_api_v1_SendPayloadRequest_new() {
    return new google::protobuf::Map<bool, ::business_api::v1::SendPayloadRequest>();
  }
  void __rust_proto_thunk__Map_bool_business_api_v1_SendPayloadRequest_free(const google::protobuf::Map<bool, ::business_api::v1::SendPayloadRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_business_api_v1_SendPayloadRequest_clear(google::protobuf::Map<bool, ::business_api::v1::SendPayloadRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_business_api_v1_SendPayloadRequest_size(const google::protobuf::Map<bool, ::business_api::v1::SendPayloadRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_business_api_v1_SendPayloadRequest_insert(google::protobuf::Map<bool, ::business_api::v1::SendPayloadRequest> * m,
                          bool key, ::business_api::v1::SendPayloadRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_business_api_v1_SendPayloadRequest_get(const google::protobuf::Map<bool, ::business_api::v1::SendPayloadRequest>* m,
                       bool key, const ::business_api::v1::SendPayloadRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::SendPayloadRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_business_api_v1_SendPayloadRequest_remove(google::protobuf::Map<bool, ::business_api::v1::SendPayloadRequest> * m,
                          bool key, ::business_api::v1::SendPayloadRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_business_api_v1_SendPayloadRequest_iter(
      const google::protobuf::Map<bool, ::business_api::v1::SendPayloadRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_business_api_v1_SendPayloadRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::business_api::v1::SendPayloadRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::business_api::v1::SendPayloadRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::business_api::v1::SendPayloadRequest>* __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendPayloadRequest_new() {
    return new google::protobuf::Map<std::string, ::business_api::v1::SendPayloadRequest>();
  }
  void __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendPayloadRequest_free(const google::protobuf::Map<std::string, ::business_api::v1::SendPayloadRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendPayloadRequest_clear(google::protobuf::Map<std::string, ::business_api::v1::SendPayloadRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendPayloadRequest_size(const google::protobuf::Map<std::string, ::business_api::v1::SendPayloadRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendPayloadRequest_insert(google::protobuf::Map<std::string, ::business_api::v1::SendPayloadRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::business_api::v1::SendPayloadRequest value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendPayloadRequest_get(const google::protobuf::Map<std::string, ::business_api::v1::SendPayloadRequest>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::business_api::v1::SendPayloadRequest** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::SendPayloadRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendPayloadRequest_remove(google::protobuf::Map<std::string, ::business_api::v1::SendPayloadRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::business_api::v1::SendPayloadRequest * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendPayloadRequest_iter(
      const google::protobuf::Map<std::string, ::business_api::v1::SendPayloadRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendPayloadRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::business_api::v1::SendPayloadRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::business_api::v1::SendPayloadRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// business_api.v1.SendPayloadResponse
        // clang-format off
extern "C" {
void* __rust_proto_thunk__business_api_v1_SendPayloadResponse_new() { return new ::business_api::v1::SendPayloadResponse(); }
void __rust_proto_thunk__business_api_v1_SendPayloadResponse_delete(void* ptr) { delete static_cast<::business_api::v1::SendPayloadResponse*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__business_api_v1_SendPayloadResponse_serialize(::business_api::v1::SendPayloadResponse* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__business_api_v1_SendPayloadResponse_parse(::business_api::v1::SendPayloadResponse* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__business_api_v1_SendPayloadResponse_copy_from(::business_api::v1::SendPayloadResponse* dst, const ::business_api::v1::SendPayloadResponse* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__business_api_v1_SendPayloadResponse_repeated_len(google::protobuf::RepeatedPtrField<::business_api::v1::SendPayloadResponse>* field) {
  return field->size();
}
const ::business_api::v1::SendPayloadResponse& __rust_proto_thunk__business_api_v1_SendPayloadResponse_repeated_get(
  google::protobuf::RepeatedPtrField<::business_api::v1::SendPayloadResponse>* field,
  size_t index) {
  return field->Get(index);
}
::business_api::v1::SendPayloadResponse* __rust_proto_thunk__business_api_v1_SendPayloadResponse_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::business_api::v1::SendPayloadResponse>* field,
  size_t index) {
  return field->Mutable(index);
}
::business_api::v1::SendPayloadResponse* __rust_proto_thunk__business_api_v1_SendPayloadResponse_repeated_add(google::protobuf::RepeatedPtrField<::business_api::v1::SendPayloadResponse>* field) {
  return field->Add();
}
void __rust_proto_thunk__business_api_v1_SendPayloadResponse_repeated_clear(google::protobuf::RepeatedPtrField<::business_api::v1::SendPayloadResponse>* field) {
  field->Clear();
}
void __rust_proto_thunk__business_api_v1_SendPayloadResponse_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::business_api::v1::SendPayloadResponse>& dst,
  const google::protobuf::RepeatedPtrField<::business_api::v1::SendPayloadResponse>& src) {
  dst = src;
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::business_api::v1::SendPayloadResponse>* __rust_proto_thunk__Map_i32_business_api_v1_SendPayloadResponse_new() {
    return new google::protobuf::Map<int32_t, ::business_api::v1::SendPayloadResponse>();
  }
  void __rust_proto_thunk__Map_i32_business_api_v1_SendPayloadResponse_free(const google::protobuf::Map<int32_t, ::business_api::v1::SendPayloadResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_business_api_v1_SendPayloadResponse_clear(google::protobuf::Map<int32_t, ::business_api::v1::SendPayloadResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_business_api_v1_SendPayloadResponse_size(const google::protobuf::Map<int32_t, ::business_api::v1::SendPayloadResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_business_api_v1_SendPayloadResponse_insert(google::protobuf::Map<int32_t, ::business_api::v1::SendPayloadResponse> * m,
                          int32_t key, ::business_api::v1::SendPayloadResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_business_api_v1_SendPayloadResponse_get(const google::protobuf::Map<int32_t, ::business_api::v1::SendPayloadResponse>* m,
                       int32_t key, const ::business_api::v1::SendPayloadResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::SendPayloadResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_business_api_v1_SendPayloadResponse_remove(google::protobuf::Map<int32_t, ::business_api::v1::SendPayloadResponse> * m,
                          int32_t key, ::business_api::v1::SendPayloadResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_business_api_v1_SendPayloadResponse_iter(
      const google::protobuf::Map<int32_t, ::business_api::v1::SendPayloadResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_business_api_v1_SendPayloadResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::business_api::v1::SendPayloadResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::business_api::v1::SendPayloadResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::business_api::v1::SendPayloadResponse>* __rust_proto_thunk__Map_u32_business_api_v1_SendPayloadResponse_new() {
    return new google::protobuf::Map<uint32_t, ::business_api::v1::SendPayloadResponse>();
  }
  void __rust_proto_thunk__Map_u32_business_api_v1_SendPayloadResponse_free(const google::protobuf::Map<uint32_t, ::business_api::v1::SendPayloadResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_business_api_v1_SendPayloadResponse_clear(google::protobuf::Map<uint32_t, ::business_api::v1::SendPayloadResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_business_api_v1_SendPayloadResponse_size(const google::protobuf::Map<uint32_t, ::business_api::v1::SendPayloadResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_business_api_v1_SendPayloadResponse_insert(google::protobuf::Map<uint32_t, ::business_api::v1::SendPayloadResponse> * m,
                          uint32_t key, ::business_api::v1::SendPayloadResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_business_api_v1_SendPayloadResponse_get(const google::protobuf::Map<uint32_t, ::business_api::v1::SendPayloadResponse>* m,
                       uint32_t key, const ::business_api::v1::SendPayloadResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::SendPayloadResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_business_api_v1_SendPayloadResponse_remove(google::protobuf::Map<uint32_t, ::business_api::v1::SendPayloadResponse> * m,
                          uint32_t key, ::business_api::v1::SendPayloadResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_business_api_v1_SendPayloadResponse_iter(
      const google::protobuf::Map<uint32_t, ::business_api::v1::SendPayloadResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_business_api_v1_SendPayloadResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::business_api::v1::SendPayloadResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::business_api::v1::SendPayloadResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::business_api::v1::SendPayloadResponse>* __rust_proto_thunk__Map_i64_business_api_v1_SendPayloadResponse_new() {
    return new google::protobuf::Map<int64_t, ::business_api::v1::SendPayloadResponse>();
  }
  void __rust_proto_thunk__Map_i64_business_api_v1_SendPayloadResponse_free(const google::protobuf::Map<int64_t, ::business_api::v1::SendPayloadResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_business_api_v1_SendPayloadResponse_clear(google::protobuf::Map<int64_t, ::business_api::v1::SendPayloadResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_business_api_v1_SendPayloadResponse_size(const google::protobuf::Map<int64_t, ::business_api::v1::SendPayloadResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_business_api_v1_SendPayloadResponse_insert(google::protobuf::Map<int64_t, ::business_api::v1::SendPayloadResponse> * m,
                          int64_t key, ::business_api::v1::SendPayloadResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_business_api_v1_SendPayloadResponse_get(const google::protobuf::Map<int64_t, ::business_api::v1::SendPayloadResponse>* m,
                       int64_t key, const ::business_api::v1::SendPayloadResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::SendPayloadResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_business_api_v1_SendPayloadResponse_remove(google::protobuf::Map<int64_t, ::business_api::v1::SendPayloadResponse> * m,
                          int64_t key, ::business_api::v1::SendPayloadResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_business_api_v1_SendPayloadResponse_iter(
      const google::protobuf::Map<int64_t, ::business_api::v1::SendPayloadResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_business_api_v1_SendPayloadResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::business_api::v1::SendPayloadResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::business_api::v1::SendPayloadResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::business_api::v1::SendPayloadResponse>* __rust_proto_thunk__Map_u64_business_api_v1_SendPayloadResponse_new() {
    return new google::protobuf::Map<uint64_t, ::business_api::v1::SendPayloadResponse>();
  }
  void __rust_proto_thunk__Map_u64_business_api_v1_SendPayloadResponse_free(const google::protobuf::Map<uint64_t, ::business_api::v1::SendPayloadResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_business_api_v1_SendPayloadResponse_clear(google::protobuf::Map<uint64_t, ::business_api::v1::SendPayloadResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_business_api_v1_SendPayloadResponse_size(const google::protobuf::Map<uint64_t, ::business_api::v1::SendPayloadResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_business_api_v1_SendPayloadResponse_insert(google::protobuf::Map<uint64_t, ::business_api::v1::SendPayloadResponse> * m,
                          uint64_t key, ::business_api::v1::SendPayloadResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_business_api_v1_SendPayloadResponse_get(const google::protobuf::Map<uint64_t, ::business_api::v1::SendPayloadResponse>* m,
                       uint64_t key, const ::business_api::v1::SendPayloadResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::SendPayloadResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_business_api_v1_SendPayloadResponse_remove(google::protobuf::Map<uint64_t, ::business_api::v1::SendPayloadResponse> * m,
                          uint64_t key, ::business_api::v1::SendPayloadResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_business_api_v1_SendPayloadResponse_iter(
      const google::protobuf::Map<uint64_t, ::business_api::v1::SendPayloadResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_business_api_v1_SendPayloadResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::business_api::v1::SendPayloadResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::business_api::v1::SendPayloadResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::business_api::v1::SendPayloadResponse>* __rust_proto_thunk__Map_bool_business_api_v1_SendPayloadResponse_new() {
    return new google::protobuf::Map<bool, ::business_api::v1::SendPayloadResponse>();
  }
  void __rust_proto_thunk__Map_bool_business_api_v1_SendPayloadResponse_free(const google::protobuf::Map<bool, ::business_api::v1::SendPayloadResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_business_api_v1_SendPayloadResponse_clear(google::protobuf::Map<bool, ::business_api::v1::SendPayloadResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_business_api_v1_SendPayloadResponse_size(const google::protobuf::Map<bool, ::business_api::v1::SendPayloadResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_business_api_v1_SendPayloadResponse_insert(google::protobuf::Map<bool, ::business_api::v1::SendPayloadResponse> * m,
                          bool key, ::business_api::v1::SendPayloadResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_business_api_v1_SendPayloadResponse_get(const google::protobuf::Map<bool, ::business_api::v1::SendPayloadResponse>* m,
                       bool key, const ::business_api::v1::SendPayloadResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::SendPayloadResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_business_api_v1_SendPayloadResponse_remove(google::protobuf::Map<bool, ::business_api::v1::SendPayloadResponse> * m,
                          bool key, ::business_api::v1::SendPayloadResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_business_api_v1_SendPayloadResponse_iter(
      const google::protobuf::Map<bool, ::business_api::v1::SendPayloadResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_business_api_v1_SendPayloadResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::business_api::v1::SendPayloadResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::business_api::v1::SendPayloadResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::business_api::v1::SendPayloadResponse>* __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendPayloadResponse_new() {
    return new google::protobuf::Map<std::string, ::business_api::v1::SendPayloadResponse>();
  }
  void __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendPayloadResponse_free(const google::protobuf::Map<std::string, ::business_api::v1::SendPayloadResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendPayloadResponse_clear(google::protobuf::Map<std::string, ::business_api::v1::SendPayloadResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendPayloadResponse_size(const google::protobuf::Map<std::string, ::business_api::v1::SendPayloadResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendPayloadResponse_insert(google::protobuf::Map<std::string, ::business_api::v1::SendPayloadResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::business_api::v1::SendPayloadResponse value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendPayloadResponse_get(const google::protobuf::Map<std::string, ::business_api::v1::SendPayloadResponse>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::business_api::v1::SendPayloadResponse** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::SendPayloadResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendPayloadResponse_remove(google::protobuf::Map<std::string, ::business_api::v1::SendPayloadResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::business_api::v1::SendPayloadResponse * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendPayloadResponse_iter(
      const google::protobuf::Map<std::string, ::business_api::v1::SendPayloadResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendPayloadResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::business_api::v1::SendPayloadResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::business_api::v1::SendPayloadResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// business_api.v1.SendNewTransactionRequest
        // clang-format off
extern "C" {
void* __rust_proto_thunk__business_api_v1_SendNewTransactionRequest_new() { return new ::business_api::v1::SendNewTransactionRequest(); }
void __rust_proto_thunk__business_api_v1_SendNewTransactionRequest_delete(void* ptr) { delete static_cast<::business_api::v1::SendNewTransactionRequest*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__business_api_v1_SendNewTransactionRequest_serialize(::business_api::v1::SendNewTransactionRequest* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__business_api_v1_SendNewTransactionRequest_parse(::business_api::v1::SendNewTransactionRequest* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__business_api_v1_SendNewTransactionRequest_copy_from(::business_api::v1::SendNewTransactionRequest* dst, const ::business_api::v1::SendNewTransactionRequest* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__business_api_v1_SendNewTransactionRequest_repeated_len(google::protobuf::RepeatedPtrField<::business_api::v1::SendNewTransactionRequest>* field) {
  return field->size();
}
const ::business_api::v1::SendNewTransactionRequest& __rust_proto_thunk__business_api_v1_SendNewTransactionRequest_repeated_get(
  google::protobuf::RepeatedPtrField<::business_api::v1::SendNewTransactionRequest>* field,
  size_t index) {
  return field->Get(index);
}
::business_api::v1::SendNewTransactionRequest* __rust_proto_thunk__business_api_v1_SendNewTransactionRequest_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::business_api::v1::SendNewTransactionRequest>* field,
  size_t index) {
  return field->Mutable(index);
}
::business_api::v1::SendNewTransactionRequest* __rust_proto_thunk__business_api_v1_SendNewTransactionRequest_repeated_add(google::protobuf::RepeatedPtrField<::business_api::v1::SendNewTransactionRequest>* field) {
  return field->Add();
}
void __rust_proto_thunk__business_api_v1_SendNewTransactionRequest_repeated_clear(google::protobuf::RepeatedPtrField<::business_api::v1::SendNewTransactionRequest>* field) {
  field->Clear();
}
void __rust_proto_thunk__business_api_v1_SendNewTransactionRequest_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::business_api::v1::SendNewTransactionRequest>& dst,
  const google::protobuf::RepeatedPtrField<::business_api::v1::SendNewTransactionRequest>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__business_api_v1_SendNewTransactionRequest_get_participant(::business_api::v1::SendNewTransactionRequest* msg) {
  absl::string_view val = msg->participant();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__business_api_v1_SendNewTransactionRequest_set_participant(::business_api::v1::SendNewTransactionRequest* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_participant(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__business_api_v1_SendNewTransactionRequest_get_memo(::business_api::v1::SendNewTransactionRequest* msg) {
  absl::string_view val = msg->memo();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__business_api_v1_SendNewTransactionRequest_set_memo(::business_api::v1::SendNewTransactionRequest* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_memo(absl::string_view(s.ptr, s.len));
}

void __rust_proto_thunk__business_api_v1_SendNewTransactionRequest_clear_commitments(::business_api::v1::SendNewTransactionRequest* msg) {
  msg->clear_commitments();
}
google::protobuf::RepeatedPtrField<::common::Commitment>* __rust_proto_thunk__business_api_v1_SendNewTransactionRequest_get_mut_commitments(::business_api::v1::SendNewTransactionRequest* msg) {
  return msg->mutable_commitments();
}
const google::protobuf::RepeatedPtrField<::common::Commitment>* __rust_proto_thunk__business_api_v1_SendNewTransactionRequest_get_commitments(
    const ::business_api::v1::SendNewTransactionRequest* msg) {
  return &msg->commitments();
}

void __rust_proto_thunk__business_api_v1_SendNewTransactionRequest_clear_conditions(::business_api::v1::SendNewTransactionRequest* msg) {
  msg->clear_conditions();
}
google::protobuf::RepeatedPtrField<::common::Condition>* __rust_proto_thunk__business_api_v1_SendNewTransactionRequest_get_mut_conditions(::business_api::v1::SendNewTransactionRequest* msg) {
  return msg->mutable_conditions();
}
const google::protobuf::RepeatedPtrField<::common::Condition>* __rust_proto_thunk__business_api_v1_SendNewTransactionRequest_get_conditions(
    const ::business_api::v1::SendNewTransactionRequest* msg) {
  return &msg->conditions();
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::business_api::v1::SendNewTransactionRequest>* __rust_proto_thunk__Map_i32_business_api_v1_SendNewTransactionRequest_new() {
    return new google::protobuf::Map<int32_t, ::business_api::v1::SendNewTransactionRequest>();
  }
  void __rust_proto_thunk__Map_i32_business_api_v1_SendNewTransactionRequest_free(const google::protobuf::Map<int32_t, ::business_api::v1::SendNewTransactionRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_business_api_v1_SendNewTransactionRequest_clear(google::protobuf::Map<int32_t, ::business_api::v1::SendNewTransactionRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_business_api_v1_SendNewTransactionRequest_size(const google::protobuf::Map<int32_t, ::business_api::v1::SendNewTransactionRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_business_api_v1_SendNewTransactionRequest_insert(google::protobuf::Map<int32_t, ::business_api::v1::SendNewTransactionRequest> * m,
                          int32_t key, ::business_api::v1::SendNewTransactionRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_business_api_v1_SendNewTransactionRequest_get(const google::protobuf::Map<int32_t, ::business_api::v1::SendNewTransactionRequest>* m,
                       int32_t key, const ::business_api::v1::SendNewTransactionRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::SendNewTransactionRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_business_api_v1_SendNewTransactionRequest_remove(google::protobuf::Map<int32_t, ::business_api::v1::SendNewTransactionRequest> * m,
                          int32_t key, ::business_api::v1::SendNewTransactionRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_business_api_v1_SendNewTransactionRequest_iter(
      const google::protobuf::Map<int32_t, ::business_api::v1::SendNewTransactionRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_business_api_v1_SendNewTransactionRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::business_api::v1::SendNewTransactionRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::business_api::v1::SendNewTransactionRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::business_api::v1::SendNewTransactionRequest>* __rust_proto_thunk__Map_u32_business_api_v1_SendNewTransactionRequest_new() {
    return new google::protobuf::Map<uint32_t, ::business_api::v1::SendNewTransactionRequest>();
  }
  void __rust_proto_thunk__Map_u32_business_api_v1_SendNewTransactionRequest_free(const google::protobuf::Map<uint32_t, ::business_api::v1::SendNewTransactionRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_business_api_v1_SendNewTransactionRequest_clear(google::protobuf::Map<uint32_t, ::business_api::v1::SendNewTransactionRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_business_api_v1_SendNewTransactionRequest_size(const google::protobuf::Map<uint32_t, ::business_api::v1::SendNewTransactionRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_business_api_v1_SendNewTransactionRequest_insert(google::protobuf::Map<uint32_t, ::business_api::v1::SendNewTransactionRequest> * m,
                          uint32_t key, ::business_api::v1::SendNewTransactionRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_business_api_v1_SendNewTransactionRequest_get(const google::protobuf::Map<uint32_t, ::business_api::v1::SendNewTransactionRequest>* m,
                       uint32_t key, const ::business_api::v1::SendNewTransactionRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::SendNewTransactionRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_business_api_v1_SendNewTransactionRequest_remove(google::protobuf::Map<uint32_t, ::business_api::v1::SendNewTransactionRequest> * m,
                          uint32_t key, ::business_api::v1::SendNewTransactionRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_business_api_v1_SendNewTransactionRequest_iter(
      const google::protobuf::Map<uint32_t, ::business_api::v1::SendNewTransactionRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_business_api_v1_SendNewTransactionRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::business_api::v1::SendNewTransactionRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::business_api::v1::SendNewTransactionRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::business_api::v1::SendNewTransactionRequest>* __rust_proto_thunk__Map_i64_business_api_v1_SendNewTransactionRequest_new() {
    return new google::protobuf::Map<int64_t, ::business_api::v1::SendNewTransactionRequest>();
  }
  void __rust_proto_thunk__Map_i64_business_api_v1_SendNewTransactionRequest_free(const google::protobuf::Map<int64_t, ::business_api::v1::SendNewTransactionRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_business_api_v1_SendNewTransactionRequest_clear(google::protobuf::Map<int64_t, ::business_api::v1::SendNewTransactionRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_business_api_v1_SendNewTransactionRequest_size(const google::protobuf::Map<int64_t, ::business_api::v1::SendNewTransactionRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_business_api_v1_SendNewTransactionRequest_insert(google::protobuf::Map<int64_t, ::business_api::v1::SendNewTransactionRequest> * m,
                          int64_t key, ::business_api::v1::SendNewTransactionRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_business_api_v1_SendNewTransactionRequest_get(const google::protobuf::Map<int64_t, ::business_api::v1::SendNewTransactionRequest>* m,
                       int64_t key, const ::business_api::v1::SendNewTransactionRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::SendNewTransactionRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_business_api_v1_SendNewTransactionRequest_remove(google::protobuf::Map<int64_t, ::business_api::v1::SendNewTransactionRequest> * m,
                          int64_t key, ::business_api::v1::SendNewTransactionRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_business_api_v1_SendNewTransactionRequest_iter(
      const google::protobuf::Map<int64_t, ::business_api::v1::SendNewTransactionRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_business_api_v1_SendNewTransactionRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::business_api::v1::SendNewTransactionRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::business_api::v1::SendNewTransactionRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::business_api::v1::SendNewTransactionRequest>* __rust_proto_thunk__Map_u64_business_api_v1_SendNewTransactionRequest_new() {
    return new google::protobuf::Map<uint64_t, ::business_api::v1::SendNewTransactionRequest>();
  }
  void __rust_proto_thunk__Map_u64_business_api_v1_SendNewTransactionRequest_free(const google::protobuf::Map<uint64_t, ::business_api::v1::SendNewTransactionRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_business_api_v1_SendNewTransactionRequest_clear(google::protobuf::Map<uint64_t, ::business_api::v1::SendNewTransactionRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_business_api_v1_SendNewTransactionRequest_size(const google::protobuf::Map<uint64_t, ::business_api::v1::SendNewTransactionRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_business_api_v1_SendNewTransactionRequest_insert(google::protobuf::Map<uint64_t, ::business_api::v1::SendNewTransactionRequest> * m,
                          uint64_t key, ::business_api::v1::SendNewTransactionRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_business_api_v1_SendNewTransactionRequest_get(const google::protobuf::Map<uint64_t, ::business_api::v1::SendNewTransactionRequest>* m,
                       uint64_t key, const ::business_api::v1::SendNewTransactionRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::SendNewTransactionRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_business_api_v1_SendNewTransactionRequest_remove(google::protobuf::Map<uint64_t, ::business_api::v1::SendNewTransactionRequest> * m,
                          uint64_t key, ::business_api::v1::SendNewTransactionRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_business_api_v1_SendNewTransactionRequest_iter(
      const google::protobuf::Map<uint64_t, ::business_api::v1::SendNewTransactionRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_business_api_v1_SendNewTransactionRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::business_api::v1::SendNewTransactionRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::business_api::v1::SendNewTransactionRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::business_api::v1::SendNewTransactionRequest>* __rust_proto_thunk__Map_bool_business_api_v1_SendNewTransactionRequest_new() {
    return new google::protobuf::Map<bool, ::business_api::v1::SendNewTransactionRequest>();
  }
  void __rust_proto_thunk__Map_bool_business_api_v1_SendNewTransactionRequest_free(const google::protobuf::Map<bool, ::business_api::v1::SendNewTransactionRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_business_api_v1_SendNewTransactionRequest_clear(google::protobuf::Map<bool, ::business_api::v1::SendNewTransactionRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_business_api_v1_SendNewTransactionRequest_size(const google::protobuf::Map<bool, ::business_api::v1::SendNewTransactionRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_business_api_v1_SendNewTransactionRequest_insert(google::protobuf::Map<bool, ::business_api::v1::SendNewTransactionRequest> * m,
                          bool key, ::business_api::v1::SendNewTransactionRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_business_api_v1_SendNewTransactionRequest_get(const google::protobuf::Map<bool, ::business_api::v1::SendNewTransactionRequest>* m,
                       bool key, const ::business_api::v1::SendNewTransactionRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::SendNewTransactionRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_business_api_v1_SendNewTransactionRequest_remove(google::protobuf::Map<bool, ::business_api::v1::SendNewTransactionRequest> * m,
                          bool key, ::business_api::v1::SendNewTransactionRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_business_api_v1_SendNewTransactionRequest_iter(
      const google::protobuf::Map<bool, ::business_api::v1::SendNewTransactionRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_business_api_v1_SendNewTransactionRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::business_api::v1::SendNewTransactionRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::business_api::v1::SendNewTransactionRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::business_api::v1::SendNewTransactionRequest>* __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendNewTransactionRequest_new() {
    return new google::protobuf::Map<std::string, ::business_api::v1::SendNewTransactionRequest>();
  }
  void __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendNewTransactionRequest_free(const google::protobuf::Map<std::string, ::business_api::v1::SendNewTransactionRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendNewTransactionRequest_clear(google::protobuf::Map<std::string, ::business_api::v1::SendNewTransactionRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendNewTransactionRequest_size(const google::protobuf::Map<std::string, ::business_api::v1::SendNewTransactionRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendNewTransactionRequest_insert(google::protobuf::Map<std::string, ::business_api::v1::SendNewTransactionRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::business_api::v1::SendNewTransactionRequest value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendNewTransactionRequest_get(const google::protobuf::Map<std::string, ::business_api::v1::SendNewTransactionRequest>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::business_api::v1::SendNewTransactionRequest** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::SendNewTransactionRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendNewTransactionRequest_remove(google::protobuf::Map<std::string, ::business_api::v1::SendNewTransactionRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::business_api::v1::SendNewTransactionRequest * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendNewTransactionRequest_iter(
      const google::protobuf::Map<std::string, ::business_api::v1::SendNewTransactionRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendNewTransactionRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::business_api::v1::SendNewTransactionRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::business_api::v1::SendNewTransactionRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// business_api.v1.SendNewTransactionResponse
        // clang-format off
extern "C" {
void* __rust_proto_thunk__business_api_v1_SendNewTransactionResponse_new() { return new ::business_api::v1::SendNewTransactionResponse(); }
void __rust_proto_thunk__business_api_v1_SendNewTransactionResponse_delete(void* ptr) { delete static_cast<::business_api::v1::SendNewTransactionResponse*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__business_api_v1_SendNewTransactionResponse_serialize(::business_api::v1::SendNewTransactionResponse* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__business_api_v1_SendNewTransactionResponse_parse(::business_api::v1::SendNewTransactionResponse* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__business_api_v1_SendNewTransactionResponse_copy_from(::business_api::v1::SendNewTransactionResponse* dst, const ::business_api::v1::SendNewTransactionResponse* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__business_api_v1_SendNewTransactionResponse_repeated_len(google::protobuf::RepeatedPtrField<::business_api::v1::SendNewTransactionResponse>* field) {
  return field->size();
}
const ::business_api::v1::SendNewTransactionResponse& __rust_proto_thunk__business_api_v1_SendNewTransactionResponse_repeated_get(
  google::protobuf::RepeatedPtrField<::business_api::v1::SendNewTransactionResponse>* field,
  size_t index) {
  return field->Get(index);
}
::business_api::v1::SendNewTransactionResponse* __rust_proto_thunk__business_api_v1_SendNewTransactionResponse_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::business_api::v1::SendNewTransactionResponse>* field,
  size_t index) {
  return field->Mutable(index);
}
::business_api::v1::SendNewTransactionResponse* __rust_proto_thunk__business_api_v1_SendNewTransactionResponse_repeated_add(google::protobuf::RepeatedPtrField<::business_api::v1::SendNewTransactionResponse>* field) {
  return field->Add();
}
void __rust_proto_thunk__business_api_v1_SendNewTransactionResponse_repeated_clear(google::protobuf::RepeatedPtrField<::business_api::v1::SendNewTransactionResponse>* field) {
  field->Clear();
}
void __rust_proto_thunk__business_api_v1_SendNewTransactionResponse_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::business_api::v1::SendNewTransactionResponse>& dst,
  const google::protobuf::RepeatedPtrField<::business_api::v1::SendNewTransactionResponse>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__business_api_v1_SendNewTransactionResponse_get_contract_id(::business_api::v1::SendNewTransactionResponse* msg) {
  absl::string_view val = msg->contract_id();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__business_api_v1_SendNewTransactionResponse_set_contract_id(::business_api::v1::SendNewTransactionResponse* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_contract_id(absl::string_view(s.ptr, s.len));
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::business_api::v1::SendNewTransactionResponse>* __rust_proto_thunk__Map_i32_business_api_v1_SendNewTransactionResponse_new() {
    return new google::protobuf::Map<int32_t, ::business_api::v1::SendNewTransactionResponse>();
  }
  void __rust_proto_thunk__Map_i32_business_api_v1_SendNewTransactionResponse_free(const google::protobuf::Map<int32_t, ::business_api::v1::SendNewTransactionResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_business_api_v1_SendNewTransactionResponse_clear(google::protobuf::Map<int32_t, ::business_api::v1::SendNewTransactionResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_business_api_v1_SendNewTransactionResponse_size(const google::protobuf::Map<int32_t, ::business_api::v1::SendNewTransactionResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_business_api_v1_SendNewTransactionResponse_insert(google::protobuf::Map<int32_t, ::business_api::v1::SendNewTransactionResponse> * m,
                          int32_t key, ::business_api::v1::SendNewTransactionResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_business_api_v1_SendNewTransactionResponse_get(const google::protobuf::Map<int32_t, ::business_api::v1::SendNewTransactionResponse>* m,
                       int32_t key, const ::business_api::v1::SendNewTransactionResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::SendNewTransactionResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_business_api_v1_SendNewTransactionResponse_remove(google::protobuf::Map<int32_t, ::business_api::v1::SendNewTransactionResponse> * m,
                          int32_t key, ::business_api::v1::SendNewTransactionResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_business_api_v1_SendNewTransactionResponse_iter(
      const google::protobuf::Map<int32_t, ::business_api::v1::SendNewTransactionResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_business_api_v1_SendNewTransactionResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::business_api::v1::SendNewTransactionResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::business_api::v1::SendNewTransactionResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::business_api::v1::SendNewTransactionResponse>* __rust_proto_thunk__Map_u32_business_api_v1_SendNewTransactionResponse_new() {
    return new google::protobuf::Map<uint32_t, ::business_api::v1::SendNewTransactionResponse>();
  }
  void __rust_proto_thunk__Map_u32_business_api_v1_SendNewTransactionResponse_free(const google::protobuf::Map<uint32_t, ::business_api::v1::SendNewTransactionResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_business_api_v1_SendNewTransactionResponse_clear(google::protobuf::Map<uint32_t, ::business_api::v1::SendNewTransactionResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_business_api_v1_SendNewTransactionResponse_size(const google::protobuf::Map<uint32_t, ::business_api::v1::SendNewTransactionResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_business_api_v1_SendNewTransactionResponse_insert(google::protobuf::Map<uint32_t, ::business_api::v1::SendNewTransactionResponse> * m,
                          uint32_t key, ::business_api::v1::SendNewTransactionResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_business_api_v1_SendNewTransactionResponse_get(const google::protobuf::Map<uint32_t, ::business_api::v1::SendNewTransactionResponse>* m,
                       uint32_t key, const ::business_api::v1::SendNewTransactionResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::SendNewTransactionResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_business_api_v1_SendNewTransactionResponse_remove(google::protobuf::Map<uint32_t, ::business_api::v1::SendNewTransactionResponse> * m,
                          uint32_t key, ::business_api::v1::SendNewTransactionResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_business_api_v1_SendNewTransactionResponse_iter(
      const google::protobuf::Map<uint32_t, ::business_api::v1::SendNewTransactionResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_business_api_v1_SendNewTransactionResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::business_api::v1::SendNewTransactionResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::business_api::v1::SendNewTransactionResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::business_api::v1::SendNewTransactionResponse>* __rust_proto_thunk__Map_i64_business_api_v1_SendNewTransactionResponse_new() {
    return new google::protobuf::Map<int64_t, ::business_api::v1::SendNewTransactionResponse>();
  }
  void __rust_proto_thunk__Map_i64_business_api_v1_SendNewTransactionResponse_free(const google::protobuf::Map<int64_t, ::business_api::v1::SendNewTransactionResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_business_api_v1_SendNewTransactionResponse_clear(google::protobuf::Map<int64_t, ::business_api::v1::SendNewTransactionResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_business_api_v1_SendNewTransactionResponse_size(const google::protobuf::Map<int64_t, ::business_api::v1::SendNewTransactionResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_business_api_v1_SendNewTransactionResponse_insert(google::protobuf::Map<int64_t, ::business_api::v1::SendNewTransactionResponse> * m,
                          int64_t key, ::business_api::v1::SendNewTransactionResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_business_api_v1_SendNewTransactionResponse_get(const google::protobuf::Map<int64_t, ::business_api::v1::SendNewTransactionResponse>* m,
                       int64_t key, const ::business_api::v1::SendNewTransactionResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::SendNewTransactionResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_business_api_v1_SendNewTransactionResponse_remove(google::protobuf::Map<int64_t, ::business_api::v1::SendNewTransactionResponse> * m,
                          int64_t key, ::business_api::v1::SendNewTransactionResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_business_api_v1_SendNewTransactionResponse_iter(
      const google::protobuf::Map<int64_t, ::business_api::v1::SendNewTransactionResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_business_api_v1_SendNewTransactionResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::business_api::v1::SendNewTransactionResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::business_api::v1::SendNewTransactionResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::business_api::v1::SendNewTransactionResponse>* __rust_proto_thunk__Map_u64_business_api_v1_SendNewTransactionResponse_new() {
    return new google::protobuf::Map<uint64_t, ::business_api::v1::SendNewTransactionResponse>();
  }
  void __rust_proto_thunk__Map_u64_business_api_v1_SendNewTransactionResponse_free(const google::protobuf::Map<uint64_t, ::business_api::v1::SendNewTransactionResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_business_api_v1_SendNewTransactionResponse_clear(google::protobuf::Map<uint64_t, ::business_api::v1::SendNewTransactionResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_business_api_v1_SendNewTransactionResponse_size(const google::protobuf::Map<uint64_t, ::business_api::v1::SendNewTransactionResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_business_api_v1_SendNewTransactionResponse_insert(google::protobuf::Map<uint64_t, ::business_api::v1::SendNewTransactionResponse> * m,
                          uint64_t key, ::business_api::v1::SendNewTransactionResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_business_api_v1_SendNewTransactionResponse_get(const google::protobuf::Map<uint64_t, ::business_api::v1::SendNewTransactionResponse>* m,
                       uint64_t key, const ::business_api::v1::SendNewTransactionResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::SendNewTransactionResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_business_api_v1_SendNewTransactionResponse_remove(google::protobuf::Map<uint64_t, ::business_api::v1::SendNewTransactionResponse> * m,
                          uint64_t key, ::business_api::v1::SendNewTransactionResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_business_api_v1_SendNewTransactionResponse_iter(
      const google::protobuf::Map<uint64_t, ::business_api::v1::SendNewTransactionResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_business_api_v1_SendNewTransactionResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::business_api::v1::SendNewTransactionResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::business_api::v1::SendNewTransactionResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::business_api::v1::SendNewTransactionResponse>* __rust_proto_thunk__Map_bool_business_api_v1_SendNewTransactionResponse_new() {
    return new google::protobuf::Map<bool, ::business_api::v1::SendNewTransactionResponse>();
  }
  void __rust_proto_thunk__Map_bool_business_api_v1_SendNewTransactionResponse_free(const google::protobuf::Map<bool, ::business_api::v1::SendNewTransactionResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_business_api_v1_SendNewTransactionResponse_clear(google::protobuf::Map<bool, ::business_api::v1::SendNewTransactionResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_business_api_v1_SendNewTransactionResponse_size(const google::protobuf::Map<bool, ::business_api::v1::SendNewTransactionResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_business_api_v1_SendNewTransactionResponse_insert(google::protobuf::Map<bool, ::business_api::v1::SendNewTransactionResponse> * m,
                          bool key, ::business_api::v1::SendNewTransactionResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_business_api_v1_SendNewTransactionResponse_get(const google::protobuf::Map<bool, ::business_api::v1::SendNewTransactionResponse>* m,
                       bool key, const ::business_api::v1::SendNewTransactionResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::SendNewTransactionResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_business_api_v1_SendNewTransactionResponse_remove(google::protobuf::Map<bool, ::business_api::v1::SendNewTransactionResponse> * m,
                          bool key, ::business_api::v1::SendNewTransactionResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_business_api_v1_SendNewTransactionResponse_iter(
      const google::protobuf::Map<bool, ::business_api::v1::SendNewTransactionResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_business_api_v1_SendNewTransactionResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::business_api::v1::SendNewTransactionResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::business_api::v1::SendNewTransactionResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::business_api::v1::SendNewTransactionResponse>* __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendNewTransactionResponse_new() {
    return new google::protobuf::Map<std::string, ::business_api::v1::SendNewTransactionResponse>();
  }
  void __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendNewTransactionResponse_free(const google::protobuf::Map<std::string, ::business_api::v1::SendNewTransactionResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendNewTransactionResponse_clear(google::protobuf::Map<std::string, ::business_api::v1::SendNewTransactionResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendNewTransactionResponse_size(const google::protobuf::Map<std::string, ::business_api::v1::SendNewTransactionResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendNewTransactionResponse_insert(google::protobuf::Map<std::string, ::business_api::v1::SendNewTransactionResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::business_api::v1::SendNewTransactionResponse value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendNewTransactionResponse_get(const google::protobuf::Map<std::string, ::business_api::v1::SendNewTransactionResponse>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::business_api::v1::SendNewTransactionResponse** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::SendNewTransactionResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendNewTransactionResponse_remove(google::protobuf::Map<std::string, ::business_api::v1::SendNewTransactionResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::business_api::v1::SendNewTransactionResponse * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendNewTransactionResponse_iter(
      const google::protobuf::Map<std::string, ::business_api::v1::SendNewTransactionResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendNewTransactionResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::business_api::v1::SendNewTransactionResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::business_api::v1::SendNewTransactionResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// business_api.v1.SendProposalAckedRequest
        // clang-format off
extern "C" {
void* __rust_proto_thunk__business_api_v1_SendProposalAckedRequest_new() { return new ::business_api::v1::SendProposalAckedRequest(); }
void __rust_proto_thunk__business_api_v1_SendProposalAckedRequest_delete(void* ptr) { delete static_cast<::business_api::v1::SendProposalAckedRequest*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__business_api_v1_SendProposalAckedRequest_serialize(::business_api::v1::SendProposalAckedRequest* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__business_api_v1_SendProposalAckedRequest_parse(::business_api::v1::SendProposalAckedRequest* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__business_api_v1_SendProposalAckedRequest_copy_from(::business_api::v1::SendProposalAckedRequest* dst, const ::business_api::v1::SendProposalAckedRequest* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__business_api_v1_SendProposalAckedRequest_repeated_len(google::protobuf::RepeatedPtrField<::business_api::v1::SendProposalAckedRequest>* field) {
  return field->size();
}
const ::business_api::v1::SendProposalAckedRequest& __rust_proto_thunk__business_api_v1_SendProposalAckedRequest_repeated_get(
  google::protobuf::RepeatedPtrField<::business_api::v1::SendProposalAckedRequest>* field,
  size_t index) {
  return field->Get(index);
}
::business_api::v1::SendProposalAckedRequest* __rust_proto_thunk__business_api_v1_SendProposalAckedRequest_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::business_api::v1::SendProposalAckedRequest>* field,
  size_t index) {
  return field->Mutable(index);
}
::business_api::v1::SendProposalAckedRequest* __rust_proto_thunk__business_api_v1_SendProposalAckedRequest_repeated_add(google::protobuf::RepeatedPtrField<::business_api::v1::SendProposalAckedRequest>* field) {
  return field->Add();
}
void __rust_proto_thunk__business_api_v1_SendProposalAckedRequest_repeated_clear(google::protobuf::RepeatedPtrField<::business_api::v1::SendProposalAckedRequest>* field) {
  field->Clear();
}
void __rust_proto_thunk__business_api_v1_SendProposalAckedRequest_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::business_api::v1::SendProposalAckedRequest>& dst,
  const google::protobuf::RepeatedPtrField<::business_api::v1::SendProposalAckedRequest>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__business_api_v1_SendProposalAckedRequest_get_participant(::business_api::v1::SendProposalAckedRequest* msg) {
  absl::string_view val = msg->participant();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__business_api_v1_SendProposalAckedRequest_set_participant(::business_api::v1::SendProposalAckedRequest* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_participant(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__business_api_v1_SendProposalAckedRequest_get_contract_id(::business_api::v1::SendProposalAckedRequest* msg) {
  absl::string_view val = msg->contract_id();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__business_api_v1_SendProposalAckedRequest_set_contract_id(::business_api::v1::SendProposalAckedRequest* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_contract_id(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__business_api_v1_SendProposalAckedRequest_get_hash(::business_api::v1::SendProposalAckedRequest* msg) {
  absl::string_view val = msg->hash();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__business_api_v1_SendProposalAckedRequest_set_hash(::business_api::v1::SendProposalAckedRequest* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_hash(absl::string_view(s.ptr, s.len));
}

::common::HashAlgorithm __rust_proto_thunk__business_api_v1_SendProposalAckedRequest_get_algorithm(::business_api::v1::SendProposalAckedRequest* msg) { return msg->algorithm(); }
void __rust_proto_thunk__business_api_v1_SendProposalAckedRequest_set_algorithm(::business_api::v1::SendProposalAckedRequest* msg, ::common::HashAlgorithm val) {
  msg->set_algorithm(val);
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::business_api::v1::SendProposalAckedRequest>* __rust_proto_thunk__Map_i32_business_api_v1_SendProposalAckedRequest_new() {
    return new google::protobuf::Map<int32_t, ::business_api::v1::SendProposalAckedRequest>();
  }
  void __rust_proto_thunk__Map_i32_business_api_v1_SendProposalAckedRequest_free(const google::protobuf::Map<int32_t, ::business_api::v1::SendProposalAckedRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_business_api_v1_SendProposalAckedRequest_clear(google::protobuf::Map<int32_t, ::business_api::v1::SendProposalAckedRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_business_api_v1_SendProposalAckedRequest_size(const google::protobuf::Map<int32_t, ::business_api::v1::SendProposalAckedRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_business_api_v1_SendProposalAckedRequest_insert(google::protobuf::Map<int32_t, ::business_api::v1::SendProposalAckedRequest> * m,
                          int32_t key, ::business_api::v1::SendProposalAckedRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_business_api_v1_SendProposalAckedRequest_get(const google::protobuf::Map<int32_t, ::business_api::v1::SendProposalAckedRequest>* m,
                       int32_t key, const ::business_api::v1::SendProposalAckedRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::SendProposalAckedRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_business_api_v1_SendProposalAckedRequest_remove(google::protobuf::Map<int32_t, ::business_api::v1::SendProposalAckedRequest> * m,
                          int32_t key, ::business_api::v1::SendProposalAckedRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_business_api_v1_SendProposalAckedRequest_iter(
      const google::protobuf::Map<int32_t, ::business_api::v1::SendProposalAckedRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_business_api_v1_SendProposalAckedRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::business_api::v1::SendProposalAckedRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::business_api::v1::SendProposalAckedRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::business_api::v1::SendProposalAckedRequest>* __rust_proto_thunk__Map_u32_business_api_v1_SendProposalAckedRequest_new() {
    return new google::protobuf::Map<uint32_t, ::business_api::v1::SendProposalAckedRequest>();
  }
  void __rust_proto_thunk__Map_u32_business_api_v1_SendProposalAckedRequest_free(const google::protobuf::Map<uint32_t, ::business_api::v1::SendProposalAckedRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_business_api_v1_SendProposalAckedRequest_clear(google::protobuf::Map<uint32_t, ::business_api::v1::SendProposalAckedRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_business_api_v1_SendProposalAckedRequest_size(const google::protobuf::Map<uint32_t, ::business_api::v1::SendProposalAckedRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_business_api_v1_SendProposalAckedRequest_insert(google::protobuf::Map<uint32_t, ::business_api::v1::SendProposalAckedRequest> * m,
                          uint32_t key, ::business_api::v1::SendProposalAckedRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_business_api_v1_SendProposalAckedRequest_get(const google::protobuf::Map<uint32_t, ::business_api::v1::SendProposalAckedRequest>* m,
                       uint32_t key, const ::business_api::v1::SendProposalAckedRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::SendProposalAckedRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_business_api_v1_SendProposalAckedRequest_remove(google::protobuf::Map<uint32_t, ::business_api::v1::SendProposalAckedRequest> * m,
                          uint32_t key, ::business_api::v1::SendProposalAckedRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_business_api_v1_SendProposalAckedRequest_iter(
      const google::protobuf::Map<uint32_t, ::business_api::v1::SendProposalAckedRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_business_api_v1_SendProposalAckedRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::business_api::v1::SendProposalAckedRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::business_api::v1::SendProposalAckedRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::business_api::v1::SendProposalAckedRequest>* __rust_proto_thunk__Map_i64_business_api_v1_SendProposalAckedRequest_new() {
    return new google::protobuf::Map<int64_t, ::business_api::v1::SendProposalAckedRequest>();
  }
  void __rust_proto_thunk__Map_i64_business_api_v1_SendProposalAckedRequest_free(const google::protobuf::Map<int64_t, ::business_api::v1::SendProposalAckedRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_business_api_v1_SendProposalAckedRequest_clear(google::protobuf::Map<int64_t, ::business_api::v1::SendProposalAckedRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_business_api_v1_SendProposalAckedRequest_size(const google::protobuf::Map<int64_t, ::business_api::v1::SendProposalAckedRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_business_api_v1_SendProposalAckedRequest_insert(google::protobuf::Map<int64_t, ::business_api::v1::SendProposalAckedRequest> * m,
                          int64_t key, ::business_api::v1::SendProposalAckedRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_business_api_v1_SendProposalAckedRequest_get(const google::protobuf::Map<int64_t, ::business_api::v1::SendProposalAckedRequest>* m,
                       int64_t key, const ::business_api::v1::SendProposalAckedRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::SendProposalAckedRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_business_api_v1_SendProposalAckedRequest_remove(google::protobuf::Map<int64_t, ::business_api::v1::SendProposalAckedRequest> * m,
                          int64_t key, ::business_api::v1::SendProposalAckedRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_business_api_v1_SendProposalAckedRequest_iter(
      const google::protobuf::Map<int64_t, ::business_api::v1::SendProposalAckedRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_business_api_v1_SendProposalAckedRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::business_api::v1::SendProposalAckedRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::business_api::v1::SendProposalAckedRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::business_api::v1::SendProposalAckedRequest>* __rust_proto_thunk__Map_u64_business_api_v1_SendProposalAckedRequest_new() {
    return new google::protobuf::Map<uint64_t, ::business_api::v1::SendProposalAckedRequest>();
  }
  void __rust_proto_thunk__Map_u64_business_api_v1_SendProposalAckedRequest_free(const google::protobuf::Map<uint64_t, ::business_api::v1::SendProposalAckedRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_business_api_v1_SendProposalAckedRequest_clear(google::protobuf::Map<uint64_t, ::business_api::v1::SendProposalAckedRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_business_api_v1_SendProposalAckedRequest_size(const google::protobuf::Map<uint64_t, ::business_api::v1::SendProposalAckedRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_business_api_v1_SendProposalAckedRequest_insert(google::protobuf::Map<uint64_t, ::business_api::v1::SendProposalAckedRequest> * m,
                          uint64_t key, ::business_api::v1::SendProposalAckedRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_business_api_v1_SendProposalAckedRequest_get(const google::protobuf::Map<uint64_t, ::business_api::v1::SendProposalAckedRequest>* m,
                       uint64_t key, const ::business_api::v1::SendProposalAckedRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::SendProposalAckedRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_business_api_v1_SendProposalAckedRequest_remove(google::protobuf::Map<uint64_t, ::business_api::v1::SendProposalAckedRequest> * m,
                          uint64_t key, ::business_api::v1::SendProposalAckedRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_business_api_v1_SendProposalAckedRequest_iter(
      const google::protobuf::Map<uint64_t, ::business_api::v1::SendProposalAckedRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_business_api_v1_SendProposalAckedRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::business_api::v1::SendProposalAckedRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::business_api::v1::SendProposalAckedRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::business_api::v1::SendProposalAckedRequest>* __rust_proto_thunk__Map_bool_business_api_v1_SendProposalAckedRequest_new() {
    return new google::protobuf::Map<bool, ::business_api::v1::SendProposalAckedRequest>();
  }
  void __rust_proto_thunk__Map_bool_business_api_v1_SendProposalAckedRequest_free(const google::protobuf::Map<bool, ::business_api::v1::SendProposalAckedRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_business_api_v1_SendProposalAckedRequest_clear(google::protobuf::Map<bool, ::business_api::v1::SendProposalAckedRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_business_api_v1_SendProposalAckedRequest_size(const google::protobuf::Map<bool, ::business_api::v1::SendProposalAckedRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_business_api_v1_SendProposalAckedRequest_insert(google::protobuf::Map<bool, ::business_api::v1::SendProposalAckedRequest> * m,
                          bool key, ::business_api::v1::SendProposalAckedRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_business_api_v1_SendProposalAckedRequest_get(const google::protobuf::Map<bool, ::business_api::v1::SendProposalAckedRequest>* m,
                       bool key, const ::business_api::v1::SendProposalAckedRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::SendProposalAckedRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_business_api_v1_SendProposalAckedRequest_remove(google::protobuf::Map<bool, ::business_api::v1::SendProposalAckedRequest> * m,
                          bool key, ::business_api::v1::SendProposalAckedRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_business_api_v1_SendProposalAckedRequest_iter(
      const google::protobuf::Map<bool, ::business_api::v1::SendProposalAckedRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_business_api_v1_SendProposalAckedRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::business_api::v1::SendProposalAckedRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::business_api::v1::SendProposalAckedRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::business_api::v1::SendProposalAckedRequest>* __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendProposalAckedRequest_new() {
    return new google::protobuf::Map<std::string, ::business_api::v1::SendProposalAckedRequest>();
  }
  void __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendProposalAckedRequest_free(const google::protobuf::Map<std::string, ::business_api::v1::SendProposalAckedRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendProposalAckedRequest_clear(google::protobuf::Map<std::string, ::business_api::v1::SendProposalAckedRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendProposalAckedRequest_size(const google::protobuf::Map<std::string, ::business_api::v1::SendProposalAckedRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendProposalAckedRequest_insert(google::protobuf::Map<std::string, ::business_api::v1::SendProposalAckedRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::business_api::v1::SendProposalAckedRequest value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendProposalAckedRequest_get(const google::protobuf::Map<std::string, ::business_api::v1::SendProposalAckedRequest>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::business_api::v1::SendProposalAckedRequest** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::SendProposalAckedRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendProposalAckedRequest_remove(google::protobuf::Map<std::string, ::business_api::v1::SendProposalAckedRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::business_api::v1::SendProposalAckedRequest * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendProposalAckedRequest_iter(
      const google::protobuf::Map<std::string, ::business_api::v1::SendProposalAckedRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendProposalAckedRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::business_api::v1::SendProposalAckedRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::business_api::v1::SendProposalAckedRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// business_api.v1.SendProposalAckedResponse
        // clang-format off
extern "C" {
void* __rust_proto_thunk__business_api_v1_SendProposalAckedResponse_new() { return new ::business_api::v1::SendProposalAckedResponse(); }
void __rust_proto_thunk__business_api_v1_SendProposalAckedResponse_delete(void* ptr) { delete static_cast<::business_api::v1::SendProposalAckedResponse*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__business_api_v1_SendProposalAckedResponse_serialize(::business_api::v1::SendProposalAckedResponse* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__business_api_v1_SendProposalAckedResponse_parse(::business_api::v1::SendProposalAckedResponse* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__business_api_v1_SendProposalAckedResponse_copy_from(::business_api::v1::SendProposalAckedResponse* dst, const ::business_api::v1::SendProposalAckedResponse* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__business_api_v1_SendProposalAckedResponse_repeated_len(google::protobuf::RepeatedPtrField<::business_api::v1::SendProposalAckedResponse>* field) {
  return field->size();
}
const ::business_api::v1::SendProposalAckedResponse& __rust_proto_thunk__business_api_v1_SendProposalAckedResponse_repeated_get(
  google::protobuf::RepeatedPtrField<::business_api::v1::SendProposalAckedResponse>* field,
  size_t index) {
  return field->Get(index);
}
::business_api::v1::SendProposalAckedResponse* __rust_proto_thunk__business_api_v1_SendProposalAckedResponse_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::business_api::v1::SendProposalAckedResponse>* field,
  size_t index) {
  return field->Mutable(index);
}
::business_api::v1::SendProposalAckedResponse* __rust_proto_thunk__business_api_v1_SendProposalAckedResponse_repeated_add(google::protobuf::RepeatedPtrField<::business_api::v1::SendProposalAckedResponse>* field) {
  return field->Add();
}
void __rust_proto_thunk__business_api_v1_SendProposalAckedResponse_repeated_clear(google::protobuf::RepeatedPtrField<::business_api::v1::SendProposalAckedResponse>* field) {
  field->Clear();
}
void __rust_proto_thunk__business_api_v1_SendProposalAckedResponse_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::business_api::v1::SendProposalAckedResponse>& dst,
  const google::protobuf::RepeatedPtrField<::business_api::v1::SendProposalAckedResponse>& src) {
  dst = src;
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::business_api::v1::SendProposalAckedResponse>* __rust_proto_thunk__Map_i32_business_api_v1_SendProposalAckedResponse_new() {
    return new google::protobuf::Map<int32_t, ::business_api::v1::SendProposalAckedResponse>();
  }
  void __rust_proto_thunk__Map_i32_business_api_v1_SendProposalAckedResponse_free(const google::protobuf::Map<int32_t, ::business_api::v1::SendProposalAckedResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_business_api_v1_SendProposalAckedResponse_clear(google::protobuf::Map<int32_t, ::business_api::v1::SendProposalAckedResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_business_api_v1_SendProposalAckedResponse_size(const google::protobuf::Map<int32_t, ::business_api::v1::SendProposalAckedResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_business_api_v1_SendProposalAckedResponse_insert(google::protobuf::Map<int32_t, ::business_api::v1::SendProposalAckedResponse> * m,
                          int32_t key, ::business_api::v1::SendProposalAckedResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_business_api_v1_SendProposalAckedResponse_get(const google::protobuf::Map<int32_t, ::business_api::v1::SendProposalAckedResponse>* m,
                       int32_t key, const ::business_api::v1::SendProposalAckedResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::SendProposalAckedResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_business_api_v1_SendProposalAckedResponse_remove(google::protobuf::Map<int32_t, ::business_api::v1::SendProposalAckedResponse> * m,
                          int32_t key, ::business_api::v1::SendProposalAckedResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_business_api_v1_SendProposalAckedResponse_iter(
      const google::protobuf::Map<int32_t, ::business_api::v1::SendProposalAckedResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_business_api_v1_SendProposalAckedResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::business_api::v1::SendProposalAckedResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::business_api::v1::SendProposalAckedResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::business_api::v1::SendProposalAckedResponse>* __rust_proto_thunk__Map_u32_business_api_v1_SendProposalAckedResponse_new() {
    return new google::protobuf::Map<uint32_t, ::business_api::v1::SendProposalAckedResponse>();
  }
  void __rust_proto_thunk__Map_u32_business_api_v1_SendProposalAckedResponse_free(const google::protobuf::Map<uint32_t, ::business_api::v1::SendProposalAckedResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_business_api_v1_SendProposalAckedResponse_clear(google::protobuf::Map<uint32_t, ::business_api::v1::SendProposalAckedResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_business_api_v1_SendProposalAckedResponse_size(const google::protobuf::Map<uint32_t, ::business_api::v1::SendProposalAckedResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_business_api_v1_SendProposalAckedResponse_insert(google::protobuf::Map<uint32_t, ::business_api::v1::SendProposalAckedResponse> * m,
                          uint32_t key, ::business_api::v1::SendProposalAckedResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_business_api_v1_SendProposalAckedResponse_get(const google::protobuf::Map<uint32_t, ::business_api::v1::SendProposalAckedResponse>* m,
                       uint32_t key, const ::business_api::v1::SendProposalAckedResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::SendProposalAckedResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_business_api_v1_SendProposalAckedResponse_remove(google::protobuf::Map<uint32_t, ::business_api::v1::SendProposalAckedResponse> * m,
                          uint32_t key, ::business_api::v1::SendProposalAckedResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_business_api_v1_SendProposalAckedResponse_iter(
      const google::protobuf::Map<uint32_t, ::business_api::v1::SendProposalAckedResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_business_api_v1_SendProposalAckedResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::business_api::v1::SendProposalAckedResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::business_api::v1::SendProposalAckedResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::business_api::v1::SendProposalAckedResponse>* __rust_proto_thunk__Map_i64_business_api_v1_SendProposalAckedResponse_new() {
    return new google::protobuf::Map<int64_t, ::business_api::v1::SendProposalAckedResponse>();
  }
  void __rust_proto_thunk__Map_i64_business_api_v1_SendProposalAckedResponse_free(const google::protobuf::Map<int64_t, ::business_api::v1::SendProposalAckedResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_business_api_v1_SendProposalAckedResponse_clear(google::protobuf::Map<int64_t, ::business_api::v1::SendProposalAckedResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_business_api_v1_SendProposalAckedResponse_size(const google::protobuf::Map<int64_t, ::business_api::v1::SendProposalAckedResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_business_api_v1_SendProposalAckedResponse_insert(google::protobuf::Map<int64_t, ::business_api::v1::SendProposalAckedResponse> * m,
                          int64_t key, ::business_api::v1::SendProposalAckedResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_business_api_v1_SendProposalAckedResponse_get(const google::protobuf::Map<int64_t, ::business_api::v1::SendProposalAckedResponse>* m,
                       int64_t key, const ::business_api::v1::SendProposalAckedResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::SendProposalAckedResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_business_api_v1_SendProposalAckedResponse_remove(google::protobuf::Map<int64_t, ::business_api::v1::SendProposalAckedResponse> * m,
                          int64_t key, ::business_api::v1::SendProposalAckedResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_business_api_v1_SendProposalAckedResponse_iter(
      const google::protobuf::Map<int64_t, ::business_api::v1::SendProposalAckedResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_business_api_v1_SendProposalAckedResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::business_api::v1::SendProposalAckedResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::business_api::v1::SendProposalAckedResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::business_api::v1::SendProposalAckedResponse>* __rust_proto_thunk__Map_u64_business_api_v1_SendProposalAckedResponse_new() {
    return new google::protobuf::Map<uint64_t, ::business_api::v1::SendProposalAckedResponse>();
  }
  void __rust_proto_thunk__Map_u64_business_api_v1_SendProposalAckedResponse_free(const google::protobuf::Map<uint64_t, ::business_api::v1::SendProposalAckedResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_business_api_v1_SendProposalAckedResponse_clear(google::protobuf::Map<uint64_t, ::business_api::v1::SendProposalAckedResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_business_api_v1_SendProposalAckedResponse_size(const google::protobuf::Map<uint64_t, ::business_api::v1::SendProposalAckedResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_business_api_v1_SendProposalAckedResponse_insert(google::protobuf::Map<uint64_t, ::business_api::v1::SendProposalAckedResponse> * m,
                          uint64_t key, ::business_api::v1::SendProposalAckedResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_business_api_v1_SendProposalAckedResponse_get(const google::protobuf::Map<uint64_t, ::business_api::v1::SendProposalAckedResponse>* m,
                       uint64_t key, const ::business_api::v1::SendProposalAckedResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::SendProposalAckedResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_business_api_v1_SendProposalAckedResponse_remove(google::protobuf::Map<uint64_t, ::business_api::v1::SendProposalAckedResponse> * m,
                          uint64_t key, ::business_api::v1::SendProposalAckedResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_business_api_v1_SendProposalAckedResponse_iter(
      const google::protobuf::Map<uint64_t, ::business_api::v1::SendProposalAckedResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_business_api_v1_SendProposalAckedResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::business_api::v1::SendProposalAckedResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::business_api::v1::SendProposalAckedResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::business_api::v1::SendProposalAckedResponse>* __rust_proto_thunk__Map_bool_business_api_v1_SendProposalAckedResponse_new() {
    return new google::protobuf::Map<bool, ::business_api::v1::SendProposalAckedResponse>();
  }
  void __rust_proto_thunk__Map_bool_business_api_v1_SendProposalAckedResponse_free(const google::protobuf::Map<bool, ::business_api::v1::SendProposalAckedResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_business_api_v1_SendProposalAckedResponse_clear(google::protobuf::Map<bool, ::business_api::v1::SendProposalAckedResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_business_api_v1_SendProposalAckedResponse_size(const google::protobuf::Map<bool, ::business_api::v1::SendProposalAckedResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_business_api_v1_SendProposalAckedResponse_insert(google::protobuf::Map<bool, ::business_api::v1::SendProposalAckedResponse> * m,
                          bool key, ::business_api::v1::SendProposalAckedResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_business_api_v1_SendProposalAckedResponse_get(const google::protobuf::Map<bool, ::business_api::v1::SendProposalAckedResponse>* m,
                       bool key, const ::business_api::v1::SendProposalAckedResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::SendProposalAckedResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_business_api_v1_SendProposalAckedResponse_remove(google::protobuf::Map<bool, ::business_api::v1::SendProposalAckedResponse> * m,
                          bool key, ::business_api::v1::SendProposalAckedResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_business_api_v1_SendProposalAckedResponse_iter(
      const google::protobuf::Map<bool, ::business_api::v1::SendProposalAckedResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_business_api_v1_SendProposalAckedResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::business_api::v1::SendProposalAckedResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::business_api::v1::SendProposalAckedResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::business_api::v1::SendProposalAckedResponse>* __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendProposalAckedResponse_new() {
    return new google::protobuf::Map<std::string, ::business_api::v1::SendProposalAckedResponse>();
  }
  void __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendProposalAckedResponse_free(const google::protobuf::Map<std::string, ::business_api::v1::SendProposalAckedResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendProposalAckedResponse_clear(google::protobuf::Map<std::string, ::business_api::v1::SendProposalAckedResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendProposalAckedResponse_size(const google::protobuf::Map<std::string, ::business_api::v1::SendProposalAckedResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendProposalAckedResponse_insert(google::protobuf::Map<std::string, ::business_api::v1::SendProposalAckedResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::business_api::v1::SendProposalAckedResponse value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendProposalAckedResponse_get(const google::protobuf::Map<std::string, ::business_api::v1::SendProposalAckedResponse>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::business_api::v1::SendProposalAckedResponse** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::SendProposalAckedResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendProposalAckedResponse_remove(google::protobuf::Map<std::string, ::business_api::v1::SendProposalAckedResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::business_api::v1::SendProposalAckedResponse * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendProposalAckedResponse_iter(
      const google::protobuf::Map<std::string, ::business_api::v1::SendProposalAckedResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendProposalAckedResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::business_api::v1::SendProposalAckedResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::business_api::v1::SendProposalAckedResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// business_api.v1.SendProposalNackedRequest
        // clang-format off
extern "C" {
void* __rust_proto_thunk__business_api_v1_SendProposalNackedRequest_new() { return new ::business_api::v1::SendProposalNackedRequest(); }
void __rust_proto_thunk__business_api_v1_SendProposalNackedRequest_delete(void* ptr) { delete static_cast<::business_api::v1::SendProposalNackedRequest*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__business_api_v1_SendProposalNackedRequest_serialize(::business_api::v1::SendProposalNackedRequest* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__business_api_v1_SendProposalNackedRequest_parse(::business_api::v1::SendProposalNackedRequest* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__business_api_v1_SendProposalNackedRequest_copy_from(::business_api::v1::SendProposalNackedRequest* dst, const ::business_api::v1::SendProposalNackedRequest* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__business_api_v1_SendProposalNackedRequest_repeated_len(google::protobuf::RepeatedPtrField<::business_api::v1::SendProposalNackedRequest>* field) {
  return field->size();
}
const ::business_api::v1::SendProposalNackedRequest& __rust_proto_thunk__business_api_v1_SendProposalNackedRequest_repeated_get(
  google::protobuf::RepeatedPtrField<::business_api::v1::SendProposalNackedRequest>* field,
  size_t index) {
  return field->Get(index);
}
::business_api::v1::SendProposalNackedRequest* __rust_proto_thunk__business_api_v1_SendProposalNackedRequest_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::business_api::v1::SendProposalNackedRequest>* field,
  size_t index) {
  return field->Mutable(index);
}
::business_api::v1::SendProposalNackedRequest* __rust_proto_thunk__business_api_v1_SendProposalNackedRequest_repeated_add(google::protobuf::RepeatedPtrField<::business_api::v1::SendProposalNackedRequest>* field) {
  return field->Add();
}
void __rust_proto_thunk__business_api_v1_SendProposalNackedRequest_repeated_clear(google::protobuf::RepeatedPtrField<::business_api::v1::SendProposalNackedRequest>* field) {
  field->Clear();
}
void __rust_proto_thunk__business_api_v1_SendProposalNackedRequest_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::business_api::v1::SendProposalNackedRequest>& dst,
  const google::protobuf::RepeatedPtrField<::business_api::v1::SendProposalNackedRequest>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__business_api_v1_SendProposalNackedRequest_get_participant(::business_api::v1::SendProposalNackedRequest* msg) {
  absl::string_view val = msg->participant();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__business_api_v1_SendProposalNackedRequest_set_participant(::business_api::v1::SendProposalNackedRequest* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_participant(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__business_api_v1_SendProposalNackedRequest_get_contract_id(::business_api::v1::SendProposalNackedRequest* msg) {
  absl::string_view val = msg->contract_id();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__business_api_v1_SendProposalNackedRequest_set_contract_id(::business_api::v1::SendProposalNackedRequest* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_contract_id(absl::string_view(s.ptr, s.len));
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::business_api::v1::SendProposalNackedRequest>* __rust_proto_thunk__Map_i32_business_api_v1_SendProposalNackedRequest_new() {
    return new google::protobuf::Map<int32_t, ::business_api::v1::SendProposalNackedRequest>();
  }
  void __rust_proto_thunk__Map_i32_business_api_v1_SendProposalNackedRequest_free(const google::protobuf::Map<int32_t, ::business_api::v1::SendProposalNackedRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_business_api_v1_SendProposalNackedRequest_clear(google::protobuf::Map<int32_t, ::business_api::v1::SendProposalNackedRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_business_api_v1_SendProposalNackedRequest_size(const google::protobuf::Map<int32_t, ::business_api::v1::SendProposalNackedRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_business_api_v1_SendProposalNackedRequest_insert(google::protobuf::Map<int32_t, ::business_api::v1::SendProposalNackedRequest> * m,
                          int32_t key, ::business_api::v1::SendProposalNackedRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_business_api_v1_SendProposalNackedRequest_get(const google::protobuf::Map<int32_t, ::business_api::v1::SendProposalNackedRequest>* m,
                       int32_t key, const ::business_api::v1::SendProposalNackedRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::SendProposalNackedRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_business_api_v1_SendProposalNackedRequest_remove(google::protobuf::Map<int32_t, ::business_api::v1::SendProposalNackedRequest> * m,
                          int32_t key, ::business_api::v1::SendProposalNackedRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_business_api_v1_SendProposalNackedRequest_iter(
      const google::protobuf::Map<int32_t, ::business_api::v1::SendProposalNackedRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_business_api_v1_SendProposalNackedRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::business_api::v1::SendProposalNackedRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::business_api::v1::SendProposalNackedRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::business_api::v1::SendProposalNackedRequest>* __rust_proto_thunk__Map_u32_business_api_v1_SendProposalNackedRequest_new() {
    return new google::protobuf::Map<uint32_t, ::business_api::v1::SendProposalNackedRequest>();
  }
  void __rust_proto_thunk__Map_u32_business_api_v1_SendProposalNackedRequest_free(const google::protobuf::Map<uint32_t, ::business_api::v1::SendProposalNackedRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_business_api_v1_SendProposalNackedRequest_clear(google::protobuf::Map<uint32_t, ::business_api::v1::SendProposalNackedRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_business_api_v1_SendProposalNackedRequest_size(const google::protobuf::Map<uint32_t, ::business_api::v1::SendProposalNackedRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_business_api_v1_SendProposalNackedRequest_insert(google::protobuf::Map<uint32_t, ::business_api::v1::SendProposalNackedRequest> * m,
                          uint32_t key, ::business_api::v1::SendProposalNackedRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_business_api_v1_SendProposalNackedRequest_get(const google::protobuf::Map<uint32_t, ::business_api::v1::SendProposalNackedRequest>* m,
                       uint32_t key, const ::business_api::v1::SendProposalNackedRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::SendProposalNackedRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_business_api_v1_SendProposalNackedRequest_remove(google::protobuf::Map<uint32_t, ::business_api::v1::SendProposalNackedRequest> * m,
                          uint32_t key, ::business_api::v1::SendProposalNackedRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_business_api_v1_SendProposalNackedRequest_iter(
      const google::protobuf::Map<uint32_t, ::business_api::v1::SendProposalNackedRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_business_api_v1_SendProposalNackedRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::business_api::v1::SendProposalNackedRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::business_api::v1::SendProposalNackedRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::business_api::v1::SendProposalNackedRequest>* __rust_proto_thunk__Map_i64_business_api_v1_SendProposalNackedRequest_new() {
    return new google::protobuf::Map<int64_t, ::business_api::v1::SendProposalNackedRequest>();
  }
  void __rust_proto_thunk__Map_i64_business_api_v1_SendProposalNackedRequest_free(const google::protobuf::Map<int64_t, ::business_api::v1::SendProposalNackedRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_business_api_v1_SendProposalNackedRequest_clear(google::protobuf::Map<int64_t, ::business_api::v1::SendProposalNackedRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_business_api_v1_SendProposalNackedRequest_size(const google::protobuf::Map<int64_t, ::business_api::v1::SendProposalNackedRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_business_api_v1_SendProposalNackedRequest_insert(google::protobuf::Map<int64_t, ::business_api::v1::SendProposalNackedRequest> * m,
                          int64_t key, ::business_api::v1::SendProposalNackedRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_business_api_v1_SendProposalNackedRequest_get(const google::protobuf::Map<int64_t, ::business_api::v1::SendProposalNackedRequest>* m,
                       int64_t key, const ::business_api::v1::SendProposalNackedRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::SendProposalNackedRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_business_api_v1_SendProposalNackedRequest_remove(google::protobuf::Map<int64_t, ::business_api::v1::SendProposalNackedRequest> * m,
                          int64_t key, ::business_api::v1::SendProposalNackedRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_business_api_v1_SendProposalNackedRequest_iter(
      const google::protobuf::Map<int64_t, ::business_api::v1::SendProposalNackedRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_business_api_v1_SendProposalNackedRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::business_api::v1::SendProposalNackedRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::business_api::v1::SendProposalNackedRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::business_api::v1::SendProposalNackedRequest>* __rust_proto_thunk__Map_u64_business_api_v1_SendProposalNackedRequest_new() {
    return new google::protobuf::Map<uint64_t, ::business_api::v1::SendProposalNackedRequest>();
  }
  void __rust_proto_thunk__Map_u64_business_api_v1_SendProposalNackedRequest_free(const google::protobuf::Map<uint64_t, ::business_api::v1::SendProposalNackedRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_business_api_v1_SendProposalNackedRequest_clear(google::protobuf::Map<uint64_t, ::business_api::v1::SendProposalNackedRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_business_api_v1_SendProposalNackedRequest_size(const google::protobuf::Map<uint64_t, ::business_api::v1::SendProposalNackedRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_business_api_v1_SendProposalNackedRequest_insert(google::protobuf::Map<uint64_t, ::business_api::v1::SendProposalNackedRequest> * m,
                          uint64_t key, ::business_api::v1::SendProposalNackedRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_business_api_v1_SendProposalNackedRequest_get(const google::protobuf::Map<uint64_t, ::business_api::v1::SendProposalNackedRequest>* m,
                       uint64_t key, const ::business_api::v1::SendProposalNackedRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::SendProposalNackedRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_business_api_v1_SendProposalNackedRequest_remove(google::protobuf::Map<uint64_t, ::business_api::v1::SendProposalNackedRequest> * m,
                          uint64_t key, ::business_api::v1::SendProposalNackedRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_business_api_v1_SendProposalNackedRequest_iter(
      const google::protobuf::Map<uint64_t, ::business_api::v1::SendProposalNackedRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_business_api_v1_SendProposalNackedRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::business_api::v1::SendProposalNackedRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::business_api::v1::SendProposalNackedRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::business_api::v1::SendProposalNackedRequest>* __rust_proto_thunk__Map_bool_business_api_v1_SendProposalNackedRequest_new() {
    return new google::protobuf::Map<bool, ::business_api::v1::SendProposalNackedRequest>();
  }
  void __rust_proto_thunk__Map_bool_business_api_v1_SendProposalNackedRequest_free(const google::protobuf::Map<bool, ::business_api::v1::SendProposalNackedRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_business_api_v1_SendProposalNackedRequest_clear(google::protobuf::Map<bool, ::business_api::v1::SendProposalNackedRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_business_api_v1_SendProposalNackedRequest_size(const google::protobuf::Map<bool, ::business_api::v1::SendProposalNackedRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_business_api_v1_SendProposalNackedRequest_insert(google::protobuf::Map<bool, ::business_api::v1::SendProposalNackedRequest> * m,
                          bool key, ::business_api::v1::SendProposalNackedRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_business_api_v1_SendProposalNackedRequest_get(const google::protobuf::Map<bool, ::business_api::v1::SendProposalNackedRequest>* m,
                       bool key, const ::business_api::v1::SendProposalNackedRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::SendProposalNackedRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_business_api_v1_SendProposalNackedRequest_remove(google::protobuf::Map<bool, ::business_api::v1::SendProposalNackedRequest> * m,
                          bool key, ::business_api::v1::SendProposalNackedRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_business_api_v1_SendProposalNackedRequest_iter(
      const google::protobuf::Map<bool, ::business_api::v1::SendProposalNackedRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_business_api_v1_SendProposalNackedRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::business_api::v1::SendProposalNackedRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::business_api::v1::SendProposalNackedRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::business_api::v1::SendProposalNackedRequest>* __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendProposalNackedRequest_new() {
    return new google::protobuf::Map<std::string, ::business_api::v1::SendProposalNackedRequest>();
  }
  void __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendProposalNackedRequest_free(const google::protobuf::Map<std::string, ::business_api::v1::SendProposalNackedRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendProposalNackedRequest_clear(google::protobuf::Map<std::string, ::business_api::v1::SendProposalNackedRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendProposalNackedRequest_size(const google::protobuf::Map<std::string, ::business_api::v1::SendProposalNackedRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendProposalNackedRequest_insert(google::protobuf::Map<std::string, ::business_api::v1::SendProposalNackedRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::business_api::v1::SendProposalNackedRequest value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendProposalNackedRequest_get(const google::protobuf::Map<std::string, ::business_api::v1::SendProposalNackedRequest>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::business_api::v1::SendProposalNackedRequest** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::SendProposalNackedRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendProposalNackedRequest_remove(google::protobuf::Map<std::string, ::business_api::v1::SendProposalNackedRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::business_api::v1::SendProposalNackedRequest * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendProposalNackedRequest_iter(
      const google::protobuf::Map<std::string, ::business_api::v1::SendProposalNackedRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendProposalNackedRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::business_api::v1::SendProposalNackedRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::business_api::v1::SendProposalNackedRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// business_api.v1.SendProposalNackedResponse
        // clang-format off
extern "C" {
void* __rust_proto_thunk__business_api_v1_SendProposalNackedResponse_new() { return new ::business_api::v1::SendProposalNackedResponse(); }
void __rust_proto_thunk__business_api_v1_SendProposalNackedResponse_delete(void* ptr) { delete static_cast<::business_api::v1::SendProposalNackedResponse*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__business_api_v1_SendProposalNackedResponse_serialize(::business_api::v1::SendProposalNackedResponse* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__business_api_v1_SendProposalNackedResponse_parse(::business_api::v1::SendProposalNackedResponse* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__business_api_v1_SendProposalNackedResponse_copy_from(::business_api::v1::SendProposalNackedResponse* dst, const ::business_api::v1::SendProposalNackedResponse* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__business_api_v1_SendProposalNackedResponse_repeated_len(google::protobuf::RepeatedPtrField<::business_api::v1::SendProposalNackedResponse>* field) {
  return field->size();
}
const ::business_api::v1::SendProposalNackedResponse& __rust_proto_thunk__business_api_v1_SendProposalNackedResponse_repeated_get(
  google::protobuf::RepeatedPtrField<::business_api::v1::SendProposalNackedResponse>* field,
  size_t index) {
  return field->Get(index);
}
::business_api::v1::SendProposalNackedResponse* __rust_proto_thunk__business_api_v1_SendProposalNackedResponse_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::business_api::v1::SendProposalNackedResponse>* field,
  size_t index) {
  return field->Mutable(index);
}
::business_api::v1::SendProposalNackedResponse* __rust_proto_thunk__business_api_v1_SendProposalNackedResponse_repeated_add(google::protobuf::RepeatedPtrField<::business_api::v1::SendProposalNackedResponse>* field) {
  return field->Add();
}
void __rust_proto_thunk__business_api_v1_SendProposalNackedResponse_repeated_clear(google::protobuf::RepeatedPtrField<::business_api::v1::SendProposalNackedResponse>* field) {
  field->Clear();
}
void __rust_proto_thunk__business_api_v1_SendProposalNackedResponse_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::business_api::v1::SendProposalNackedResponse>& dst,
  const google::protobuf::RepeatedPtrField<::business_api::v1::SendProposalNackedResponse>& src) {
  dst = src;
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::business_api::v1::SendProposalNackedResponse>* __rust_proto_thunk__Map_i32_business_api_v1_SendProposalNackedResponse_new() {
    return new google::protobuf::Map<int32_t, ::business_api::v1::SendProposalNackedResponse>();
  }
  void __rust_proto_thunk__Map_i32_business_api_v1_SendProposalNackedResponse_free(const google::protobuf::Map<int32_t, ::business_api::v1::SendProposalNackedResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_business_api_v1_SendProposalNackedResponse_clear(google::protobuf::Map<int32_t, ::business_api::v1::SendProposalNackedResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_business_api_v1_SendProposalNackedResponse_size(const google::protobuf::Map<int32_t, ::business_api::v1::SendProposalNackedResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_business_api_v1_SendProposalNackedResponse_insert(google::protobuf::Map<int32_t, ::business_api::v1::SendProposalNackedResponse> * m,
                          int32_t key, ::business_api::v1::SendProposalNackedResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_business_api_v1_SendProposalNackedResponse_get(const google::protobuf::Map<int32_t, ::business_api::v1::SendProposalNackedResponse>* m,
                       int32_t key, const ::business_api::v1::SendProposalNackedResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::SendProposalNackedResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_business_api_v1_SendProposalNackedResponse_remove(google::protobuf::Map<int32_t, ::business_api::v1::SendProposalNackedResponse> * m,
                          int32_t key, ::business_api::v1::SendProposalNackedResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_business_api_v1_SendProposalNackedResponse_iter(
      const google::protobuf::Map<int32_t, ::business_api::v1::SendProposalNackedResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_business_api_v1_SendProposalNackedResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::business_api::v1::SendProposalNackedResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::business_api::v1::SendProposalNackedResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::business_api::v1::SendProposalNackedResponse>* __rust_proto_thunk__Map_u32_business_api_v1_SendProposalNackedResponse_new() {
    return new google::protobuf::Map<uint32_t, ::business_api::v1::SendProposalNackedResponse>();
  }
  void __rust_proto_thunk__Map_u32_business_api_v1_SendProposalNackedResponse_free(const google::protobuf::Map<uint32_t, ::business_api::v1::SendProposalNackedResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_business_api_v1_SendProposalNackedResponse_clear(google::protobuf::Map<uint32_t, ::business_api::v1::SendProposalNackedResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_business_api_v1_SendProposalNackedResponse_size(const google::protobuf::Map<uint32_t, ::business_api::v1::SendProposalNackedResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_business_api_v1_SendProposalNackedResponse_insert(google::protobuf::Map<uint32_t, ::business_api::v1::SendProposalNackedResponse> * m,
                          uint32_t key, ::business_api::v1::SendProposalNackedResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_business_api_v1_SendProposalNackedResponse_get(const google::protobuf::Map<uint32_t, ::business_api::v1::SendProposalNackedResponse>* m,
                       uint32_t key, const ::business_api::v1::SendProposalNackedResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::SendProposalNackedResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_business_api_v1_SendProposalNackedResponse_remove(google::protobuf::Map<uint32_t, ::business_api::v1::SendProposalNackedResponse> * m,
                          uint32_t key, ::business_api::v1::SendProposalNackedResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_business_api_v1_SendProposalNackedResponse_iter(
      const google::protobuf::Map<uint32_t, ::business_api::v1::SendProposalNackedResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_business_api_v1_SendProposalNackedResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::business_api::v1::SendProposalNackedResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::business_api::v1::SendProposalNackedResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::business_api::v1::SendProposalNackedResponse>* __rust_proto_thunk__Map_i64_business_api_v1_SendProposalNackedResponse_new() {
    return new google::protobuf::Map<int64_t, ::business_api::v1::SendProposalNackedResponse>();
  }
  void __rust_proto_thunk__Map_i64_business_api_v1_SendProposalNackedResponse_free(const google::protobuf::Map<int64_t, ::business_api::v1::SendProposalNackedResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_business_api_v1_SendProposalNackedResponse_clear(google::protobuf::Map<int64_t, ::business_api::v1::SendProposalNackedResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_business_api_v1_SendProposalNackedResponse_size(const google::protobuf::Map<int64_t, ::business_api::v1::SendProposalNackedResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_business_api_v1_SendProposalNackedResponse_insert(google::protobuf::Map<int64_t, ::business_api::v1::SendProposalNackedResponse> * m,
                          int64_t key, ::business_api::v1::SendProposalNackedResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_business_api_v1_SendProposalNackedResponse_get(const google::protobuf::Map<int64_t, ::business_api::v1::SendProposalNackedResponse>* m,
                       int64_t key, const ::business_api::v1::SendProposalNackedResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::SendProposalNackedResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_business_api_v1_SendProposalNackedResponse_remove(google::protobuf::Map<int64_t, ::business_api::v1::SendProposalNackedResponse> * m,
                          int64_t key, ::business_api::v1::SendProposalNackedResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_business_api_v1_SendProposalNackedResponse_iter(
      const google::protobuf::Map<int64_t, ::business_api::v1::SendProposalNackedResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_business_api_v1_SendProposalNackedResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::business_api::v1::SendProposalNackedResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::business_api::v1::SendProposalNackedResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::business_api::v1::SendProposalNackedResponse>* __rust_proto_thunk__Map_u64_business_api_v1_SendProposalNackedResponse_new() {
    return new google::protobuf::Map<uint64_t, ::business_api::v1::SendProposalNackedResponse>();
  }
  void __rust_proto_thunk__Map_u64_business_api_v1_SendProposalNackedResponse_free(const google::protobuf::Map<uint64_t, ::business_api::v1::SendProposalNackedResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_business_api_v1_SendProposalNackedResponse_clear(google::protobuf::Map<uint64_t, ::business_api::v1::SendProposalNackedResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_business_api_v1_SendProposalNackedResponse_size(const google::protobuf::Map<uint64_t, ::business_api::v1::SendProposalNackedResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_business_api_v1_SendProposalNackedResponse_insert(google::protobuf::Map<uint64_t, ::business_api::v1::SendProposalNackedResponse> * m,
                          uint64_t key, ::business_api::v1::SendProposalNackedResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_business_api_v1_SendProposalNackedResponse_get(const google::protobuf::Map<uint64_t, ::business_api::v1::SendProposalNackedResponse>* m,
                       uint64_t key, const ::business_api::v1::SendProposalNackedResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::SendProposalNackedResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_business_api_v1_SendProposalNackedResponse_remove(google::protobuf::Map<uint64_t, ::business_api::v1::SendProposalNackedResponse> * m,
                          uint64_t key, ::business_api::v1::SendProposalNackedResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_business_api_v1_SendProposalNackedResponse_iter(
      const google::protobuf::Map<uint64_t, ::business_api::v1::SendProposalNackedResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_business_api_v1_SendProposalNackedResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::business_api::v1::SendProposalNackedResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::business_api::v1::SendProposalNackedResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::business_api::v1::SendProposalNackedResponse>* __rust_proto_thunk__Map_bool_business_api_v1_SendProposalNackedResponse_new() {
    return new google::protobuf::Map<bool, ::business_api::v1::SendProposalNackedResponse>();
  }
  void __rust_proto_thunk__Map_bool_business_api_v1_SendProposalNackedResponse_free(const google::protobuf::Map<bool, ::business_api::v1::SendProposalNackedResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_business_api_v1_SendProposalNackedResponse_clear(google::protobuf::Map<bool, ::business_api::v1::SendProposalNackedResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_business_api_v1_SendProposalNackedResponse_size(const google::protobuf::Map<bool, ::business_api::v1::SendProposalNackedResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_business_api_v1_SendProposalNackedResponse_insert(google::protobuf::Map<bool, ::business_api::v1::SendProposalNackedResponse> * m,
                          bool key, ::business_api::v1::SendProposalNackedResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_business_api_v1_SendProposalNackedResponse_get(const google::protobuf::Map<bool, ::business_api::v1::SendProposalNackedResponse>* m,
                       bool key, const ::business_api::v1::SendProposalNackedResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::SendProposalNackedResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_business_api_v1_SendProposalNackedResponse_remove(google::protobuf::Map<bool, ::business_api::v1::SendProposalNackedResponse> * m,
                          bool key, ::business_api::v1::SendProposalNackedResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_business_api_v1_SendProposalNackedResponse_iter(
      const google::protobuf::Map<bool, ::business_api::v1::SendProposalNackedResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_business_api_v1_SendProposalNackedResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::business_api::v1::SendProposalNackedResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::business_api::v1::SendProposalNackedResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::business_api::v1::SendProposalNackedResponse>* __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendProposalNackedResponse_new() {
    return new google::protobuf::Map<std::string, ::business_api::v1::SendProposalNackedResponse>();
  }
  void __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendProposalNackedResponse_free(const google::protobuf::Map<std::string, ::business_api::v1::SendProposalNackedResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendProposalNackedResponse_clear(google::protobuf::Map<std::string, ::business_api::v1::SendProposalNackedResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendProposalNackedResponse_size(const google::protobuf::Map<std::string, ::business_api::v1::SendProposalNackedResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendProposalNackedResponse_insert(google::protobuf::Map<std::string, ::business_api::v1::SendProposalNackedResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::business_api::v1::SendProposalNackedResponse value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendProposalNackedResponse_get(const google::protobuf::Map<std::string, ::business_api::v1::SendProposalNackedResponse>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::business_api::v1::SendProposalNackedResponse** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::SendProposalNackedResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendProposalNackedResponse_remove(google::protobuf::Map<std::string, ::business_api::v1::SendProposalNackedResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::business_api::v1::SendProposalNackedResponse * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendProposalNackedResponse_iter(
      const google::protobuf::Map<std::string, ::business_api::v1::SendProposalNackedResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendProposalNackedResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::business_api::v1::SendProposalNackedResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::business_api::v1::SendProposalNackedResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// business_api.v1.SendSecretRevealedRequest
        // clang-format off
extern "C" {
void* __rust_proto_thunk__business_api_v1_SendSecretRevealedRequest_new() { return new ::business_api::v1::SendSecretRevealedRequest(); }
void __rust_proto_thunk__business_api_v1_SendSecretRevealedRequest_delete(void* ptr) { delete static_cast<::business_api::v1::SendSecretRevealedRequest*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__business_api_v1_SendSecretRevealedRequest_serialize(::business_api::v1::SendSecretRevealedRequest* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__business_api_v1_SendSecretRevealedRequest_parse(::business_api::v1::SendSecretRevealedRequest* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__business_api_v1_SendSecretRevealedRequest_copy_from(::business_api::v1::SendSecretRevealedRequest* dst, const ::business_api::v1::SendSecretRevealedRequest* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__business_api_v1_SendSecretRevealedRequest_repeated_len(google::protobuf::RepeatedPtrField<::business_api::v1::SendSecretRevealedRequest>* field) {
  return field->size();
}
const ::business_api::v1::SendSecretRevealedRequest& __rust_proto_thunk__business_api_v1_SendSecretRevealedRequest_repeated_get(
  google::protobuf::RepeatedPtrField<::business_api::v1::SendSecretRevealedRequest>* field,
  size_t index) {
  return field->Get(index);
}
::business_api::v1::SendSecretRevealedRequest* __rust_proto_thunk__business_api_v1_SendSecretRevealedRequest_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::business_api::v1::SendSecretRevealedRequest>* field,
  size_t index) {
  return field->Mutable(index);
}
::business_api::v1::SendSecretRevealedRequest* __rust_proto_thunk__business_api_v1_SendSecretRevealedRequest_repeated_add(google::protobuf::RepeatedPtrField<::business_api::v1::SendSecretRevealedRequest>* field) {
  return field->Add();
}
void __rust_proto_thunk__business_api_v1_SendSecretRevealedRequest_repeated_clear(google::protobuf::RepeatedPtrField<::business_api::v1::SendSecretRevealedRequest>* field) {
  field->Clear();
}
void __rust_proto_thunk__business_api_v1_SendSecretRevealedRequest_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::business_api::v1::SendSecretRevealedRequest>& dst,
  const google::protobuf::RepeatedPtrField<::business_api::v1::SendSecretRevealedRequest>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__business_api_v1_SendSecretRevealedRequest_get_participant(::business_api::v1::SendSecretRevealedRequest* msg) {
  absl::string_view val = msg->participant();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__business_api_v1_SendSecretRevealedRequest_set_participant(::business_api::v1::SendSecretRevealedRequest* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_participant(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__business_api_v1_SendSecretRevealedRequest_get_contract_id(::business_api::v1::SendSecretRevealedRequest* msg) {
  absl::string_view val = msg->contract_id();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__business_api_v1_SendSecretRevealedRequest_set_contract_id(::business_api::v1::SendSecretRevealedRequest* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_contract_id(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__business_api_v1_SendSecretRevealedRequest_get_secret(::business_api::v1::SendSecretRevealedRequest* msg) {
  absl::string_view val = msg->secret();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__business_api_v1_SendSecretRevealedRequest_set_secret(::business_api::v1::SendSecretRevealedRequest* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_secret(absl::string_view(s.ptr, s.len));
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::business_api::v1::SendSecretRevealedRequest>* __rust_proto_thunk__Map_i32_business_api_v1_SendSecretRevealedRequest_new() {
    return new google::protobuf::Map<int32_t, ::business_api::v1::SendSecretRevealedRequest>();
  }
  void __rust_proto_thunk__Map_i32_business_api_v1_SendSecretRevealedRequest_free(const google::protobuf::Map<int32_t, ::business_api::v1::SendSecretRevealedRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_business_api_v1_SendSecretRevealedRequest_clear(google::protobuf::Map<int32_t, ::business_api::v1::SendSecretRevealedRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_business_api_v1_SendSecretRevealedRequest_size(const google::protobuf::Map<int32_t, ::business_api::v1::SendSecretRevealedRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_business_api_v1_SendSecretRevealedRequest_insert(google::protobuf::Map<int32_t, ::business_api::v1::SendSecretRevealedRequest> * m,
                          int32_t key, ::business_api::v1::SendSecretRevealedRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_business_api_v1_SendSecretRevealedRequest_get(const google::protobuf::Map<int32_t, ::business_api::v1::SendSecretRevealedRequest>* m,
                       int32_t key, const ::business_api::v1::SendSecretRevealedRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::SendSecretRevealedRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_business_api_v1_SendSecretRevealedRequest_remove(google::protobuf::Map<int32_t, ::business_api::v1::SendSecretRevealedRequest> * m,
                          int32_t key, ::business_api::v1::SendSecretRevealedRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_business_api_v1_SendSecretRevealedRequest_iter(
      const google::protobuf::Map<int32_t, ::business_api::v1::SendSecretRevealedRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_business_api_v1_SendSecretRevealedRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::business_api::v1::SendSecretRevealedRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::business_api::v1::SendSecretRevealedRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::business_api::v1::SendSecretRevealedRequest>* __rust_proto_thunk__Map_u32_business_api_v1_SendSecretRevealedRequest_new() {
    return new google::protobuf::Map<uint32_t, ::business_api::v1::SendSecretRevealedRequest>();
  }
  void __rust_proto_thunk__Map_u32_business_api_v1_SendSecretRevealedRequest_free(const google::protobuf::Map<uint32_t, ::business_api::v1::SendSecretRevealedRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_business_api_v1_SendSecretRevealedRequest_clear(google::protobuf::Map<uint32_t, ::business_api::v1::SendSecretRevealedRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_business_api_v1_SendSecretRevealedRequest_size(const google::protobuf::Map<uint32_t, ::business_api::v1::SendSecretRevealedRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_business_api_v1_SendSecretRevealedRequest_insert(google::protobuf::Map<uint32_t, ::business_api::v1::SendSecretRevealedRequest> * m,
                          uint32_t key, ::business_api::v1::SendSecretRevealedRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_business_api_v1_SendSecretRevealedRequest_get(const google::protobuf::Map<uint32_t, ::business_api::v1::SendSecretRevealedRequest>* m,
                       uint32_t key, const ::business_api::v1::SendSecretRevealedRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::SendSecretRevealedRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_business_api_v1_SendSecretRevealedRequest_remove(google::protobuf::Map<uint32_t, ::business_api::v1::SendSecretRevealedRequest> * m,
                          uint32_t key, ::business_api::v1::SendSecretRevealedRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_business_api_v1_SendSecretRevealedRequest_iter(
      const google::protobuf::Map<uint32_t, ::business_api::v1::SendSecretRevealedRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_business_api_v1_SendSecretRevealedRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::business_api::v1::SendSecretRevealedRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::business_api::v1::SendSecretRevealedRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::business_api::v1::SendSecretRevealedRequest>* __rust_proto_thunk__Map_i64_business_api_v1_SendSecretRevealedRequest_new() {
    return new google::protobuf::Map<int64_t, ::business_api::v1::SendSecretRevealedRequest>();
  }
  void __rust_proto_thunk__Map_i64_business_api_v1_SendSecretRevealedRequest_free(const google::protobuf::Map<int64_t, ::business_api::v1::SendSecretRevealedRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_business_api_v1_SendSecretRevealedRequest_clear(google::protobuf::Map<int64_t, ::business_api::v1::SendSecretRevealedRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_business_api_v1_SendSecretRevealedRequest_size(const google::protobuf::Map<int64_t, ::business_api::v1::SendSecretRevealedRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_business_api_v1_SendSecretRevealedRequest_insert(google::protobuf::Map<int64_t, ::business_api::v1::SendSecretRevealedRequest> * m,
                          int64_t key, ::business_api::v1::SendSecretRevealedRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_business_api_v1_SendSecretRevealedRequest_get(const google::protobuf::Map<int64_t, ::business_api::v1::SendSecretRevealedRequest>* m,
                       int64_t key, const ::business_api::v1::SendSecretRevealedRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::SendSecretRevealedRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_business_api_v1_SendSecretRevealedRequest_remove(google::protobuf::Map<int64_t, ::business_api::v1::SendSecretRevealedRequest> * m,
                          int64_t key, ::business_api::v1::SendSecretRevealedRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_business_api_v1_SendSecretRevealedRequest_iter(
      const google::protobuf::Map<int64_t, ::business_api::v1::SendSecretRevealedRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_business_api_v1_SendSecretRevealedRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::business_api::v1::SendSecretRevealedRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::business_api::v1::SendSecretRevealedRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::business_api::v1::SendSecretRevealedRequest>* __rust_proto_thunk__Map_u64_business_api_v1_SendSecretRevealedRequest_new() {
    return new google::protobuf::Map<uint64_t, ::business_api::v1::SendSecretRevealedRequest>();
  }
  void __rust_proto_thunk__Map_u64_business_api_v1_SendSecretRevealedRequest_free(const google::protobuf::Map<uint64_t, ::business_api::v1::SendSecretRevealedRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_business_api_v1_SendSecretRevealedRequest_clear(google::protobuf::Map<uint64_t, ::business_api::v1::SendSecretRevealedRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_business_api_v1_SendSecretRevealedRequest_size(const google::protobuf::Map<uint64_t, ::business_api::v1::SendSecretRevealedRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_business_api_v1_SendSecretRevealedRequest_insert(google::protobuf::Map<uint64_t, ::business_api::v1::SendSecretRevealedRequest> * m,
                          uint64_t key, ::business_api::v1::SendSecretRevealedRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_business_api_v1_SendSecretRevealedRequest_get(const google::protobuf::Map<uint64_t, ::business_api::v1::SendSecretRevealedRequest>* m,
                       uint64_t key, const ::business_api::v1::SendSecretRevealedRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::SendSecretRevealedRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_business_api_v1_SendSecretRevealedRequest_remove(google::protobuf::Map<uint64_t, ::business_api::v1::SendSecretRevealedRequest> * m,
                          uint64_t key, ::business_api::v1::SendSecretRevealedRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_business_api_v1_SendSecretRevealedRequest_iter(
      const google::protobuf::Map<uint64_t, ::business_api::v1::SendSecretRevealedRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_business_api_v1_SendSecretRevealedRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::business_api::v1::SendSecretRevealedRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::business_api::v1::SendSecretRevealedRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::business_api::v1::SendSecretRevealedRequest>* __rust_proto_thunk__Map_bool_business_api_v1_SendSecretRevealedRequest_new() {
    return new google::protobuf::Map<bool, ::business_api::v1::SendSecretRevealedRequest>();
  }
  void __rust_proto_thunk__Map_bool_business_api_v1_SendSecretRevealedRequest_free(const google::protobuf::Map<bool, ::business_api::v1::SendSecretRevealedRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_business_api_v1_SendSecretRevealedRequest_clear(google::protobuf::Map<bool, ::business_api::v1::SendSecretRevealedRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_business_api_v1_SendSecretRevealedRequest_size(const google::protobuf::Map<bool, ::business_api::v1::SendSecretRevealedRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_business_api_v1_SendSecretRevealedRequest_insert(google::protobuf::Map<bool, ::business_api::v1::SendSecretRevealedRequest> * m,
                          bool key, ::business_api::v1::SendSecretRevealedRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_business_api_v1_SendSecretRevealedRequest_get(const google::protobuf::Map<bool, ::business_api::v1::SendSecretRevealedRequest>* m,
                       bool key, const ::business_api::v1::SendSecretRevealedRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::SendSecretRevealedRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_business_api_v1_SendSecretRevealedRequest_remove(google::protobuf::Map<bool, ::business_api::v1::SendSecretRevealedRequest> * m,
                          bool key, ::business_api::v1::SendSecretRevealedRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_business_api_v1_SendSecretRevealedRequest_iter(
      const google::protobuf::Map<bool, ::business_api::v1::SendSecretRevealedRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_business_api_v1_SendSecretRevealedRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::business_api::v1::SendSecretRevealedRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::business_api::v1::SendSecretRevealedRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::business_api::v1::SendSecretRevealedRequest>* __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendSecretRevealedRequest_new() {
    return new google::protobuf::Map<std::string, ::business_api::v1::SendSecretRevealedRequest>();
  }
  void __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendSecretRevealedRequest_free(const google::protobuf::Map<std::string, ::business_api::v1::SendSecretRevealedRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendSecretRevealedRequest_clear(google::protobuf::Map<std::string, ::business_api::v1::SendSecretRevealedRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendSecretRevealedRequest_size(const google::protobuf::Map<std::string, ::business_api::v1::SendSecretRevealedRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendSecretRevealedRequest_insert(google::protobuf::Map<std::string, ::business_api::v1::SendSecretRevealedRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::business_api::v1::SendSecretRevealedRequest value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendSecretRevealedRequest_get(const google::protobuf::Map<std::string, ::business_api::v1::SendSecretRevealedRequest>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::business_api::v1::SendSecretRevealedRequest** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::SendSecretRevealedRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendSecretRevealedRequest_remove(google::protobuf::Map<std::string, ::business_api::v1::SendSecretRevealedRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::business_api::v1::SendSecretRevealedRequest * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendSecretRevealedRequest_iter(
      const google::protobuf::Map<std::string, ::business_api::v1::SendSecretRevealedRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendSecretRevealedRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::business_api::v1::SendSecretRevealedRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::business_api::v1::SendSecretRevealedRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// business_api.v1.SendSecretRevealedResponse
        // clang-format off
extern "C" {
void* __rust_proto_thunk__business_api_v1_SendSecretRevealedResponse_new() { return new ::business_api::v1::SendSecretRevealedResponse(); }
void __rust_proto_thunk__business_api_v1_SendSecretRevealedResponse_delete(void* ptr) { delete static_cast<::business_api::v1::SendSecretRevealedResponse*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__business_api_v1_SendSecretRevealedResponse_serialize(::business_api::v1::SendSecretRevealedResponse* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__business_api_v1_SendSecretRevealedResponse_parse(::business_api::v1::SendSecretRevealedResponse* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__business_api_v1_SendSecretRevealedResponse_copy_from(::business_api::v1::SendSecretRevealedResponse* dst, const ::business_api::v1::SendSecretRevealedResponse* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__business_api_v1_SendSecretRevealedResponse_repeated_len(google::protobuf::RepeatedPtrField<::business_api::v1::SendSecretRevealedResponse>* field) {
  return field->size();
}
const ::business_api::v1::SendSecretRevealedResponse& __rust_proto_thunk__business_api_v1_SendSecretRevealedResponse_repeated_get(
  google::protobuf::RepeatedPtrField<::business_api::v1::SendSecretRevealedResponse>* field,
  size_t index) {
  return field->Get(index);
}
::business_api::v1::SendSecretRevealedResponse* __rust_proto_thunk__business_api_v1_SendSecretRevealedResponse_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::business_api::v1::SendSecretRevealedResponse>* field,
  size_t index) {
  return field->Mutable(index);
}
::business_api::v1::SendSecretRevealedResponse* __rust_proto_thunk__business_api_v1_SendSecretRevealedResponse_repeated_add(google::protobuf::RepeatedPtrField<::business_api::v1::SendSecretRevealedResponse>* field) {
  return field->Add();
}
void __rust_proto_thunk__business_api_v1_SendSecretRevealedResponse_repeated_clear(google::protobuf::RepeatedPtrField<::business_api::v1::SendSecretRevealedResponse>* field) {
  field->Clear();
}
void __rust_proto_thunk__business_api_v1_SendSecretRevealedResponse_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::business_api::v1::SendSecretRevealedResponse>& dst,
  const google::protobuf::RepeatedPtrField<::business_api::v1::SendSecretRevealedResponse>& src) {
  dst = src;
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::business_api::v1::SendSecretRevealedResponse>* __rust_proto_thunk__Map_i32_business_api_v1_SendSecretRevealedResponse_new() {
    return new google::protobuf::Map<int32_t, ::business_api::v1::SendSecretRevealedResponse>();
  }
  void __rust_proto_thunk__Map_i32_business_api_v1_SendSecretRevealedResponse_free(const google::protobuf::Map<int32_t, ::business_api::v1::SendSecretRevealedResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_business_api_v1_SendSecretRevealedResponse_clear(google::protobuf::Map<int32_t, ::business_api::v1::SendSecretRevealedResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_business_api_v1_SendSecretRevealedResponse_size(const google::protobuf::Map<int32_t, ::business_api::v1::SendSecretRevealedResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_business_api_v1_SendSecretRevealedResponse_insert(google::protobuf::Map<int32_t, ::business_api::v1::SendSecretRevealedResponse> * m,
                          int32_t key, ::business_api::v1::SendSecretRevealedResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_business_api_v1_SendSecretRevealedResponse_get(const google::protobuf::Map<int32_t, ::business_api::v1::SendSecretRevealedResponse>* m,
                       int32_t key, const ::business_api::v1::SendSecretRevealedResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::SendSecretRevealedResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_business_api_v1_SendSecretRevealedResponse_remove(google::protobuf::Map<int32_t, ::business_api::v1::SendSecretRevealedResponse> * m,
                          int32_t key, ::business_api::v1::SendSecretRevealedResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_business_api_v1_SendSecretRevealedResponse_iter(
      const google::protobuf::Map<int32_t, ::business_api::v1::SendSecretRevealedResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_business_api_v1_SendSecretRevealedResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::business_api::v1::SendSecretRevealedResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::business_api::v1::SendSecretRevealedResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::business_api::v1::SendSecretRevealedResponse>* __rust_proto_thunk__Map_u32_business_api_v1_SendSecretRevealedResponse_new() {
    return new google::protobuf::Map<uint32_t, ::business_api::v1::SendSecretRevealedResponse>();
  }
  void __rust_proto_thunk__Map_u32_business_api_v1_SendSecretRevealedResponse_free(const google::protobuf::Map<uint32_t, ::business_api::v1::SendSecretRevealedResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_business_api_v1_SendSecretRevealedResponse_clear(google::protobuf::Map<uint32_t, ::business_api::v1::SendSecretRevealedResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_business_api_v1_SendSecretRevealedResponse_size(const google::protobuf::Map<uint32_t, ::business_api::v1::SendSecretRevealedResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_business_api_v1_SendSecretRevealedResponse_insert(google::protobuf::Map<uint32_t, ::business_api::v1::SendSecretRevealedResponse> * m,
                          uint32_t key, ::business_api::v1::SendSecretRevealedResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_business_api_v1_SendSecretRevealedResponse_get(const google::protobuf::Map<uint32_t, ::business_api::v1::SendSecretRevealedResponse>* m,
                       uint32_t key, const ::business_api::v1::SendSecretRevealedResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::SendSecretRevealedResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_business_api_v1_SendSecretRevealedResponse_remove(google::protobuf::Map<uint32_t, ::business_api::v1::SendSecretRevealedResponse> * m,
                          uint32_t key, ::business_api::v1::SendSecretRevealedResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_business_api_v1_SendSecretRevealedResponse_iter(
      const google::protobuf::Map<uint32_t, ::business_api::v1::SendSecretRevealedResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_business_api_v1_SendSecretRevealedResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::business_api::v1::SendSecretRevealedResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::business_api::v1::SendSecretRevealedResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::business_api::v1::SendSecretRevealedResponse>* __rust_proto_thunk__Map_i64_business_api_v1_SendSecretRevealedResponse_new() {
    return new google::protobuf::Map<int64_t, ::business_api::v1::SendSecretRevealedResponse>();
  }
  void __rust_proto_thunk__Map_i64_business_api_v1_SendSecretRevealedResponse_free(const google::protobuf::Map<int64_t, ::business_api::v1::SendSecretRevealedResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_business_api_v1_SendSecretRevealedResponse_clear(google::protobuf::Map<int64_t, ::business_api::v1::SendSecretRevealedResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_business_api_v1_SendSecretRevealedResponse_size(const google::protobuf::Map<int64_t, ::business_api::v1::SendSecretRevealedResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_business_api_v1_SendSecretRevealedResponse_insert(google::protobuf::Map<int64_t, ::business_api::v1::SendSecretRevealedResponse> * m,
                          int64_t key, ::business_api::v1::SendSecretRevealedResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_business_api_v1_SendSecretRevealedResponse_get(const google::protobuf::Map<int64_t, ::business_api::v1::SendSecretRevealedResponse>* m,
                       int64_t key, const ::business_api::v1::SendSecretRevealedResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::SendSecretRevealedResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_business_api_v1_SendSecretRevealedResponse_remove(google::protobuf::Map<int64_t, ::business_api::v1::SendSecretRevealedResponse> * m,
                          int64_t key, ::business_api::v1::SendSecretRevealedResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_business_api_v1_SendSecretRevealedResponse_iter(
      const google::protobuf::Map<int64_t, ::business_api::v1::SendSecretRevealedResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_business_api_v1_SendSecretRevealedResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::business_api::v1::SendSecretRevealedResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::business_api::v1::SendSecretRevealedResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::business_api::v1::SendSecretRevealedResponse>* __rust_proto_thunk__Map_u64_business_api_v1_SendSecretRevealedResponse_new() {
    return new google::protobuf::Map<uint64_t, ::business_api::v1::SendSecretRevealedResponse>();
  }
  void __rust_proto_thunk__Map_u64_business_api_v1_SendSecretRevealedResponse_free(const google::protobuf::Map<uint64_t, ::business_api::v1::SendSecretRevealedResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_business_api_v1_SendSecretRevealedResponse_clear(google::protobuf::Map<uint64_t, ::business_api::v1::SendSecretRevealedResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_business_api_v1_SendSecretRevealedResponse_size(const google::protobuf::Map<uint64_t, ::business_api::v1::SendSecretRevealedResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_business_api_v1_SendSecretRevealedResponse_insert(google::protobuf::Map<uint64_t, ::business_api::v1::SendSecretRevealedResponse> * m,
                          uint64_t key, ::business_api::v1::SendSecretRevealedResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_business_api_v1_SendSecretRevealedResponse_get(const google::protobuf::Map<uint64_t, ::business_api::v1::SendSecretRevealedResponse>* m,
                       uint64_t key, const ::business_api::v1::SendSecretRevealedResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::SendSecretRevealedResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_business_api_v1_SendSecretRevealedResponse_remove(google::protobuf::Map<uint64_t, ::business_api::v1::SendSecretRevealedResponse> * m,
                          uint64_t key, ::business_api::v1::SendSecretRevealedResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_business_api_v1_SendSecretRevealedResponse_iter(
      const google::protobuf::Map<uint64_t, ::business_api::v1::SendSecretRevealedResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_business_api_v1_SendSecretRevealedResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::business_api::v1::SendSecretRevealedResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::business_api::v1::SendSecretRevealedResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::business_api::v1::SendSecretRevealedResponse>* __rust_proto_thunk__Map_bool_business_api_v1_SendSecretRevealedResponse_new() {
    return new google::protobuf::Map<bool, ::business_api::v1::SendSecretRevealedResponse>();
  }
  void __rust_proto_thunk__Map_bool_business_api_v1_SendSecretRevealedResponse_free(const google::protobuf::Map<bool, ::business_api::v1::SendSecretRevealedResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_business_api_v1_SendSecretRevealedResponse_clear(google::protobuf::Map<bool, ::business_api::v1::SendSecretRevealedResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_business_api_v1_SendSecretRevealedResponse_size(const google::protobuf::Map<bool, ::business_api::v1::SendSecretRevealedResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_business_api_v1_SendSecretRevealedResponse_insert(google::protobuf::Map<bool, ::business_api::v1::SendSecretRevealedResponse> * m,
                          bool key, ::business_api::v1::SendSecretRevealedResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_business_api_v1_SendSecretRevealedResponse_get(const google::protobuf::Map<bool, ::business_api::v1::SendSecretRevealedResponse>* m,
                       bool key, const ::business_api::v1::SendSecretRevealedResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::SendSecretRevealedResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_business_api_v1_SendSecretRevealedResponse_remove(google::protobuf::Map<bool, ::business_api::v1::SendSecretRevealedResponse> * m,
                          bool key, ::business_api::v1::SendSecretRevealedResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_business_api_v1_SendSecretRevealedResponse_iter(
      const google::protobuf::Map<bool, ::business_api::v1::SendSecretRevealedResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_business_api_v1_SendSecretRevealedResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::business_api::v1::SendSecretRevealedResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::business_api::v1::SendSecretRevealedResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::business_api::v1::SendSecretRevealedResponse>* __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendSecretRevealedResponse_new() {
    return new google::protobuf::Map<std::string, ::business_api::v1::SendSecretRevealedResponse>();
  }
  void __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendSecretRevealedResponse_free(const google::protobuf::Map<std::string, ::business_api::v1::SendSecretRevealedResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendSecretRevealedResponse_clear(google::protobuf::Map<std::string, ::business_api::v1::SendSecretRevealedResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendSecretRevealedResponse_size(const google::protobuf::Map<std::string, ::business_api::v1::SendSecretRevealedResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendSecretRevealedResponse_insert(google::protobuf::Map<std::string, ::business_api::v1::SendSecretRevealedResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::business_api::v1::SendSecretRevealedResponse value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendSecretRevealedResponse_get(const google::protobuf::Map<std::string, ::business_api::v1::SendSecretRevealedResponse>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::business_api::v1::SendSecretRevealedResponse** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::business_api::v1::SendSecretRevealedResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendSecretRevealedResponse_remove(google::protobuf::Map<std::string, ::business_api::v1::SendSecretRevealedResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::business_api::v1::SendSecretRevealedResponse * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendSecretRevealedResponse_iter(
      const google::protobuf::Map<std::string, ::business_api::v1::SendSecretRevealedResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_business_api_v1_SendSecretRevealedResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::business_api::v1::SendSecretRevealedResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::business_api::v1::SendSecretRevealedResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

