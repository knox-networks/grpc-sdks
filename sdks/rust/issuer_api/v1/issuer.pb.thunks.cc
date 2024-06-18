#include "issuer_api/v1/issuer.proto.h"
#include "google/protobuf/rust/cpp_kernel/cpp_api.h"
// issuer_api.v1.IssueRequest
        // clang-format off
extern "C" {
void* __rust_proto_thunk__issuer_api_v1_IssueRequest_new() { return new ::issuer_api::v1::IssueRequest(); }
void __rust_proto_thunk__issuer_api_v1_IssueRequest_delete(void* ptr) { delete static_cast<::issuer_api::v1::IssueRequest*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__issuer_api_v1_IssueRequest_serialize(::issuer_api::v1::IssueRequest* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__issuer_api_v1_IssueRequest_parse(::issuer_api::v1::IssueRequest* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__issuer_api_v1_IssueRequest_copy_from(::issuer_api::v1::IssueRequest* dst, const ::issuer_api::v1::IssueRequest* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__issuer_api_v1_IssueRequest_repeated_len(google::protobuf::RepeatedPtrField<::issuer_api::v1::IssueRequest>* field) {
  return field->size();
}
const ::issuer_api::v1::IssueRequest& __rust_proto_thunk__issuer_api_v1_IssueRequest_repeated_get(
  google::protobuf::RepeatedPtrField<::issuer_api::v1::IssueRequest>* field,
  size_t index) {
  return field->Get(index);
}
::issuer_api::v1::IssueRequest* __rust_proto_thunk__issuer_api_v1_IssueRequest_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::issuer_api::v1::IssueRequest>* field,
  size_t index) {
  return field->Mutable(index);
}
::issuer_api::v1::IssueRequest* __rust_proto_thunk__issuer_api_v1_IssueRequest_repeated_add(google::protobuf::RepeatedPtrField<::issuer_api::v1::IssueRequest>* field) {
  return field->Add();
}
void __rust_proto_thunk__issuer_api_v1_IssueRequest_repeated_clear(google::protobuf::RepeatedPtrField<::issuer_api::v1::IssueRequest>* field) {
  field->Clear();
}
void __rust_proto_thunk__issuer_api_v1_IssueRequest_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::issuer_api::v1::IssueRequest>& dst,
  const google::protobuf::RepeatedPtrField<::issuer_api::v1::IssueRequest>& src) {
  dst = src;
}

const void* __rust_proto_thunk__issuer_api_v1_IssueRequest_get_authentication_signature(::issuer_api::v1::IssueRequest* msg) {
  return static_cast<const void*>(&msg->authentication_signature());
}
void* __rust_proto_thunk__issuer_api_v1_IssueRequest_get_mut_authentication_signature(::issuer_api::v1::IssueRequest* msg) {
  return static_cast<void*>(msg->mutable_authentication_signature());
}
void __rust_proto_thunk__issuer_api_v1_IssueRequest_clear_authentication_signature(::issuer_api::v1::IssueRequest* msg) { msg->clear_authentication_signature(); }
bool __rust_proto_thunk__issuer_api_v1_IssueRequest_has_authentication_signature(::issuer_api::v1::IssueRequest* msg) { return msg->has_authentication_signature(); }

const void* __rust_proto_thunk__issuer_api_v1_IssueRequest_get_amount(::issuer_api::v1::IssueRequest* msg) {
  return static_cast<const void*>(&msg->amount());
}
void* __rust_proto_thunk__issuer_api_v1_IssueRequest_get_mut_amount(::issuer_api::v1::IssueRequest* msg) {
  return static_cast<void*>(msg->mutable_amount());
}
void __rust_proto_thunk__issuer_api_v1_IssueRequest_clear_amount(::issuer_api::v1::IssueRequest* msg) { msg->clear_amount(); }
bool __rust_proto_thunk__issuer_api_v1_IssueRequest_has_amount(::issuer_api::v1::IssueRequest* msg) { return msg->has_amount(); }

const void* __rust_proto_thunk__issuer_api_v1_IssueRequest_get_authority(::issuer_api::v1::IssueRequest* msg) {
  return static_cast<const void*>(&msg->authority());
}
void* __rust_proto_thunk__issuer_api_v1_IssueRequest_get_mut_authority(::issuer_api::v1::IssueRequest* msg) {
  return static_cast<void*>(msg->mutable_authority());
}
void __rust_proto_thunk__issuer_api_v1_IssueRequest_clear_authority(::issuer_api::v1::IssueRequest* msg) { msg->clear_authority(); }
bool __rust_proto_thunk__issuer_api_v1_IssueRequest_has_authority(::issuer_api::v1::IssueRequest* msg) { return msg->has_authority(); }

const void* __rust_proto_thunk__issuer_api_v1_IssueRequest_get_recipient(::issuer_api::v1::IssueRequest* msg) {
  return static_cast<const void*>(&msg->recipient());
}
void* __rust_proto_thunk__issuer_api_v1_IssueRequest_get_mut_recipient(::issuer_api::v1::IssueRequest* msg) {
  return static_cast<void*>(msg->mutable_recipient());
}
void __rust_proto_thunk__issuer_api_v1_IssueRequest_clear_recipient(::issuer_api::v1::IssueRequest* msg) { msg->clear_recipient(); }
bool __rust_proto_thunk__issuer_api_v1_IssueRequest_has_recipient(::issuer_api::v1::IssueRequest* msg) { return msg->has_recipient(); }

const void* __rust_proto_thunk__issuer_api_v1_IssueRequest_get_distribution(::issuer_api::v1::IssueRequest* msg) {
  return static_cast<const void*>(&msg->distribution());
}
void* __rust_proto_thunk__issuer_api_v1_IssueRequest_get_mut_distribution(::issuer_api::v1::IssueRequest* msg) {
  return static_cast<void*>(msg->mutable_distribution());
}
void __rust_proto_thunk__issuer_api_v1_IssueRequest_clear_distribution(::issuer_api::v1::IssueRequest* msg) { msg->clear_distribution(); }
bool __rust_proto_thunk__issuer_api_v1_IssueRequest_has_distribution(::issuer_api::v1::IssueRequest* msg) { return msg->has_distribution(); }

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__issuer_api_v1_IssueRequest_get_session(::issuer_api::v1::IssueRequest* msg) {
  absl::string_view val = msg->session();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__issuer_api_v1_IssueRequest_set_session(::issuer_api::v1::IssueRequest* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_session(absl::string_view(s.ptr, s.len));
}

::common::SignatureSystem __rust_proto_thunk__issuer_api_v1_IssueRequest_get_signature_system(::issuer_api::v1::IssueRequest* msg) { return msg->signature_system(); }
void __rust_proto_thunk__issuer_api_v1_IssueRequest_set_signature_system(::issuer_api::v1::IssueRequest* msg, ::common::SignatureSystem val) {
  msg->set_signature_system(val);
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::issuer_api::v1::IssueRequest>* __rust_proto_thunk__Map_i32_issuer_api_v1_IssueRequest_new() {
    return new google::protobuf::Map<int32_t, ::issuer_api::v1::IssueRequest>();
  }
  void __rust_proto_thunk__Map_i32_issuer_api_v1_IssueRequest_free(const google::protobuf::Map<int32_t, ::issuer_api::v1::IssueRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_issuer_api_v1_IssueRequest_clear(google::protobuf::Map<int32_t, ::issuer_api::v1::IssueRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_issuer_api_v1_IssueRequest_size(const google::protobuf::Map<int32_t, ::issuer_api::v1::IssueRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_issuer_api_v1_IssueRequest_insert(google::protobuf::Map<int32_t, ::issuer_api::v1::IssueRequest> * m,
                          int32_t key, ::issuer_api::v1::IssueRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_issuer_api_v1_IssueRequest_get(const google::protobuf::Map<int32_t, ::issuer_api::v1::IssueRequest>* m,
                       int32_t key, const ::issuer_api::v1::IssueRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::IssueRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_issuer_api_v1_IssueRequest_remove(google::protobuf::Map<int32_t, ::issuer_api::v1::IssueRequest> * m,
                          int32_t key, ::issuer_api::v1::IssueRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_issuer_api_v1_IssueRequest_iter(
      const google::protobuf::Map<int32_t, ::issuer_api::v1::IssueRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_issuer_api_v1_IssueRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::issuer_api::v1::IssueRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::issuer_api::v1::IssueRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::issuer_api::v1::IssueRequest>* __rust_proto_thunk__Map_u32_issuer_api_v1_IssueRequest_new() {
    return new google::protobuf::Map<uint32_t, ::issuer_api::v1::IssueRequest>();
  }
  void __rust_proto_thunk__Map_u32_issuer_api_v1_IssueRequest_free(const google::protobuf::Map<uint32_t, ::issuer_api::v1::IssueRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_issuer_api_v1_IssueRequest_clear(google::protobuf::Map<uint32_t, ::issuer_api::v1::IssueRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_issuer_api_v1_IssueRequest_size(const google::protobuf::Map<uint32_t, ::issuer_api::v1::IssueRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_issuer_api_v1_IssueRequest_insert(google::protobuf::Map<uint32_t, ::issuer_api::v1::IssueRequest> * m,
                          uint32_t key, ::issuer_api::v1::IssueRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_issuer_api_v1_IssueRequest_get(const google::protobuf::Map<uint32_t, ::issuer_api::v1::IssueRequest>* m,
                       uint32_t key, const ::issuer_api::v1::IssueRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::IssueRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_issuer_api_v1_IssueRequest_remove(google::protobuf::Map<uint32_t, ::issuer_api::v1::IssueRequest> * m,
                          uint32_t key, ::issuer_api::v1::IssueRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_issuer_api_v1_IssueRequest_iter(
      const google::protobuf::Map<uint32_t, ::issuer_api::v1::IssueRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_issuer_api_v1_IssueRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::issuer_api::v1::IssueRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::issuer_api::v1::IssueRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::issuer_api::v1::IssueRequest>* __rust_proto_thunk__Map_i64_issuer_api_v1_IssueRequest_new() {
    return new google::protobuf::Map<int64_t, ::issuer_api::v1::IssueRequest>();
  }
  void __rust_proto_thunk__Map_i64_issuer_api_v1_IssueRequest_free(const google::protobuf::Map<int64_t, ::issuer_api::v1::IssueRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_issuer_api_v1_IssueRequest_clear(google::protobuf::Map<int64_t, ::issuer_api::v1::IssueRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_issuer_api_v1_IssueRequest_size(const google::protobuf::Map<int64_t, ::issuer_api::v1::IssueRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_issuer_api_v1_IssueRequest_insert(google::protobuf::Map<int64_t, ::issuer_api::v1::IssueRequest> * m,
                          int64_t key, ::issuer_api::v1::IssueRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_issuer_api_v1_IssueRequest_get(const google::protobuf::Map<int64_t, ::issuer_api::v1::IssueRequest>* m,
                       int64_t key, const ::issuer_api::v1::IssueRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::IssueRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_issuer_api_v1_IssueRequest_remove(google::protobuf::Map<int64_t, ::issuer_api::v1::IssueRequest> * m,
                          int64_t key, ::issuer_api::v1::IssueRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_issuer_api_v1_IssueRequest_iter(
      const google::protobuf::Map<int64_t, ::issuer_api::v1::IssueRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_issuer_api_v1_IssueRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::issuer_api::v1::IssueRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::issuer_api::v1::IssueRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::issuer_api::v1::IssueRequest>* __rust_proto_thunk__Map_u64_issuer_api_v1_IssueRequest_new() {
    return new google::protobuf::Map<uint64_t, ::issuer_api::v1::IssueRequest>();
  }
  void __rust_proto_thunk__Map_u64_issuer_api_v1_IssueRequest_free(const google::protobuf::Map<uint64_t, ::issuer_api::v1::IssueRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_issuer_api_v1_IssueRequest_clear(google::protobuf::Map<uint64_t, ::issuer_api::v1::IssueRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_issuer_api_v1_IssueRequest_size(const google::protobuf::Map<uint64_t, ::issuer_api::v1::IssueRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_issuer_api_v1_IssueRequest_insert(google::protobuf::Map<uint64_t, ::issuer_api::v1::IssueRequest> * m,
                          uint64_t key, ::issuer_api::v1::IssueRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_issuer_api_v1_IssueRequest_get(const google::protobuf::Map<uint64_t, ::issuer_api::v1::IssueRequest>* m,
                       uint64_t key, const ::issuer_api::v1::IssueRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::IssueRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_issuer_api_v1_IssueRequest_remove(google::protobuf::Map<uint64_t, ::issuer_api::v1::IssueRequest> * m,
                          uint64_t key, ::issuer_api::v1::IssueRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_issuer_api_v1_IssueRequest_iter(
      const google::protobuf::Map<uint64_t, ::issuer_api::v1::IssueRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_issuer_api_v1_IssueRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::issuer_api::v1::IssueRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::issuer_api::v1::IssueRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::issuer_api::v1::IssueRequest>* __rust_proto_thunk__Map_bool_issuer_api_v1_IssueRequest_new() {
    return new google::protobuf::Map<bool, ::issuer_api::v1::IssueRequest>();
  }
  void __rust_proto_thunk__Map_bool_issuer_api_v1_IssueRequest_free(const google::protobuf::Map<bool, ::issuer_api::v1::IssueRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_issuer_api_v1_IssueRequest_clear(google::protobuf::Map<bool, ::issuer_api::v1::IssueRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_issuer_api_v1_IssueRequest_size(const google::protobuf::Map<bool, ::issuer_api::v1::IssueRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_issuer_api_v1_IssueRequest_insert(google::protobuf::Map<bool, ::issuer_api::v1::IssueRequest> * m,
                          bool key, ::issuer_api::v1::IssueRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_issuer_api_v1_IssueRequest_get(const google::protobuf::Map<bool, ::issuer_api::v1::IssueRequest>* m,
                       bool key, const ::issuer_api::v1::IssueRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::IssueRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_issuer_api_v1_IssueRequest_remove(google::protobuf::Map<bool, ::issuer_api::v1::IssueRequest> * m,
                          bool key, ::issuer_api::v1::IssueRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_issuer_api_v1_IssueRequest_iter(
      const google::protobuf::Map<bool, ::issuer_api::v1::IssueRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_issuer_api_v1_IssueRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::issuer_api::v1::IssueRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::issuer_api::v1::IssueRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::issuer_api::v1::IssueRequest>* __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssueRequest_new() {
    return new google::protobuf::Map<std::string, ::issuer_api::v1::IssueRequest>();
  }
  void __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssueRequest_free(const google::protobuf::Map<std::string, ::issuer_api::v1::IssueRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssueRequest_clear(google::protobuf::Map<std::string, ::issuer_api::v1::IssueRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssueRequest_size(const google::protobuf::Map<std::string, ::issuer_api::v1::IssueRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssueRequest_insert(google::protobuf::Map<std::string, ::issuer_api::v1::IssueRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::issuer_api::v1::IssueRequest value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssueRequest_get(const google::protobuf::Map<std::string, ::issuer_api::v1::IssueRequest>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::issuer_api::v1::IssueRequest** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::IssueRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssueRequest_remove(google::protobuf::Map<std::string, ::issuer_api::v1::IssueRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::issuer_api::v1::IssueRequest * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssueRequest_iter(
      const google::protobuf::Map<std::string, ::issuer_api::v1::IssueRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssueRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::issuer_api::v1::IssueRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::issuer_api::v1::IssueRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// issuer_api.v1.IssueResponse
        // clang-format off
extern "C" {
void* __rust_proto_thunk__issuer_api_v1_IssueResponse_new() { return new ::issuer_api::v1::IssueResponse(); }
void __rust_proto_thunk__issuer_api_v1_IssueResponse_delete(void* ptr) { delete static_cast<::issuer_api::v1::IssueResponse*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__issuer_api_v1_IssueResponse_serialize(::issuer_api::v1::IssueResponse* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__issuer_api_v1_IssueResponse_parse(::issuer_api::v1::IssueResponse* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__issuer_api_v1_IssueResponse_copy_from(::issuer_api::v1::IssueResponse* dst, const ::issuer_api::v1::IssueResponse* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__issuer_api_v1_IssueResponse_repeated_len(google::protobuf::RepeatedPtrField<::issuer_api::v1::IssueResponse>* field) {
  return field->size();
}
const ::issuer_api::v1::IssueResponse& __rust_proto_thunk__issuer_api_v1_IssueResponse_repeated_get(
  google::protobuf::RepeatedPtrField<::issuer_api::v1::IssueResponse>* field,
  size_t index) {
  return field->Get(index);
}
::issuer_api::v1::IssueResponse* __rust_proto_thunk__issuer_api_v1_IssueResponse_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::issuer_api::v1::IssueResponse>* field,
  size_t index) {
  return field->Mutable(index);
}
::issuer_api::v1::IssueResponse* __rust_proto_thunk__issuer_api_v1_IssueResponse_repeated_add(google::protobuf::RepeatedPtrField<::issuer_api::v1::IssueResponse>* field) {
  return field->Add();
}
void __rust_proto_thunk__issuer_api_v1_IssueResponse_repeated_clear(google::protobuf::RepeatedPtrField<::issuer_api::v1::IssueResponse>* field) {
  field->Clear();
}
void __rust_proto_thunk__issuer_api_v1_IssueResponse_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::issuer_api::v1::IssueResponse>& dst,
  const google::protobuf::RepeatedPtrField<::issuer_api::v1::IssueResponse>& src) {
  dst = src;
}

::issuer_api::v1::IssueResponse_Status __rust_proto_thunk__issuer_api_v1_IssueResponse_get_status(::issuer_api::v1::IssueResponse* msg) { return msg->status(); }
void __rust_proto_thunk__issuer_api_v1_IssueResponse_set_status(::issuer_api::v1::IssueResponse* msg, ::issuer_api::v1::IssueResponse_Status val) {
  msg->set_status(val);
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__issuer_api_v1_IssueResponse_get_uetr(::issuer_api::v1::IssueResponse* msg) {
  absl::string_view val = msg->uetr();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__issuer_api_v1_IssueResponse_set_uetr(::issuer_api::v1::IssueResponse* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_uetr(absl::string_view(s.ptr, s.len));
}

::uint32_t __rust_proto_thunk__issuer_api_v1_IssueResponse_get_num_promissory_files(::issuer_api::v1::IssueResponse* msg) { return msg->num_promissory_files(); }
void __rust_proto_thunk__issuer_api_v1_IssueResponse_set_num_promissory_files(::issuer_api::v1::IssueResponse* msg, ::uint32_t val) {
  msg->set_num_promissory_files(val);
}


}  // extern "C"
// clang-format on

extern "C" {
  __PB_RUST_EXPOSE_SCALAR_MAP_METHODS_FOR_VALUE_TYPE(
      ::issuer_api::v1::IssueResponse_Status, issuer_api_v1_IssueResponse_Status, ::issuer_api::v1::IssueResponse_Status, value, cpp_value)
}
extern "C" {
  const google::protobuf::Map<int32_t, ::issuer_api::v1::IssueResponse>* __rust_proto_thunk__Map_i32_issuer_api_v1_IssueResponse_new() {
    return new google::protobuf::Map<int32_t, ::issuer_api::v1::IssueResponse>();
  }
  void __rust_proto_thunk__Map_i32_issuer_api_v1_IssueResponse_free(const google::protobuf::Map<int32_t, ::issuer_api::v1::IssueResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_issuer_api_v1_IssueResponse_clear(google::protobuf::Map<int32_t, ::issuer_api::v1::IssueResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_issuer_api_v1_IssueResponse_size(const google::protobuf::Map<int32_t, ::issuer_api::v1::IssueResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_issuer_api_v1_IssueResponse_insert(google::protobuf::Map<int32_t, ::issuer_api::v1::IssueResponse> * m,
                          int32_t key, ::issuer_api::v1::IssueResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_issuer_api_v1_IssueResponse_get(const google::protobuf::Map<int32_t, ::issuer_api::v1::IssueResponse>* m,
                       int32_t key, const ::issuer_api::v1::IssueResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::IssueResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_issuer_api_v1_IssueResponse_remove(google::protobuf::Map<int32_t, ::issuer_api::v1::IssueResponse> * m,
                          int32_t key, ::issuer_api::v1::IssueResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_issuer_api_v1_IssueResponse_iter(
      const google::protobuf::Map<int32_t, ::issuer_api::v1::IssueResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_issuer_api_v1_IssueResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::issuer_api::v1::IssueResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::issuer_api::v1::IssueResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::issuer_api::v1::IssueResponse>* __rust_proto_thunk__Map_u32_issuer_api_v1_IssueResponse_new() {
    return new google::protobuf::Map<uint32_t, ::issuer_api::v1::IssueResponse>();
  }
  void __rust_proto_thunk__Map_u32_issuer_api_v1_IssueResponse_free(const google::protobuf::Map<uint32_t, ::issuer_api::v1::IssueResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_issuer_api_v1_IssueResponse_clear(google::protobuf::Map<uint32_t, ::issuer_api::v1::IssueResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_issuer_api_v1_IssueResponse_size(const google::protobuf::Map<uint32_t, ::issuer_api::v1::IssueResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_issuer_api_v1_IssueResponse_insert(google::protobuf::Map<uint32_t, ::issuer_api::v1::IssueResponse> * m,
                          uint32_t key, ::issuer_api::v1::IssueResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_issuer_api_v1_IssueResponse_get(const google::protobuf::Map<uint32_t, ::issuer_api::v1::IssueResponse>* m,
                       uint32_t key, const ::issuer_api::v1::IssueResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::IssueResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_issuer_api_v1_IssueResponse_remove(google::protobuf::Map<uint32_t, ::issuer_api::v1::IssueResponse> * m,
                          uint32_t key, ::issuer_api::v1::IssueResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_issuer_api_v1_IssueResponse_iter(
      const google::protobuf::Map<uint32_t, ::issuer_api::v1::IssueResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_issuer_api_v1_IssueResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::issuer_api::v1::IssueResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::issuer_api::v1::IssueResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::issuer_api::v1::IssueResponse>* __rust_proto_thunk__Map_i64_issuer_api_v1_IssueResponse_new() {
    return new google::protobuf::Map<int64_t, ::issuer_api::v1::IssueResponse>();
  }
  void __rust_proto_thunk__Map_i64_issuer_api_v1_IssueResponse_free(const google::protobuf::Map<int64_t, ::issuer_api::v1::IssueResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_issuer_api_v1_IssueResponse_clear(google::protobuf::Map<int64_t, ::issuer_api::v1::IssueResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_issuer_api_v1_IssueResponse_size(const google::protobuf::Map<int64_t, ::issuer_api::v1::IssueResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_issuer_api_v1_IssueResponse_insert(google::protobuf::Map<int64_t, ::issuer_api::v1::IssueResponse> * m,
                          int64_t key, ::issuer_api::v1::IssueResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_issuer_api_v1_IssueResponse_get(const google::protobuf::Map<int64_t, ::issuer_api::v1::IssueResponse>* m,
                       int64_t key, const ::issuer_api::v1::IssueResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::IssueResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_issuer_api_v1_IssueResponse_remove(google::protobuf::Map<int64_t, ::issuer_api::v1::IssueResponse> * m,
                          int64_t key, ::issuer_api::v1::IssueResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_issuer_api_v1_IssueResponse_iter(
      const google::protobuf::Map<int64_t, ::issuer_api::v1::IssueResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_issuer_api_v1_IssueResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::issuer_api::v1::IssueResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::issuer_api::v1::IssueResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::issuer_api::v1::IssueResponse>* __rust_proto_thunk__Map_u64_issuer_api_v1_IssueResponse_new() {
    return new google::protobuf::Map<uint64_t, ::issuer_api::v1::IssueResponse>();
  }
  void __rust_proto_thunk__Map_u64_issuer_api_v1_IssueResponse_free(const google::protobuf::Map<uint64_t, ::issuer_api::v1::IssueResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_issuer_api_v1_IssueResponse_clear(google::protobuf::Map<uint64_t, ::issuer_api::v1::IssueResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_issuer_api_v1_IssueResponse_size(const google::protobuf::Map<uint64_t, ::issuer_api::v1::IssueResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_issuer_api_v1_IssueResponse_insert(google::protobuf::Map<uint64_t, ::issuer_api::v1::IssueResponse> * m,
                          uint64_t key, ::issuer_api::v1::IssueResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_issuer_api_v1_IssueResponse_get(const google::protobuf::Map<uint64_t, ::issuer_api::v1::IssueResponse>* m,
                       uint64_t key, const ::issuer_api::v1::IssueResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::IssueResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_issuer_api_v1_IssueResponse_remove(google::protobuf::Map<uint64_t, ::issuer_api::v1::IssueResponse> * m,
                          uint64_t key, ::issuer_api::v1::IssueResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_issuer_api_v1_IssueResponse_iter(
      const google::protobuf::Map<uint64_t, ::issuer_api::v1::IssueResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_issuer_api_v1_IssueResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::issuer_api::v1::IssueResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::issuer_api::v1::IssueResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::issuer_api::v1::IssueResponse>* __rust_proto_thunk__Map_bool_issuer_api_v1_IssueResponse_new() {
    return new google::protobuf::Map<bool, ::issuer_api::v1::IssueResponse>();
  }
  void __rust_proto_thunk__Map_bool_issuer_api_v1_IssueResponse_free(const google::protobuf::Map<bool, ::issuer_api::v1::IssueResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_issuer_api_v1_IssueResponse_clear(google::protobuf::Map<bool, ::issuer_api::v1::IssueResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_issuer_api_v1_IssueResponse_size(const google::protobuf::Map<bool, ::issuer_api::v1::IssueResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_issuer_api_v1_IssueResponse_insert(google::protobuf::Map<bool, ::issuer_api::v1::IssueResponse> * m,
                          bool key, ::issuer_api::v1::IssueResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_issuer_api_v1_IssueResponse_get(const google::protobuf::Map<bool, ::issuer_api::v1::IssueResponse>* m,
                       bool key, const ::issuer_api::v1::IssueResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::IssueResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_issuer_api_v1_IssueResponse_remove(google::protobuf::Map<bool, ::issuer_api::v1::IssueResponse> * m,
                          bool key, ::issuer_api::v1::IssueResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_issuer_api_v1_IssueResponse_iter(
      const google::protobuf::Map<bool, ::issuer_api::v1::IssueResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_issuer_api_v1_IssueResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::issuer_api::v1::IssueResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::issuer_api::v1::IssueResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::issuer_api::v1::IssueResponse>* __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssueResponse_new() {
    return new google::protobuf::Map<std::string, ::issuer_api::v1::IssueResponse>();
  }
  void __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssueResponse_free(const google::protobuf::Map<std::string, ::issuer_api::v1::IssueResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssueResponse_clear(google::protobuf::Map<std::string, ::issuer_api::v1::IssueResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssueResponse_size(const google::protobuf::Map<std::string, ::issuer_api::v1::IssueResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssueResponse_insert(google::protobuf::Map<std::string, ::issuer_api::v1::IssueResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::issuer_api::v1::IssueResponse value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssueResponse_get(const google::protobuf::Map<std::string, ::issuer_api::v1::IssueResponse>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::issuer_api::v1::IssueResponse** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::IssueResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssueResponse_remove(google::protobuf::Map<std::string, ::issuer_api::v1::IssueResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::issuer_api::v1::IssueResponse * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssueResponse_iter(
      const google::protobuf::Map<std::string, ::issuer_api::v1::IssueResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssueResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::issuer_api::v1::IssueResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::issuer_api::v1::IssueResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// issuer_api.v1.AuthenticateRequest
        // clang-format off
extern "C" {
void* __rust_proto_thunk__issuer_api_v1_AuthenticateRequest_new() { return new ::issuer_api::v1::AuthenticateRequest(); }
void __rust_proto_thunk__issuer_api_v1_AuthenticateRequest_delete(void* ptr) { delete static_cast<::issuer_api::v1::AuthenticateRequest*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__issuer_api_v1_AuthenticateRequest_serialize(::issuer_api::v1::AuthenticateRequest* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__issuer_api_v1_AuthenticateRequest_parse(::issuer_api::v1::AuthenticateRequest* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__issuer_api_v1_AuthenticateRequest_copy_from(::issuer_api::v1::AuthenticateRequest* dst, const ::issuer_api::v1::AuthenticateRequest* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__issuer_api_v1_AuthenticateRequest_repeated_len(google::protobuf::RepeatedPtrField<::issuer_api::v1::AuthenticateRequest>* field) {
  return field->size();
}
const ::issuer_api::v1::AuthenticateRequest& __rust_proto_thunk__issuer_api_v1_AuthenticateRequest_repeated_get(
  google::protobuf::RepeatedPtrField<::issuer_api::v1::AuthenticateRequest>* field,
  size_t index) {
  return field->Get(index);
}
::issuer_api::v1::AuthenticateRequest* __rust_proto_thunk__issuer_api_v1_AuthenticateRequest_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::issuer_api::v1::AuthenticateRequest>* field,
  size_t index) {
  return field->Mutable(index);
}
::issuer_api::v1::AuthenticateRequest* __rust_proto_thunk__issuer_api_v1_AuthenticateRequest_repeated_add(google::protobuf::RepeatedPtrField<::issuer_api::v1::AuthenticateRequest>* field) {
  return field->Add();
}
void __rust_proto_thunk__issuer_api_v1_AuthenticateRequest_repeated_clear(google::protobuf::RepeatedPtrField<::issuer_api::v1::AuthenticateRequest>* field) {
  field->Clear();
}
void __rust_proto_thunk__issuer_api_v1_AuthenticateRequest_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::issuer_api::v1::AuthenticateRequest>& dst,
  const google::protobuf::RepeatedPtrField<::issuer_api::v1::AuthenticateRequest>& src) {
  dst = src;
}

const void* __rust_proto_thunk__issuer_api_v1_AuthenticateRequest_get_verifier(::issuer_api::v1::AuthenticateRequest* msg) {
  return static_cast<const void*>(&msg->verifier());
}
void* __rust_proto_thunk__issuer_api_v1_AuthenticateRequest_get_mut_verifier(::issuer_api::v1::AuthenticateRequest* msg) {
  return static_cast<void*>(msg->mutable_verifier());
}
void __rust_proto_thunk__issuer_api_v1_AuthenticateRequest_clear_verifier(::issuer_api::v1::AuthenticateRequest* msg) { msg->clear_verifier(); }
bool __rust_proto_thunk__issuer_api_v1_AuthenticateRequest_has_verifier(::issuer_api::v1::AuthenticateRequest* msg) { return msg->has_verifier(); }


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::issuer_api::v1::AuthenticateRequest>* __rust_proto_thunk__Map_i32_issuer_api_v1_AuthenticateRequest_new() {
    return new google::protobuf::Map<int32_t, ::issuer_api::v1::AuthenticateRequest>();
  }
  void __rust_proto_thunk__Map_i32_issuer_api_v1_AuthenticateRequest_free(const google::protobuf::Map<int32_t, ::issuer_api::v1::AuthenticateRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_issuer_api_v1_AuthenticateRequest_clear(google::protobuf::Map<int32_t, ::issuer_api::v1::AuthenticateRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_issuer_api_v1_AuthenticateRequest_size(const google::protobuf::Map<int32_t, ::issuer_api::v1::AuthenticateRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_issuer_api_v1_AuthenticateRequest_insert(google::protobuf::Map<int32_t, ::issuer_api::v1::AuthenticateRequest> * m,
                          int32_t key, ::issuer_api::v1::AuthenticateRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_issuer_api_v1_AuthenticateRequest_get(const google::protobuf::Map<int32_t, ::issuer_api::v1::AuthenticateRequest>* m,
                       int32_t key, const ::issuer_api::v1::AuthenticateRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::AuthenticateRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_issuer_api_v1_AuthenticateRequest_remove(google::protobuf::Map<int32_t, ::issuer_api::v1::AuthenticateRequest> * m,
                          int32_t key, ::issuer_api::v1::AuthenticateRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_issuer_api_v1_AuthenticateRequest_iter(
      const google::protobuf::Map<int32_t, ::issuer_api::v1::AuthenticateRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_issuer_api_v1_AuthenticateRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::issuer_api::v1::AuthenticateRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::issuer_api::v1::AuthenticateRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::issuer_api::v1::AuthenticateRequest>* __rust_proto_thunk__Map_u32_issuer_api_v1_AuthenticateRequest_new() {
    return new google::protobuf::Map<uint32_t, ::issuer_api::v1::AuthenticateRequest>();
  }
  void __rust_proto_thunk__Map_u32_issuer_api_v1_AuthenticateRequest_free(const google::protobuf::Map<uint32_t, ::issuer_api::v1::AuthenticateRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_issuer_api_v1_AuthenticateRequest_clear(google::protobuf::Map<uint32_t, ::issuer_api::v1::AuthenticateRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_issuer_api_v1_AuthenticateRequest_size(const google::protobuf::Map<uint32_t, ::issuer_api::v1::AuthenticateRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_issuer_api_v1_AuthenticateRequest_insert(google::protobuf::Map<uint32_t, ::issuer_api::v1::AuthenticateRequest> * m,
                          uint32_t key, ::issuer_api::v1::AuthenticateRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_issuer_api_v1_AuthenticateRequest_get(const google::protobuf::Map<uint32_t, ::issuer_api::v1::AuthenticateRequest>* m,
                       uint32_t key, const ::issuer_api::v1::AuthenticateRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::AuthenticateRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_issuer_api_v1_AuthenticateRequest_remove(google::protobuf::Map<uint32_t, ::issuer_api::v1::AuthenticateRequest> * m,
                          uint32_t key, ::issuer_api::v1::AuthenticateRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_issuer_api_v1_AuthenticateRequest_iter(
      const google::protobuf::Map<uint32_t, ::issuer_api::v1::AuthenticateRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_issuer_api_v1_AuthenticateRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::issuer_api::v1::AuthenticateRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::issuer_api::v1::AuthenticateRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::issuer_api::v1::AuthenticateRequest>* __rust_proto_thunk__Map_i64_issuer_api_v1_AuthenticateRequest_new() {
    return new google::protobuf::Map<int64_t, ::issuer_api::v1::AuthenticateRequest>();
  }
  void __rust_proto_thunk__Map_i64_issuer_api_v1_AuthenticateRequest_free(const google::protobuf::Map<int64_t, ::issuer_api::v1::AuthenticateRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_issuer_api_v1_AuthenticateRequest_clear(google::protobuf::Map<int64_t, ::issuer_api::v1::AuthenticateRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_issuer_api_v1_AuthenticateRequest_size(const google::protobuf::Map<int64_t, ::issuer_api::v1::AuthenticateRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_issuer_api_v1_AuthenticateRequest_insert(google::protobuf::Map<int64_t, ::issuer_api::v1::AuthenticateRequest> * m,
                          int64_t key, ::issuer_api::v1::AuthenticateRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_issuer_api_v1_AuthenticateRequest_get(const google::protobuf::Map<int64_t, ::issuer_api::v1::AuthenticateRequest>* m,
                       int64_t key, const ::issuer_api::v1::AuthenticateRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::AuthenticateRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_issuer_api_v1_AuthenticateRequest_remove(google::protobuf::Map<int64_t, ::issuer_api::v1::AuthenticateRequest> * m,
                          int64_t key, ::issuer_api::v1::AuthenticateRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_issuer_api_v1_AuthenticateRequest_iter(
      const google::protobuf::Map<int64_t, ::issuer_api::v1::AuthenticateRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_issuer_api_v1_AuthenticateRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::issuer_api::v1::AuthenticateRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::issuer_api::v1::AuthenticateRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::issuer_api::v1::AuthenticateRequest>* __rust_proto_thunk__Map_u64_issuer_api_v1_AuthenticateRequest_new() {
    return new google::protobuf::Map<uint64_t, ::issuer_api::v1::AuthenticateRequest>();
  }
  void __rust_proto_thunk__Map_u64_issuer_api_v1_AuthenticateRequest_free(const google::protobuf::Map<uint64_t, ::issuer_api::v1::AuthenticateRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_issuer_api_v1_AuthenticateRequest_clear(google::protobuf::Map<uint64_t, ::issuer_api::v1::AuthenticateRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_issuer_api_v1_AuthenticateRequest_size(const google::protobuf::Map<uint64_t, ::issuer_api::v1::AuthenticateRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_issuer_api_v1_AuthenticateRequest_insert(google::protobuf::Map<uint64_t, ::issuer_api::v1::AuthenticateRequest> * m,
                          uint64_t key, ::issuer_api::v1::AuthenticateRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_issuer_api_v1_AuthenticateRequest_get(const google::protobuf::Map<uint64_t, ::issuer_api::v1::AuthenticateRequest>* m,
                       uint64_t key, const ::issuer_api::v1::AuthenticateRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::AuthenticateRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_issuer_api_v1_AuthenticateRequest_remove(google::protobuf::Map<uint64_t, ::issuer_api::v1::AuthenticateRequest> * m,
                          uint64_t key, ::issuer_api::v1::AuthenticateRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_issuer_api_v1_AuthenticateRequest_iter(
      const google::protobuf::Map<uint64_t, ::issuer_api::v1::AuthenticateRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_issuer_api_v1_AuthenticateRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::issuer_api::v1::AuthenticateRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::issuer_api::v1::AuthenticateRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::issuer_api::v1::AuthenticateRequest>* __rust_proto_thunk__Map_bool_issuer_api_v1_AuthenticateRequest_new() {
    return new google::protobuf::Map<bool, ::issuer_api::v1::AuthenticateRequest>();
  }
  void __rust_proto_thunk__Map_bool_issuer_api_v1_AuthenticateRequest_free(const google::protobuf::Map<bool, ::issuer_api::v1::AuthenticateRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_issuer_api_v1_AuthenticateRequest_clear(google::protobuf::Map<bool, ::issuer_api::v1::AuthenticateRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_issuer_api_v1_AuthenticateRequest_size(const google::protobuf::Map<bool, ::issuer_api::v1::AuthenticateRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_issuer_api_v1_AuthenticateRequest_insert(google::protobuf::Map<bool, ::issuer_api::v1::AuthenticateRequest> * m,
                          bool key, ::issuer_api::v1::AuthenticateRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_issuer_api_v1_AuthenticateRequest_get(const google::protobuf::Map<bool, ::issuer_api::v1::AuthenticateRequest>* m,
                       bool key, const ::issuer_api::v1::AuthenticateRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::AuthenticateRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_issuer_api_v1_AuthenticateRequest_remove(google::protobuf::Map<bool, ::issuer_api::v1::AuthenticateRequest> * m,
                          bool key, ::issuer_api::v1::AuthenticateRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_issuer_api_v1_AuthenticateRequest_iter(
      const google::protobuf::Map<bool, ::issuer_api::v1::AuthenticateRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_issuer_api_v1_AuthenticateRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::issuer_api::v1::AuthenticateRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::issuer_api::v1::AuthenticateRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::issuer_api::v1::AuthenticateRequest>* __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AuthenticateRequest_new() {
    return new google::protobuf::Map<std::string, ::issuer_api::v1::AuthenticateRequest>();
  }
  void __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AuthenticateRequest_free(const google::protobuf::Map<std::string, ::issuer_api::v1::AuthenticateRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AuthenticateRequest_clear(google::protobuf::Map<std::string, ::issuer_api::v1::AuthenticateRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AuthenticateRequest_size(const google::protobuf::Map<std::string, ::issuer_api::v1::AuthenticateRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AuthenticateRequest_insert(google::protobuf::Map<std::string, ::issuer_api::v1::AuthenticateRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::issuer_api::v1::AuthenticateRequest value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AuthenticateRequest_get(const google::protobuf::Map<std::string, ::issuer_api::v1::AuthenticateRequest>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::issuer_api::v1::AuthenticateRequest** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::AuthenticateRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AuthenticateRequest_remove(google::protobuf::Map<std::string, ::issuer_api::v1::AuthenticateRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::issuer_api::v1::AuthenticateRequest * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AuthenticateRequest_iter(
      const google::protobuf::Map<std::string, ::issuer_api::v1::AuthenticateRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AuthenticateRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::issuer_api::v1::AuthenticateRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::issuer_api::v1::AuthenticateRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// issuer_api.v1.AuthenticateResponse
        // clang-format off
extern "C" {
void* __rust_proto_thunk__issuer_api_v1_AuthenticateResponse_new() { return new ::issuer_api::v1::AuthenticateResponse(); }
void __rust_proto_thunk__issuer_api_v1_AuthenticateResponse_delete(void* ptr) { delete static_cast<::issuer_api::v1::AuthenticateResponse*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__issuer_api_v1_AuthenticateResponse_serialize(::issuer_api::v1::AuthenticateResponse* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__issuer_api_v1_AuthenticateResponse_parse(::issuer_api::v1::AuthenticateResponse* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__issuer_api_v1_AuthenticateResponse_copy_from(::issuer_api::v1::AuthenticateResponse* dst, const ::issuer_api::v1::AuthenticateResponse* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__issuer_api_v1_AuthenticateResponse_repeated_len(google::protobuf::RepeatedPtrField<::issuer_api::v1::AuthenticateResponse>* field) {
  return field->size();
}
const ::issuer_api::v1::AuthenticateResponse& __rust_proto_thunk__issuer_api_v1_AuthenticateResponse_repeated_get(
  google::protobuf::RepeatedPtrField<::issuer_api::v1::AuthenticateResponse>* field,
  size_t index) {
  return field->Get(index);
}
::issuer_api::v1::AuthenticateResponse* __rust_proto_thunk__issuer_api_v1_AuthenticateResponse_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::issuer_api::v1::AuthenticateResponse>* field,
  size_t index) {
  return field->Mutable(index);
}
::issuer_api::v1::AuthenticateResponse* __rust_proto_thunk__issuer_api_v1_AuthenticateResponse_repeated_add(google::protobuf::RepeatedPtrField<::issuer_api::v1::AuthenticateResponse>* field) {
  return field->Add();
}
void __rust_proto_thunk__issuer_api_v1_AuthenticateResponse_repeated_clear(google::protobuf::RepeatedPtrField<::issuer_api::v1::AuthenticateResponse>* field) {
  field->Clear();
}
void __rust_proto_thunk__issuer_api_v1_AuthenticateResponse_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::issuer_api::v1::AuthenticateResponse>& dst,
  const google::protobuf::RepeatedPtrField<::issuer_api::v1::AuthenticateResponse>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__issuer_api_v1_AuthenticateResponse_get_challenge(::issuer_api::v1::AuthenticateResponse* msg) {
  absl::string_view val = msg->challenge();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__issuer_api_v1_AuthenticateResponse_set_challenge(::issuer_api::v1::AuthenticateResponse* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_challenge(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__issuer_api_v1_AuthenticateResponse_get_session(::issuer_api::v1::AuthenticateResponse* msg) {
  absl::string_view val = msg->session();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__issuer_api_v1_AuthenticateResponse_set_session(::issuer_api::v1::AuthenticateResponse* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_session(absl::string_view(s.ptr, s.len));
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::issuer_api::v1::AuthenticateResponse>* __rust_proto_thunk__Map_i32_issuer_api_v1_AuthenticateResponse_new() {
    return new google::protobuf::Map<int32_t, ::issuer_api::v1::AuthenticateResponse>();
  }
  void __rust_proto_thunk__Map_i32_issuer_api_v1_AuthenticateResponse_free(const google::protobuf::Map<int32_t, ::issuer_api::v1::AuthenticateResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_issuer_api_v1_AuthenticateResponse_clear(google::protobuf::Map<int32_t, ::issuer_api::v1::AuthenticateResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_issuer_api_v1_AuthenticateResponse_size(const google::protobuf::Map<int32_t, ::issuer_api::v1::AuthenticateResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_issuer_api_v1_AuthenticateResponse_insert(google::protobuf::Map<int32_t, ::issuer_api::v1::AuthenticateResponse> * m,
                          int32_t key, ::issuer_api::v1::AuthenticateResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_issuer_api_v1_AuthenticateResponse_get(const google::protobuf::Map<int32_t, ::issuer_api::v1::AuthenticateResponse>* m,
                       int32_t key, const ::issuer_api::v1::AuthenticateResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::AuthenticateResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_issuer_api_v1_AuthenticateResponse_remove(google::protobuf::Map<int32_t, ::issuer_api::v1::AuthenticateResponse> * m,
                          int32_t key, ::issuer_api::v1::AuthenticateResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_issuer_api_v1_AuthenticateResponse_iter(
      const google::protobuf::Map<int32_t, ::issuer_api::v1::AuthenticateResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_issuer_api_v1_AuthenticateResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::issuer_api::v1::AuthenticateResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::issuer_api::v1::AuthenticateResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::issuer_api::v1::AuthenticateResponse>* __rust_proto_thunk__Map_u32_issuer_api_v1_AuthenticateResponse_new() {
    return new google::protobuf::Map<uint32_t, ::issuer_api::v1::AuthenticateResponse>();
  }
  void __rust_proto_thunk__Map_u32_issuer_api_v1_AuthenticateResponse_free(const google::protobuf::Map<uint32_t, ::issuer_api::v1::AuthenticateResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_issuer_api_v1_AuthenticateResponse_clear(google::protobuf::Map<uint32_t, ::issuer_api::v1::AuthenticateResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_issuer_api_v1_AuthenticateResponse_size(const google::protobuf::Map<uint32_t, ::issuer_api::v1::AuthenticateResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_issuer_api_v1_AuthenticateResponse_insert(google::protobuf::Map<uint32_t, ::issuer_api::v1::AuthenticateResponse> * m,
                          uint32_t key, ::issuer_api::v1::AuthenticateResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_issuer_api_v1_AuthenticateResponse_get(const google::protobuf::Map<uint32_t, ::issuer_api::v1::AuthenticateResponse>* m,
                       uint32_t key, const ::issuer_api::v1::AuthenticateResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::AuthenticateResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_issuer_api_v1_AuthenticateResponse_remove(google::protobuf::Map<uint32_t, ::issuer_api::v1::AuthenticateResponse> * m,
                          uint32_t key, ::issuer_api::v1::AuthenticateResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_issuer_api_v1_AuthenticateResponse_iter(
      const google::protobuf::Map<uint32_t, ::issuer_api::v1::AuthenticateResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_issuer_api_v1_AuthenticateResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::issuer_api::v1::AuthenticateResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::issuer_api::v1::AuthenticateResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::issuer_api::v1::AuthenticateResponse>* __rust_proto_thunk__Map_i64_issuer_api_v1_AuthenticateResponse_new() {
    return new google::protobuf::Map<int64_t, ::issuer_api::v1::AuthenticateResponse>();
  }
  void __rust_proto_thunk__Map_i64_issuer_api_v1_AuthenticateResponse_free(const google::protobuf::Map<int64_t, ::issuer_api::v1::AuthenticateResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_issuer_api_v1_AuthenticateResponse_clear(google::protobuf::Map<int64_t, ::issuer_api::v1::AuthenticateResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_issuer_api_v1_AuthenticateResponse_size(const google::protobuf::Map<int64_t, ::issuer_api::v1::AuthenticateResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_issuer_api_v1_AuthenticateResponse_insert(google::protobuf::Map<int64_t, ::issuer_api::v1::AuthenticateResponse> * m,
                          int64_t key, ::issuer_api::v1::AuthenticateResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_issuer_api_v1_AuthenticateResponse_get(const google::protobuf::Map<int64_t, ::issuer_api::v1::AuthenticateResponse>* m,
                       int64_t key, const ::issuer_api::v1::AuthenticateResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::AuthenticateResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_issuer_api_v1_AuthenticateResponse_remove(google::protobuf::Map<int64_t, ::issuer_api::v1::AuthenticateResponse> * m,
                          int64_t key, ::issuer_api::v1::AuthenticateResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_issuer_api_v1_AuthenticateResponse_iter(
      const google::protobuf::Map<int64_t, ::issuer_api::v1::AuthenticateResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_issuer_api_v1_AuthenticateResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::issuer_api::v1::AuthenticateResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::issuer_api::v1::AuthenticateResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::issuer_api::v1::AuthenticateResponse>* __rust_proto_thunk__Map_u64_issuer_api_v1_AuthenticateResponse_new() {
    return new google::protobuf::Map<uint64_t, ::issuer_api::v1::AuthenticateResponse>();
  }
  void __rust_proto_thunk__Map_u64_issuer_api_v1_AuthenticateResponse_free(const google::protobuf::Map<uint64_t, ::issuer_api::v1::AuthenticateResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_issuer_api_v1_AuthenticateResponse_clear(google::protobuf::Map<uint64_t, ::issuer_api::v1::AuthenticateResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_issuer_api_v1_AuthenticateResponse_size(const google::protobuf::Map<uint64_t, ::issuer_api::v1::AuthenticateResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_issuer_api_v1_AuthenticateResponse_insert(google::protobuf::Map<uint64_t, ::issuer_api::v1::AuthenticateResponse> * m,
                          uint64_t key, ::issuer_api::v1::AuthenticateResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_issuer_api_v1_AuthenticateResponse_get(const google::protobuf::Map<uint64_t, ::issuer_api::v1::AuthenticateResponse>* m,
                       uint64_t key, const ::issuer_api::v1::AuthenticateResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::AuthenticateResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_issuer_api_v1_AuthenticateResponse_remove(google::protobuf::Map<uint64_t, ::issuer_api::v1::AuthenticateResponse> * m,
                          uint64_t key, ::issuer_api::v1::AuthenticateResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_issuer_api_v1_AuthenticateResponse_iter(
      const google::protobuf::Map<uint64_t, ::issuer_api::v1::AuthenticateResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_issuer_api_v1_AuthenticateResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::issuer_api::v1::AuthenticateResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::issuer_api::v1::AuthenticateResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::issuer_api::v1::AuthenticateResponse>* __rust_proto_thunk__Map_bool_issuer_api_v1_AuthenticateResponse_new() {
    return new google::protobuf::Map<bool, ::issuer_api::v1::AuthenticateResponse>();
  }
  void __rust_proto_thunk__Map_bool_issuer_api_v1_AuthenticateResponse_free(const google::protobuf::Map<bool, ::issuer_api::v1::AuthenticateResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_issuer_api_v1_AuthenticateResponse_clear(google::protobuf::Map<bool, ::issuer_api::v1::AuthenticateResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_issuer_api_v1_AuthenticateResponse_size(const google::protobuf::Map<bool, ::issuer_api::v1::AuthenticateResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_issuer_api_v1_AuthenticateResponse_insert(google::protobuf::Map<bool, ::issuer_api::v1::AuthenticateResponse> * m,
                          bool key, ::issuer_api::v1::AuthenticateResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_issuer_api_v1_AuthenticateResponse_get(const google::protobuf::Map<bool, ::issuer_api::v1::AuthenticateResponse>* m,
                       bool key, const ::issuer_api::v1::AuthenticateResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::AuthenticateResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_issuer_api_v1_AuthenticateResponse_remove(google::protobuf::Map<bool, ::issuer_api::v1::AuthenticateResponse> * m,
                          bool key, ::issuer_api::v1::AuthenticateResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_issuer_api_v1_AuthenticateResponse_iter(
      const google::protobuf::Map<bool, ::issuer_api::v1::AuthenticateResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_issuer_api_v1_AuthenticateResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::issuer_api::v1::AuthenticateResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::issuer_api::v1::AuthenticateResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::issuer_api::v1::AuthenticateResponse>* __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AuthenticateResponse_new() {
    return new google::protobuf::Map<std::string, ::issuer_api::v1::AuthenticateResponse>();
  }
  void __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AuthenticateResponse_free(const google::protobuf::Map<std::string, ::issuer_api::v1::AuthenticateResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AuthenticateResponse_clear(google::protobuf::Map<std::string, ::issuer_api::v1::AuthenticateResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AuthenticateResponse_size(const google::protobuf::Map<std::string, ::issuer_api::v1::AuthenticateResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AuthenticateResponse_insert(google::protobuf::Map<std::string, ::issuer_api::v1::AuthenticateResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::issuer_api::v1::AuthenticateResponse value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AuthenticateResponse_get(const google::protobuf::Map<std::string, ::issuer_api::v1::AuthenticateResponse>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::issuer_api::v1::AuthenticateResponse** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::AuthenticateResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AuthenticateResponse_remove(google::protobuf::Map<std::string, ::issuer_api::v1::AuthenticateResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::issuer_api::v1::AuthenticateResponse * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AuthenticateResponse_iter(
      const google::protobuf::Map<std::string, ::issuer_api::v1::AuthenticateResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AuthenticateResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::issuer_api::v1::AuthenticateResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::issuer_api::v1::AuthenticateResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// issuer_api.v1.GetLimitsRequest
        // clang-format off
extern "C" {
void* __rust_proto_thunk__issuer_api_v1_GetLimitsRequest_new() { return new ::issuer_api::v1::GetLimitsRequest(); }
void __rust_proto_thunk__issuer_api_v1_GetLimitsRequest_delete(void* ptr) { delete static_cast<::issuer_api::v1::GetLimitsRequest*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__issuer_api_v1_GetLimitsRequest_serialize(::issuer_api::v1::GetLimitsRequest* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__issuer_api_v1_GetLimitsRequest_parse(::issuer_api::v1::GetLimitsRequest* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__issuer_api_v1_GetLimitsRequest_copy_from(::issuer_api::v1::GetLimitsRequest* dst, const ::issuer_api::v1::GetLimitsRequest* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__issuer_api_v1_GetLimitsRequest_repeated_len(google::protobuf::RepeatedPtrField<::issuer_api::v1::GetLimitsRequest>* field) {
  return field->size();
}
const ::issuer_api::v1::GetLimitsRequest& __rust_proto_thunk__issuer_api_v1_GetLimitsRequest_repeated_get(
  google::protobuf::RepeatedPtrField<::issuer_api::v1::GetLimitsRequest>* field,
  size_t index) {
  return field->Get(index);
}
::issuer_api::v1::GetLimitsRequest* __rust_proto_thunk__issuer_api_v1_GetLimitsRequest_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::issuer_api::v1::GetLimitsRequest>* field,
  size_t index) {
  return field->Mutable(index);
}
::issuer_api::v1::GetLimitsRequest* __rust_proto_thunk__issuer_api_v1_GetLimitsRequest_repeated_add(google::protobuf::RepeatedPtrField<::issuer_api::v1::GetLimitsRequest>* field) {
  return field->Add();
}
void __rust_proto_thunk__issuer_api_v1_GetLimitsRequest_repeated_clear(google::protobuf::RepeatedPtrField<::issuer_api::v1::GetLimitsRequest>* field) {
  field->Clear();
}
void __rust_proto_thunk__issuer_api_v1_GetLimitsRequest_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::issuer_api::v1::GetLimitsRequest>& dst,
  const google::protobuf::RepeatedPtrField<::issuer_api::v1::GetLimitsRequest>& src) {
  dst = src;
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::issuer_api::v1::GetLimitsRequest>* __rust_proto_thunk__Map_i32_issuer_api_v1_GetLimitsRequest_new() {
    return new google::protobuf::Map<int32_t, ::issuer_api::v1::GetLimitsRequest>();
  }
  void __rust_proto_thunk__Map_i32_issuer_api_v1_GetLimitsRequest_free(const google::protobuf::Map<int32_t, ::issuer_api::v1::GetLimitsRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_issuer_api_v1_GetLimitsRequest_clear(google::protobuf::Map<int32_t, ::issuer_api::v1::GetLimitsRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_issuer_api_v1_GetLimitsRequest_size(const google::protobuf::Map<int32_t, ::issuer_api::v1::GetLimitsRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_issuer_api_v1_GetLimitsRequest_insert(google::protobuf::Map<int32_t, ::issuer_api::v1::GetLimitsRequest> * m,
                          int32_t key, ::issuer_api::v1::GetLimitsRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_issuer_api_v1_GetLimitsRequest_get(const google::protobuf::Map<int32_t, ::issuer_api::v1::GetLimitsRequest>* m,
                       int32_t key, const ::issuer_api::v1::GetLimitsRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::GetLimitsRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_issuer_api_v1_GetLimitsRequest_remove(google::protobuf::Map<int32_t, ::issuer_api::v1::GetLimitsRequest> * m,
                          int32_t key, ::issuer_api::v1::GetLimitsRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_issuer_api_v1_GetLimitsRequest_iter(
      const google::protobuf::Map<int32_t, ::issuer_api::v1::GetLimitsRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_issuer_api_v1_GetLimitsRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::issuer_api::v1::GetLimitsRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::issuer_api::v1::GetLimitsRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::issuer_api::v1::GetLimitsRequest>* __rust_proto_thunk__Map_u32_issuer_api_v1_GetLimitsRequest_new() {
    return new google::protobuf::Map<uint32_t, ::issuer_api::v1::GetLimitsRequest>();
  }
  void __rust_proto_thunk__Map_u32_issuer_api_v1_GetLimitsRequest_free(const google::protobuf::Map<uint32_t, ::issuer_api::v1::GetLimitsRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_issuer_api_v1_GetLimitsRequest_clear(google::protobuf::Map<uint32_t, ::issuer_api::v1::GetLimitsRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_issuer_api_v1_GetLimitsRequest_size(const google::protobuf::Map<uint32_t, ::issuer_api::v1::GetLimitsRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_issuer_api_v1_GetLimitsRequest_insert(google::protobuf::Map<uint32_t, ::issuer_api::v1::GetLimitsRequest> * m,
                          uint32_t key, ::issuer_api::v1::GetLimitsRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_issuer_api_v1_GetLimitsRequest_get(const google::protobuf::Map<uint32_t, ::issuer_api::v1::GetLimitsRequest>* m,
                       uint32_t key, const ::issuer_api::v1::GetLimitsRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::GetLimitsRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_issuer_api_v1_GetLimitsRequest_remove(google::protobuf::Map<uint32_t, ::issuer_api::v1::GetLimitsRequest> * m,
                          uint32_t key, ::issuer_api::v1::GetLimitsRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_issuer_api_v1_GetLimitsRequest_iter(
      const google::protobuf::Map<uint32_t, ::issuer_api::v1::GetLimitsRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_issuer_api_v1_GetLimitsRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::issuer_api::v1::GetLimitsRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::issuer_api::v1::GetLimitsRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::issuer_api::v1::GetLimitsRequest>* __rust_proto_thunk__Map_i64_issuer_api_v1_GetLimitsRequest_new() {
    return new google::protobuf::Map<int64_t, ::issuer_api::v1::GetLimitsRequest>();
  }
  void __rust_proto_thunk__Map_i64_issuer_api_v1_GetLimitsRequest_free(const google::protobuf::Map<int64_t, ::issuer_api::v1::GetLimitsRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_issuer_api_v1_GetLimitsRequest_clear(google::protobuf::Map<int64_t, ::issuer_api::v1::GetLimitsRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_issuer_api_v1_GetLimitsRequest_size(const google::protobuf::Map<int64_t, ::issuer_api::v1::GetLimitsRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_issuer_api_v1_GetLimitsRequest_insert(google::protobuf::Map<int64_t, ::issuer_api::v1::GetLimitsRequest> * m,
                          int64_t key, ::issuer_api::v1::GetLimitsRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_issuer_api_v1_GetLimitsRequest_get(const google::protobuf::Map<int64_t, ::issuer_api::v1::GetLimitsRequest>* m,
                       int64_t key, const ::issuer_api::v1::GetLimitsRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::GetLimitsRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_issuer_api_v1_GetLimitsRequest_remove(google::protobuf::Map<int64_t, ::issuer_api::v1::GetLimitsRequest> * m,
                          int64_t key, ::issuer_api::v1::GetLimitsRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_issuer_api_v1_GetLimitsRequest_iter(
      const google::protobuf::Map<int64_t, ::issuer_api::v1::GetLimitsRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_issuer_api_v1_GetLimitsRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::issuer_api::v1::GetLimitsRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::issuer_api::v1::GetLimitsRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::issuer_api::v1::GetLimitsRequest>* __rust_proto_thunk__Map_u64_issuer_api_v1_GetLimitsRequest_new() {
    return new google::protobuf::Map<uint64_t, ::issuer_api::v1::GetLimitsRequest>();
  }
  void __rust_proto_thunk__Map_u64_issuer_api_v1_GetLimitsRequest_free(const google::protobuf::Map<uint64_t, ::issuer_api::v1::GetLimitsRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_issuer_api_v1_GetLimitsRequest_clear(google::protobuf::Map<uint64_t, ::issuer_api::v1::GetLimitsRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_issuer_api_v1_GetLimitsRequest_size(const google::protobuf::Map<uint64_t, ::issuer_api::v1::GetLimitsRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_issuer_api_v1_GetLimitsRequest_insert(google::protobuf::Map<uint64_t, ::issuer_api::v1::GetLimitsRequest> * m,
                          uint64_t key, ::issuer_api::v1::GetLimitsRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_issuer_api_v1_GetLimitsRequest_get(const google::protobuf::Map<uint64_t, ::issuer_api::v1::GetLimitsRequest>* m,
                       uint64_t key, const ::issuer_api::v1::GetLimitsRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::GetLimitsRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_issuer_api_v1_GetLimitsRequest_remove(google::protobuf::Map<uint64_t, ::issuer_api::v1::GetLimitsRequest> * m,
                          uint64_t key, ::issuer_api::v1::GetLimitsRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_issuer_api_v1_GetLimitsRequest_iter(
      const google::protobuf::Map<uint64_t, ::issuer_api::v1::GetLimitsRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_issuer_api_v1_GetLimitsRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::issuer_api::v1::GetLimitsRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::issuer_api::v1::GetLimitsRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::issuer_api::v1::GetLimitsRequest>* __rust_proto_thunk__Map_bool_issuer_api_v1_GetLimitsRequest_new() {
    return new google::protobuf::Map<bool, ::issuer_api::v1::GetLimitsRequest>();
  }
  void __rust_proto_thunk__Map_bool_issuer_api_v1_GetLimitsRequest_free(const google::protobuf::Map<bool, ::issuer_api::v1::GetLimitsRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_issuer_api_v1_GetLimitsRequest_clear(google::protobuf::Map<bool, ::issuer_api::v1::GetLimitsRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_issuer_api_v1_GetLimitsRequest_size(const google::protobuf::Map<bool, ::issuer_api::v1::GetLimitsRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_issuer_api_v1_GetLimitsRequest_insert(google::protobuf::Map<bool, ::issuer_api::v1::GetLimitsRequest> * m,
                          bool key, ::issuer_api::v1::GetLimitsRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_issuer_api_v1_GetLimitsRequest_get(const google::protobuf::Map<bool, ::issuer_api::v1::GetLimitsRequest>* m,
                       bool key, const ::issuer_api::v1::GetLimitsRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::GetLimitsRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_issuer_api_v1_GetLimitsRequest_remove(google::protobuf::Map<bool, ::issuer_api::v1::GetLimitsRequest> * m,
                          bool key, ::issuer_api::v1::GetLimitsRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_issuer_api_v1_GetLimitsRequest_iter(
      const google::protobuf::Map<bool, ::issuer_api::v1::GetLimitsRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_issuer_api_v1_GetLimitsRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::issuer_api::v1::GetLimitsRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::issuer_api::v1::GetLimitsRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::issuer_api::v1::GetLimitsRequest>* __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetLimitsRequest_new() {
    return new google::protobuf::Map<std::string, ::issuer_api::v1::GetLimitsRequest>();
  }
  void __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetLimitsRequest_free(const google::protobuf::Map<std::string, ::issuer_api::v1::GetLimitsRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetLimitsRequest_clear(google::protobuf::Map<std::string, ::issuer_api::v1::GetLimitsRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetLimitsRequest_size(const google::protobuf::Map<std::string, ::issuer_api::v1::GetLimitsRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetLimitsRequest_insert(google::protobuf::Map<std::string, ::issuer_api::v1::GetLimitsRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::issuer_api::v1::GetLimitsRequest value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetLimitsRequest_get(const google::protobuf::Map<std::string, ::issuer_api::v1::GetLimitsRequest>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::issuer_api::v1::GetLimitsRequest** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::GetLimitsRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetLimitsRequest_remove(google::protobuf::Map<std::string, ::issuer_api::v1::GetLimitsRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::issuer_api::v1::GetLimitsRequest * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetLimitsRequest_iter(
      const google::protobuf::Map<std::string, ::issuer_api::v1::GetLimitsRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetLimitsRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::issuer_api::v1::GetLimitsRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::issuer_api::v1::GetLimitsRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// issuer_api.v1.IssuanceLimits
        // clang-format off
extern "C" {
void* __rust_proto_thunk__issuer_api_v1_IssuanceLimits_new() { return new ::issuer_api::v1::IssuanceLimits(); }
void __rust_proto_thunk__issuer_api_v1_IssuanceLimits_delete(void* ptr) { delete static_cast<::issuer_api::v1::IssuanceLimits*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__issuer_api_v1_IssuanceLimits_serialize(::issuer_api::v1::IssuanceLimits* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__issuer_api_v1_IssuanceLimits_parse(::issuer_api::v1::IssuanceLimits* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__issuer_api_v1_IssuanceLimits_copy_from(::issuer_api::v1::IssuanceLimits* dst, const ::issuer_api::v1::IssuanceLimits* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__issuer_api_v1_IssuanceLimits_repeated_len(google::protobuf::RepeatedPtrField<::issuer_api::v1::IssuanceLimits>* field) {
  return field->size();
}
const ::issuer_api::v1::IssuanceLimits& __rust_proto_thunk__issuer_api_v1_IssuanceLimits_repeated_get(
  google::protobuf::RepeatedPtrField<::issuer_api::v1::IssuanceLimits>* field,
  size_t index) {
  return field->Get(index);
}
::issuer_api::v1::IssuanceLimits* __rust_proto_thunk__issuer_api_v1_IssuanceLimits_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::issuer_api::v1::IssuanceLimits>* field,
  size_t index) {
  return field->Mutable(index);
}
::issuer_api::v1::IssuanceLimits* __rust_proto_thunk__issuer_api_v1_IssuanceLimits_repeated_add(google::protobuf::RepeatedPtrField<::issuer_api::v1::IssuanceLimits>* field) {
  return field->Add();
}
void __rust_proto_thunk__issuer_api_v1_IssuanceLimits_repeated_clear(google::protobuf::RepeatedPtrField<::issuer_api::v1::IssuanceLimits>* field) {
  field->Clear();
}
void __rust_proto_thunk__issuer_api_v1_IssuanceLimits_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::issuer_api::v1::IssuanceLimits>& dst,
  const google::protobuf::RepeatedPtrField<::issuer_api::v1::IssuanceLimits>& src) {
  dst = src;
}

void __rust_proto_thunk__issuer_api_v1_IssuanceLimits_clear_inner(::issuer_api::v1::IssuanceLimits* msg) {
  msg->clear_inner();
}
google::protobuf::RepeatedPtrField<::common::Amount>* __rust_proto_thunk__issuer_api_v1_IssuanceLimits_get_mut_inner(::issuer_api::v1::IssuanceLimits* msg) {
  return msg->mutable_inner();
}
const google::protobuf::RepeatedPtrField<::common::Amount>* __rust_proto_thunk__issuer_api_v1_IssuanceLimits_get_inner(
    const ::issuer_api::v1::IssuanceLimits* msg) {
  return &msg->inner();
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::issuer_api::v1::IssuanceLimits>* __rust_proto_thunk__Map_i32_issuer_api_v1_IssuanceLimits_new() {
    return new google::protobuf::Map<int32_t, ::issuer_api::v1::IssuanceLimits>();
  }
  void __rust_proto_thunk__Map_i32_issuer_api_v1_IssuanceLimits_free(const google::protobuf::Map<int32_t, ::issuer_api::v1::IssuanceLimits>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_issuer_api_v1_IssuanceLimits_clear(google::protobuf::Map<int32_t, ::issuer_api::v1::IssuanceLimits> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_issuer_api_v1_IssuanceLimits_size(const google::protobuf::Map<int32_t, ::issuer_api::v1::IssuanceLimits>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_issuer_api_v1_IssuanceLimits_insert(google::protobuf::Map<int32_t, ::issuer_api::v1::IssuanceLimits> * m,
                          int32_t key, ::issuer_api::v1::IssuanceLimits value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_issuer_api_v1_IssuanceLimits_get(const google::protobuf::Map<int32_t, ::issuer_api::v1::IssuanceLimits>* m,
                       int32_t key, const ::issuer_api::v1::IssuanceLimits** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::IssuanceLimits* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_issuer_api_v1_IssuanceLimits_remove(google::protobuf::Map<int32_t, ::issuer_api::v1::IssuanceLimits> * m,
                          int32_t key, ::issuer_api::v1::IssuanceLimits * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_issuer_api_v1_IssuanceLimits_iter(
      const google::protobuf::Map<int32_t, ::issuer_api::v1::IssuanceLimits>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_issuer_api_v1_IssuanceLimits_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::issuer_api::v1::IssuanceLimits** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::issuer_api::v1::IssuanceLimits>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::issuer_api::v1::IssuanceLimits>* __rust_proto_thunk__Map_u32_issuer_api_v1_IssuanceLimits_new() {
    return new google::protobuf::Map<uint32_t, ::issuer_api::v1::IssuanceLimits>();
  }
  void __rust_proto_thunk__Map_u32_issuer_api_v1_IssuanceLimits_free(const google::protobuf::Map<uint32_t, ::issuer_api::v1::IssuanceLimits>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_issuer_api_v1_IssuanceLimits_clear(google::protobuf::Map<uint32_t, ::issuer_api::v1::IssuanceLimits> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_issuer_api_v1_IssuanceLimits_size(const google::protobuf::Map<uint32_t, ::issuer_api::v1::IssuanceLimits>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_issuer_api_v1_IssuanceLimits_insert(google::protobuf::Map<uint32_t, ::issuer_api::v1::IssuanceLimits> * m,
                          uint32_t key, ::issuer_api::v1::IssuanceLimits value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_issuer_api_v1_IssuanceLimits_get(const google::protobuf::Map<uint32_t, ::issuer_api::v1::IssuanceLimits>* m,
                       uint32_t key, const ::issuer_api::v1::IssuanceLimits** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::IssuanceLimits* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_issuer_api_v1_IssuanceLimits_remove(google::protobuf::Map<uint32_t, ::issuer_api::v1::IssuanceLimits> * m,
                          uint32_t key, ::issuer_api::v1::IssuanceLimits * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_issuer_api_v1_IssuanceLimits_iter(
      const google::protobuf::Map<uint32_t, ::issuer_api::v1::IssuanceLimits>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_issuer_api_v1_IssuanceLimits_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::issuer_api::v1::IssuanceLimits** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::issuer_api::v1::IssuanceLimits>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::issuer_api::v1::IssuanceLimits>* __rust_proto_thunk__Map_i64_issuer_api_v1_IssuanceLimits_new() {
    return new google::protobuf::Map<int64_t, ::issuer_api::v1::IssuanceLimits>();
  }
  void __rust_proto_thunk__Map_i64_issuer_api_v1_IssuanceLimits_free(const google::protobuf::Map<int64_t, ::issuer_api::v1::IssuanceLimits>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_issuer_api_v1_IssuanceLimits_clear(google::protobuf::Map<int64_t, ::issuer_api::v1::IssuanceLimits> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_issuer_api_v1_IssuanceLimits_size(const google::protobuf::Map<int64_t, ::issuer_api::v1::IssuanceLimits>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_issuer_api_v1_IssuanceLimits_insert(google::protobuf::Map<int64_t, ::issuer_api::v1::IssuanceLimits> * m,
                          int64_t key, ::issuer_api::v1::IssuanceLimits value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_issuer_api_v1_IssuanceLimits_get(const google::protobuf::Map<int64_t, ::issuer_api::v1::IssuanceLimits>* m,
                       int64_t key, const ::issuer_api::v1::IssuanceLimits** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::IssuanceLimits* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_issuer_api_v1_IssuanceLimits_remove(google::protobuf::Map<int64_t, ::issuer_api::v1::IssuanceLimits> * m,
                          int64_t key, ::issuer_api::v1::IssuanceLimits * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_issuer_api_v1_IssuanceLimits_iter(
      const google::protobuf::Map<int64_t, ::issuer_api::v1::IssuanceLimits>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_issuer_api_v1_IssuanceLimits_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::issuer_api::v1::IssuanceLimits** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::issuer_api::v1::IssuanceLimits>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::issuer_api::v1::IssuanceLimits>* __rust_proto_thunk__Map_u64_issuer_api_v1_IssuanceLimits_new() {
    return new google::protobuf::Map<uint64_t, ::issuer_api::v1::IssuanceLimits>();
  }
  void __rust_proto_thunk__Map_u64_issuer_api_v1_IssuanceLimits_free(const google::protobuf::Map<uint64_t, ::issuer_api::v1::IssuanceLimits>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_issuer_api_v1_IssuanceLimits_clear(google::protobuf::Map<uint64_t, ::issuer_api::v1::IssuanceLimits> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_issuer_api_v1_IssuanceLimits_size(const google::protobuf::Map<uint64_t, ::issuer_api::v1::IssuanceLimits>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_issuer_api_v1_IssuanceLimits_insert(google::protobuf::Map<uint64_t, ::issuer_api::v1::IssuanceLimits> * m,
                          uint64_t key, ::issuer_api::v1::IssuanceLimits value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_issuer_api_v1_IssuanceLimits_get(const google::protobuf::Map<uint64_t, ::issuer_api::v1::IssuanceLimits>* m,
                       uint64_t key, const ::issuer_api::v1::IssuanceLimits** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::IssuanceLimits* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_issuer_api_v1_IssuanceLimits_remove(google::protobuf::Map<uint64_t, ::issuer_api::v1::IssuanceLimits> * m,
                          uint64_t key, ::issuer_api::v1::IssuanceLimits * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_issuer_api_v1_IssuanceLimits_iter(
      const google::protobuf::Map<uint64_t, ::issuer_api::v1::IssuanceLimits>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_issuer_api_v1_IssuanceLimits_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::issuer_api::v1::IssuanceLimits** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::issuer_api::v1::IssuanceLimits>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::issuer_api::v1::IssuanceLimits>* __rust_proto_thunk__Map_bool_issuer_api_v1_IssuanceLimits_new() {
    return new google::protobuf::Map<bool, ::issuer_api::v1::IssuanceLimits>();
  }
  void __rust_proto_thunk__Map_bool_issuer_api_v1_IssuanceLimits_free(const google::protobuf::Map<bool, ::issuer_api::v1::IssuanceLimits>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_issuer_api_v1_IssuanceLimits_clear(google::protobuf::Map<bool, ::issuer_api::v1::IssuanceLimits> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_issuer_api_v1_IssuanceLimits_size(const google::protobuf::Map<bool, ::issuer_api::v1::IssuanceLimits>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_issuer_api_v1_IssuanceLimits_insert(google::protobuf::Map<bool, ::issuer_api::v1::IssuanceLimits> * m,
                          bool key, ::issuer_api::v1::IssuanceLimits value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_issuer_api_v1_IssuanceLimits_get(const google::protobuf::Map<bool, ::issuer_api::v1::IssuanceLimits>* m,
                       bool key, const ::issuer_api::v1::IssuanceLimits** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::IssuanceLimits* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_issuer_api_v1_IssuanceLimits_remove(google::protobuf::Map<bool, ::issuer_api::v1::IssuanceLimits> * m,
                          bool key, ::issuer_api::v1::IssuanceLimits * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_issuer_api_v1_IssuanceLimits_iter(
      const google::protobuf::Map<bool, ::issuer_api::v1::IssuanceLimits>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_issuer_api_v1_IssuanceLimits_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::issuer_api::v1::IssuanceLimits** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::issuer_api::v1::IssuanceLimits>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::issuer_api::v1::IssuanceLimits>* __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssuanceLimits_new() {
    return new google::protobuf::Map<std::string, ::issuer_api::v1::IssuanceLimits>();
  }
  void __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssuanceLimits_free(const google::protobuf::Map<std::string, ::issuer_api::v1::IssuanceLimits>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssuanceLimits_clear(google::protobuf::Map<std::string, ::issuer_api::v1::IssuanceLimits> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssuanceLimits_size(const google::protobuf::Map<std::string, ::issuer_api::v1::IssuanceLimits>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssuanceLimits_insert(google::protobuf::Map<std::string, ::issuer_api::v1::IssuanceLimits> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::issuer_api::v1::IssuanceLimits value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssuanceLimits_get(const google::protobuf::Map<std::string, ::issuer_api::v1::IssuanceLimits>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::issuer_api::v1::IssuanceLimits** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::IssuanceLimits* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssuanceLimits_remove(google::protobuf::Map<std::string, ::issuer_api::v1::IssuanceLimits> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::issuer_api::v1::IssuanceLimits * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssuanceLimits_iter(
      const google::protobuf::Map<std::string, ::issuer_api::v1::IssuanceLimits>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssuanceLimits_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::issuer_api::v1::IssuanceLimits** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::issuer_api::v1::IssuanceLimits>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// issuer_api.v1.GetLimitsResponse
        // clang-format off
extern "C" {
void* __rust_proto_thunk__issuer_api_v1_GetLimitsResponse_new() { return new ::issuer_api::v1::GetLimitsResponse(); }
void __rust_proto_thunk__issuer_api_v1_GetLimitsResponse_delete(void* ptr) { delete static_cast<::issuer_api::v1::GetLimitsResponse*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__issuer_api_v1_GetLimitsResponse_serialize(::issuer_api::v1::GetLimitsResponse* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__issuer_api_v1_GetLimitsResponse_parse(::issuer_api::v1::GetLimitsResponse* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__issuer_api_v1_GetLimitsResponse_copy_from(::issuer_api::v1::GetLimitsResponse* dst, const ::issuer_api::v1::GetLimitsResponse* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__issuer_api_v1_GetLimitsResponse_repeated_len(google::protobuf::RepeatedPtrField<::issuer_api::v1::GetLimitsResponse>* field) {
  return field->size();
}
const ::issuer_api::v1::GetLimitsResponse& __rust_proto_thunk__issuer_api_v1_GetLimitsResponse_repeated_get(
  google::protobuf::RepeatedPtrField<::issuer_api::v1::GetLimitsResponse>* field,
  size_t index) {
  return field->Get(index);
}
::issuer_api::v1::GetLimitsResponse* __rust_proto_thunk__issuer_api_v1_GetLimitsResponse_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::issuer_api::v1::GetLimitsResponse>* field,
  size_t index) {
  return field->Mutable(index);
}
::issuer_api::v1::GetLimitsResponse* __rust_proto_thunk__issuer_api_v1_GetLimitsResponse_repeated_add(google::protobuf::RepeatedPtrField<::issuer_api::v1::GetLimitsResponse>* field) {
  return field->Add();
}
void __rust_proto_thunk__issuer_api_v1_GetLimitsResponse_repeated_clear(google::protobuf::RepeatedPtrField<::issuer_api::v1::GetLimitsResponse>* field) {
  field->Clear();
}
void __rust_proto_thunk__issuer_api_v1_GetLimitsResponse_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::issuer_api::v1::GetLimitsResponse>& dst,
  const google::protobuf::RepeatedPtrField<::issuer_api::v1::GetLimitsResponse>& src) {
  dst = src;
}

const void* __rust_proto_thunk__issuer_api_v1_GetLimitsResponse_get_limits(const ::issuer_api::v1::GetLimitsResponse* msg) {
  return &msg->limits();
}
void* __rust_proto_thunk__issuer_api_v1_GetLimitsResponse_get_mut_limits(::issuer_api::v1::GetLimitsResponse* msg) { return msg->mutable_limits(); }


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::issuer_api::v1::GetLimitsResponse>* __rust_proto_thunk__Map_i32_issuer_api_v1_GetLimitsResponse_new() {
    return new google::protobuf::Map<int32_t, ::issuer_api::v1::GetLimitsResponse>();
  }
  void __rust_proto_thunk__Map_i32_issuer_api_v1_GetLimitsResponse_free(const google::protobuf::Map<int32_t, ::issuer_api::v1::GetLimitsResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_issuer_api_v1_GetLimitsResponse_clear(google::protobuf::Map<int32_t, ::issuer_api::v1::GetLimitsResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_issuer_api_v1_GetLimitsResponse_size(const google::protobuf::Map<int32_t, ::issuer_api::v1::GetLimitsResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_issuer_api_v1_GetLimitsResponse_insert(google::protobuf::Map<int32_t, ::issuer_api::v1::GetLimitsResponse> * m,
                          int32_t key, ::issuer_api::v1::GetLimitsResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_issuer_api_v1_GetLimitsResponse_get(const google::protobuf::Map<int32_t, ::issuer_api::v1::GetLimitsResponse>* m,
                       int32_t key, const ::issuer_api::v1::GetLimitsResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::GetLimitsResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_issuer_api_v1_GetLimitsResponse_remove(google::protobuf::Map<int32_t, ::issuer_api::v1::GetLimitsResponse> * m,
                          int32_t key, ::issuer_api::v1::GetLimitsResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_issuer_api_v1_GetLimitsResponse_iter(
      const google::protobuf::Map<int32_t, ::issuer_api::v1::GetLimitsResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_issuer_api_v1_GetLimitsResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::issuer_api::v1::GetLimitsResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::issuer_api::v1::GetLimitsResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::issuer_api::v1::GetLimitsResponse>* __rust_proto_thunk__Map_u32_issuer_api_v1_GetLimitsResponse_new() {
    return new google::protobuf::Map<uint32_t, ::issuer_api::v1::GetLimitsResponse>();
  }
  void __rust_proto_thunk__Map_u32_issuer_api_v1_GetLimitsResponse_free(const google::protobuf::Map<uint32_t, ::issuer_api::v1::GetLimitsResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_issuer_api_v1_GetLimitsResponse_clear(google::protobuf::Map<uint32_t, ::issuer_api::v1::GetLimitsResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_issuer_api_v1_GetLimitsResponse_size(const google::protobuf::Map<uint32_t, ::issuer_api::v1::GetLimitsResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_issuer_api_v1_GetLimitsResponse_insert(google::protobuf::Map<uint32_t, ::issuer_api::v1::GetLimitsResponse> * m,
                          uint32_t key, ::issuer_api::v1::GetLimitsResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_issuer_api_v1_GetLimitsResponse_get(const google::protobuf::Map<uint32_t, ::issuer_api::v1::GetLimitsResponse>* m,
                       uint32_t key, const ::issuer_api::v1::GetLimitsResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::GetLimitsResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_issuer_api_v1_GetLimitsResponse_remove(google::protobuf::Map<uint32_t, ::issuer_api::v1::GetLimitsResponse> * m,
                          uint32_t key, ::issuer_api::v1::GetLimitsResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_issuer_api_v1_GetLimitsResponse_iter(
      const google::protobuf::Map<uint32_t, ::issuer_api::v1::GetLimitsResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_issuer_api_v1_GetLimitsResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::issuer_api::v1::GetLimitsResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::issuer_api::v1::GetLimitsResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::issuer_api::v1::GetLimitsResponse>* __rust_proto_thunk__Map_i64_issuer_api_v1_GetLimitsResponse_new() {
    return new google::protobuf::Map<int64_t, ::issuer_api::v1::GetLimitsResponse>();
  }
  void __rust_proto_thunk__Map_i64_issuer_api_v1_GetLimitsResponse_free(const google::protobuf::Map<int64_t, ::issuer_api::v1::GetLimitsResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_issuer_api_v1_GetLimitsResponse_clear(google::protobuf::Map<int64_t, ::issuer_api::v1::GetLimitsResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_issuer_api_v1_GetLimitsResponse_size(const google::protobuf::Map<int64_t, ::issuer_api::v1::GetLimitsResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_issuer_api_v1_GetLimitsResponse_insert(google::protobuf::Map<int64_t, ::issuer_api::v1::GetLimitsResponse> * m,
                          int64_t key, ::issuer_api::v1::GetLimitsResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_issuer_api_v1_GetLimitsResponse_get(const google::protobuf::Map<int64_t, ::issuer_api::v1::GetLimitsResponse>* m,
                       int64_t key, const ::issuer_api::v1::GetLimitsResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::GetLimitsResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_issuer_api_v1_GetLimitsResponse_remove(google::protobuf::Map<int64_t, ::issuer_api::v1::GetLimitsResponse> * m,
                          int64_t key, ::issuer_api::v1::GetLimitsResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_issuer_api_v1_GetLimitsResponse_iter(
      const google::protobuf::Map<int64_t, ::issuer_api::v1::GetLimitsResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_issuer_api_v1_GetLimitsResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::issuer_api::v1::GetLimitsResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::issuer_api::v1::GetLimitsResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::issuer_api::v1::GetLimitsResponse>* __rust_proto_thunk__Map_u64_issuer_api_v1_GetLimitsResponse_new() {
    return new google::protobuf::Map<uint64_t, ::issuer_api::v1::GetLimitsResponse>();
  }
  void __rust_proto_thunk__Map_u64_issuer_api_v1_GetLimitsResponse_free(const google::protobuf::Map<uint64_t, ::issuer_api::v1::GetLimitsResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_issuer_api_v1_GetLimitsResponse_clear(google::protobuf::Map<uint64_t, ::issuer_api::v1::GetLimitsResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_issuer_api_v1_GetLimitsResponse_size(const google::protobuf::Map<uint64_t, ::issuer_api::v1::GetLimitsResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_issuer_api_v1_GetLimitsResponse_insert(google::protobuf::Map<uint64_t, ::issuer_api::v1::GetLimitsResponse> * m,
                          uint64_t key, ::issuer_api::v1::GetLimitsResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_issuer_api_v1_GetLimitsResponse_get(const google::protobuf::Map<uint64_t, ::issuer_api::v1::GetLimitsResponse>* m,
                       uint64_t key, const ::issuer_api::v1::GetLimitsResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::GetLimitsResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_issuer_api_v1_GetLimitsResponse_remove(google::protobuf::Map<uint64_t, ::issuer_api::v1::GetLimitsResponse> * m,
                          uint64_t key, ::issuer_api::v1::GetLimitsResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_issuer_api_v1_GetLimitsResponse_iter(
      const google::protobuf::Map<uint64_t, ::issuer_api::v1::GetLimitsResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_issuer_api_v1_GetLimitsResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::issuer_api::v1::GetLimitsResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::issuer_api::v1::GetLimitsResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::issuer_api::v1::GetLimitsResponse>* __rust_proto_thunk__Map_bool_issuer_api_v1_GetLimitsResponse_new() {
    return new google::protobuf::Map<bool, ::issuer_api::v1::GetLimitsResponse>();
  }
  void __rust_proto_thunk__Map_bool_issuer_api_v1_GetLimitsResponse_free(const google::protobuf::Map<bool, ::issuer_api::v1::GetLimitsResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_issuer_api_v1_GetLimitsResponse_clear(google::protobuf::Map<bool, ::issuer_api::v1::GetLimitsResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_issuer_api_v1_GetLimitsResponse_size(const google::protobuf::Map<bool, ::issuer_api::v1::GetLimitsResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_issuer_api_v1_GetLimitsResponse_insert(google::protobuf::Map<bool, ::issuer_api::v1::GetLimitsResponse> * m,
                          bool key, ::issuer_api::v1::GetLimitsResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_issuer_api_v1_GetLimitsResponse_get(const google::protobuf::Map<bool, ::issuer_api::v1::GetLimitsResponse>* m,
                       bool key, const ::issuer_api::v1::GetLimitsResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::GetLimitsResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_issuer_api_v1_GetLimitsResponse_remove(google::protobuf::Map<bool, ::issuer_api::v1::GetLimitsResponse> * m,
                          bool key, ::issuer_api::v1::GetLimitsResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_issuer_api_v1_GetLimitsResponse_iter(
      const google::protobuf::Map<bool, ::issuer_api::v1::GetLimitsResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_issuer_api_v1_GetLimitsResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::issuer_api::v1::GetLimitsResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::issuer_api::v1::GetLimitsResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::issuer_api::v1::GetLimitsResponse>* __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetLimitsResponse_new() {
    return new google::protobuf::Map<std::string, ::issuer_api::v1::GetLimitsResponse>();
  }
  void __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetLimitsResponse_free(const google::protobuf::Map<std::string, ::issuer_api::v1::GetLimitsResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetLimitsResponse_clear(google::protobuf::Map<std::string, ::issuer_api::v1::GetLimitsResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetLimitsResponse_size(const google::protobuf::Map<std::string, ::issuer_api::v1::GetLimitsResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetLimitsResponse_insert(google::protobuf::Map<std::string, ::issuer_api::v1::GetLimitsResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::issuer_api::v1::GetLimitsResponse value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetLimitsResponse_get(const google::protobuf::Map<std::string, ::issuer_api::v1::GetLimitsResponse>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::issuer_api::v1::GetLimitsResponse** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::GetLimitsResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetLimitsResponse_remove(google::protobuf::Map<std::string, ::issuer_api::v1::GetLimitsResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::issuer_api::v1::GetLimitsResponse * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetLimitsResponse_iter(
      const google::protobuf::Map<std::string, ::issuer_api::v1::GetLimitsResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetLimitsResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::issuer_api::v1::GetLimitsResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::issuer_api::v1::GetLimitsResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// issuer_api.v1.SetRoleRequest
        // clang-format off
extern "C" {
void* __rust_proto_thunk__issuer_api_v1_SetRoleRequest_new() { return new ::issuer_api::v1::SetRoleRequest(); }
void __rust_proto_thunk__issuer_api_v1_SetRoleRequest_delete(void* ptr) { delete static_cast<::issuer_api::v1::SetRoleRequest*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__issuer_api_v1_SetRoleRequest_serialize(::issuer_api::v1::SetRoleRequest* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__issuer_api_v1_SetRoleRequest_parse(::issuer_api::v1::SetRoleRequest* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__issuer_api_v1_SetRoleRequest_copy_from(::issuer_api::v1::SetRoleRequest* dst, const ::issuer_api::v1::SetRoleRequest* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__issuer_api_v1_SetRoleRequest_repeated_len(google::protobuf::RepeatedPtrField<::issuer_api::v1::SetRoleRequest>* field) {
  return field->size();
}
const ::issuer_api::v1::SetRoleRequest& __rust_proto_thunk__issuer_api_v1_SetRoleRequest_repeated_get(
  google::protobuf::RepeatedPtrField<::issuer_api::v1::SetRoleRequest>* field,
  size_t index) {
  return field->Get(index);
}
::issuer_api::v1::SetRoleRequest* __rust_proto_thunk__issuer_api_v1_SetRoleRequest_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::issuer_api::v1::SetRoleRequest>* field,
  size_t index) {
  return field->Mutable(index);
}
::issuer_api::v1::SetRoleRequest* __rust_proto_thunk__issuer_api_v1_SetRoleRequest_repeated_add(google::protobuf::RepeatedPtrField<::issuer_api::v1::SetRoleRequest>* field) {
  return field->Add();
}
void __rust_proto_thunk__issuer_api_v1_SetRoleRequest_repeated_clear(google::protobuf::RepeatedPtrField<::issuer_api::v1::SetRoleRequest>* field) {
  field->Clear();
}
void __rust_proto_thunk__issuer_api_v1_SetRoleRequest_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::issuer_api::v1::SetRoleRequest>& dst,
  const google::protobuf::RepeatedPtrField<::issuer_api::v1::SetRoleRequest>& src) {
  dst = src;
}

const void* __rust_proto_thunk__issuer_api_v1_SetRoleRequest_get_role(::issuer_api::v1::SetRoleRequest* msg) {
  return static_cast<const void*>(&msg->role());
}
void* __rust_proto_thunk__issuer_api_v1_SetRoleRequest_get_mut_role(::issuer_api::v1::SetRoleRequest* msg) {
  return static_cast<void*>(msg->mutable_role());
}
void __rust_proto_thunk__issuer_api_v1_SetRoleRequest_clear_role(::issuer_api::v1::SetRoleRequest* msg) { msg->clear_role(); }
bool __rust_proto_thunk__issuer_api_v1_SetRoleRequest_has_role(::issuer_api::v1::SetRoleRequest* msg) { return msg->has_role(); }


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::issuer_api::v1::SetRoleRequest>* __rust_proto_thunk__Map_i32_issuer_api_v1_SetRoleRequest_new() {
    return new google::protobuf::Map<int32_t, ::issuer_api::v1::SetRoleRequest>();
  }
  void __rust_proto_thunk__Map_i32_issuer_api_v1_SetRoleRequest_free(const google::protobuf::Map<int32_t, ::issuer_api::v1::SetRoleRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_issuer_api_v1_SetRoleRequest_clear(google::protobuf::Map<int32_t, ::issuer_api::v1::SetRoleRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_issuer_api_v1_SetRoleRequest_size(const google::protobuf::Map<int32_t, ::issuer_api::v1::SetRoleRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_issuer_api_v1_SetRoleRequest_insert(google::protobuf::Map<int32_t, ::issuer_api::v1::SetRoleRequest> * m,
                          int32_t key, ::issuer_api::v1::SetRoleRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_issuer_api_v1_SetRoleRequest_get(const google::protobuf::Map<int32_t, ::issuer_api::v1::SetRoleRequest>* m,
                       int32_t key, const ::issuer_api::v1::SetRoleRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::SetRoleRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_issuer_api_v1_SetRoleRequest_remove(google::protobuf::Map<int32_t, ::issuer_api::v1::SetRoleRequest> * m,
                          int32_t key, ::issuer_api::v1::SetRoleRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_issuer_api_v1_SetRoleRequest_iter(
      const google::protobuf::Map<int32_t, ::issuer_api::v1::SetRoleRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_issuer_api_v1_SetRoleRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::issuer_api::v1::SetRoleRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::issuer_api::v1::SetRoleRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::issuer_api::v1::SetRoleRequest>* __rust_proto_thunk__Map_u32_issuer_api_v1_SetRoleRequest_new() {
    return new google::protobuf::Map<uint32_t, ::issuer_api::v1::SetRoleRequest>();
  }
  void __rust_proto_thunk__Map_u32_issuer_api_v1_SetRoleRequest_free(const google::protobuf::Map<uint32_t, ::issuer_api::v1::SetRoleRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_issuer_api_v1_SetRoleRequest_clear(google::protobuf::Map<uint32_t, ::issuer_api::v1::SetRoleRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_issuer_api_v1_SetRoleRequest_size(const google::protobuf::Map<uint32_t, ::issuer_api::v1::SetRoleRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_issuer_api_v1_SetRoleRequest_insert(google::protobuf::Map<uint32_t, ::issuer_api::v1::SetRoleRequest> * m,
                          uint32_t key, ::issuer_api::v1::SetRoleRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_issuer_api_v1_SetRoleRequest_get(const google::protobuf::Map<uint32_t, ::issuer_api::v1::SetRoleRequest>* m,
                       uint32_t key, const ::issuer_api::v1::SetRoleRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::SetRoleRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_issuer_api_v1_SetRoleRequest_remove(google::protobuf::Map<uint32_t, ::issuer_api::v1::SetRoleRequest> * m,
                          uint32_t key, ::issuer_api::v1::SetRoleRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_issuer_api_v1_SetRoleRequest_iter(
      const google::protobuf::Map<uint32_t, ::issuer_api::v1::SetRoleRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_issuer_api_v1_SetRoleRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::issuer_api::v1::SetRoleRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::issuer_api::v1::SetRoleRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::issuer_api::v1::SetRoleRequest>* __rust_proto_thunk__Map_i64_issuer_api_v1_SetRoleRequest_new() {
    return new google::protobuf::Map<int64_t, ::issuer_api::v1::SetRoleRequest>();
  }
  void __rust_proto_thunk__Map_i64_issuer_api_v1_SetRoleRequest_free(const google::protobuf::Map<int64_t, ::issuer_api::v1::SetRoleRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_issuer_api_v1_SetRoleRequest_clear(google::protobuf::Map<int64_t, ::issuer_api::v1::SetRoleRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_issuer_api_v1_SetRoleRequest_size(const google::protobuf::Map<int64_t, ::issuer_api::v1::SetRoleRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_issuer_api_v1_SetRoleRequest_insert(google::protobuf::Map<int64_t, ::issuer_api::v1::SetRoleRequest> * m,
                          int64_t key, ::issuer_api::v1::SetRoleRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_issuer_api_v1_SetRoleRequest_get(const google::protobuf::Map<int64_t, ::issuer_api::v1::SetRoleRequest>* m,
                       int64_t key, const ::issuer_api::v1::SetRoleRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::SetRoleRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_issuer_api_v1_SetRoleRequest_remove(google::protobuf::Map<int64_t, ::issuer_api::v1::SetRoleRequest> * m,
                          int64_t key, ::issuer_api::v1::SetRoleRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_issuer_api_v1_SetRoleRequest_iter(
      const google::protobuf::Map<int64_t, ::issuer_api::v1::SetRoleRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_issuer_api_v1_SetRoleRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::issuer_api::v1::SetRoleRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::issuer_api::v1::SetRoleRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::issuer_api::v1::SetRoleRequest>* __rust_proto_thunk__Map_u64_issuer_api_v1_SetRoleRequest_new() {
    return new google::protobuf::Map<uint64_t, ::issuer_api::v1::SetRoleRequest>();
  }
  void __rust_proto_thunk__Map_u64_issuer_api_v1_SetRoleRequest_free(const google::protobuf::Map<uint64_t, ::issuer_api::v1::SetRoleRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_issuer_api_v1_SetRoleRequest_clear(google::protobuf::Map<uint64_t, ::issuer_api::v1::SetRoleRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_issuer_api_v1_SetRoleRequest_size(const google::protobuf::Map<uint64_t, ::issuer_api::v1::SetRoleRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_issuer_api_v1_SetRoleRequest_insert(google::protobuf::Map<uint64_t, ::issuer_api::v1::SetRoleRequest> * m,
                          uint64_t key, ::issuer_api::v1::SetRoleRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_issuer_api_v1_SetRoleRequest_get(const google::protobuf::Map<uint64_t, ::issuer_api::v1::SetRoleRequest>* m,
                       uint64_t key, const ::issuer_api::v1::SetRoleRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::SetRoleRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_issuer_api_v1_SetRoleRequest_remove(google::protobuf::Map<uint64_t, ::issuer_api::v1::SetRoleRequest> * m,
                          uint64_t key, ::issuer_api::v1::SetRoleRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_issuer_api_v1_SetRoleRequest_iter(
      const google::protobuf::Map<uint64_t, ::issuer_api::v1::SetRoleRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_issuer_api_v1_SetRoleRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::issuer_api::v1::SetRoleRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::issuer_api::v1::SetRoleRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::issuer_api::v1::SetRoleRequest>* __rust_proto_thunk__Map_bool_issuer_api_v1_SetRoleRequest_new() {
    return new google::protobuf::Map<bool, ::issuer_api::v1::SetRoleRequest>();
  }
  void __rust_proto_thunk__Map_bool_issuer_api_v1_SetRoleRequest_free(const google::protobuf::Map<bool, ::issuer_api::v1::SetRoleRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_issuer_api_v1_SetRoleRequest_clear(google::protobuf::Map<bool, ::issuer_api::v1::SetRoleRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_issuer_api_v1_SetRoleRequest_size(const google::protobuf::Map<bool, ::issuer_api::v1::SetRoleRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_issuer_api_v1_SetRoleRequest_insert(google::protobuf::Map<bool, ::issuer_api::v1::SetRoleRequest> * m,
                          bool key, ::issuer_api::v1::SetRoleRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_issuer_api_v1_SetRoleRequest_get(const google::protobuf::Map<bool, ::issuer_api::v1::SetRoleRequest>* m,
                       bool key, const ::issuer_api::v1::SetRoleRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::SetRoleRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_issuer_api_v1_SetRoleRequest_remove(google::protobuf::Map<bool, ::issuer_api::v1::SetRoleRequest> * m,
                          bool key, ::issuer_api::v1::SetRoleRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_issuer_api_v1_SetRoleRequest_iter(
      const google::protobuf::Map<bool, ::issuer_api::v1::SetRoleRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_issuer_api_v1_SetRoleRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::issuer_api::v1::SetRoleRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::issuer_api::v1::SetRoleRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::issuer_api::v1::SetRoleRequest>* __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetRoleRequest_new() {
    return new google::protobuf::Map<std::string, ::issuer_api::v1::SetRoleRequest>();
  }
  void __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetRoleRequest_free(const google::protobuf::Map<std::string, ::issuer_api::v1::SetRoleRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetRoleRequest_clear(google::protobuf::Map<std::string, ::issuer_api::v1::SetRoleRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetRoleRequest_size(const google::protobuf::Map<std::string, ::issuer_api::v1::SetRoleRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetRoleRequest_insert(google::protobuf::Map<std::string, ::issuer_api::v1::SetRoleRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::issuer_api::v1::SetRoleRequest value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetRoleRequest_get(const google::protobuf::Map<std::string, ::issuer_api::v1::SetRoleRequest>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::issuer_api::v1::SetRoleRequest** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::SetRoleRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetRoleRequest_remove(google::protobuf::Map<std::string, ::issuer_api::v1::SetRoleRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::issuer_api::v1::SetRoleRequest * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetRoleRequest_iter(
      const google::protobuf::Map<std::string, ::issuer_api::v1::SetRoleRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetRoleRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::issuer_api::v1::SetRoleRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::issuer_api::v1::SetRoleRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// issuer_api.v1.SetRoleResponse
        // clang-format off
extern "C" {
void* __rust_proto_thunk__issuer_api_v1_SetRoleResponse_new() { return new ::issuer_api::v1::SetRoleResponse(); }
void __rust_proto_thunk__issuer_api_v1_SetRoleResponse_delete(void* ptr) { delete static_cast<::issuer_api::v1::SetRoleResponse*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__issuer_api_v1_SetRoleResponse_serialize(::issuer_api::v1::SetRoleResponse* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__issuer_api_v1_SetRoleResponse_parse(::issuer_api::v1::SetRoleResponse* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__issuer_api_v1_SetRoleResponse_copy_from(::issuer_api::v1::SetRoleResponse* dst, const ::issuer_api::v1::SetRoleResponse* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__issuer_api_v1_SetRoleResponse_repeated_len(google::protobuf::RepeatedPtrField<::issuer_api::v1::SetRoleResponse>* field) {
  return field->size();
}
const ::issuer_api::v1::SetRoleResponse& __rust_proto_thunk__issuer_api_v1_SetRoleResponse_repeated_get(
  google::protobuf::RepeatedPtrField<::issuer_api::v1::SetRoleResponse>* field,
  size_t index) {
  return field->Get(index);
}
::issuer_api::v1::SetRoleResponse* __rust_proto_thunk__issuer_api_v1_SetRoleResponse_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::issuer_api::v1::SetRoleResponse>* field,
  size_t index) {
  return field->Mutable(index);
}
::issuer_api::v1::SetRoleResponse* __rust_proto_thunk__issuer_api_v1_SetRoleResponse_repeated_add(google::protobuf::RepeatedPtrField<::issuer_api::v1::SetRoleResponse>* field) {
  return field->Add();
}
void __rust_proto_thunk__issuer_api_v1_SetRoleResponse_repeated_clear(google::protobuf::RepeatedPtrField<::issuer_api::v1::SetRoleResponse>* field) {
  field->Clear();
}
void __rust_proto_thunk__issuer_api_v1_SetRoleResponse_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::issuer_api::v1::SetRoleResponse>& dst,
  const google::protobuf::RepeatedPtrField<::issuer_api::v1::SetRoleResponse>& src) {
  dst = src;
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::issuer_api::v1::SetRoleResponse>* __rust_proto_thunk__Map_i32_issuer_api_v1_SetRoleResponse_new() {
    return new google::protobuf::Map<int32_t, ::issuer_api::v1::SetRoleResponse>();
  }
  void __rust_proto_thunk__Map_i32_issuer_api_v1_SetRoleResponse_free(const google::protobuf::Map<int32_t, ::issuer_api::v1::SetRoleResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_issuer_api_v1_SetRoleResponse_clear(google::protobuf::Map<int32_t, ::issuer_api::v1::SetRoleResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_issuer_api_v1_SetRoleResponse_size(const google::protobuf::Map<int32_t, ::issuer_api::v1::SetRoleResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_issuer_api_v1_SetRoleResponse_insert(google::protobuf::Map<int32_t, ::issuer_api::v1::SetRoleResponse> * m,
                          int32_t key, ::issuer_api::v1::SetRoleResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_issuer_api_v1_SetRoleResponse_get(const google::protobuf::Map<int32_t, ::issuer_api::v1::SetRoleResponse>* m,
                       int32_t key, const ::issuer_api::v1::SetRoleResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::SetRoleResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_issuer_api_v1_SetRoleResponse_remove(google::protobuf::Map<int32_t, ::issuer_api::v1::SetRoleResponse> * m,
                          int32_t key, ::issuer_api::v1::SetRoleResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_issuer_api_v1_SetRoleResponse_iter(
      const google::protobuf::Map<int32_t, ::issuer_api::v1::SetRoleResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_issuer_api_v1_SetRoleResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::issuer_api::v1::SetRoleResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::issuer_api::v1::SetRoleResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::issuer_api::v1::SetRoleResponse>* __rust_proto_thunk__Map_u32_issuer_api_v1_SetRoleResponse_new() {
    return new google::protobuf::Map<uint32_t, ::issuer_api::v1::SetRoleResponse>();
  }
  void __rust_proto_thunk__Map_u32_issuer_api_v1_SetRoleResponse_free(const google::protobuf::Map<uint32_t, ::issuer_api::v1::SetRoleResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_issuer_api_v1_SetRoleResponse_clear(google::protobuf::Map<uint32_t, ::issuer_api::v1::SetRoleResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_issuer_api_v1_SetRoleResponse_size(const google::protobuf::Map<uint32_t, ::issuer_api::v1::SetRoleResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_issuer_api_v1_SetRoleResponse_insert(google::protobuf::Map<uint32_t, ::issuer_api::v1::SetRoleResponse> * m,
                          uint32_t key, ::issuer_api::v1::SetRoleResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_issuer_api_v1_SetRoleResponse_get(const google::protobuf::Map<uint32_t, ::issuer_api::v1::SetRoleResponse>* m,
                       uint32_t key, const ::issuer_api::v1::SetRoleResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::SetRoleResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_issuer_api_v1_SetRoleResponse_remove(google::protobuf::Map<uint32_t, ::issuer_api::v1::SetRoleResponse> * m,
                          uint32_t key, ::issuer_api::v1::SetRoleResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_issuer_api_v1_SetRoleResponse_iter(
      const google::protobuf::Map<uint32_t, ::issuer_api::v1::SetRoleResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_issuer_api_v1_SetRoleResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::issuer_api::v1::SetRoleResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::issuer_api::v1::SetRoleResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::issuer_api::v1::SetRoleResponse>* __rust_proto_thunk__Map_i64_issuer_api_v1_SetRoleResponse_new() {
    return new google::protobuf::Map<int64_t, ::issuer_api::v1::SetRoleResponse>();
  }
  void __rust_proto_thunk__Map_i64_issuer_api_v1_SetRoleResponse_free(const google::protobuf::Map<int64_t, ::issuer_api::v1::SetRoleResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_issuer_api_v1_SetRoleResponse_clear(google::protobuf::Map<int64_t, ::issuer_api::v1::SetRoleResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_issuer_api_v1_SetRoleResponse_size(const google::protobuf::Map<int64_t, ::issuer_api::v1::SetRoleResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_issuer_api_v1_SetRoleResponse_insert(google::protobuf::Map<int64_t, ::issuer_api::v1::SetRoleResponse> * m,
                          int64_t key, ::issuer_api::v1::SetRoleResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_issuer_api_v1_SetRoleResponse_get(const google::protobuf::Map<int64_t, ::issuer_api::v1::SetRoleResponse>* m,
                       int64_t key, const ::issuer_api::v1::SetRoleResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::SetRoleResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_issuer_api_v1_SetRoleResponse_remove(google::protobuf::Map<int64_t, ::issuer_api::v1::SetRoleResponse> * m,
                          int64_t key, ::issuer_api::v1::SetRoleResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_issuer_api_v1_SetRoleResponse_iter(
      const google::protobuf::Map<int64_t, ::issuer_api::v1::SetRoleResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_issuer_api_v1_SetRoleResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::issuer_api::v1::SetRoleResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::issuer_api::v1::SetRoleResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::issuer_api::v1::SetRoleResponse>* __rust_proto_thunk__Map_u64_issuer_api_v1_SetRoleResponse_new() {
    return new google::protobuf::Map<uint64_t, ::issuer_api::v1::SetRoleResponse>();
  }
  void __rust_proto_thunk__Map_u64_issuer_api_v1_SetRoleResponse_free(const google::protobuf::Map<uint64_t, ::issuer_api::v1::SetRoleResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_issuer_api_v1_SetRoleResponse_clear(google::protobuf::Map<uint64_t, ::issuer_api::v1::SetRoleResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_issuer_api_v1_SetRoleResponse_size(const google::protobuf::Map<uint64_t, ::issuer_api::v1::SetRoleResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_issuer_api_v1_SetRoleResponse_insert(google::protobuf::Map<uint64_t, ::issuer_api::v1::SetRoleResponse> * m,
                          uint64_t key, ::issuer_api::v1::SetRoleResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_issuer_api_v1_SetRoleResponse_get(const google::protobuf::Map<uint64_t, ::issuer_api::v1::SetRoleResponse>* m,
                       uint64_t key, const ::issuer_api::v1::SetRoleResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::SetRoleResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_issuer_api_v1_SetRoleResponse_remove(google::protobuf::Map<uint64_t, ::issuer_api::v1::SetRoleResponse> * m,
                          uint64_t key, ::issuer_api::v1::SetRoleResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_issuer_api_v1_SetRoleResponse_iter(
      const google::protobuf::Map<uint64_t, ::issuer_api::v1::SetRoleResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_issuer_api_v1_SetRoleResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::issuer_api::v1::SetRoleResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::issuer_api::v1::SetRoleResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::issuer_api::v1::SetRoleResponse>* __rust_proto_thunk__Map_bool_issuer_api_v1_SetRoleResponse_new() {
    return new google::protobuf::Map<bool, ::issuer_api::v1::SetRoleResponse>();
  }
  void __rust_proto_thunk__Map_bool_issuer_api_v1_SetRoleResponse_free(const google::protobuf::Map<bool, ::issuer_api::v1::SetRoleResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_issuer_api_v1_SetRoleResponse_clear(google::protobuf::Map<bool, ::issuer_api::v1::SetRoleResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_issuer_api_v1_SetRoleResponse_size(const google::protobuf::Map<bool, ::issuer_api::v1::SetRoleResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_issuer_api_v1_SetRoleResponse_insert(google::protobuf::Map<bool, ::issuer_api::v1::SetRoleResponse> * m,
                          bool key, ::issuer_api::v1::SetRoleResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_issuer_api_v1_SetRoleResponse_get(const google::protobuf::Map<bool, ::issuer_api::v1::SetRoleResponse>* m,
                       bool key, const ::issuer_api::v1::SetRoleResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::SetRoleResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_issuer_api_v1_SetRoleResponse_remove(google::protobuf::Map<bool, ::issuer_api::v1::SetRoleResponse> * m,
                          bool key, ::issuer_api::v1::SetRoleResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_issuer_api_v1_SetRoleResponse_iter(
      const google::protobuf::Map<bool, ::issuer_api::v1::SetRoleResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_issuer_api_v1_SetRoleResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::issuer_api::v1::SetRoleResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::issuer_api::v1::SetRoleResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::issuer_api::v1::SetRoleResponse>* __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetRoleResponse_new() {
    return new google::protobuf::Map<std::string, ::issuer_api::v1::SetRoleResponse>();
  }
  void __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetRoleResponse_free(const google::protobuf::Map<std::string, ::issuer_api::v1::SetRoleResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetRoleResponse_clear(google::protobuf::Map<std::string, ::issuer_api::v1::SetRoleResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetRoleResponse_size(const google::protobuf::Map<std::string, ::issuer_api::v1::SetRoleResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetRoleResponse_insert(google::protobuf::Map<std::string, ::issuer_api::v1::SetRoleResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::issuer_api::v1::SetRoleResponse value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetRoleResponse_get(const google::protobuf::Map<std::string, ::issuer_api::v1::SetRoleResponse>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::issuer_api::v1::SetRoleResponse** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::SetRoleResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetRoleResponse_remove(google::protobuf::Map<std::string, ::issuer_api::v1::SetRoleResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::issuer_api::v1::SetRoleResponse * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetRoleResponse_iter(
      const google::protobuf::Map<std::string, ::issuer_api::v1::SetRoleResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetRoleResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::issuer_api::v1::SetRoleResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::issuer_api::v1::SetRoleResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// issuer_api.v1.SetAssetAuthoritiesRequest
        // clang-format off
extern "C" {
void* __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesRequest_new() { return new ::issuer_api::v1::SetAssetAuthoritiesRequest(); }
void __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesRequest_delete(void* ptr) { delete static_cast<::issuer_api::v1::SetAssetAuthoritiesRequest*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesRequest_serialize(::issuer_api::v1::SetAssetAuthoritiesRequest* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesRequest_parse(::issuer_api::v1::SetAssetAuthoritiesRequest* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesRequest_copy_from(::issuer_api::v1::SetAssetAuthoritiesRequest* dst, const ::issuer_api::v1::SetAssetAuthoritiesRequest* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesRequest_repeated_len(google::protobuf::RepeatedPtrField<::issuer_api::v1::SetAssetAuthoritiesRequest>* field) {
  return field->size();
}
const ::issuer_api::v1::SetAssetAuthoritiesRequest& __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesRequest_repeated_get(
  google::protobuf::RepeatedPtrField<::issuer_api::v1::SetAssetAuthoritiesRequest>* field,
  size_t index) {
  return field->Get(index);
}
::issuer_api::v1::SetAssetAuthoritiesRequest* __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesRequest_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::issuer_api::v1::SetAssetAuthoritiesRequest>* field,
  size_t index) {
  return field->Mutable(index);
}
::issuer_api::v1::SetAssetAuthoritiesRequest* __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesRequest_repeated_add(google::protobuf::RepeatedPtrField<::issuer_api::v1::SetAssetAuthoritiesRequest>* field) {
  return field->Add();
}
void __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesRequest_repeated_clear(google::protobuf::RepeatedPtrField<::issuer_api::v1::SetAssetAuthoritiesRequest>* field) {
  field->Clear();
}
void __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesRequest_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::issuer_api::v1::SetAssetAuthoritiesRequest>& dst,
  const google::protobuf::RepeatedPtrField<::issuer_api::v1::SetAssetAuthoritiesRequest>& src) {
  dst = src;
}

const void* __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesRequest_get_authorities(const ::issuer_api::v1::SetAssetAuthoritiesRequest* msg) {
  return &msg->authorities();
}
void* __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesRequest_get_mut_authorities(::issuer_api::v1::SetAssetAuthoritiesRequest* msg) { return msg->mutable_authorities(); }


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::issuer_api::v1::SetAssetAuthoritiesRequest>* __rust_proto_thunk__Map_i32_issuer_api_v1_SetAssetAuthoritiesRequest_new() {
    return new google::protobuf::Map<int32_t, ::issuer_api::v1::SetAssetAuthoritiesRequest>();
  }
  void __rust_proto_thunk__Map_i32_issuer_api_v1_SetAssetAuthoritiesRequest_free(const google::protobuf::Map<int32_t, ::issuer_api::v1::SetAssetAuthoritiesRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_issuer_api_v1_SetAssetAuthoritiesRequest_clear(google::protobuf::Map<int32_t, ::issuer_api::v1::SetAssetAuthoritiesRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_issuer_api_v1_SetAssetAuthoritiesRequest_size(const google::protobuf::Map<int32_t, ::issuer_api::v1::SetAssetAuthoritiesRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_issuer_api_v1_SetAssetAuthoritiesRequest_insert(google::protobuf::Map<int32_t, ::issuer_api::v1::SetAssetAuthoritiesRequest> * m,
                          int32_t key, ::issuer_api::v1::SetAssetAuthoritiesRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_issuer_api_v1_SetAssetAuthoritiesRequest_get(const google::protobuf::Map<int32_t, ::issuer_api::v1::SetAssetAuthoritiesRequest>* m,
                       int32_t key, const ::issuer_api::v1::SetAssetAuthoritiesRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::SetAssetAuthoritiesRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_issuer_api_v1_SetAssetAuthoritiesRequest_remove(google::protobuf::Map<int32_t, ::issuer_api::v1::SetAssetAuthoritiesRequest> * m,
                          int32_t key, ::issuer_api::v1::SetAssetAuthoritiesRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_issuer_api_v1_SetAssetAuthoritiesRequest_iter(
      const google::protobuf::Map<int32_t, ::issuer_api::v1::SetAssetAuthoritiesRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_issuer_api_v1_SetAssetAuthoritiesRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::issuer_api::v1::SetAssetAuthoritiesRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::issuer_api::v1::SetAssetAuthoritiesRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::issuer_api::v1::SetAssetAuthoritiesRequest>* __rust_proto_thunk__Map_u32_issuer_api_v1_SetAssetAuthoritiesRequest_new() {
    return new google::protobuf::Map<uint32_t, ::issuer_api::v1::SetAssetAuthoritiesRequest>();
  }
  void __rust_proto_thunk__Map_u32_issuer_api_v1_SetAssetAuthoritiesRequest_free(const google::protobuf::Map<uint32_t, ::issuer_api::v1::SetAssetAuthoritiesRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_issuer_api_v1_SetAssetAuthoritiesRequest_clear(google::protobuf::Map<uint32_t, ::issuer_api::v1::SetAssetAuthoritiesRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_issuer_api_v1_SetAssetAuthoritiesRequest_size(const google::protobuf::Map<uint32_t, ::issuer_api::v1::SetAssetAuthoritiesRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_issuer_api_v1_SetAssetAuthoritiesRequest_insert(google::protobuf::Map<uint32_t, ::issuer_api::v1::SetAssetAuthoritiesRequest> * m,
                          uint32_t key, ::issuer_api::v1::SetAssetAuthoritiesRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_issuer_api_v1_SetAssetAuthoritiesRequest_get(const google::protobuf::Map<uint32_t, ::issuer_api::v1::SetAssetAuthoritiesRequest>* m,
                       uint32_t key, const ::issuer_api::v1::SetAssetAuthoritiesRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::SetAssetAuthoritiesRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_issuer_api_v1_SetAssetAuthoritiesRequest_remove(google::protobuf::Map<uint32_t, ::issuer_api::v1::SetAssetAuthoritiesRequest> * m,
                          uint32_t key, ::issuer_api::v1::SetAssetAuthoritiesRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_issuer_api_v1_SetAssetAuthoritiesRequest_iter(
      const google::protobuf::Map<uint32_t, ::issuer_api::v1::SetAssetAuthoritiesRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_issuer_api_v1_SetAssetAuthoritiesRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::issuer_api::v1::SetAssetAuthoritiesRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::issuer_api::v1::SetAssetAuthoritiesRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::issuer_api::v1::SetAssetAuthoritiesRequest>* __rust_proto_thunk__Map_i64_issuer_api_v1_SetAssetAuthoritiesRequest_new() {
    return new google::protobuf::Map<int64_t, ::issuer_api::v1::SetAssetAuthoritiesRequest>();
  }
  void __rust_proto_thunk__Map_i64_issuer_api_v1_SetAssetAuthoritiesRequest_free(const google::protobuf::Map<int64_t, ::issuer_api::v1::SetAssetAuthoritiesRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_issuer_api_v1_SetAssetAuthoritiesRequest_clear(google::protobuf::Map<int64_t, ::issuer_api::v1::SetAssetAuthoritiesRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_issuer_api_v1_SetAssetAuthoritiesRequest_size(const google::protobuf::Map<int64_t, ::issuer_api::v1::SetAssetAuthoritiesRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_issuer_api_v1_SetAssetAuthoritiesRequest_insert(google::protobuf::Map<int64_t, ::issuer_api::v1::SetAssetAuthoritiesRequest> * m,
                          int64_t key, ::issuer_api::v1::SetAssetAuthoritiesRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_issuer_api_v1_SetAssetAuthoritiesRequest_get(const google::protobuf::Map<int64_t, ::issuer_api::v1::SetAssetAuthoritiesRequest>* m,
                       int64_t key, const ::issuer_api::v1::SetAssetAuthoritiesRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::SetAssetAuthoritiesRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_issuer_api_v1_SetAssetAuthoritiesRequest_remove(google::protobuf::Map<int64_t, ::issuer_api::v1::SetAssetAuthoritiesRequest> * m,
                          int64_t key, ::issuer_api::v1::SetAssetAuthoritiesRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_issuer_api_v1_SetAssetAuthoritiesRequest_iter(
      const google::protobuf::Map<int64_t, ::issuer_api::v1::SetAssetAuthoritiesRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_issuer_api_v1_SetAssetAuthoritiesRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::issuer_api::v1::SetAssetAuthoritiesRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::issuer_api::v1::SetAssetAuthoritiesRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::issuer_api::v1::SetAssetAuthoritiesRequest>* __rust_proto_thunk__Map_u64_issuer_api_v1_SetAssetAuthoritiesRequest_new() {
    return new google::protobuf::Map<uint64_t, ::issuer_api::v1::SetAssetAuthoritiesRequest>();
  }
  void __rust_proto_thunk__Map_u64_issuer_api_v1_SetAssetAuthoritiesRequest_free(const google::protobuf::Map<uint64_t, ::issuer_api::v1::SetAssetAuthoritiesRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_issuer_api_v1_SetAssetAuthoritiesRequest_clear(google::protobuf::Map<uint64_t, ::issuer_api::v1::SetAssetAuthoritiesRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_issuer_api_v1_SetAssetAuthoritiesRequest_size(const google::protobuf::Map<uint64_t, ::issuer_api::v1::SetAssetAuthoritiesRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_issuer_api_v1_SetAssetAuthoritiesRequest_insert(google::protobuf::Map<uint64_t, ::issuer_api::v1::SetAssetAuthoritiesRequest> * m,
                          uint64_t key, ::issuer_api::v1::SetAssetAuthoritiesRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_issuer_api_v1_SetAssetAuthoritiesRequest_get(const google::protobuf::Map<uint64_t, ::issuer_api::v1::SetAssetAuthoritiesRequest>* m,
                       uint64_t key, const ::issuer_api::v1::SetAssetAuthoritiesRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::SetAssetAuthoritiesRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_issuer_api_v1_SetAssetAuthoritiesRequest_remove(google::protobuf::Map<uint64_t, ::issuer_api::v1::SetAssetAuthoritiesRequest> * m,
                          uint64_t key, ::issuer_api::v1::SetAssetAuthoritiesRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_issuer_api_v1_SetAssetAuthoritiesRequest_iter(
      const google::protobuf::Map<uint64_t, ::issuer_api::v1::SetAssetAuthoritiesRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_issuer_api_v1_SetAssetAuthoritiesRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::issuer_api::v1::SetAssetAuthoritiesRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::issuer_api::v1::SetAssetAuthoritiesRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::issuer_api::v1::SetAssetAuthoritiesRequest>* __rust_proto_thunk__Map_bool_issuer_api_v1_SetAssetAuthoritiesRequest_new() {
    return new google::protobuf::Map<bool, ::issuer_api::v1::SetAssetAuthoritiesRequest>();
  }
  void __rust_proto_thunk__Map_bool_issuer_api_v1_SetAssetAuthoritiesRequest_free(const google::protobuf::Map<bool, ::issuer_api::v1::SetAssetAuthoritiesRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_issuer_api_v1_SetAssetAuthoritiesRequest_clear(google::protobuf::Map<bool, ::issuer_api::v1::SetAssetAuthoritiesRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_issuer_api_v1_SetAssetAuthoritiesRequest_size(const google::protobuf::Map<bool, ::issuer_api::v1::SetAssetAuthoritiesRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_issuer_api_v1_SetAssetAuthoritiesRequest_insert(google::protobuf::Map<bool, ::issuer_api::v1::SetAssetAuthoritiesRequest> * m,
                          bool key, ::issuer_api::v1::SetAssetAuthoritiesRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_issuer_api_v1_SetAssetAuthoritiesRequest_get(const google::protobuf::Map<bool, ::issuer_api::v1::SetAssetAuthoritiesRequest>* m,
                       bool key, const ::issuer_api::v1::SetAssetAuthoritiesRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::SetAssetAuthoritiesRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_issuer_api_v1_SetAssetAuthoritiesRequest_remove(google::protobuf::Map<bool, ::issuer_api::v1::SetAssetAuthoritiesRequest> * m,
                          bool key, ::issuer_api::v1::SetAssetAuthoritiesRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_issuer_api_v1_SetAssetAuthoritiesRequest_iter(
      const google::protobuf::Map<bool, ::issuer_api::v1::SetAssetAuthoritiesRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_issuer_api_v1_SetAssetAuthoritiesRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::issuer_api::v1::SetAssetAuthoritiesRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::issuer_api::v1::SetAssetAuthoritiesRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::issuer_api::v1::SetAssetAuthoritiesRequest>* __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetAssetAuthoritiesRequest_new() {
    return new google::protobuf::Map<std::string, ::issuer_api::v1::SetAssetAuthoritiesRequest>();
  }
  void __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetAssetAuthoritiesRequest_free(const google::protobuf::Map<std::string, ::issuer_api::v1::SetAssetAuthoritiesRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetAssetAuthoritiesRequest_clear(google::protobuf::Map<std::string, ::issuer_api::v1::SetAssetAuthoritiesRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetAssetAuthoritiesRequest_size(const google::protobuf::Map<std::string, ::issuer_api::v1::SetAssetAuthoritiesRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetAssetAuthoritiesRequest_insert(google::protobuf::Map<std::string, ::issuer_api::v1::SetAssetAuthoritiesRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::issuer_api::v1::SetAssetAuthoritiesRequest value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetAssetAuthoritiesRequest_get(const google::protobuf::Map<std::string, ::issuer_api::v1::SetAssetAuthoritiesRequest>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::issuer_api::v1::SetAssetAuthoritiesRequest** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::SetAssetAuthoritiesRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetAssetAuthoritiesRequest_remove(google::protobuf::Map<std::string, ::issuer_api::v1::SetAssetAuthoritiesRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::issuer_api::v1::SetAssetAuthoritiesRequest * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetAssetAuthoritiesRequest_iter(
      const google::protobuf::Map<std::string, ::issuer_api::v1::SetAssetAuthoritiesRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetAssetAuthoritiesRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::issuer_api::v1::SetAssetAuthoritiesRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::issuer_api::v1::SetAssetAuthoritiesRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// issuer_api.v1.SetAssetAuthoritiesResponse
        // clang-format off
extern "C" {
void* __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesResponse_new() { return new ::issuer_api::v1::SetAssetAuthoritiesResponse(); }
void __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesResponse_delete(void* ptr) { delete static_cast<::issuer_api::v1::SetAssetAuthoritiesResponse*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesResponse_serialize(::issuer_api::v1::SetAssetAuthoritiesResponse* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesResponse_parse(::issuer_api::v1::SetAssetAuthoritiesResponse* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesResponse_copy_from(::issuer_api::v1::SetAssetAuthoritiesResponse* dst, const ::issuer_api::v1::SetAssetAuthoritiesResponse* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesResponse_repeated_len(google::protobuf::RepeatedPtrField<::issuer_api::v1::SetAssetAuthoritiesResponse>* field) {
  return field->size();
}
const ::issuer_api::v1::SetAssetAuthoritiesResponse& __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesResponse_repeated_get(
  google::protobuf::RepeatedPtrField<::issuer_api::v1::SetAssetAuthoritiesResponse>* field,
  size_t index) {
  return field->Get(index);
}
::issuer_api::v1::SetAssetAuthoritiesResponse* __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesResponse_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::issuer_api::v1::SetAssetAuthoritiesResponse>* field,
  size_t index) {
  return field->Mutable(index);
}
::issuer_api::v1::SetAssetAuthoritiesResponse* __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesResponse_repeated_add(google::protobuf::RepeatedPtrField<::issuer_api::v1::SetAssetAuthoritiesResponse>* field) {
  return field->Add();
}
void __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesResponse_repeated_clear(google::protobuf::RepeatedPtrField<::issuer_api::v1::SetAssetAuthoritiesResponse>* field) {
  field->Clear();
}
void __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesResponse_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::issuer_api::v1::SetAssetAuthoritiesResponse>& dst,
  const google::protobuf::RepeatedPtrField<::issuer_api::v1::SetAssetAuthoritiesResponse>& src) {
  dst = src;
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::issuer_api::v1::SetAssetAuthoritiesResponse>* __rust_proto_thunk__Map_i32_issuer_api_v1_SetAssetAuthoritiesResponse_new() {
    return new google::protobuf::Map<int32_t, ::issuer_api::v1::SetAssetAuthoritiesResponse>();
  }
  void __rust_proto_thunk__Map_i32_issuer_api_v1_SetAssetAuthoritiesResponse_free(const google::protobuf::Map<int32_t, ::issuer_api::v1::SetAssetAuthoritiesResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_issuer_api_v1_SetAssetAuthoritiesResponse_clear(google::protobuf::Map<int32_t, ::issuer_api::v1::SetAssetAuthoritiesResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_issuer_api_v1_SetAssetAuthoritiesResponse_size(const google::protobuf::Map<int32_t, ::issuer_api::v1::SetAssetAuthoritiesResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_issuer_api_v1_SetAssetAuthoritiesResponse_insert(google::protobuf::Map<int32_t, ::issuer_api::v1::SetAssetAuthoritiesResponse> * m,
                          int32_t key, ::issuer_api::v1::SetAssetAuthoritiesResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_issuer_api_v1_SetAssetAuthoritiesResponse_get(const google::protobuf::Map<int32_t, ::issuer_api::v1::SetAssetAuthoritiesResponse>* m,
                       int32_t key, const ::issuer_api::v1::SetAssetAuthoritiesResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::SetAssetAuthoritiesResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_issuer_api_v1_SetAssetAuthoritiesResponse_remove(google::protobuf::Map<int32_t, ::issuer_api::v1::SetAssetAuthoritiesResponse> * m,
                          int32_t key, ::issuer_api::v1::SetAssetAuthoritiesResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_issuer_api_v1_SetAssetAuthoritiesResponse_iter(
      const google::protobuf::Map<int32_t, ::issuer_api::v1::SetAssetAuthoritiesResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_issuer_api_v1_SetAssetAuthoritiesResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::issuer_api::v1::SetAssetAuthoritiesResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::issuer_api::v1::SetAssetAuthoritiesResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::issuer_api::v1::SetAssetAuthoritiesResponse>* __rust_proto_thunk__Map_u32_issuer_api_v1_SetAssetAuthoritiesResponse_new() {
    return new google::protobuf::Map<uint32_t, ::issuer_api::v1::SetAssetAuthoritiesResponse>();
  }
  void __rust_proto_thunk__Map_u32_issuer_api_v1_SetAssetAuthoritiesResponse_free(const google::protobuf::Map<uint32_t, ::issuer_api::v1::SetAssetAuthoritiesResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_issuer_api_v1_SetAssetAuthoritiesResponse_clear(google::protobuf::Map<uint32_t, ::issuer_api::v1::SetAssetAuthoritiesResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_issuer_api_v1_SetAssetAuthoritiesResponse_size(const google::protobuf::Map<uint32_t, ::issuer_api::v1::SetAssetAuthoritiesResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_issuer_api_v1_SetAssetAuthoritiesResponse_insert(google::protobuf::Map<uint32_t, ::issuer_api::v1::SetAssetAuthoritiesResponse> * m,
                          uint32_t key, ::issuer_api::v1::SetAssetAuthoritiesResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_issuer_api_v1_SetAssetAuthoritiesResponse_get(const google::protobuf::Map<uint32_t, ::issuer_api::v1::SetAssetAuthoritiesResponse>* m,
                       uint32_t key, const ::issuer_api::v1::SetAssetAuthoritiesResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::SetAssetAuthoritiesResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_issuer_api_v1_SetAssetAuthoritiesResponse_remove(google::protobuf::Map<uint32_t, ::issuer_api::v1::SetAssetAuthoritiesResponse> * m,
                          uint32_t key, ::issuer_api::v1::SetAssetAuthoritiesResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_issuer_api_v1_SetAssetAuthoritiesResponse_iter(
      const google::protobuf::Map<uint32_t, ::issuer_api::v1::SetAssetAuthoritiesResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_issuer_api_v1_SetAssetAuthoritiesResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::issuer_api::v1::SetAssetAuthoritiesResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::issuer_api::v1::SetAssetAuthoritiesResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::issuer_api::v1::SetAssetAuthoritiesResponse>* __rust_proto_thunk__Map_i64_issuer_api_v1_SetAssetAuthoritiesResponse_new() {
    return new google::protobuf::Map<int64_t, ::issuer_api::v1::SetAssetAuthoritiesResponse>();
  }
  void __rust_proto_thunk__Map_i64_issuer_api_v1_SetAssetAuthoritiesResponse_free(const google::protobuf::Map<int64_t, ::issuer_api::v1::SetAssetAuthoritiesResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_issuer_api_v1_SetAssetAuthoritiesResponse_clear(google::protobuf::Map<int64_t, ::issuer_api::v1::SetAssetAuthoritiesResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_issuer_api_v1_SetAssetAuthoritiesResponse_size(const google::protobuf::Map<int64_t, ::issuer_api::v1::SetAssetAuthoritiesResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_issuer_api_v1_SetAssetAuthoritiesResponse_insert(google::protobuf::Map<int64_t, ::issuer_api::v1::SetAssetAuthoritiesResponse> * m,
                          int64_t key, ::issuer_api::v1::SetAssetAuthoritiesResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_issuer_api_v1_SetAssetAuthoritiesResponse_get(const google::protobuf::Map<int64_t, ::issuer_api::v1::SetAssetAuthoritiesResponse>* m,
                       int64_t key, const ::issuer_api::v1::SetAssetAuthoritiesResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::SetAssetAuthoritiesResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_issuer_api_v1_SetAssetAuthoritiesResponse_remove(google::protobuf::Map<int64_t, ::issuer_api::v1::SetAssetAuthoritiesResponse> * m,
                          int64_t key, ::issuer_api::v1::SetAssetAuthoritiesResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_issuer_api_v1_SetAssetAuthoritiesResponse_iter(
      const google::protobuf::Map<int64_t, ::issuer_api::v1::SetAssetAuthoritiesResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_issuer_api_v1_SetAssetAuthoritiesResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::issuer_api::v1::SetAssetAuthoritiesResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::issuer_api::v1::SetAssetAuthoritiesResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::issuer_api::v1::SetAssetAuthoritiesResponse>* __rust_proto_thunk__Map_u64_issuer_api_v1_SetAssetAuthoritiesResponse_new() {
    return new google::protobuf::Map<uint64_t, ::issuer_api::v1::SetAssetAuthoritiesResponse>();
  }
  void __rust_proto_thunk__Map_u64_issuer_api_v1_SetAssetAuthoritiesResponse_free(const google::protobuf::Map<uint64_t, ::issuer_api::v1::SetAssetAuthoritiesResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_issuer_api_v1_SetAssetAuthoritiesResponse_clear(google::protobuf::Map<uint64_t, ::issuer_api::v1::SetAssetAuthoritiesResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_issuer_api_v1_SetAssetAuthoritiesResponse_size(const google::protobuf::Map<uint64_t, ::issuer_api::v1::SetAssetAuthoritiesResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_issuer_api_v1_SetAssetAuthoritiesResponse_insert(google::protobuf::Map<uint64_t, ::issuer_api::v1::SetAssetAuthoritiesResponse> * m,
                          uint64_t key, ::issuer_api::v1::SetAssetAuthoritiesResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_issuer_api_v1_SetAssetAuthoritiesResponse_get(const google::protobuf::Map<uint64_t, ::issuer_api::v1::SetAssetAuthoritiesResponse>* m,
                       uint64_t key, const ::issuer_api::v1::SetAssetAuthoritiesResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::SetAssetAuthoritiesResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_issuer_api_v1_SetAssetAuthoritiesResponse_remove(google::protobuf::Map<uint64_t, ::issuer_api::v1::SetAssetAuthoritiesResponse> * m,
                          uint64_t key, ::issuer_api::v1::SetAssetAuthoritiesResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_issuer_api_v1_SetAssetAuthoritiesResponse_iter(
      const google::protobuf::Map<uint64_t, ::issuer_api::v1::SetAssetAuthoritiesResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_issuer_api_v1_SetAssetAuthoritiesResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::issuer_api::v1::SetAssetAuthoritiesResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::issuer_api::v1::SetAssetAuthoritiesResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::issuer_api::v1::SetAssetAuthoritiesResponse>* __rust_proto_thunk__Map_bool_issuer_api_v1_SetAssetAuthoritiesResponse_new() {
    return new google::protobuf::Map<bool, ::issuer_api::v1::SetAssetAuthoritiesResponse>();
  }
  void __rust_proto_thunk__Map_bool_issuer_api_v1_SetAssetAuthoritiesResponse_free(const google::protobuf::Map<bool, ::issuer_api::v1::SetAssetAuthoritiesResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_issuer_api_v1_SetAssetAuthoritiesResponse_clear(google::protobuf::Map<bool, ::issuer_api::v1::SetAssetAuthoritiesResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_issuer_api_v1_SetAssetAuthoritiesResponse_size(const google::protobuf::Map<bool, ::issuer_api::v1::SetAssetAuthoritiesResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_issuer_api_v1_SetAssetAuthoritiesResponse_insert(google::protobuf::Map<bool, ::issuer_api::v1::SetAssetAuthoritiesResponse> * m,
                          bool key, ::issuer_api::v1::SetAssetAuthoritiesResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_issuer_api_v1_SetAssetAuthoritiesResponse_get(const google::protobuf::Map<bool, ::issuer_api::v1::SetAssetAuthoritiesResponse>* m,
                       bool key, const ::issuer_api::v1::SetAssetAuthoritiesResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::SetAssetAuthoritiesResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_issuer_api_v1_SetAssetAuthoritiesResponse_remove(google::protobuf::Map<bool, ::issuer_api::v1::SetAssetAuthoritiesResponse> * m,
                          bool key, ::issuer_api::v1::SetAssetAuthoritiesResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_issuer_api_v1_SetAssetAuthoritiesResponse_iter(
      const google::protobuf::Map<bool, ::issuer_api::v1::SetAssetAuthoritiesResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_issuer_api_v1_SetAssetAuthoritiesResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::issuer_api::v1::SetAssetAuthoritiesResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::issuer_api::v1::SetAssetAuthoritiesResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::issuer_api::v1::SetAssetAuthoritiesResponse>* __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetAssetAuthoritiesResponse_new() {
    return new google::protobuf::Map<std::string, ::issuer_api::v1::SetAssetAuthoritiesResponse>();
  }
  void __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetAssetAuthoritiesResponse_free(const google::protobuf::Map<std::string, ::issuer_api::v1::SetAssetAuthoritiesResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetAssetAuthoritiesResponse_clear(google::protobuf::Map<std::string, ::issuer_api::v1::SetAssetAuthoritiesResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetAssetAuthoritiesResponse_size(const google::protobuf::Map<std::string, ::issuer_api::v1::SetAssetAuthoritiesResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetAssetAuthoritiesResponse_insert(google::protobuf::Map<std::string, ::issuer_api::v1::SetAssetAuthoritiesResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::issuer_api::v1::SetAssetAuthoritiesResponse value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetAssetAuthoritiesResponse_get(const google::protobuf::Map<std::string, ::issuer_api::v1::SetAssetAuthoritiesResponse>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::issuer_api::v1::SetAssetAuthoritiesResponse** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::SetAssetAuthoritiesResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetAssetAuthoritiesResponse_remove(google::protobuf::Map<std::string, ::issuer_api::v1::SetAssetAuthoritiesResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::issuer_api::v1::SetAssetAuthoritiesResponse * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetAssetAuthoritiesResponse_iter(
      const google::protobuf::Map<std::string, ::issuer_api::v1::SetAssetAuthoritiesResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetAssetAuthoritiesResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::issuer_api::v1::SetAssetAuthoritiesResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::issuer_api::v1::SetAssetAuthoritiesResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// issuer_api.v1.AddAssetAuthorityRequest
        // clang-format off
extern "C" {
void* __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityRequest_new() { return new ::issuer_api::v1::AddAssetAuthorityRequest(); }
void __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityRequest_delete(void* ptr) { delete static_cast<::issuer_api::v1::AddAssetAuthorityRequest*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityRequest_serialize(::issuer_api::v1::AddAssetAuthorityRequest* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityRequest_parse(::issuer_api::v1::AddAssetAuthorityRequest* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityRequest_copy_from(::issuer_api::v1::AddAssetAuthorityRequest* dst, const ::issuer_api::v1::AddAssetAuthorityRequest* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityRequest_repeated_len(google::protobuf::RepeatedPtrField<::issuer_api::v1::AddAssetAuthorityRequest>* field) {
  return field->size();
}
const ::issuer_api::v1::AddAssetAuthorityRequest& __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityRequest_repeated_get(
  google::protobuf::RepeatedPtrField<::issuer_api::v1::AddAssetAuthorityRequest>* field,
  size_t index) {
  return field->Get(index);
}
::issuer_api::v1::AddAssetAuthorityRequest* __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityRequest_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::issuer_api::v1::AddAssetAuthorityRequest>* field,
  size_t index) {
  return field->Mutable(index);
}
::issuer_api::v1::AddAssetAuthorityRequest* __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityRequest_repeated_add(google::protobuf::RepeatedPtrField<::issuer_api::v1::AddAssetAuthorityRequest>* field) {
  return field->Add();
}
void __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityRequest_repeated_clear(google::protobuf::RepeatedPtrField<::issuer_api::v1::AddAssetAuthorityRequest>* field) {
  field->Clear();
}
void __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityRequest_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::issuer_api::v1::AddAssetAuthorityRequest>& dst,
  const google::protobuf::RepeatedPtrField<::issuer_api::v1::AddAssetAuthorityRequest>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityRequest_get_currency_code(::issuer_api::v1::AddAssetAuthorityRequest* msg) {
  absl::string_view val = msg->currency_code();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityRequest_set_currency_code(::issuer_api::v1::AddAssetAuthorityRequest* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_currency_code(absl::string_view(s.ptr, s.len));
}

const void* __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityRequest_get_authority(::issuer_api::v1::AddAssetAuthorityRequest* msg) {
  return static_cast<const void*>(&msg->authority());
}
void* __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityRequest_get_mut_authority(::issuer_api::v1::AddAssetAuthorityRequest* msg) {
  return static_cast<void*>(msg->mutable_authority());
}
void __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityRequest_clear_authority(::issuer_api::v1::AddAssetAuthorityRequest* msg) { msg->clear_authority(); }
bool __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityRequest_has_authority(::issuer_api::v1::AddAssetAuthorityRequest* msg) { return msg->has_authority(); }


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::issuer_api::v1::AddAssetAuthorityRequest>* __rust_proto_thunk__Map_i32_issuer_api_v1_AddAssetAuthorityRequest_new() {
    return new google::protobuf::Map<int32_t, ::issuer_api::v1::AddAssetAuthorityRequest>();
  }
  void __rust_proto_thunk__Map_i32_issuer_api_v1_AddAssetAuthorityRequest_free(const google::protobuf::Map<int32_t, ::issuer_api::v1::AddAssetAuthorityRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_issuer_api_v1_AddAssetAuthorityRequest_clear(google::protobuf::Map<int32_t, ::issuer_api::v1::AddAssetAuthorityRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_issuer_api_v1_AddAssetAuthorityRequest_size(const google::protobuf::Map<int32_t, ::issuer_api::v1::AddAssetAuthorityRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_issuer_api_v1_AddAssetAuthorityRequest_insert(google::protobuf::Map<int32_t, ::issuer_api::v1::AddAssetAuthorityRequest> * m,
                          int32_t key, ::issuer_api::v1::AddAssetAuthorityRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_issuer_api_v1_AddAssetAuthorityRequest_get(const google::protobuf::Map<int32_t, ::issuer_api::v1::AddAssetAuthorityRequest>* m,
                       int32_t key, const ::issuer_api::v1::AddAssetAuthorityRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::AddAssetAuthorityRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_issuer_api_v1_AddAssetAuthorityRequest_remove(google::protobuf::Map<int32_t, ::issuer_api::v1::AddAssetAuthorityRequest> * m,
                          int32_t key, ::issuer_api::v1::AddAssetAuthorityRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_issuer_api_v1_AddAssetAuthorityRequest_iter(
      const google::protobuf::Map<int32_t, ::issuer_api::v1::AddAssetAuthorityRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_issuer_api_v1_AddAssetAuthorityRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::issuer_api::v1::AddAssetAuthorityRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::issuer_api::v1::AddAssetAuthorityRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::issuer_api::v1::AddAssetAuthorityRequest>* __rust_proto_thunk__Map_u32_issuer_api_v1_AddAssetAuthorityRequest_new() {
    return new google::protobuf::Map<uint32_t, ::issuer_api::v1::AddAssetAuthorityRequest>();
  }
  void __rust_proto_thunk__Map_u32_issuer_api_v1_AddAssetAuthorityRequest_free(const google::protobuf::Map<uint32_t, ::issuer_api::v1::AddAssetAuthorityRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_issuer_api_v1_AddAssetAuthorityRequest_clear(google::protobuf::Map<uint32_t, ::issuer_api::v1::AddAssetAuthorityRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_issuer_api_v1_AddAssetAuthorityRequest_size(const google::protobuf::Map<uint32_t, ::issuer_api::v1::AddAssetAuthorityRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_issuer_api_v1_AddAssetAuthorityRequest_insert(google::protobuf::Map<uint32_t, ::issuer_api::v1::AddAssetAuthorityRequest> * m,
                          uint32_t key, ::issuer_api::v1::AddAssetAuthorityRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_issuer_api_v1_AddAssetAuthorityRequest_get(const google::protobuf::Map<uint32_t, ::issuer_api::v1::AddAssetAuthorityRequest>* m,
                       uint32_t key, const ::issuer_api::v1::AddAssetAuthorityRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::AddAssetAuthorityRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_issuer_api_v1_AddAssetAuthorityRequest_remove(google::protobuf::Map<uint32_t, ::issuer_api::v1::AddAssetAuthorityRequest> * m,
                          uint32_t key, ::issuer_api::v1::AddAssetAuthorityRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_issuer_api_v1_AddAssetAuthorityRequest_iter(
      const google::protobuf::Map<uint32_t, ::issuer_api::v1::AddAssetAuthorityRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_issuer_api_v1_AddAssetAuthorityRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::issuer_api::v1::AddAssetAuthorityRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::issuer_api::v1::AddAssetAuthorityRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::issuer_api::v1::AddAssetAuthorityRequest>* __rust_proto_thunk__Map_i64_issuer_api_v1_AddAssetAuthorityRequest_new() {
    return new google::protobuf::Map<int64_t, ::issuer_api::v1::AddAssetAuthorityRequest>();
  }
  void __rust_proto_thunk__Map_i64_issuer_api_v1_AddAssetAuthorityRequest_free(const google::protobuf::Map<int64_t, ::issuer_api::v1::AddAssetAuthorityRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_issuer_api_v1_AddAssetAuthorityRequest_clear(google::protobuf::Map<int64_t, ::issuer_api::v1::AddAssetAuthorityRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_issuer_api_v1_AddAssetAuthorityRequest_size(const google::protobuf::Map<int64_t, ::issuer_api::v1::AddAssetAuthorityRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_issuer_api_v1_AddAssetAuthorityRequest_insert(google::protobuf::Map<int64_t, ::issuer_api::v1::AddAssetAuthorityRequest> * m,
                          int64_t key, ::issuer_api::v1::AddAssetAuthorityRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_issuer_api_v1_AddAssetAuthorityRequest_get(const google::protobuf::Map<int64_t, ::issuer_api::v1::AddAssetAuthorityRequest>* m,
                       int64_t key, const ::issuer_api::v1::AddAssetAuthorityRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::AddAssetAuthorityRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_issuer_api_v1_AddAssetAuthorityRequest_remove(google::protobuf::Map<int64_t, ::issuer_api::v1::AddAssetAuthorityRequest> * m,
                          int64_t key, ::issuer_api::v1::AddAssetAuthorityRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_issuer_api_v1_AddAssetAuthorityRequest_iter(
      const google::protobuf::Map<int64_t, ::issuer_api::v1::AddAssetAuthorityRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_issuer_api_v1_AddAssetAuthorityRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::issuer_api::v1::AddAssetAuthorityRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::issuer_api::v1::AddAssetAuthorityRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::issuer_api::v1::AddAssetAuthorityRequest>* __rust_proto_thunk__Map_u64_issuer_api_v1_AddAssetAuthorityRequest_new() {
    return new google::protobuf::Map<uint64_t, ::issuer_api::v1::AddAssetAuthorityRequest>();
  }
  void __rust_proto_thunk__Map_u64_issuer_api_v1_AddAssetAuthorityRequest_free(const google::protobuf::Map<uint64_t, ::issuer_api::v1::AddAssetAuthorityRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_issuer_api_v1_AddAssetAuthorityRequest_clear(google::protobuf::Map<uint64_t, ::issuer_api::v1::AddAssetAuthorityRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_issuer_api_v1_AddAssetAuthorityRequest_size(const google::protobuf::Map<uint64_t, ::issuer_api::v1::AddAssetAuthorityRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_issuer_api_v1_AddAssetAuthorityRequest_insert(google::protobuf::Map<uint64_t, ::issuer_api::v1::AddAssetAuthorityRequest> * m,
                          uint64_t key, ::issuer_api::v1::AddAssetAuthorityRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_issuer_api_v1_AddAssetAuthorityRequest_get(const google::protobuf::Map<uint64_t, ::issuer_api::v1::AddAssetAuthorityRequest>* m,
                       uint64_t key, const ::issuer_api::v1::AddAssetAuthorityRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::AddAssetAuthorityRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_issuer_api_v1_AddAssetAuthorityRequest_remove(google::protobuf::Map<uint64_t, ::issuer_api::v1::AddAssetAuthorityRequest> * m,
                          uint64_t key, ::issuer_api::v1::AddAssetAuthorityRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_issuer_api_v1_AddAssetAuthorityRequest_iter(
      const google::protobuf::Map<uint64_t, ::issuer_api::v1::AddAssetAuthorityRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_issuer_api_v1_AddAssetAuthorityRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::issuer_api::v1::AddAssetAuthorityRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::issuer_api::v1::AddAssetAuthorityRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::issuer_api::v1::AddAssetAuthorityRequest>* __rust_proto_thunk__Map_bool_issuer_api_v1_AddAssetAuthorityRequest_new() {
    return new google::protobuf::Map<bool, ::issuer_api::v1::AddAssetAuthorityRequest>();
  }
  void __rust_proto_thunk__Map_bool_issuer_api_v1_AddAssetAuthorityRequest_free(const google::protobuf::Map<bool, ::issuer_api::v1::AddAssetAuthorityRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_issuer_api_v1_AddAssetAuthorityRequest_clear(google::protobuf::Map<bool, ::issuer_api::v1::AddAssetAuthorityRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_issuer_api_v1_AddAssetAuthorityRequest_size(const google::protobuf::Map<bool, ::issuer_api::v1::AddAssetAuthorityRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_issuer_api_v1_AddAssetAuthorityRequest_insert(google::protobuf::Map<bool, ::issuer_api::v1::AddAssetAuthorityRequest> * m,
                          bool key, ::issuer_api::v1::AddAssetAuthorityRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_issuer_api_v1_AddAssetAuthorityRequest_get(const google::protobuf::Map<bool, ::issuer_api::v1::AddAssetAuthorityRequest>* m,
                       bool key, const ::issuer_api::v1::AddAssetAuthorityRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::AddAssetAuthorityRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_issuer_api_v1_AddAssetAuthorityRequest_remove(google::protobuf::Map<bool, ::issuer_api::v1::AddAssetAuthorityRequest> * m,
                          bool key, ::issuer_api::v1::AddAssetAuthorityRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_issuer_api_v1_AddAssetAuthorityRequest_iter(
      const google::protobuf::Map<bool, ::issuer_api::v1::AddAssetAuthorityRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_issuer_api_v1_AddAssetAuthorityRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::issuer_api::v1::AddAssetAuthorityRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::issuer_api::v1::AddAssetAuthorityRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::issuer_api::v1::AddAssetAuthorityRequest>* __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AddAssetAuthorityRequest_new() {
    return new google::protobuf::Map<std::string, ::issuer_api::v1::AddAssetAuthorityRequest>();
  }
  void __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AddAssetAuthorityRequest_free(const google::protobuf::Map<std::string, ::issuer_api::v1::AddAssetAuthorityRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AddAssetAuthorityRequest_clear(google::protobuf::Map<std::string, ::issuer_api::v1::AddAssetAuthorityRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AddAssetAuthorityRequest_size(const google::protobuf::Map<std::string, ::issuer_api::v1::AddAssetAuthorityRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AddAssetAuthorityRequest_insert(google::protobuf::Map<std::string, ::issuer_api::v1::AddAssetAuthorityRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::issuer_api::v1::AddAssetAuthorityRequest value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AddAssetAuthorityRequest_get(const google::protobuf::Map<std::string, ::issuer_api::v1::AddAssetAuthorityRequest>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::issuer_api::v1::AddAssetAuthorityRequest** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::AddAssetAuthorityRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AddAssetAuthorityRequest_remove(google::protobuf::Map<std::string, ::issuer_api::v1::AddAssetAuthorityRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::issuer_api::v1::AddAssetAuthorityRequest * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AddAssetAuthorityRequest_iter(
      const google::protobuf::Map<std::string, ::issuer_api::v1::AddAssetAuthorityRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AddAssetAuthorityRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::issuer_api::v1::AddAssetAuthorityRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::issuer_api::v1::AddAssetAuthorityRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// issuer_api.v1.AddAssetAuthorityResponse
        // clang-format off
extern "C" {
void* __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityResponse_new() { return new ::issuer_api::v1::AddAssetAuthorityResponse(); }
void __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityResponse_delete(void* ptr) { delete static_cast<::issuer_api::v1::AddAssetAuthorityResponse*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityResponse_serialize(::issuer_api::v1::AddAssetAuthorityResponse* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityResponse_parse(::issuer_api::v1::AddAssetAuthorityResponse* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityResponse_copy_from(::issuer_api::v1::AddAssetAuthorityResponse* dst, const ::issuer_api::v1::AddAssetAuthorityResponse* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityResponse_repeated_len(google::protobuf::RepeatedPtrField<::issuer_api::v1::AddAssetAuthorityResponse>* field) {
  return field->size();
}
const ::issuer_api::v1::AddAssetAuthorityResponse& __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityResponse_repeated_get(
  google::protobuf::RepeatedPtrField<::issuer_api::v1::AddAssetAuthorityResponse>* field,
  size_t index) {
  return field->Get(index);
}
::issuer_api::v1::AddAssetAuthorityResponse* __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityResponse_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::issuer_api::v1::AddAssetAuthorityResponse>* field,
  size_t index) {
  return field->Mutable(index);
}
::issuer_api::v1::AddAssetAuthorityResponse* __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityResponse_repeated_add(google::protobuf::RepeatedPtrField<::issuer_api::v1::AddAssetAuthorityResponse>* field) {
  return field->Add();
}
void __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityResponse_repeated_clear(google::protobuf::RepeatedPtrField<::issuer_api::v1::AddAssetAuthorityResponse>* field) {
  field->Clear();
}
void __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityResponse_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::issuer_api::v1::AddAssetAuthorityResponse>& dst,
  const google::protobuf::RepeatedPtrField<::issuer_api::v1::AddAssetAuthorityResponse>& src) {
  dst = src;
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::issuer_api::v1::AddAssetAuthorityResponse>* __rust_proto_thunk__Map_i32_issuer_api_v1_AddAssetAuthorityResponse_new() {
    return new google::protobuf::Map<int32_t, ::issuer_api::v1::AddAssetAuthorityResponse>();
  }
  void __rust_proto_thunk__Map_i32_issuer_api_v1_AddAssetAuthorityResponse_free(const google::protobuf::Map<int32_t, ::issuer_api::v1::AddAssetAuthorityResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_issuer_api_v1_AddAssetAuthorityResponse_clear(google::protobuf::Map<int32_t, ::issuer_api::v1::AddAssetAuthorityResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_issuer_api_v1_AddAssetAuthorityResponse_size(const google::protobuf::Map<int32_t, ::issuer_api::v1::AddAssetAuthorityResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_issuer_api_v1_AddAssetAuthorityResponse_insert(google::protobuf::Map<int32_t, ::issuer_api::v1::AddAssetAuthorityResponse> * m,
                          int32_t key, ::issuer_api::v1::AddAssetAuthorityResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_issuer_api_v1_AddAssetAuthorityResponse_get(const google::protobuf::Map<int32_t, ::issuer_api::v1::AddAssetAuthorityResponse>* m,
                       int32_t key, const ::issuer_api::v1::AddAssetAuthorityResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::AddAssetAuthorityResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_issuer_api_v1_AddAssetAuthorityResponse_remove(google::protobuf::Map<int32_t, ::issuer_api::v1::AddAssetAuthorityResponse> * m,
                          int32_t key, ::issuer_api::v1::AddAssetAuthorityResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_issuer_api_v1_AddAssetAuthorityResponse_iter(
      const google::protobuf::Map<int32_t, ::issuer_api::v1::AddAssetAuthorityResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_issuer_api_v1_AddAssetAuthorityResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::issuer_api::v1::AddAssetAuthorityResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::issuer_api::v1::AddAssetAuthorityResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::issuer_api::v1::AddAssetAuthorityResponse>* __rust_proto_thunk__Map_u32_issuer_api_v1_AddAssetAuthorityResponse_new() {
    return new google::protobuf::Map<uint32_t, ::issuer_api::v1::AddAssetAuthorityResponse>();
  }
  void __rust_proto_thunk__Map_u32_issuer_api_v1_AddAssetAuthorityResponse_free(const google::protobuf::Map<uint32_t, ::issuer_api::v1::AddAssetAuthorityResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_issuer_api_v1_AddAssetAuthorityResponse_clear(google::protobuf::Map<uint32_t, ::issuer_api::v1::AddAssetAuthorityResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_issuer_api_v1_AddAssetAuthorityResponse_size(const google::protobuf::Map<uint32_t, ::issuer_api::v1::AddAssetAuthorityResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_issuer_api_v1_AddAssetAuthorityResponse_insert(google::protobuf::Map<uint32_t, ::issuer_api::v1::AddAssetAuthorityResponse> * m,
                          uint32_t key, ::issuer_api::v1::AddAssetAuthorityResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_issuer_api_v1_AddAssetAuthorityResponse_get(const google::protobuf::Map<uint32_t, ::issuer_api::v1::AddAssetAuthorityResponse>* m,
                       uint32_t key, const ::issuer_api::v1::AddAssetAuthorityResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::AddAssetAuthorityResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_issuer_api_v1_AddAssetAuthorityResponse_remove(google::protobuf::Map<uint32_t, ::issuer_api::v1::AddAssetAuthorityResponse> * m,
                          uint32_t key, ::issuer_api::v1::AddAssetAuthorityResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_issuer_api_v1_AddAssetAuthorityResponse_iter(
      const google::protobuf::Map<uint32_t, ::issuer_api::v1::AddAssetAuthorityResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_issuer_api_v1_AddAssetAuthorityResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::issuer_api::v1::AddAssetAuthorityResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::issuer_api::v1::AddAssetAuthorityResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::issuer_api::v1::AddAssetAuthorityResponse>* __rust_proto_thunk__Map_i64_issuer_api_v1_AddAssetAuthorityResponse_new() {
    return new google::protobuf::Map<int64_t, ::issuer_api::v1::AddAssetAuthorityResponse>();
  }
  void __rust_proto_thunk__Map_i64_issuer_api_v1_AddAssetAuthorityResponse_free(const google::protobuf::Map<int64_t, ::issuer_api::v1::AddAssetAuthorityResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_issuer_api_v1_AddAssetAuthorityResponse_clear(google::protobuf::Map<int64_t, ::issuer_api::v1::AddAssetAuthorityResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_issuer_api_v1_AddAssetAuthorityResponse_size(const google::protobuf::Map<int64_t, ::issuer_api::v1::AddAssetAuthorityResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_issuer_api_v1_AddAssetAuthorityResponse_insert(google::protobuf::Map<int64_t, ::issuer_api::v1::AddAssetAuthorityResponse> * m,
                          int64_t key, ::issuer_api::v1::AddAssetAuthorityResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_issuer_api_v1_AddAssetAuthorityResponse_get(const google::protobuf::Map<int64_t, ::issuer_api::v1::AddAssetAuthorityResponse>* m,
                       int64_t key, const ::issuer_api::v1::AddAssetAuthorityResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::AddAssetAuthorityResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_issuer_api_v1_AddAssetAuthorityResponse_remove(google::protobuf::Map<int64_t, ::issuer_api::v1::AddAssetAuthorityResponse> * m,
                          int64_t key, ::issuer_api::v1::AddAssetAuthorityResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_issuer_api_v1_AddAssetAuthorityResponse_iter(
      const google::protobuf::Map<int64_t, ::issuer_api::v1::AddAssetAuthorityResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_issuer_api_v1_AddAssetAuthorityResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::issuer_api::v1::AddAssetAuthorityResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::issuer_api::v1::AddAssetAuthorityResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::issuer_api::v1::AddAssetAuthorityResponse>* __rust_proto_thunk__Map_u64_issuer_api_v1_AddAssetAuthorityResponse_new() {
    return new google::protobuf::Map<uint64_t, ::issuer_api::v1::AddAssetAuthorityResponse>();
  }
  void __rust_proto_thunk__Map_u64_issuer_api_v1_AddAssetAuthorityResponse_free(const google::protobuf::Map<uint64_t, ::issuer_api::v1::AddAssetAuthorityResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_issuer_api_v1_AddAssetAuthorityResponse_clear(google::protobuf::Map<uint64_t, ::issuer_api::v1::AddAssetAuthorityResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_issuer_api_v1_AddAssetAuthorityResponse_size(const google::protobuf::Map<uint64_t, ::issuer_api::v1::AddAssetAuthorityResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_issuer_api_v1_AddAssetAuthorityResponse_insert(google::protobuf::Map<uint64_t, ::issuer_api::v1::AddAssetAuthorityResponse> * m,
                          uint64_t key, ::issuer_api::v1::AddAssetAuthorityResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_issuer_api_v1_AddAssetAuthorityResponse_get(const google::protobuf::Map<uint64_t, ::issuer_api::v1::AddAssetAuthorityResponse>* m,
                       uint64_t key, const ::issuer_api::v1::AddAssetAuthorityResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::AddAssetAuthorityResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_issuer_api_v1_AddAssetAuthorityResponse_remove(google::protobuf::Map<uint64_t, ::issuer_api::v1::AddAssetAuthorityResponse> * m,
                          uint64_t key, ::issuer_api::v1::AddAssetAuthorityResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_issuer_api_v1_AddAssetAuthorityResponse_iter(
      const google::protobuf::Map<uint64_t, ::issuer_api::v1::AddAssetAuthorityResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_issuer_api_v1_AddAssetAuthorityResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::issuer_api::v1::AddAssetAuthorityResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::issuer_api::v1::AddAssetAuthorityResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::issuer_api::v1::AddAssetAuthorityResponse>* __rust_proto_thunk__Map_bool_issuer_api_v1_AddAssetAuthorityResponse_new() {
    return new google::protobuf::Map<bool, ::issuer_api::v1::AddAssetAuthorityResponse>();
  }
  void __rust_proto_thunk__Map_bool_issuer_api_v1_AddAssetAuthorityResponse_free(const google::protobuf::Map<bool, ::issuer_api::v1::AddAssetAuthorityResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_issuer_api_v1_AddAssetAuthorityResponse_clear(google::protobuf::Map<bool, ::issuer_api::v1::AddAssetAuthorityResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_issuer_api_v1_AddAssetAuthorityResponse_size(const google::protobuf::Map<bool, ::issuer_api::v1::AddAssetAuthorityResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_issuer_api_v1_AddAssetAuthorityResponse_insert(google::protobuf::Map<bool, ::issuer_api::v1::AddAssetAuthorityResponse> * m,
                          bool key, ::issuer_api::v1::AddAssetAuthorityResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_issuer_api_v1_AddAssetAuthorityResponse_get(const google::protobuf::Map<bool, ::issuer_api::v1::AddAssetAuthorityResponse>* m,
                       bool key, const ::issuer_api::v1::AddAssetAuthorityResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::AddAssetAuthorityResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_issuer_api_v1_AddAssetAuthorityResponse_remove(google::protobuf::Map<bool, ::issuer_api::v1::AddAssetAuthorityResponse> * m,
                          bool key, ::issuer_api::v1::AddAssetAuthorityResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_issuer_api_v1_AddAssetAuthorityResponse_iter(
      const google::protobuf::Map<bool, ::issuer_api::v1::AddAssetAuthorityResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_issuer_api_v1_AddAssetAuthorityResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::issuer_api::v1::AddAssetAuthorityResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::issuer_api::v1::AddAssetAuthorityResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::issuer_api::v1::AddAssetAuthorityResponse>* __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AddAssetAuthorityResponse_new() {
    return new google::protobuf::Map<std::string, ::issuer_api::v1::AddAssetAuthorityResponse>();
  }
  void __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AddAssetAuthorityResponse_free(const google::protobuf::Map<std::string, ::issuer_api::v1::AddAssetAuthorityResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AddAssetAuthorityResponse_clear(google::protobuf::Map<std::string, ::issuer_api::v1::AddAssetAuthorityResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AddAssetAuthorityResponse_size(const google::protobuf::Map<std::string, ::issuer_api::v1::AddAssetAuthorityResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AddAssetAuthorityResponse_insert(google::protobuf::Map<std::string, ::issuer_api::v1::AddAssetAuthorityResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::issuer_api::v1::AddAssetAuthorityResponse value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AddAssetAuthorityResponse_get(const google::protobuf::Map<std::string, ::issuer_api::v1::AddAssetAuthorityResponse>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::issuer_api::v1::AddAssetAuthorityResponse** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::AddAssetAuthorityResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AddAssetAuthorityResponse_remove(google::protobuf::Map<std::string, ::issuer_api::v1::AddAssetAuthorityResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::issuer_api::v1::AddAssetAuthorityResponse * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AddAssetAuthorityResponse_iter(
      const google::protobuf::Map<std::string, ::issuer_api::v1::AddAssetAuthorityResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AddAssetAuthorityResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::issuer_api::v1::AddAssetAuthorityResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::issuer_api::v1::AddAssetAuthorityResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// issuer_api.v1.GetAssetAuthoritiesRequest
        // clang-format off
extern "C" {
void* __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesRequest_new() { return new ::issuer_api::v1::GetAssetAuthoritiesRequest(); }
void __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesRequest_delete(void* ptr) { delete static_cast<::issuer_api::v1::GetAssetAuthoritiesRequest*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesRequest_serialize(::issuer_api::v1::GetAssetAuthoritiesRequest* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesRequest_parse(::issuer_api::v1::GetAssetAuthoritiesRequest* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesRequest_copy_from(::issuer_api::v1::GetAssetAuthoritiesRequest* dst, const ::issuer_api::v1::GetAssetAuthoritiesRequest* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesRequest_repeated_len(google::protobuf::RepeatedPtrField<::issuer_api::v1::GetAssetAuthoritiesRequest>* field) {
  return field->size();
}
const ::issuer_api::v1::GetAssetAuthoritiesRequest& __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesRequest_repeated_get(
  google::protobuf::RepeatedPtrField<::issuer_api::v1::GetAssetAuthoritiesRequest>* field,
  size_t index) {
  return field->Get(index);
}
::issuer_api::v1::GetAssetAuthoritiesRequest* __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesRequest_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::issuer_api::v1::GetAssetAuthoritiesRequest>* field,
  size_t index) {
  return field->Mutable(index);
}
::issuer_api::v1::GetAssetAuthoritiesRequest* __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesRequest_repeated_add(google::protobuf::RepeatedPtrField<::issuer_api::v1::GetAssetAuthoritiesRequest>* field) {
  return field->Add();
}
void __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesRequest_repeated_clear(google::protobuf::RepeatedPtrField<::issuer_api::v1::GetAssetAuthoritiesRequest>* field) {
  field->Clear();
}
void __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesRequest_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::issuer_api::v1::GetAssetAuthoritiesRequest>& dst,
  const google::protobuf::RepeatedPtrField<::issuer_api::v1::GetAssetAuthoritiesRequest>& src) {
  dst = src;
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::issuer_api::v1::GetAssetAuthoritiesRequest>* __rust_proto_thunk__Map_i32_issuer_api_v1_GetAssetAuthoritiesRequest_new() {
    return new google::protobuf::Map<int32_t, ::issuer_api::v1::GetAssetAuthoritiesRequest>();
  }
  void __rust_proto_thunk__Map_i32_issuer_api_v1_GetAssetAuthoritiesRequest_free(const google::protobuf::Map<int32_t, ::issuer_api::v1::GetAssetAuthoritiesRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_issuer_api_v1_GetAssetAuthoritiesRequest_clear(google::protobuf::Map<int32_t, ::issuer_api::v1::GetAssetAuthoritiesRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_issuer_api_v1_GetAssetAuthoritiesRequest_size(const google::protobuf::Map<int32_t, ::issuer_api::v1::GetAssetAuthoritiesRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_issuer_api_v1_GetAssetAuthoritiesRequest_insert(google::protobuf::Map<int32_t, ::issuer_api::v1::GetAssetAuthoritiesRequest> * m,
                          int32_t key, ::issuer_api::v1::GetAssetAuthoritiesRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_issuer_api_v1_GetAssetAuthoritiesRequest_get(const google::protobuf::Map<int32_t, ::issuer_api::v1::GetAssetAuthoritiesRequest>* m,
                       int32_t key, const ::issuer_api::v1::GetAssetAuthoritiesRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::GetAssetAuthoritiesRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_issuer_api_v1_GetAssetAuthoritiesRequest_remove(google::protobuf::Map<int32_t, ::issuer_api::v1::GetAssetAuthoritiesRequest> * m,
                          int32_t key, ::issuer_api::v1::GetAssetAuthoritiesRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_issuer_api_v1_GetAssetAuthoritiesRequest_iter(
      const google::protobuf::Map<int32_t, ::issuer_api::v1::GetAssetAuthoritiesRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_issuer_api_v1_GetAssetAuthoritiesRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::issuer_api::v1::GetAssetAuthoritiesRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::issuer_api::v1::GetAssetAuthoritiesRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::issuer_api::v1::GetAssetAuthoritiesRequest>* __rust_proto_thunk__Map_u32_issuer_api_v1_GetAssetAuthoritiesRequest_new() {
    return new google::protobuf::Map<uint32_t, ::issuer_api::v1::GetAssetAuthoritiesRequest>();
  }
  void __rust_proto_thunk__Map_u32_issuer_api_v1_GetAssetAuthoritiesRequest_free(const google::protobuf::Map<uint32_t, ::issuer_api::v1::GetAssetAuthoritiesRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_issuer_api_v1_GetAssetAuthoritiesRequest_clear(google::protobuf::Map<uint32_t, ::issuer_api::v1::GetAssetAuthoritiesRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_issuer_api_v1_GetAssetAuthoritiesRequest_size(const google::protobuf::Map<uint32_t, ::issuer_api::v1::GetAssetAuthoritiesRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_issuer_api_v1_GetAssetAuthoritiesRequest_insert(google::protobuf::Map<uint32_t, ::issuer_api::v1::GetAssetAuthoritiesRequest> * m,
                          uint32_t key, ::issuer_api::v1::GetAssetAuthoritiesRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_issuer_api_v1_GetAssetAuthoritiesRequest_get(const google::protobuf::Map<uint32_t, ::issuer_api::v1::GetAssetAuthoritiesRequest>* m,
                       uint32_t key, const ::issuer_api::v1::GetAssetAuthoritiesRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::GetAssetAuthoritiesRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_issuer_api_v1_GetAssetAuthoritiesRequest_remove(google::protobuf::Map<uint32_t, ::issuer_api::v1::GetAssetAuthoritiesRequest> * m,
                          uint32_t key, ::issuer_api::v1::GetAssetAuthoritiesRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_issuer_api_v1_GetAssetAuthoritiesRequest_iter(
      const google::protobuf::Map<uint32_t, ::issuer_api::v1::GetAssetAuthoritiesRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_issuer_api_v1_GetAssetAuthoritiesRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::issuer_api::v1::GetAssetAuthoritiesRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::issuer_api::v1::GetAssetAuthoritiesRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::issuer_api::v1::GetAssetAuthoritiesRequest>* __rust_proto_thunk__Map_i64_issuer_api_v1_GetAssetAuthoritiesRequest_new() {
    return new google::protobuf::Map<int64_t, ::issuer_api::v1::GetAssetAuthoritiesRequest>();
  }
  void __rust_proto_thunk__Map_i64_issuer_api_v1_GetAssetAuthoritiesRequest_free(const google::protobuf::Map<int64_t, ::issuer_api::v1::GetAssetAuthoritiesRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_issuer_api_v1_GetAssetAuthoritiesRequest_clear(google::protobuf::Map<int64_t, ::issuer_api::v1::GetAssetAuthoritiesRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_issuer_api_v1_GetAssetAuthoritiesRequest_size(const google::protobuf::Map<int64_t, ::issuer_api::v1::GetAssetAuthoritiesRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_issuer_api_v1_GetAssetAuthoritiesRequest_insert(google::protobuf::Map<int64_t, ::issuer_api::v1::GetAssetAuthoritiesRequest> * m,
                          int64_t key, ::issuer_api::v1::GetAssetAuthoritiesRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_issuer_api_v1_GetAssetAuthoritiesRequest_get(const google::protobuf::Map<int64_t, ::issuer_api::v1::GetAssetAuthoritiesRequest>* m,
                       int64_t key, const ::issuer_api::v1::GetAssetAuthoritiesRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::GetAssetAuthoritiesRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_issuer_api_v1_GetAssetAuthoritiesRequest_remove(google::protobuf::Map<int64_t, ::issuer_api::v1::GetAssetAuthoritiesRequest> * m,
                          int64_t key, ::issuer_api::v1::GetAssetAuthoritiesRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_issuer_api_v1_GetAssetAuthoritiesRequest_iter(
      const google::protobuf::Map<int64_t, ::issuer_api::v1::GetAssetAuthoritiesRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_issuer_api_v1_GetAssetAuthoritiesRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::issuer_api::v1::GetAssetAuthoritiesRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::issuer_api::v1::GetAssetAuthoritiesRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::issuer_api::v1::GetAssetAuthoritiesRequest>* __rust_proto_thunk__Map_u64_issuer_api_v1_GetAssetAuthoritiesRequest_new() {
    return new google::protobuf::Map<uint64_t, ::issuer_api::v1::GetAssetAuthoritiesRequest>();
  }
  void __rust_proto_thunk__Map_u64_issuer_api_v1_GetAssetAuthoritiesRequest_free(const google::protobuf::Map<uint64_t, ::issuer_api::v1::GetAssetAuthoritiesRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_issuer_api_v1_GetAssetAuthoritiesRequest_clear(google::protobuf::Map<uint64_t, ::issuer_api::v1::GetAssetAuthoritiesRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_issuer_api_v1_GetAssetAuthoritiesRequest_size(const google::protobuf::Map<uint64_t, ::issuer_api::v1::GetAssetAuthoritiesRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_issuer_api_v1_GetAssetAuthoritiesRequest_insert(google::protobuf::Map<uint64_t, ::issuer_api::v1::GetAssetAuthoritiesRequest> * m,
                          uint64_t key, ::issuer_api::v1::GetAssetAuthoritiesRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_issuer_api_v1_GetAssetAuthoritiesRequest_get(const google::protobuf::Map<uint64_t, ::issuer_api::v1::GetAssetAuthoritiesRequest>* m,
                       uint64_t key, const ::issuer_api::v1::GetAssetAuthoritiesRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::GetAssetAuthoritiesRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_issuer_api_v1_GetAssetAuthoritiesRequest_remove(google::protobuf::Map<uint64_t, ::issuer_api::v1::GetAssetAuthoritiesRequest> * m,
                          uint64_t key, ::issuer_api::v1::GetAssetAuthoritiesRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_issuer_api_v1_GetAssetAuthoritiesRequest_iter(
      const google::protobuf::Map<uint64_t, ::issuer_api::v1::GetAssetAuthoritiesRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_issuer_api_v1_GetAssetAuthoritiesRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::issuer_api::v1::GetAssetAuthoritiesRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::issuer_api::v1::GetAssetAuthoritiesRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::issuer_api::v1::GetAssetAuthoritiesRequest>* __rust_proto_thunk__Map_bool_issuer_api_v1_GetAssetAuthoritiesRequest_new() {
    return new google::protobuf::Map<bool, ::issuer_api::v1::GetAssetAuthoritiesRequest>();
  }
  void __rust_proto_thunk__Map_bool_issuer_api_v1_GetAssetAuthoritiesRequest_free(const google::protobuf::Map<bool, ::issuer_api::v1::GetAssetAuthoritiesRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_issuer_api_v1_GetAssetAuthoritiesRequest_clear(google::protobuf::Map<bool, ::issuer_api::v1::GetAssetAuthoritiesRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_issuer_api_v1_GetAssetAuthoritiesRequest_size(const google::protobuf::Map<bool, ::issuer_api::v1::GetAssetAuthoritiesRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_issuer_api_v1_GetAssetAuthoritiesRequest_insert(google::protobuf::Map<bool, ::issuer_api::v1::GetAssetAuthoritiesRequest> * m,
                          bool key, ::issuer_api::v1::GetAssetAuthoritiesRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_issuer_api_v1_GetAssetAuthoritiesRequest_get(const google::protobuf::Map<bool, ::issuer_api::v1::GetAssetAuthoritiesRequest>* m,
                       bool key, const ::issuer_api::v1::GetAssetAuthoritiesRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::GetAssetAuthoritiesRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_issuer_api_v1_GetAssetAuthoritiesRequest_remove(google::protobuf::Map<bool, ::issuer_api::v1::GetAssetAuthoritiesRequest> * m,
                          bool key, ::issuer_api::v1::GetAssetAuthoritiesRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_issuer_api_v1_GetAssetAuthoritiesRequest_iter(
      const google::protobuf::Map<bool, ::issuer_api::v1::GetAssetAuthoritiesRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_issuer_api_v1_GetAssetAuthoritiesRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::issuer_api::v1::GetAssetAuthoritiesRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::issuer_api::v1::GetAssetAuthoritiesRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::issuer_api::v1::GetAssetAuthoritiesRequest>* __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetAssetAuthoritiesRequest_new() {
    return new google::protobuf::Map<std::string, ::issuer_api::v1::GetAssetAuthoritiesRequest>();
  }
  void __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetAssetAuthoritiesRequest_free(const google::protobuf::Map<std::string, ::issuer_api::v1::GetAssetAuthoritiesRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetAssetAuthoritiesRequest_clear(google::protobuf::Map<std::string, ::issuer_api::v1::GetAssetAuthoritiesRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetAssetAuthoritiesRequest_size(const google::protobuf::Map<std::string, ::issuer_api::v1::GetAssetAuthoritiesRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetAssetAuthoritiesRequest_insert(google::protobuf::Map<std::string, ::issuer_api::v1::GetAssetAuthoritiesRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::issuer_api::v1::GetAssetAuthoritiesRequest value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetAssetAuthoritiesRequest_get(const google::protobuf::Map<std::string, ::issuer_api::v1::GetAssetAuthoritiesRequest>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::issuer_api::v1::GetAssetAuthoritiesRequest** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::GetAssetAuthoritiesRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetAssetAuthoritiesRequest_remove(google::protobuf::Map<std::string, ::issuer_api::v1::GetAssetAuthoritiesRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::issuer_api::v1::GetAssetAuthoritiesRequest * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetAssetAuthoritiesRequest_iter(
      const google::protobuf::Map<std::string, ::issuer_api::v1::GetAssetAuthoritiesRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetAssetAuthoritiesRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::issuer_api::v1::GetAssetAuthoritiesRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::issuer_api::v1::GetAssetAuthoritiesRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// issuer_api.v1.GetAssetAuthoritiesResponse
        // clang-format off
extern "C" {
void* __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesResponse_new() { return new ::issuer_api::v1::GetAssetAuthoritiesResponse(); }
void __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesResponse_delete(void* ptr) { delete static_cast<::issuer_api::v1::GetAssetAuthoritiesResponse*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesResponse_serialize(::issuer_api::v1::GetAssetAuthoritiesResponse* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesResponse_parse(::issuer_api::v1::GetAssetAuthoritiesResponse* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesResponse_copy_from(::issuer_api::v1::GetAssetAuthoritiesResponse* dst, const ::issuer_api::v1::GetAssetAuthoritiesResponse* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesResponse_repeated_len(google::protobuf::RepeatedPtrField<::issuer_api::v1::GetAssetAuthoritiesResponse>* field) {
  return field->size();
}
const ::issuer_api::v1::GetAssetAuthoritiesResponse& __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesResponse_repeated_get(
  google::protobuf::RepeatedPtrField<::issuer_api::v1::GetAssetAuthoritiesResponse>* field,
  size_t index) {
  return field->Get(index);
}
::issuer_api::v1::GetAssetAuthoritiesResponse* __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesResponse_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::issuer_api::v1::GetAssetAuthoritiesResponse>* field,
  size_t index) {
  return field->Mutable(index);
}
::issuer_api::v1::GetAssetAuthoritiesResponse* __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesResponse_repeated_add(google::protobuf::RepeatedPtrField<::issuer_api::v1::GetAssetAuthoritiesResponse>* field) {
  return field->Add();
}
void __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesResponse_repeated_clear(google::protobuf::RepeatedPtrField<::issuer_api::v1::GetAssetAuthoritiesResponse>* field) {
  field->Clear();
}
void __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesResponse_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::issuer_api::v1::GetAssetAuthoritiesResponse>& dst,
  const google::protobuf::RepeatedPtrField<::issuer_api::v1::GetAssetAuthoritiesResponse>& src) {
  dst = src;
}

const void* __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesResponse_get_authorities(const ::issuer_api::v1::GetAssetAuthoritiesResponse* msg) {
  return &msg->authorities();
}
void* __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesResponse_get_mut_authorities(::issuer_api::v1::GetAssetAuthoritiesResponse* msg) { return msg->mutable_authorities(); }


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::issuer_api::v1::GetAssetAuthoritiesResponse>* __rust_proto_thunk__Map_i32_issuer_api_v1_GetAssetAuthoritiesResponse_new() {
    return new google::protobuf::Map<int32_t, ::issuer_api::v1::GetAssetAuthoritiesResponse>();
  }
  void __rust_proto_thunk__Map_i32_issuer_api_v1_GetAssetAuthoritiesResponse_free(const google::protobuf::Map<int32_t, ::issuer_api::v1::GetAssetAuthoritiesResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_issuer_api_v1_GetAssetAuthoritiesResponse_clear(google::protobuf::Map<int32_t, ::issuer_api::v1::GetAssetAuthoritiesResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_issuer_api_v1_GetAssetAuthoritiesResponse_size(const google::protobuf::Map<int32_t, ::issuer_api::v1::GetAssetAuthoritiesResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_issuer_api_v1_GetAssetAuthoritiesResponse_insert(google::protobuf::Map<int32_t, ::issuer_api::v1::GetAssetAuthoritiesResponse> * m,
                          int32_t key, ::issuer_api::v1::GetAssetAuthoritiesResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_issuer_api_v1_GetAssetAuthoritiesResponse_get(const google::protobuf::Map<int32_t, ::issuer_api::v1::GetAssetAuthoritiesResponse>* m,
                       int32_t key, const ::issuer_api::v1::GetAssetAuthoritiesResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::GetAssetAuthoritiesResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_issuer_api_v1_GetAssetAuthoritiesResponse_remove(google::protobuf::Map<int32_t, ::issuer_api::v1::GetAssetAuthoritiesResponse> * m,
                          int32_t key, ::issuer_api::v1::GetAssetAuthoritiesResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_issuer_api_v1_GetAssetAuthoritiesResponse_iter(
      const google::protobuf::Map<int32_t, ::issuer_api::v1::GetAssetAuthoritiesResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_issuer_api_v1_GetAssetAuthoritiesResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::issuer_api::v1::GetAssetAuthoritiesResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::issuer_api::v1::GetAssetAuthoritiesResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::issuer_api::v1::GetAssetAuthoritiesResponse>* __rust_proto_thunk__Map_u32_issuer_api_v1_GetAssetAuthoritiesResponse_new() {
    return new google::protobuf::Map<uint32_t, ::issuer_api::v1::GetAssetAuthoritiesResponse>();
  }
  void __rust_proto_thunk__Map_u32_issuer_api_v1_GetAssetAuthoritiesResponse_free(const google::protobuf::Map<uint32_t, ::issuer_api::v1::GetAssetAuthoritiesResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_issuer_api_v1_GetAssetAuthoritiesResponse_clear(google::protobuf::Map<uint32_t, ::issuer_api::v1::GetAssetAuthoritiesResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_issuer_api_v1_GetAssetAuthoritiesResponse_size(const google::protobuf::Map<uint32_t, ::issuer_api::v1::GetAssetAuthoritiesResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_issuer_api_v1_GetAssetAuthoritiesResponse_insert(google::protobuf::Map<uint32_t, ::issuer_api::v1::GetAssetAuthoritiesResponse> * m,
                          uint32_t key, ::issuer_api::v1::GetAssetAuthoritiesResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_issuer_api_v1_GetAssetAuthoritiesResponse_get(const google::protobuf::Map<uint32_t, ::issuer_api::v1::GetAssetAuthoritiesResponse>* m,
                       uint32_t key, const ::issuer_api::v1::GetAssetAuthoritiesResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::GetAssetAuthoritiesResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_issuer_api_v1_GetAssetAuthoritiesResponse_remove(google::protobuf::Map<uint32_t, ::issuer_api::v1::GetAssetAuthoritiesResponse> * m,
                          uint32_t key, ::issuer_api::v1::GetAssetAuthoritiesResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_issuer_api_v1_GetAssetAuthoritiesResponse_iter(
      const google::protobuf::Map<uint32_t, ::issuer_api::v1::GetAssetAuthoritiesResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_issuer_api_v1_GetAssetAuthoritiesResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::issuer_api::v1::GetAssetAuthoritiesResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::issuer_api::v1::GetAssetAuthoritiesResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::issuer_api::v1::GetAssetAuthoritiesResponse>* __rust_proto_thunk__Map_i64_issuer_api_v1_GetAssetAuthoritiesResponse_new() {
    return new google::protobuf::Map<int64_t, ::issuer_api::v1::GetAssetAuthoritiesResponse>();
  }
  void __rust_proto_thunk__Map_i64_issuer_api_v1_GetAssetAuthoritiesResponse_free(const google::protobuf::Map<int64_t, ::issuer_api::v1::GetAssetAuthoritiesResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_issuer_api_v1_GetAssetAuthoritiesResponse_clear(google::protobuf::Map<int64_t, ::issuer_api::v1::GetAssetAuthoritiesResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_issuer_api_v1_GetAssetAuthoritiesResponse_size(const google::protobuf::Map<int64_t, ::issuer_api::v1::GetAssetAuthoritiesResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_issuer_api_v1_GetAssetAuthoritiesResponse_insert(google::protobuf::Map<int64_t, ::issuer_api::v1::GetAssetAuthoritiesResponse> * m,
                          int64_t key, ::issuer_api::v1::GetAssetAuthoritiesResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_issuer_api_v1_GetAssetAuthoritiesResponse_get(const google::protobuf::Map<int64_t, ::issuer_api::v1::GetAssetAuthoritiesResponse>* m,
                       int64_t key, const ::issuer_api::v1::GetAssetAuthoritiesResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::GetAssetAuthoritiesResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_issuer_api_v1_GetAssetAuthoritiesResponse_remove(google::protobuf::Map<int64_t, ::issuer_api::v1::GetAssetAuthoritiesResponse> * m,
                          int64_t key, ::issuer_api::v1::GetAssetAuthoritiesResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_issuer_api_v1_GetAssetAuthoritiesResponse_iter(
      const google::protobuf::Map<int64_t, ::issuer_api::v1::GetAssetAuthoritiesResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_issuer_api_v1_GetAssetAuthoritiesResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::issuer_api::v1::GetAssetAuthoritiesResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::issuer_api::v1::GetAssetAuthoritiesResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::issuer_api::v1::GetAssetAuthoritiesResponse>* __rust_proto_thunk__Map_u64_issuer_api_v1_GetAssetAuthoritiesResponse_new() {
    return new google::protobuf::Map<uint64_t, ::issuer_api::v1::GetAssetAuthoritiesResponse>();
  }
  void __rust_proto_thunk__Map_u64_issuer_api_v1_GetAssetAuthoritiesResponse_free(const google::protobuf::Map<uint64_t, ::issuer_api::v1::GetAssetAuthoritiesResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_issuer_api_v1_GetAssetAuthoritiesResponse_clear(google::protobuf::Map<uint64_t, ::issuer_api::v1::GetAssetAuthoritiesResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_issuer_api_v1_GetAssetAuthoritiesResponse_size(const google::protobuf::Map<uint64_t, ::issuer_api::v1::GetAssetAuthoritiesResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_issuer_api_v1_GetAssetAuthoritiesResponse_insert(google::protobuf::Map<uint64_t, ::issuer_api::v1::GetAssetAuthoritiesResponse> * m,
                          uint64_t key, ::issuer_api::v1::GetAssetAuthoritiesResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_issuer_api_v1_GetAssetAuthoritiesResponse_get(const google::protobuf::Map<uint64_t, ::issuer_api::v1::GetAssetAuthoritiesResponse>* m,
                       uint64_t key, const ::issuer_api::v1::GetAssetAuthoritiesResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::GetAssetAuthoritiesResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_issuer_api_v1_GetAssetAuthoritiesResponse_remove(google::protobuf::Map<uint64_t, ::issuer_api::v1::GetAssetAuthoritiesResponse> * m,
                          uint64_t key, ::issuer_api::v1::GetAssetAuthoritiesResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_issuer_api_v1_GetAssetAuthoritiesResponse_iter(
      const google::protobuf::Map<uint64_t, ::issuer_api::v1::GetAssetAuthoritiesResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_issuer_api_v1_GetAssetAuthoritiesResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::issuer_api::v1::GetAssetAuthoritiesResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::issuer_api::v1::GetAssetAuthoritiesResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::issuer_api::v1::GetAssetAuthoritiesResponse>* __rust_proto_thunk__Map_bool_issuer_api_v1_GetAssetAuthoritiesResponse_new() {
    return new google::protobuf::Map<bool, ::issuer_api::v1::GetAssetAuthoritiesResponse>();
  }
  void __rust_proto_thunk__Map_bool_issuer_api_v1_GetAssetAuthoritiesResponse_free(const google::protobuf::Map<bool, ::issuer_api::v1::GetAssetAuthoritiesResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_issuer_api_v1_GetAssetAuthoritiesResponse_clear(google::protobuf::Map<bool, ::issuer_api::v1::GetAssetAuthoritiesResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_issuer_api_v1_GetAssetAuthoritiesResponse_size(const google::protobuf::Map<bool, ::issuer_api::v1::GetAssetAuthoritiesResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_issuer_api_v1_GetAssetAuthoritiesResponse_insert(google::protobuf::Map<bool, ::issuer_api::v1::GetAssetAuthoritiesResponse> * m,
                          bool key, ::issuer_api::v1::GetAssetAuthoritiesResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_issuer_api_v1_GetAssetAuthoritiesResponse_get(const google::protobuf::Map<bool, ::issuer_api::v1::GetAssetAuthoritiesResponse>* m,
                       bool key, const ::issuer_api::v1::GetAssetAuthoritiesResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::GetAssetAuthoritiesResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_issuer_api_v1_GetAssetAuthoritiesResponse_remove(google::protobuf::Map<bool, ::issuer_api::v1::GetAssetAuthoritiesResponse> * m,
                          bool key, ::issuer_api::v1::GetAssetAuthoritiesResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_issuer_api_v1_GetAssetAuthoritiesResponse_iter(
      const google::protobuf::Map<bool, ::issuer_api::v1::GetAssetAuthoritiesResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_issuer_api_v1_GetAssetAuthoritiesResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::issuer_api::v1::GetAssetAuthoritiesResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::issuer_api::v1::GetAssetAuthoritiesResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::issuer_api::v1::GetAssetAuthoritiesResponse>* __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetAssetAuthoritiesResponse_new() {
    return new google::protobuf::Map<std::string, ::issuer_api::v1::GetAssetAuthoritiesResponse>();
  }
  void __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetAssetAuthoritiesResponse_free(const google::protobuf::Map<std::string, ::issuer_api::v1::GetAssetAuthoritiesResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetAssetAuthoritiesResponse_clear(google::protobuf::Map<std::string, ::issuer_api::v1::GetAssetAuthoritiesResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetAssetAuthoritiesResponse_size(const google::protobuf::Map<std::string, ::issuer_api::v1::GetAssetAuthoritiesResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetAssetAuthoritiesResponse_insert(google::protobuf::Map<std::string, ::issuer_api::v1::GetAssetAuthoritiesResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::issuer_api::v1::GetAssetAuthoritiesResponse value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetAssetAuthoritiesResponse_get(const google::protobuf::Map<std::string, ::issuer_api::v1::GetAssetAuthoritiesResponse>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::issuer_api::v1::GetAssetAuthoritiesResponse** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::issuer_api::v1::GetAssetAuthoritiesResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetAssetAuthoritiesResponse_remove(google::protobuf::Map<std::string, ::issuer_api::v1::GetAssetAuthoritiesResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::issuer_api::v1::GetAssetAuthoritiesResponse * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetAssetAuthoritiesResponse_iter(
      const google::protobuf::Map<std::string, ::issuer_api::v1::GetAssetAuthoritiesResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetAssetAuthoritiesResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::issuer_api::v1::GetAssetAuthoritiesResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::issuer_api::v1::GetAssetAuthoritiesResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

