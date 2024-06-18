#include "vc_api/v1/vc.proto.h"
#include "google/protobuf/rust/cpp_kernel/cpp_api.h"
// vc_api.v1.CredentialProof
        // clang-format off
extern "C" {
void* __rust_proto_thunk__vc_api_v1_CredentialProof_new() { return new ::vc_api::v1::CredentialProof(); }
void __rust_proto_thunk__vc_api_v1_CredentialProof_delete(void* ptr) { delete static_cast<::vc_api::v1::CredentialProof*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__vc_api_v1_CredentialProof_serialize(::vc_api::v1::CredentialProof* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__vc_api_v1_CredentialProof_parse(::vc_api::v1::CredentialProof* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__vc_api_v1_CredentialProof_copy_from(::vc_api::v1::CredentialProof* dst, const ::vc_api::v1::CredentialProof* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__vc_api_v1_CredentialProof_repeated_len(google::protobuf::RepeatedPtrField<::vc_api::v1::CredentialProof>* field) {
  return field->size();
}
const ::vc_api::v1::CredentialProof& __rust_proto_thunk__vc_api_v1_CredentialProof_repeated_get(
  google::protobuf::RepeatedPtrField<::vc_api::v1::CredentialProof>* field,
  size_t index) {
  return field->Get(index);
}
::vc_api::v1::CredentialProof* __rust_proto_thunk__vc_api_v1_CredentialProof_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::vc_api::v1::CredentialProof>* field,
  size_t index) {
  return field->Mutable(index);
}
::vc_api::v1::CredentialProof* __rust_proto_thunk__vc_api_v1_CredentialProof_repeated_add(google::protobuf::RepeatedPtrField<::vc_api::v1::CredentialProof>* field) {
  return field->Add();
}
void __rust_proto_thunk__vc_api_v1_CredentialProof_repeated_clear(google::protobuf::RepeatedPtrField<::vc_api::v1::CredentialProof>* field) {
  field->Clear();
}
void __rust_proto_thunk__vc_api_v1_CredentialProof_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::vc_api::v1::CredentialProof>& dst,
  const google::protobuf::RepeatedPtrField<::vc_api::v1::CredentialProof>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__vc_api_v1_CredentialProof_get_type(::vc_api::v1::CredentialProof* msg) {
  absl::string_view val = msg->type();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__vc_api_v1_CredentialProof_set_type(::vc_api::v1::CredentialProof* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_type(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__vc_api_v1_CredentialProof_get_created(::vc_api::v1::CredentialProof* msg) {
  absl::string_view val = msg->created();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__vc_api_v1_CredentialProof_set_created(::vc_api::v1::CredentialProof* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_created(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__vc_api_v1_CredentialProof_get_verification_method(::vc_api::v1::CredentialProof* msg) {
  absl::string_view val = msg->verification_method();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__vc_api_v1_CredentialProof_set_verification_method(::vc_api::v1::CredentialProof* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_verification_method(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__vc_api_v1_CredentialProof_get_proof_purpose(::vc_api::v1::CredentialProof* msg) {
  absl::string_view val = msg->proof_purpose();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__vc_api_v1_CredentialProof_set_proof_purpose(::vc_api::v1::CredentialProof* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_proof_purpose(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__vc_api_v1_CredentialProof_get_proof_value(::vc_api::v1::CredentialProof* msg) {
  absl::string_view val = msg->proof_value();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__vc_api_v1_CredentialProof_set_proof_value(::vc_api::v1::CredentialProof* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_proof_value(absl::string_view(s.ptr, s.len));
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::vc_api::v1::CredentialProof>* __rust_proto_thunk__Map_i32_vc_api_v1_CredentialProof_new() {
    return new google::protobuf::Map<int32_t, ::vc_api::v1::CredentialProof>();
  }
  void __rust_proto_thunk__Map_i32_vc_api_v1_CredentialProof_free(const google::protobuf::Map<int32_t, ::vc_api::v1::CredentialProof>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_vc_api_v1_CredentialProof_clear(google::protobuf::Map<int32_t, ::vc_api::v1::CredentialProof> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_vc_api_v1_CredentialProof_size(const google::protobuf::Map<int32_t, ::vc_api::v1::CredentialProof>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_vc_api_v1_CredentialProof_insert(google::protobuf::Map<int32_t, ::vc_api::v1::CredentialProof> * m,
                          int32_t key, ::vc_api::v1::CredentialProof value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_vc_api_v1_CredentialProof_get(const google::protobuf::Map<int32_t, ::vc_api::v1::CredentialProof>* m,
                       int32_t key, const ::vc_api::v1::CredentialProof** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::CredentialProof* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_vc_api_v1_CredentialProof_remove(google::protobuf::Map<int32_t, ::vc_api::v1::CredentialProof> * m,
                          int32_t key, ::vc_api::v1::CredentialProof * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_vc_api_v1_CredentialProof_iter(
      const google::protobuf::Map<int32_t, ::vc_api::v1::CredentialProof>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_vc_api_v1_CredentialProof_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::vc_api::v1::CredentialProof** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::vc_api::v1::CredentialProof>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::vc_api::v1::CredentialProof>* __rust_proto_thunk__Map_u32_vc_api_v1_CredentialProof_new() {
    return new google::protobuf::Map<uint32_t, ::vc_api::v1::CredentialProof>();
  }
  void __rust_proto_thunk__Map_u32_vc_api_v1_CredentialProof_free(const google::protobuf::Map<uint32_t, ::vc_api::v1::CredentialProof>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_vc_api_v1_CredentialProof_clear(google::protobuf::Map<uint32_t, ::vc_api::v1::CredentialProof> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_vc_api_v1_CredentialProof_size(const google::protobuf::Map<uint32_t, ::vc_api::v1::CredentialProof>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_vc_api_v1_CredentialProof_insert(google::protobuf::Map<uint32_t, ::vc_api::v1::CredentialProof> * m,
                          uint32_t key, ::vc_api::v1::CredentialProof value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_vc_api_v1_CredentialProof_get(const google::protobuf::Map<uint32_t, ::vc_api::v1::CredentialProof>* m,
                       uint32_t key, const ::vc_api::v1::CredentialProof** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::CredentialProof* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_vc_api_v1_CredentialProof_remove(google::protobuf::Map<uint32_t, ::vc_api::v1::CredentialProof> * m,
                          uint32_t key, ::vc_api::v1::CredentialProof * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_vc_api_v1_CredentialProof_iter(
      const google::protobuf::Map<uint32_t, ::vc_api::v1::CredentialProof>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_vc_api_v1_CredentialProof_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::vc_api::v1::CredentialProof** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::vc_api::v1::CredentialProof>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::vc_api::v1::CredentialProof>* __rust_proto_thunk__Map_i64_vc_api_v1_CredentialProof_new() {
    return new google::protobuf::Map<int64_t, ::vc_api::v1::CredentialProof>();
  }
  void __rust_proto_thunk__Map_i64_vc_api_v1_CredentialProof_free(const google::protobuf::Map<int64_t, ::vc_api::v1::CredentialProof>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_vc_api_v1_CredentialProof_clear(google::protobuf::Map<int64_t, ::vc_api::v1::CredentialProof> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_vc_api_v1_CredentialProof_size(const google::protobuf::Map<int64_t, ::vc_api::v1::CredentialProof>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_vc_api_v1_CredentialProof_insert(google::protobuf::Map<int64_t, ::vc_api::v1::CredentialProof> * m,
                          int64_t key, ::vc_api::v1::CredentialProof value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_vc_api_v1_CredentialProof_get(const google::protobuf::Map<int64_t, ::vc_api::v1::CredentialProof>* m,
                       int64_t key, const ::vc_api::v1::CredentialProof** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::CredentialProof* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_vc_api_v1_CredentialProof_remove(google::protobuf::Map<int64_t, ::vc_api::v1::CredentialProof> * m,
                          int64_t key, ::vc_api::v1::CredentialProof * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_vc_api_v1_CredentialProof_iter(
      const google::protobuf::Map<int64_t, ::vc_api::v1::CredentialProof>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_vc_api_v1_CredentialProof_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::vc_api::v1::CredentialProof** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::vc_api::v1::CredentialProof>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::vc_api::v1::CredentialProof>* __rust_proto_thunk__Map_u64_vc_api_v1_CredentialProof_new() {
    return new google::protobuf::Map<uint64_t, ::vc_api::v1::CredentialProof>();
  }
  void __rust_proto_thunk__Map_u64_vc_api_v1_CredentialProof_free(const google::protobuf::Map<uint64_t, ::vc_api::v1::CredentialProof>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_vc_api_v1_CredentialProof_clear(google::protobuf::Map<uint64_t, ::vc_api::v1::CredentialProof> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_vc_api_v1_CredentialProof_size(const google::protobuf::Map<uint64_t, ::vc_api::v1::CredentialProof>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_vc_api_v1_CredentialProof_insert(google::protobuf::Map<uint64_t, ::vc_api::v1::CredentialProof> * m,
                          uint64_t key, ::vc_api::v1::CredentialProof value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_vc_api_v1_CredentialProof_get(const google::protobuf::Map<uint64_t, ::vc_api::v1::CredentialProof>* m,
                       uint64_t key, const ::vc_api::v1::CredentialProof** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::CredentialProof* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_vc_api_v1_CredentialProof_remove(google::protobuf::Map<uint64_t, ::vc_api::v1::CredentialProof> * m,
                          uint64_t key, ::vc_api::v1::CredentialProof * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_vc_api_v1_CredentialProof_iter(
      const google::protobuf::Map<uint64_t, ::vc_api::v1::CredentialProof>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_vc_api_v1_CredentialProof_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::vc_api::v1::CredentialProof** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::vc_api::v1::CredentialProof>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::vc_api::v1::CredentialProof>* __rust_proto_thunk__Map_bool_vc_api_v1_CredentialProof_new() {
    return new google::protobuf::Map<bool, ::vc_api::v1::CredentialProof>();
  }
  void __rust_proto_thunk__Map_bool_vc_api_v1_CredentialProof_free(const google::protobuf::Map<bool, ::vc_api::v1::CredentialProof>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_vc_api_v1_CredentialProof_clear(google::protobuf::Map<bool, ::vc_api::v1::CredentialProof> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_vc_api_v1_CredentialProof_size(const google::protobuf::Map<bool, ::vc_api::v1::CredentialProof>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_vc_api_v1_CredentialProof_insert(google::protobuf::Map<bool, ::vc_api::v1::CredentialProof> * m,
                          bool key, ::vc_api::v1::CredentialProof value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_vc_api_v1_CredentialProof_get(const google::protobuf::Map<bool, ::vc_api::v1::CredentialProof>* m,
                       bool key, const ::vc_api::v1::CredentialProof** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::CredentialProof* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_vc_api_v1_CredentialProof_remove(google::protobuf::Map<bool, ::vc_api::v1::CredentialProof> * m,
                          bool key, ::vc_api::v1::CredentialProof * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_vc_api_v1_CredentialProof_iter(
      const google::protobuf::Map<bool, ::vc_api::v1::CredentialProof>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_vc_api_v1_CredentialProof_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::vc_api::v1::CredentialProof** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::vc_api::v1::CredentialProof>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::vc_api::v1::CredentialProof>* __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CredentialProof_new() {
    return new google::protobuf::Map<std::string, ::vc_api::v1::CredentialProof>();
  }
  void __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CredentialProof_free(const google::protobuf::Map<std::string, ::vc_api::v1::CredentialProof>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CredentialProof_clear(google::protobuf::Map<std::string, ::vc_api::v1::CredentialProof> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CredentialProof_size(const google::protobuf::Map<std::string, ::vc_api::v1::CredentialProof>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CredentialProof_insert(google::protobuf::Map<std::string, ::vc_api::v1::CredentialProof> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::vc_api::v1::CredentialProof value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CredentialProof_get(const google::protobuf::Map<std::string, ::vc_api::v1::CredentialProof>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::vc_api::v1::CredentialProof** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::CredentialProof* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CredentialProof_remove(google::protobuf::Map<std::string, ::vc_api::v1::CredentialProof> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::vc_api::v1::CredentialProof * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CredentialProof_iter(
      const google::protobuf::Map<std::string, ::vc_api::v1::CredentialProof>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CredentialProof_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::vc_api::v1::CredentialProof** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::vc_api::v1::CredentialProof>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// vc_api.v1.CreateIssuanceChallengeRequest
        // clang-format off
extern "C" {
void* __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeRequest_new() { return new ::vc_api::v1::CreateIssuanceChallengeRequest(); }
void __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeRequest_delete(void* ptr) { delete static_cast<::vc_api::v1::CreateIssuanceChallengeRequest*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeRequest_serialize(::vc_api::v1::CreateIssuanceChallengeRequest* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeRequest_parse(::vc_api::v1::CreateIssuanceChallengeRequest* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeRequest_copy_from(::vc_api::v1::CreateIssuanceChallengeRequest* dst, const ::vc_api::v1::CreateIssuanceChallengeRequest* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeRequest_repeated_len(google::protobuf::RepeatedPtrField<::vc_api::v1::CreateIssuanceChallengeRequest>* field) {
  return field->size();
}
const ::vc_api::v1::CreateIssuanceChallengeRequest& __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeRequest_repeated_get(
  google::protobuf::RepeatedPtrField<::vc_api::v1::CreateIssuanceChallengeRequest>* field,
  size_t index) {
  return field->Get(index);
}
::vc_api::v1::CreateIssuanceChallengeRequest* __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeRequest_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::vc_api::v1::CreateIssuanceChallengeRequest>* field,
  size_t index) {
  return field->Mutable(index);
}
::vc_api::v1::CreateIssuanceChallengeRequest* __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeRequest_repeated_add(google::protobuf::RepeatedPtrField<::vc_api::v1::CreateIssuanceChallengeRequest>* field) {
  return field->Add();
}
void __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeRequest_repeated_clear(google::protobuf::RepeatedPtrField<::vc_api::v1::CreateIssuanceChallengeRequest>* field) {
  field->Clear();
}
void __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeRequest_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::vc_api::v1::CreateIssuanceChallengeRequest>& dst,
  const google::protobuf::RepeatedPtrField<::vc_api::v1::CreateIssuanceChallengeRequest>& src) {
  dst = src;
}

::vc_api::v1::CredentialType __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeRequest_get_credential_type(::vc_api::v1::CreateIssuanceChallengeRequest* msg) { return msg->credential_type(); }
void __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeRequest_set_credential_type(::vc_api::v1::CreateIssuanceChallengeRequest* msg, ::vc_api::v1::CredentialType val) {
  msg->set_credential_type(val);
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeRequest_get_did(::vc_api::v1::CreateIssuanceChallengeRequest* msg) {
  absl::string_view val = msg->did();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeRequest_set_did(::vc_api::v1::CreateIssuanceChallengeRequest* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_did(absl::string_view(s.ptr, s.len));
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::vc_api::v1::CreateIssuanceChallengeRequest>* __rust_proto_thunk__Map_i32_vc_api_v1_CreateIssuanceChallengeRequest_new() {
    return new google::protobuf::Map<int32_t, ::vc_api::v1::CreateIssuanceChallengeRequest>();
  }
  void __rust_proto_thunk__Map_i32_vc_api_v1_CreateIssuanceChallengeRequest_free(const google::protobuf::Map<int32_t, ::vc_api::v1::CreateIssuanceChallengeRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_vc_api_v1_CreateIssuanceChallengeRequest_clear(google::protobuf::Map<int32_t, ::vc_api::v1::CreateIssuanceChallengeRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_vc_api_v1_CreateIssuanceChallengeRequest_size(const google::protobuf::Map<int32_t, ::vc_api::v1::CreateIssuanceChallengeRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_vc_api_v1_CreateIssuanceChallengeRequest_insert(google::protobuf::Map<int32_t, ::vc_api::v1::CreateIssuanceChallengeRequest> * m,
                          int32_t key, ::vc_api::v1::CreateIssuanceChallengeRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_vc_api_v1_CreateIssuanceChallengeRequest_get(const google::protobuf::Map<int32_t, ::vc_api::v1::CreateIssuanceChallengeRequest>* m,
                       int32_t key, const ::vc_api::v1::CreateIssuanceChallengeRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::CreateIssuanceChallengeRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_vc_api_v1_CreateIssuanceChallengeRequest_remove(google::protobuf::Map<int32_t, ::vc_api::v1::CreateIssuanceChallengeRequest> * m,
                          int32_t key, ::vc_api::v1::CreateIssuanceChallengeRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_vc_api_v1_CreateIssuanceChallengeRequest_iter(
      const google::protobuf::Map<int32_t, ::vc_api::v1::CreateIssuanceChallengeRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_vc_api_v1_CreateIssuanceChallengeRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::vc_api::v1::CreateIssuanceChallengeRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::vc_api::v1::CreateIssuanceChallengeRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::vc_api::v1::CreateIssuanceChallengeRequest>* __rust_proto_thunk__Map_u32_vc_api_v1_CreateIssuanceChallengeRequest_new() {
    return new google::protobuf::Map<uint32_t, ::vc_api::v1::CreateIssuanceChallengeRequest>();
  }
  void __rust_proto_thunk__Map_u32_vc_api_v1_CreateIssuanceChallengeRequest_free(const google::protobuf::Map<uint32_t, ::vc_api::v1::CreateIssuanceChallengeRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_vc_api_v1_CreateIssuanceChallengeRequest_clear(google::protobuf::Map<uint32_t, ::vc_api::v1::CreateIssuanceChallengeRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_vc_api_v1_CreateIssuanceChallengeRequest_size(const google::protobuf::Map<uint32_t, ::vc_api::v1::CreateIssuanceChallengeRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_vc_api_v1_CreateIssuanceChallengeRequest_insert(google::protobuf::Map<uint32_t, ::vc_api::v1::CreateIssuanceChallengeRequest> * m,
                          uint32_t key, ::vc_api::v1::CreateIssuanceChallengeRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_vc_api_v1_CreateIssuanceChallengeRequest_get(const google::protobuf::Map<uint32_t, ::vc_api::v1::CreateIssuanceChallengeRequest>* m,
                       uint32_t key, const ::vc_api::v1::CreateIssuanceChallengeRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::CreateIssuanceChallengeRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_vc_api_v1_CreateIssuanceChallengeRequest_remove(google::protobuf::Map<uint32_t, ::vc_api::v1::CreateIssuanceChallengeRequest> * m,
                          uint32_t key, ::vc_api::v1::CreateIssuanceChallengeRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_vc_api_v1_CreateIssuanceChallengeRequest_iter(
      const google::protobuf::Map<uint32_t, ::vc_api::v1::CreateIssuanceChallengeRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_vc_api_v1_CreateIssuanceChallengeRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::vc_api::v1::CreateIssuanceChallengeRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::vc_api::v1::CreateIssuanceChallengeRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::vc_api::v1::CreateIssuanceChallengeRequest>* __rust_proto_thunk__Map_i64_vc_api_v1_CreateIssuanceChallengeRequest_new() {
    return new google::protobuf::Map<int64_t, ::vc_api::v1::CreateIssuanceChallengeRequest>();
  }
  void __rust_proto_thunk__Map_i64_vc_api_v1_CreateIssuanceChallengeRequest_free(const google::protobuf::Map<int64_t, ::vc_api::v1::CreateIssuanceChallengeRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_vc_api_v1_CreateIssuanceChallengeRequest_clear(google::protobuf::Map<int64_t, ::vc_api::v1::CreateIssuanceChallengeRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_vc_api_v1_CreateIssuanceChallengeRequest_size(const google::protobuf::Map<int64_t, ::vc_api::v1::CreateIssuanceChallengeRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_vc_api_v1_CreateIssuanceChallengeRequest_insert(google::protobuf::Map<int64_t, ::vc_api::v1::CreateIssuanceChallengeRequest> * m,
                          int64_t key, ::vc_api::v1::CreateIssuanceChallengeRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_vc_api_v1_CreateIssuanceChallengeRequest_get(const google::protobuf::Map<int64_t, ::vc_api::v1::CreateIssuanceChallengeRequest>* m,
                       int64_t key, const ::vc_api::v1::CreateIssuanceChallengeRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::CreateIssuanceChallengeRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_vc_api_v1_CreateIssuanceChallengeRequest_remove(google::protobuf::Map<int64_t, ::vc_api::v1::CreateIssuanceChallengeRequest> * m,
                          int64_t key, ::vc_api::v1::CreateIssuanceChallengeRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_vc_api_v1_CreateIssuanceChallengeRequest_iter(
      const google::protobuf::Map<int64_t, ::vc_api::v1::CreateIssuanceChallengeRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_vc_api_v1_CreateIssuanceChallengeRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::vc_api::v1::CreateIssuanceChallengeRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::vc_api::v1::CreateIssuanceChallengeRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::vc_api::v1::CreateIssuanceChallengeRequest>* __rust_proto_thunk__Map_u64_vc_api_v1_CreateIssuanceChallengeRequest_new() {
    return new google::protobuf::Map<uint64_t, ::vc_api::v1::CreateIssuanceChallengeRequest>();
  }
  void __rust_proto_thunk__Map_u64_vc_api_v1_CreateIssuanceChallengeRequest_free(const google::protobuf::Map<uint64_t, ::vc_api::v1::CreateIssuanceChallengeRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_vc_api_v1_CreateIssuanceChallengeRequest_clear(google::protobuf::Map<uint64_t, ::vc_api::v1::CreateIssuanceChallengeRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_vc_api_v1_CreateIssuanceChallengeRequest_size(const google::protobuf::Map<uint64_t, ::vc_api::v1::CreateIssuanceChallengeRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_vc_api_v1_CreateIssuanceChallengeRequest_insert(google::protobuf::Map<uint64_t, ::vc_api::v1::CreateIssuanceChallengeRequest> * m,
                          uint64_t key, ::vc_api::v1::CreateIssuanceChallengeRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_vc_api_v1_CreateIssuanceChallengeRequest_get(const google::protobuf::Map<uint64_t, ::vc_api::v1::CreateIssuanceChallengeRequest>* m,
                       uint64_t key, const ::vc_api::v1::CreateIssuanceChallengeRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::CreateIssuanceChallengeRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_vc_api_v1_CreateIssuanceChallengeRequest_remove(google::protobuf::Map<uint64_t, ::vc_api::v1::CreateIssuanceChallengeRequest> * m,
                          uint64_t key, ::vc_api::v1::CreateIssuanceChallengeRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_vc_api_v1_CreateIssuanceChallengeRequest_iter(
      const google::protobuf::Map<uint64_t, ::vc_api::v1::CreateIssuanceChallengeRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_vc_api_v1_CreateIssuanceChallengeRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::vc_api::v1::CreateIssuanceChallengeRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::vc_api::v1::CreateIssuanceChallengeRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::vc_api::v1::CreateIssuanceChallengeRequest>* __rust_proto_thunk__Map_bool_vc_api_v1_CreateIssuanceChallengeRequest_new() {
    return new google::protobuf::Map<bool, ::vc_api::v1::CreateIssuanceChallengeRequest>();
  }
  void __rust_proto_thunk__Map_bool_vc_api_v1_CreateIssuanceChallengeRequest_free(const google::protobuf::Map<bool, ::vc_api::v1::CreateIssuanceChallengeRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_vc_api_v1_CreateIssuanceChallengeRequest_clear(google::protobuf::Map<bool, ::vc_api::v1::CreateIssuanceChallengeRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_vc_api_v1_CreateIssuanceChallengeRequest_size(const google::protobuf::Map<bool, ::vc_api::v1::CreateIssuanceChallengeRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_vc_api_v1_CreateIssuanceChallengeRequest_insert(google::protobuf::Map<bool, ::vc_api::v1::CreateIssuanceChallengeRequest> * m,
                          bool key, ::vc_api::v1::CreateIssuanceChallengeRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_vc_api_v1_CreateIssuanceChallengeRequest_get(const google::protobuf::Map<bool, ::vc_api::v1::CreateIssuanceChallengeRequest>* m,
                       bool key, const ::vc_api::v1::CreateIssuanceChallengeRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::CreateIssuanceChallengeRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_vc_api_v1_CreateIssuanceChallengeRequest_remove(google::protobuf::Map<bool, ::vc_api::v1::CreateIssuanceChallengeRequest> * m,
                          bool key, ::vc_api::v1::CreateIssuanceChallengeRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_vc_api_v1_CreateIssuanceChallengeRequest_iter(
      const google::protobuf::Map<bool, ::vc_api::v1::CreateIssuanceChallengeRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_vc_api_v1_CreateIssuanceChallengeRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::vc_api::v1::CreateIssuanceChallengeRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::vc_api::v1::CreateIssuanceChallengeRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::vc_api::v1::CreateIssuanceChallengeRequest>* __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreateIssuanceChallengeRequest_new() {
    return new google::protobuf::Map<std::string, ::vc_api::v1::CreateIssuanceChallengeRequest>();
  }
  void __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreateIssuanceChallengeRequest_free(const google::protobuf::Map<std::string, ::vc_api::v1::CreateIssuanceChallengeRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreateIssuanceChallengeRequest_clear(google::protobuf::Map<std::string, ::vc_api::v1::CreateIssuanceChallengeRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreateIssuanceChallengeRequest_size(const google::protobuf::Map<std::string, ::vc_api::v1::CreateIssuanceChallengeRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreateIssuanceChallengeRequest_insert(google::protobuf::Map<std::string, ::vc_api::v1::CreateIssuanceChallengeRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::vc_api::v1::CreateIssuanceChallengeRequest value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreateIssuanceChallengeRequest_get(const google::protobuf::Map<std::string, ::vc_api::v1::CreateIssuanceChallengeRequest>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::vc_api::v1::CreateIssuanceChallengeRequest** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::CreateIssuanceChallengeRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreateIssuanceChallengeRequest_remove(google::protobuf::Map<std::string, ::vc_api::v1::CreateIssuanceChallengeRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::vc_api::v1::CreateIssuanceChallengeRequest * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreateIssuanceChallengeRequest_iter(
      const google::protobuf::Map<std::string, ::vc_api::v1::CreateIssuanceChallengeRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreateIssuanceChallengeRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::vc_api::v1::CreateIssuanceChallengeRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::vc_api::v1::CreateIssuanceChallengeRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// vc_api.v1.CreateIssuanceChallengeResponse
        // clang-format off
extern "C" {
void* __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeResponse_new() { return new ::vc_api::v1::CreateIssuanceChallengeResponse(); }
void __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeResponse_delete(void* ptr) { delete static_cast<::vc_api::v1::CreateIssuanceChallengeResponse*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeResponse_serialize(::vc_api::v1::CreateIssuanceChallengeResponse* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeResponse_parse(::vc_api::v1::CreateIssuanceChallengeResponse* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeResponse_copy_from(::vc_api::v1::CreateIssuanceChallengeResponse* dst, const ::vc_api::v1::CreateIssuanceChallengeResponse* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeResponse_repeated_len(google::protobuf::RepeatedPtrField<::vc_api::v1::CreateIssuanceChallengeResponse>* field) {
  return field->size();
}
const ::vc_api::v1::CreateIssuanceChallengeResponse& __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeResponse_repeated_get(
  google::protobuf::RepeatedPtrField<::vc_api::v1::CreateIssuanceChallengeResponse>* field,
  size_t index) {
  return field->Get(index);
}
::vc_api::v1::CreateIssuanceChallengeResponse* __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeResponse_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::vc_api::v1::CreateIssuanceChallengeResponse>* field,
  size_t index) {
  return field->Mutable(index);
}
::vc_api::v1::CreateIssuanceChallengeResponse* __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeResponse_repeated_add(google::protobuf::RepeatedPtrField<::vc_api::v1::CreateIssuanceChallengeResponse>* field) {
  return field->Add();
}
void __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeResponse_repeated_clear(google::protobuf::RepeatedPtrField<::vc_api::v1::CreateIssuanceChallengeResponse>* field) {
  field->Clear();
}
void __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeResponse_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::vc_api::v1::CreateIssuanceChallengeResponse>& dst,
  const google::protobuf::RepeatedPtrField<::vc_api::v1::CreateIssuanceChallengeResponse>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeResponse_get_nonce(::vc_api::v1::CreateIssuanceChallengeResponse* msg) {
  absl::string_view val = msg->nonce();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeResponse_set_nonce(::vc_api::v1::CreateIssuanceChallengeResponse* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_nonce(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeResponse_get_endpoint(::vc_api::v1::CreateIssuanceChallengeResponse* msg) {
  absl::string_view val = msg->endpoint();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeResponse_set_endpoint(::vc_api::v1::CreateIssuanceChallengeResponse* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_endpoint(absl::string_view(s.ptr, s.len));
}

::vc_api::v1::CredentialType __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeResponse_get_credential_type(::vc_api::v1::CreateIssuanceChallengeResponse* msg) { return msg->credential_type(); }
void __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeResponse_set_credential_type(::vc_api::v1::CreateIssuanceChallengeResponse* msg, ::vc_api::v1::CredentialType val) {
  msg->set_credential_type(val);
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::vc_api::v1::CreateIssuanceChallengeResponse>* __rust_proto_thunk__Map_i32_vc_api_v1_CreateIssuanceChallengeResponse_new() {
    return new google::protobuf::Map<int32_t, ::vc_api::v1::CreateIssuanceChallengeResponse>();
  }
  void __rust_proto_thunk__Map_i32_vc_api_v1_CreateIssuanceChallengeResponse_free(const google::protobuf::Map<int32_t, ::vc_api::v1::CreateIssuanceChallengeResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_vc_api_v1_CreateIssuanceChallengeResponse_clear(google::protobuf::Map<int32_t, ::vc_api::v1::CreateIssuanceChallengeResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_vc_api_v1_CreateIssuanceChallengeResponse_size(const google::protobuf::Map<int32_t, ::vc_api::v1::CreateIssuanceChallengeResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_vc_api_v1_CreateIssuanceChallengeResponse_insert(google::protobuf::Map<int32_t, ::vc_api::v1::CreateIssuanceChallengeResponse> * m,
                          int32_t key, ::vc_api::v1::CreateIssuanceChallengeResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_vc_api_v1_CreateIssuanceChallengeResponse_get(const google::protobuf::Map<int32_t, ::vc_api::v1::CreateIssuanceChallengeResponse>* m,
                       int32_t key, const ::vc_api::v1::CreateIssuanceChallengeResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::CreateIssuanceChallengeResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_vc_api_v1_CreateIssuanceChallengeResponse_remove(google::protobuf::Map<int32_t, ::vc_api::v1::CreateIssuanceChallengeResponse> * m,
                          int32_t key, ::vc_api::v1::CreateIssuanceChallengeResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_vc_api_v1_CreateIssuanceChallengeResponse_iter(
      const google::protobuf::Map<int32_t, ::vc_api::v1::CreateIssuanceChallengeResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_vc_api_v1_CreateIssuanceChallengeResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::vc_api::v1::CreateIssuanceChallengeResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::vc_api::v1::CreateIssuanceChallengeResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::vc_api::v1::CreateIssuanceChallengeResponse>* __rust_proto_thunk__Map_u32_vc_api_v1_CreateIssuanceChallengeResponse_new() {
    return new google::protobuf::Map<uint32_t, ::vc_api::v1::CreateIssuanceChallengeResponse>();
  }
  void __rust_proto_thunk__Map_u32_vc_api_v1_CreateIssuanceChallengeResponse_free(const google::protobuf::Map<uint32_t, ::vc_api::v1::CreateIssuanceChallengeResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_vc_api_v1_CreateIssuanceChallengeResponse_clear(google::protobuf::Map<uint32_t, ::vc_api::v1::CreateIssuanceChallengeResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_vc_api_v1_CreateIssuanceChallengeResponse_size(const google::protobuf::Map<uint32_t, ::vc_api::v1::CreateIssuanceChallengeResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_vc_api_v1_CreateIssuanceChallengeResponse_insert(google::protobuf::Map<uint32_t, ::vc_api::v1::CreateIssuanceChallengeResponse> * m,
                          uint32_t key, ::vc_api::v1::CreateIssuanceChallengeResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_vc_api_v1_CreateIssuanceChallengeResponse_get(const google::protobuf::Map<uint32_t, ::vc_api::v1::CreateIssuanceChallengeResponse>* m,
                       uint32_t key, const ::vc_api::v1::CreateIssuanceChallengeResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::CreateIssuanceChallengeResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_vc_api_v1_CreateIssuanceChallengeResponse_remove(google::protobuf::Map<uint32_t, ::vc_api::v1::CreateIssuanceChallengeResponse> * m,
                          uint32_t key, ::vc_api::v1::CreateIssuanceChallengeResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_vc_api_v1_CreateIssuanceChallengeResponse_iter(
      const google::protobuf::Map<uint32_t, ::vc_api::v1::CreateIssuanceChallengeResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_vc_api_v1_CreateIssuanceChallengeResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::vc_api::v1::CreateIssuanceChallengeResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::vc_api::v1::CreateIssuanceChallengeResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::vc_api::v1::CreateIssuanceChallengeResponse>* __rust_proto_thunk__Map_i64_vc_api_v1_CreateIssuanceChallengeResponse_new() {
    return new google::protobuf::Map<int64_t, ::vc_api::v1::CreateIssuanceChallengeResponse>();
  }
  void __rust_proto_thunk__Map_i64_vc_api_v1_CreateIssuanceChallengeResponse_free(const google::protobuf::Map<int64_t, ::vc_api::v1::CreateIssuanceChallengeResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_vc_api_v1_CreateIssuanceChallengeResponse_clear(google::protobuf::Map<int64_t, ::vc_api::v1::CreateIssuanceChallengeResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_vc_api_v1_CreateIssuanceChallengeResponse_size(const google::protobuf::Map<int64_t, ::vc_api::v1::CreateIssuanceChallengeResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_vc_api_v1_CreateIssuanceChallengeResponse_insert(google::protobuf::Map<int64_t, ::vc_api::v1::CreateIssuanceChallengeResponse> * m,
                          int64_t key, ::vc_api::v1::CreateIssuanceChallengeResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_vc_api_v1_CreateIssuanceChallengeResponse_get(const google::protobuf::Map<int64_t, ::vc_api::v1::CreateIssuanceChallengeResponse>* m,
                       int64_t key, const ::vc_api::v1::CreateIssuanceChallengeResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::CreateIssuanceChallengeResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_vc_api_v1_CreateIssuanceChallengeResponse_remove(google::protobuf::Map<int64_t, ::vc_api::v1::CreateIssuanceChallengeResponse> * m,
                          int64_t key, ::vc_api::v1::CreateIssuanceChallengeResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_vc_api_v1_CreateIssuanceChallengeResponse_iter(
      const google::protobuf::Map<int64_t, ::vc_api::v1::CreateIssuanceChallengeResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_vc_api_v1_CreateIssuanceChallengeResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::vc_api::v1::CreateIssuanceChallengeResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::vc_api::v1::CreateIssuanceChallengeResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::vc_api::v1::CreateIssuanceChallengeResponse>* __rust_proto_thunk__Map_u64_vc_api_v1_CreateIssuanceChallengeResponse_new() {
    return new google::protobuf::Map<uint64_t, ::vc_api::v1::CreateIssuanceChallengeResponse>();
  }
  void __rust_proto_thunk__Map_u64_vc_api_v1_CreateIssuanceChallengeResponse_free(const google::protobuf::Map<uint64_t, ::vc_api::v1::CreateIssuanceChallengeResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_vc_api_v1_CreateIssuanceChallengeResponse_clear(google::protobuf::Map<uint64_t, ::vc_api::v1::CreateIssuanceChallengeResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_vc_api_v1_CreateIssuanceChallengeResponse_size(const google::protobuf::Map<uint64_t, ::vc_api::v1::CreateIssuanceChallengeResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_vc_api_v1_CreateIssuanceChallengeResponse_insert(google::protobuf::Map<uint64_t, ::vc_api::v1::CreateIssuanceChallengeResponse> * m,
                          uint64_t key, ::vc_api::v1::CreateIssuanceChallengeResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_vc_api_v1_CreateIssuanceChallengeResponse_get(const google::protobuf::Map<uint64_t, ::vc_api::v1::CreateIssuanceChallengeResponse>* m,
                       uint64_t key, const ::vc_api::v1::CreateIssuanceChallengeResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::CreateIssuanceChallengeResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_vc_api_v1_CreateIssuanceChallengeResponse_remove(google::protobuf::Map<uint64_t, ::vc_api::v1::CreateIssuanceChallengeResponse> * m,
                          uint64_t key, ::vc_api::v1::CreateIssuanceChallengeResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_vc_api_v1_CreateIssuanceChallengeResponse_iter(
      const google::protobuf::Map<uint64_t, ::vc_api::v1::CreateIssuanceChallengeResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_vc_api_v1_CreateIssuanceChallengeResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::vc_api::v1::CreateIssuanceChallengeResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::vc_api::v1::CreateIssuanceChallengeResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::vc_api::v1::CreateIssuanceChallengeResponse>* __rust_proto_thunk__Map_bool_vc_api_v1_CreateIssuanceChallengeResponse_new() {
    return new google::protobuf::Map<bool, ::vc_api::v1::CreateIssuanceChallengeResponse>();
  }
  void __rust_proto_thunk__Map_bool_vc_api_v1_CreateIssuanceChallengeResponse_free(const google::protobuf::Map<bool, ::vc_api::v1::CreateIssuanceChallengeResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_vc_api_v1_CreateIssuanceChallengeResponse_clear(google::protobuf::Map<bool, ::vc_api::v1::CreateIssuanceChallengeResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_vc_api_v1_CreateIssuanceChallengeResponse_size(const google::protobuf::Map<bool, ::vc_api::v1::CreateIssuanceChallengeResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_vc_api_v1_CreateIssuanceChallengeResponse_insert(google::protobuf::Map<bool, ::vc_api::v1::CreateIssuanceChallengeResponse> * m,
                          bool key, ::vc_api::v1::CreateIssuanceChallengeResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_vc_api_v1_CreateIssuanceChallengeResponse_get(const google::protobuf::Map<bool, ::vc_api::v1::CreateIssuanceChallengeResponse>* m,
                       bool key, const ::vc_api::v1::CreateIssuanceChallengeResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::CreateIssuanceChallengeResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_vc_api_v1_CreateIssuanceChallengeResponse_remove(google::protobuf::Map<bool, ::vc_api::v1::CreateIssuanceChallengeResponse> * m,
                          bool key, ::vc_api::v1::CreateIssuanceChallengeResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_vc_api_v1_CreateIssuanceChallengeResponse_iter(
      const google::protobuf::Map<bool, ::vc_api::v1::CreateIssuanceChallengeResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_vc_api_v1_CreateIssuanceChallengeResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::vc_api::v1::CreateIssuanceChallengeResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::vc_api::v1::CreateIssuanceChallengeResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::vc_api::v1::CreateIssuanceChallengeResponse>* __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreateIssuanceChallengeResponse_new() {
    return new google::protobuf::Map<std::string, ::vc_api::v1::CreateIssuanceChallengeResponse>();
  }
  void __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreateIssuanceChallengeResponse_free(const google::protobuf::Map<std::string, ::vc_api::v1::CreateIssuanceChallengeResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreateIssuanceChallengeResponse_clear(google::protobuf::Map<std::string, ::vc_api::v1::CreateIssuanceChallengeResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreateIssuanceChallengeResponse_size(const google::protobuf::Map<std::string, ::vc_api::v1::CreateIssuanceChallengeResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreateIssuanceChallengeResponse_insert(google::protobuf::Map<std::string, ::vc_api::v1::CreateIssuanceChallengeResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::vc_api::v1::CreateIssuanceChallengeResponse value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreateIssuanceChallengeResponse_get(const google::protobuf::Map<std::string, ::vc_api::v1::CreateIssuanceChallengeResponse>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::vc_api::v1::CreateIssuanceChallengeResponse** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::CreateIssuanceChallengeResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreateIssuanceChallengeResponse_remove(google::protobuf::Map<std::string, ::vc_api::v1::CreateIssuanceChallengeResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::vc_api::v1::CreateIssuanceChallengeResponse * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreateIssuanceChallengeResponse_iter(
      const google::protobuf::Map<std::string, ::vc_api::v1::CreateIssuanceChallengeResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreateIssuanceChallengeResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::vc_api::v1::CreateIssuanceChallengeResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::vc_api::v1::CreateIssuanceChallengeResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// vc_api.v1.IssueVerifiableCredentialRequest
        // clang-format off
extern "C" {
void* __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialRequest_new() { return new ::vc_api::v1::IssueVerifiableCredentialRequest(); }
void __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialRequest_delete(void* ptr) { delete static_cast<::vc_api::v1::IssueVerifiableCredentialRequest*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialRequest_serialize(::vc_api::v1::IssueVerifiableCredentialRequest* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialRequest_parse(::vc_api::v1::IssueVerifiableCredentialRequest* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialRequest_copy_from(::vc_api::v1::IssueVerifiableCredentialRequest* dst, const ::vc_api::v1::IssueVerifiableCredentialRequest* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialRequest_repeated_len(google::protobuf::RepeatedPtrField<::vc_api::v1::IssueVerifiableCredentialRequest>* field) {
  return field->size();
}
const ::vc_api::v1::IssueVerifiableCredentialRequest& __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialRequest_repeated_get(
  google::protobuf::RepeatedPtrField<::vc_api::v1::IssueVerifiableCredentialRequest>* field,
  size_t index) {
  return field->Get(index);
}
::vc_api::v1::IssueVerifiableCredentialRequest* __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialRequest_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::vc_api::v1::IssueVerifiableCredentialRequest>* field,
  size_t index) {
  return field->Mutable(index);
}
::vc_api::v1::IssueVerifiableCredentialRequest* __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialRequest_repeated_add(google::protobuf::RepeatedPtrField<::vc_api::v1::IssueVerifiableCredentialRequest>* field) {
  return field->Add();
}
void __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialRequest_repeated_clear(google::protobuf::RepeatedPtrField<::vc_api::v1::IssueVerifiableCredentialRequest>* field) {
  field->Clear();
}
void __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialRequest_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::vc_api::v1::IssueVerifiableCredentialRequest>& dst,
  const google::protobuf::RepeatedPtrField<::vc_api::v1::IssueVerifiableCredentialRequest>& src) {
  dst = src;
}

::vc_api::v1::CredentialType __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialRequest_get_credential_type(::vc_api::v1::IssueVerifiableCredentialRequest* msg) { return msg->credential_type(); }
void __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialRequest_set_credential_type(::vc_api::v1::IssueVerifiableCredentialRequest* msg, ::vc_api::v1::CredentialType val) {
  msg->set_credential_type(val);
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialRequest_get_nonce(::vc_api::v1::IssueVerifiableCredentialRequest* msg) {
  absl::string_view val = msg->nonce();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialRequest_set_nonce(::vc_api::v1::IssueVerifiableCredentialRequest* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_nonce(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialRequest_get_signature(::vc_api::v1::IssueVerifiableCredentialRequest* msg) {
  absl::string_view val = msg->signature();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialRequest_set_signature(::vc_api::v1::IssueVerifiableCredentialRequest* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_signature(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialRequest_get_did(::vc_api::v1::IssueVerifiableCredentialRequest* msg) {
  absl::string_view val = msg->did();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialRequest_set_did(::vc_api::v1::IssueVerifiableCredentialRequest* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_did(absl::string_view(s.ptr, s.len));
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::vc_api::v1::IssueVerifiableCredentialRequest>* __rust_proto_thunk__Map_i32_vc_api_v1_IssueVerifiableCredentialRequest_new() {
    return new google::protobuf::Map<int32_t, ::vc_api::v1::IssueVerifiableCredentialRequest>();
  }
  void __rust_proto_thunk__Map_i32_vc_api_v1_IssueVerifiableCredentialRequest_free(const google::protobuf::Map<int32_t, ::vc_api::v1::IssueVerifiableCredentialRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_vc_api_v1_IssueVerifiableCredentialRequest_clear(google::protobuf::Map<int32_t, ::vc_api::v1::IssueVerifiableCredentialRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_vc_api_v1_IssueVerifiableCredentialRequest_size(const google::protobuf::Map<int32_t, ::vc_api::v1::IssueVerifiableCredentialRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_vc_api_v1_IssueVerifiableCredentialRequest_insert(google::protobuf::Map<int32_t, ::vc_api::v1::IssueVerifiableCredentialRequest> * m,
                          int32_t key, ::vc_api::v1::IssueVerifiableCredentialRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_vc_api_v1_IssueVerifiableCredentialRequest_get(const google::protobuf::Map<int32_t, ::vc_api::v1::IssueVerifiableCredentialRequest>* m,
                       int32_t key, const ::vc_api::v1::IssueVerifiableCredentialRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::IssueVerifiableCredentialRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_vc_api_v1_IssueVerifiableCredentialRequest_remove(google::protobuf::Map<int32_t, ::vc_api::v1::IssueVerifiableCredentialRequest> * m,
                          int32_t key, ::vc_api::v1::IssueVerifiableCredentialRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_vc_api_v1_IssueVerifiableCredentialRequest_iter(
      const google::protobuf::Map<int32_t, ::vc_api::v1::IssueVerifiableCredentialRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_vc_api_v1_IssueVerifiableCredentialRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::vc_api::v1::IssueVerifiableCredentialRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::vc_api::v1::IssueVerifiableCredentialRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::vc_api::v1::IssueVerifiableCredentialRequest>* __rust_proto_thunk__Map_u32_vc_api_v1_IssueVerifiableCredentialRequest_new() {
    return new google::protobuf::Map<uint32_t, ::vc_api::v1::IssueVerifiableCredentialRequest>();
  }
  void __rust_proto_thunk__Map_u32_vc_api_v1_IssueVerifiableCredentialRequest_free(const google::protobuf::Map<uint32_t, ::vc_api::v1::IssueVerifiableCredentialRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_vc_api_v1_IssueVerifiableCredentialRequest_clear(google::protobuf::Map<uint32_t, ::vc_api::v1::IssueVerifiableCredentialRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_vc_api_v1_IssueVerifiableCredentialRequest_size(const google::protobuf::Map<uint32_t, ::vc_api::v1::IssueVerifiableCredentialRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_vc_api_v1_IssueVerifiableCredentialRequest_insert(google::protobuf::Map<uint32_t, ::vc_api::v1::IssueVerifiableCredentialRequest> * m,
                          uint32_t key, ::vc_api::v1::IssueVerifiableCredentialRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_vc_api_v1_IssueVerifiableCredentialRequest_get(const google::protobuf::Map<uint32_t, ::vc_api::v1::IssueVerifiableCredentialRequest>* m,
                       uint32_t key, const ::vc_api::v1::IssueVerifiableCredentialRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::IssueVerifiableCredentialRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_vc_api_v1_IssueVerifiableCredentialRequest_remove(google::protobuf::Map<uint32_t, ::vc_api::v1::IssueVerifiableCredentialRequest> * m,
                          uint32_t key, ::vc_api::v1::IssueVerifiableCredentialRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_vc_api_v1_IssueVerifiableCredentialRequest_iter(
      const google::protobuf::Map<uint32_t, ::vc_api::v1::IssueVerifiableCredentialRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_vc_api_v1_IssueVerifiableCredentialRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::vc_api::v1::IssueVerifiableCredentialRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::vc_api::v1::IssueVerifiableCredentialRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::vc_api::v1::IssueVerifiableCredentialRequest>* __rust_proto_thunk__Map_i64_vc_api_v1_IssueVerifiableCredentialRequest_new() {
    return new google::protobuf::Map<int64_t, ::vc_api::v1::IssueVerifiableCredentialRequest>();
  }
  void __rust_proto_thunk__Map_i64_vc_api_v1_IssueVerifiableCredentialRequest_free(const google::protobuf::Map<int64_t, ::vc_api::v1::IssueVerifiableCredentialRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_vc_api_v1_IssueVerifiableCredentialRequest_clear(google::protobuf::Map<int64_t, ::vc_api::v1::IssueVerifiableCredentialRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_vc_api_v1_IssueVerifiableCredentialRequest_size(const google::protobuf::Map<int64_t, ::vc_api::v1::IssueVerifiableCredentialRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_vc_api_v1_IssueVerifiableCredentialRequest_insert(google::protobuf::Map<int64_t, ::vc_api::v1::IssueVerifiableCredentialRequest> * m,
                          int64_t key, ::vc_api::v1::IssueVerifiableCredentialRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_vc_api_v1_IssueVerifiableCredentialRequest_get(const google::protobuf::Map<int64_t, ::vc_api::v1::IssueVerifiableCredentialRequest>* m,
                       int64_t key, const ::vc_api::v1::IssueVerifiableCredentialRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::IssueVerifiableCredentialRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_vc_api_v1_IssueVerifiableCredentialRequest_remove(google::protobuf::Map<int64_t, ::vc_api::v1::IssueVerifiableCredentialRequest> * m,
                          int64_t key, ::vc_api::v1::IssueVerifiableCredentialRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_vc_api_v1_IssueVerifiableCredentialRequest_iter(
      const google::protobuf::Map<int64_t, ::vc_api::v1::IssueVerifiableCredentialRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_vc_api_v1_IssueVerifiableCredentialRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::vc_api::v1::IssueVerifiableCredentialRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::vc_api::v1::IssueVerifiableCredentialRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::vc_api::v1::IssueVerifiableCredentialRequest>* __rust_proto_thunk__Map_u64_vc_api_v1_IssueVerifiableCredentialRequest_new() {
    return new google::protobuf::Map<uint64_t, ::vc_api::v1::IssueVerifiableCredentialRequest>();
  }
  void __rust_proto_thunk__Map_u64_vc_api_v1_IssueVerifiableCredentialRequest_free(const google::protobuf::Map<uint64_t, ::vc_api::v1::IssueVerifiableCredentialRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_vc_api_v1_IssueVerifiableCredentialRequest_clear(google::protobuf::Map<uint64_t, ::vc_api::v1::IssueVerifiableCredentialRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_vc_api_v1_IssueVerifiableCredentialRequest_size(const google::protobuf::Map<uint64_t, ::vc_api::v1::IssueVerifiableCredentialRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_vc_api_v1_IssueVerifiableCredentialRequest_insert(google::protobuf::Map<uint64_t, ::vc_api::v1::IssueVerifiableCredentialRequest> * m,
                          uint64_t key, ::vc_api::v1::IssueVerifiableCredentialRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_vc_api_v1_IssueVerifiableCredentialRequest_get(const google::protobuf::Map<uint64_t, ::vc_api::v1::IssueVerifiableCredentialRequest>* m,
                       uint64_t key, const ::vc_api::v1::IssueVerifiableCredentialRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::IssueVerifiableCredentialRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_vc_api_v1_IssueVerifiableCredentialRequest_remove(google::protobuf::Map<uint64_t, ::vc_api::v1::IssueVerifiableCredentialRequest> * m,
                          uint64_t key, ::vc_api::v1::IssueVerifiableCredentialRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_vc_api_v1_IssueVerifiableCredentialRequest_iter(
      const google::protobuf::Map<uint64_t, ::vc_api::v1::IssueVerifiableCredentialRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_vc_api_v1_IssueVerifiableCredentialRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::vc_api::v1::IssueVerifiableCredentialRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::vc_api::v1::IssueVerifiableCredentialRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::vc_api::v1::IssueVerifiableCredentialRequest>* __rust_proto_thunk__Map_bool_vc_api_v1_IssueVerifiableCredentialRequest_new() {
    return new google::protobuf::Map<bool, ::vc_api::v1::IssueVerifiableCredentialRequest>();
  }
  void __rust_proto_thunk__Map_bool_vc_api_v1_IssueVerifiableCredentialRequest_free(const google::protobuf::Map<bool, ::vc_api::v1::IssueVerifiableCredentialRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_vc_api_v1_IssueVerifiableCredentialRequest_clear(google::protobuf::Map<bool, ::vc_api::v1::IssueVerifiableCredentialRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_vc_api_v1_IssueVerifiableCredentialRequest_size(const google::protobuf::Map<bool, ::vc_api::v1::IssueVerifiableCredentialRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_vc_api_v1_IssueVerifiableCredentialRequest_insert(google::protobuf::Map<bool, ::vc_api::v1::IssueVerifiableCredentialRequest> * m,
                          bool key, ::vc_api::v1::IssueVerifiableCredentialRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_vc_api_v1_IssueVerifiableCredentialRequest_get(const google::protobuf::Map<bool, ::vc_api::v1::IssueVerifiableCredentialRequest>* m,
                       bool key, const ::vc_api::v1::IssueVerifiableCredentialRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::IssueVerifiableCredentialRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_vc_api_v1_IssueVerifiableCredentialRequest_remove(google::protobuf::Map<bool, ::vc_api::v1::IssueVerifiableCredentialRequest> * m,
                          bool key, ::vc_api::v1::IssueVerifiableCredentialRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_vc_api_v1_IssueVerifiableCredentialRequest_iter(
      const google::protobuf::Map<bool, ::vc_api::v1::IssueVerifiableCredentialRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_vc_api_v1_IssueVerifiableCredentialRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::vc_api::v1::IssueVerifiableCredentialRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::vc_api::v1::IssueVerifiableCredentialRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::vc_api::v1::IssueVerifiableCredentialRequest>* __rust_proto_thunk__Map_ProtoStr_vc_api_v1_IssueVerifiableCredentialRequest_new() {
    return new google::protobuf::Map<std::string, ::vc_api::v1::IssueVerifiableCredentialRequest>();
  }
  void __rust_proto_thunk__Map_ProtoStr_vc_api_v1_IssueVerifiableCredentialRequest_free(const google::protobuf::Map<std::string, ::vc_api::v1::IssueVerifiableCredentialRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_vc_api_v1_IssueVerifiableCredentialRequest_clear(google::protobuf::Map<std::string, ::vc_api::v1::IssueVerifiableCredentialRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_vc_api_v1_IssueVerifiableCredentialRequest_size(const google::protobuf::Map<std::string, ::vc_api::v1::IssueVerifiableCredentialRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_vc_api_v1_IssueVerifiableCredentialRequest_insert(google::protobuf::Map<std::string, ::vc_api::v1::IssueVerifiableCredentialRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::vc_api::v1::IssueVerifiableCredentialRequest value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_vc_api_v1_IssueVerifiableCredentialRequest_get(const google::protobuf::Map<std::string, ::vc_api::v1::IssueVerifiableCredentialRequest>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::vc_api::v1::IssueVerifiableCredentialRequest** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::IssueVerifiableCredentialRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_vc_api_v1_IssueVerifiableCredentialRequest_remove(google::protobuf::Map<std::string, ::vc_api::v1::IssueVerifiableCredentialRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::vc_api::v1::IssueVerifiableCredentialRequest * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_vc_api_v1_IssueVerifiableCredentialRequest_iter(
      const google::protobuf::Map<std::string, ::vc_api::v1::IssueVerifiableCredentialRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_vc_api_v1_IssueVerifiableCredentialRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::vc_api::v1::IssueVerifiableCredentialRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::vc_api::v1::IssueVerifiableCredentialRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// vc_api.v1.IssueVerifiableCredentialResponse
        // clang-format off
extern "C" {
void* __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialResponse_new() { return new ::vc_api::v1::IssueVerifiableCredentialResponse(); }
void __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialResponse_delete(void* ptr) { delete static_cast<::vc_api::v1::IssueVerifiableCredentialResponse*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialResponse_serialize(::vc_api::v1::IssueVerifiableCredentialResponse* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialResponse_parse(::vc_api::v1::IssueVerifiableCredentialResponse* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialResponse_copy_from(::vc_api::v1::IssueVerifiableCredentialResponse* dst, const ::vc_api::v1::IssueVerifiableCredentialResponse* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialResponse_repeated_len(google::protobuf::RepeatedPtrField<::vc_api::v1::IssueVerifiableCredentialResponse>* field) {
  return field->size();
}
const ::vc_api::v1::IssueVerifiableCredentialResponse& __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialResponse_repeated_get(
  google::protobuf::RepeatedPtrField<::vc_api::v1::IssueVerifiableCredentialResponse>* field,
  size_t index) {
  return field->Get(index);
}
::vc_api::v1::IssueVerifiableCredentialResponse* __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialResponse_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::vc_api::v1::IssueVerifiableCredentialResponse>* field,
  size_t index) {
  return field->Mutable(index);
}
::vc_api::v1::IssueVerifiableCredentialResponse* __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialResponse_repeated_add(google::protobuf::RepeatedPtrField<::vc_api::v1::IssueVerifiableCredentialResponse>* field) {
  return field->Add();
}
void __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialResponse_repeated_clear(google::protobuf::RepeatedPtrField<::vc_api::v1::IssueVerifiableCredentialResponse>* field) {
  field->Clear();
}
void __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialResponse_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::vc_api::v1::IssueVerifiableCredentialResponse>& dst,
  const google::protobuf::RepeatedPtrField<::vc_api::v1::IssueVerifiableCredentialResponse>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialResponse_get_credential(::vc_api::v1::IssueVerifiableCredentialResponse* msg) {
  absl::string_view val = msg->credential();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialResponse_set_credential(::vc_api::v1::IssueVerifiableCredentialResponse* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_credential(absl::string_view(s.ptr, s.len));
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::vc_api::v1::IssueVerifiableCredentialResponse>* __rust_proto_thunk__Map_i32_vc_api_v1_IssueVerifiableCredentialResponse_new() {
    return new google::protobuf::Map<int32_t, ::vc_api::v1::IssueVerifiableCredentialResponse>();
  }
  void __rust_proto_thunk__Map_i32_vc_api_v1_IssueVerifiableCredentialResponse_free(const google::protobuf::Map<int32_t, ::vc_api::v1::IssueVerifiableCredentialResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_vc_api_v1_IssueVerifiableCredentialResponse_clear(google::protobuf::Map<int32_t, ::vc_api::v1::IssueVerifiableCredentialResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_vc_api_v1_IssueVerifiableCredentialResponse_size(const google::protobuf::Map<int32_t, ::vc_api::v1::IssueVerifiableCredentialResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_vc_api_v1_IssueVerifiableCredentialResponse_insert(google::protobuf::Map<int32_t, ::vc_api::v1::IssueVerifiableCredentialResponse> * m,
                          int32_t key, ::vc_api::v1::IssueVerifiableCredentialResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_vc_api_v1_IssueVerifiableCredentialResponse_get(const google::protobuf::Map<int32_t, ::vc_api::v1::IssueVerifiableCredentialResponse>* m,
                       int32_t key, const ::vc_api::v1::IssueVerifiableCredentialResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::IssueVerifiableCredentialResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_vc_api_v1_IssueVerifiableCredentialResponse_remove(google::protobuf::Map<int32_t, ::vc_api::v1::IssueVerifiableCredentialResponse> * m,
                          int32_t key, ::vc_api::v1::IssueVerifiableCredentialResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_vc_api_v1_IssueVerifiableCredentialResponse_iter(
      const google::protobuf::Map<int32_t, ::vc_api::v1::IssueVerifiableCredentialResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_vc_api_v1_IssueVerifiableCredentialResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::vc_api::v1::IssueVerifiableCredentialResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::vc_api::v1::IssueVerifiableCredentialResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::vc_api::v1::IssueVerifiableCredentialResponse>* __rust_proto_thunk__Map_u32_vc_api_v1_IssueVerifiableCredentialResponse_new() {
    return new google::protobuf::Map<uint32_t, ::vc_api::v1::IssueVerifiableCredentialResponse>();
  }
  void __rust_proto_thunk__Map_u32_vc_api_v1_IssueVerifiableCredentialResponse_free(const google::protobuf::Map<uint32_t, ::vc_api::v1::IssueVerifiableCredentialResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_vc_api_v1_IssueVerifiableCredentialResponse_clear(google::protobuf::Map<uint32_t, ::vc_api::v1::IssueVerifiableCredentialResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_vc_api_v1_IssueVerifiableCredentialResponse_size(const google::protobuf::Map<uint32_t, ::vc_api::v1::IssueVerifiableCredentialResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_vc_api_v1_IssueVerifiableCredentialResponse_insert(google::protobuf::Map<uint32_t, ::vc_api::v1::IssueVerifiableCredentialResponse> * m,
                          uint32_t key, ::vc_api::v1::IssueVerifiableCredentialResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_vc_api_v1_IssueVerifiableCredentialResponse_get(const google::protobuf::Map<uint32_t, ::vc_api::v1::IssueVerifiableCredentialResponse>* m,
                       uint32_t key, const ::vc_api::v1::IssueVerifiableCredentialResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::IssueVerifiableCredentialResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_vc_api_v1_IssueVerifiableCredentialResponse_remove(google::protobuf::Map<uint32_t, ::vc_api::v1::IssueVerifiableCredentialResponse> * m,
                          uint32_t key, ::vc_api::v1::IssueVerifiableCredentialResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_vc_api_v1_IssueVerifiableCredentialResponse_iter(
      const google::protobuf::Map<uint32_t, ::vc_api::v1::IssueVerifiableCredentialResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_vc_api_v1_IssueVerifiableCredentialResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::vc_api::v1::IssueVerifiableCredentialResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::vc_api::v1::IssueVerifiableCredentialResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::vc_api::v1::IssueVerifiableCredentialResponse>* __rust_proto_thunk__Map_i64_vc_api_v1_IssueVerifiableCredentialResponse_new() {
    return new google::protobuf::Map<int64_t, ::vc_api::v1::IssueVerifiableCredentialResponse>();
  }
  void __rust_proto_thunk__Map_i64_vc_api_v1_IssueVerifiableCredentialResponse_free(const google::protobuf::Map<int64_t, ::vc_api::v1::IssueVerifiableCredentialResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_vc_api_v1_IssueVerifiableCredentialResponse_clear(google::protobuf::Map<int64_t, ::vc_api::v1::IssueVerifiableCredentialResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_vc_api_v1_IssueVerifiableCredentialResponse_size(const google::protobuf::Map<int64_t, ::vc_api::v1::IssueVerifiableCredentialResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_vc_api_v1_IssueVerifiableCredentialResponse_insert(google::protobuf::Map<int64_t, ::vc_api::v1::IssueVerifiableCredentialResponse> * m,
                          int64_t key, ::vc_api::v1::IssueVerifiableCredentialResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_vc_api_v1_IssueVerifiableCredentialResponse_get(const google::protobuf::Map<int64_t, ::vc_api::v1::IssueVerifiableCredentialResponse>* m,
                       int64_t key, const ::vc_api::v1::IssueVerifiableCredentialResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::IssueVerifiableCredentialResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_vc_api_v1_IssueVerifiableCredentialResponse_remove(google::protobuf::Map<int64_t, ::vc_api::v1::IssueVerifiableCredentialResponse> * m,
                          int64_t key, ::vc_api::v1::IssueVerifiableCredentialResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_vc_api_v1_IssueVerifiableCredentialResponse_iter(
      const google::protobuf::Map<int64_t, ::vc_api::v1::IssueVerifiableCredentialResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_vc_api_v1_IssueVerifiableCredentialResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::vc_api::v1::IssueVerifiableCredentialResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::vc_api::v1::IssueVerifiableCredentialResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::vc_api::v1::IssueVerifiableCredentialResponse>* __rust_proto_thunk__Map_u64_vc_api_v1_IssueVerifiableCredentialResponse_new() {
    return new google::protobuf::Map<uint64_t, ::vc_api::v1::IssueVerifiableCredentialResponse>();
  }
  void __rust_proto_thunk__Map_u64_vc_api_v1_IssueVerifiableCredentialResponse_free(const google::protobuf::Map<uint64_t, ::vc_api::v1::IssueVerifiableCredentialResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_vc_api_v1_IssueVerifiableCredentialResponse_clear(google::protobuf::Map<uint64_t, ::vc_api::v1::IssueVerifiableCredentialResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_vc_api_v1_IssueVerifiableCredentialResponse_size(const google::protobuf::Map<uint64_t, ::vc_api::v1::IssueVerifiableCredentialResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_vc_api_v1_IssueVerifiableCredentialResponse_insert(google::protobuf::Map<uint64_t, ::vc_api::v1::IssueVerifiableCredentialResponse> * m,
                          uint64_t key, ::vc_api::v1::IssueVerifiableCredentialResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_vc_api_v1_IssueVerifiableCredentialResponse_get(const google::protobuf::Map<uint64_t, ::vc_api::v1::IssueVerifiableCredentialResponse>* m,
                       uint64_t key, const ::vc_api::v1::IssueVerifiableCredentialResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::IssueVerifiableCredentialResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_vc_api_v1_IssueVerifiableCredentialResponse_remove(google::protobuf::Map<uint64_t, ::vc_api::v1::IssueVerifiableCredentialResponse> * m,
                          uint64_t key, ::vc_api::v1::IssueVerifiableCredentialResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_vc_api_v1_IssueVerifiableCredentialResponse_iter(
      const google::protobuf::Map<uint64_t, ::vc_api::v1::IssueVerifiableCredentialResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_vc_api_v1_IssueVerifiableCredentialResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::vc_api::v1::IssueVerifiableCredentialResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::vc_api::v1::IssueVerifiableCredentialResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::vc_api::v1::IssueVerifiableCredentialResponse>* __rust_proto_thunk__Map_bool_vc_api_v1_IssueVerifiableCredentialResponse_new() {
    return new google::protobuf::Map<bool, ::vc_api::v1::IssueVerifiableCredentialResponse>();
  }
  void __rust_proto_thunk__Map_bool_vc_api_v1_IssueVerifiableCredentialResponse_free(const google::protobuf::Map<bool, ::vc_api::v1::IssueVerifiableCredentialResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_vc_api_v1_IssueVerifiableCredentialResponse_clear(google::protobuf::Map<bool, ::vc_api::v1::IssueVerifiableCredentialResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_vc_api_v1_IssueVerifiableCredentialResponse_size(const google::protobuf::Map<bool, ::vc_api::v1::IssueVerifiableCredentialResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_vc_api_v1_IssueVerifiableCredentialResponse_insert(google::protobuf::Map<bool, ::vc_api::v1::IssueVerifiableCredentialResponse> * m,
                          bool key, ::vc_api::v1::IssueVerifiableCredentialResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_vc_api_v1_IssueVerifiableCredentialResponse_get(const google::protobuf::Map<bool, ::vc_api::v1::IssueVerifiableCredentialResponse>* m,
                       bool key, const ::vc_api::v1::IssueVerifiableCredentialResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::IssueVerifiableCredentialResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_vc_api_v1_IssueVerifiableCredentialResponse_remove(google::protobuf::Map<bool, ::vc_api::v1::IssueVerifiableCredentialResponse> * m,
                          bool key, ::vc_api::v1::IssueVerifiableCredentialResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_vc_api_v1_IssueVerifiableCredentialResponse_iter(
      const google::protobuf::Map<bool, ::vc_api::v1::IssueVerifiableCredentialResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_vc_api_v1_IssueVerifiableCredentialResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::vc_api::v1::IssueVerifiableCredentialResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::vc_api::v1::IssueVerifiableCredentialResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::vc_api::v1::IssueVerifiableCredentialResponse>* __rust_proto_thunk__Map_ProtoStr_vc_api_v1_IssueVerifiableCredentialResponse_new() {
    return new google::protobuf::Map<std::string, ::vc_api::v1::IssueVerifiableCredentialResponse>();
  }
  void __rust_proto_thunk__Map_ProtoStr_vc_api_v1_IssueVerifiableCredentialResponse_free(const google::protobuf::Map<std::string, ::vc_api::v1::IssueVerifiableCredentialResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_vc_api_v1_IssueVerifiableCredentialResponse_clear(google::protobuf::Map<std::string, ::vc_api::v1::IssueVerifiableCredentialResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_vc_api_v1_IssueVerifiableCredentialResponse_size(const google::protobuf::Map<std::string, ::vc_api::v1::IssueVerifiableCredentialResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_vc_api_v1_IssueVerifiableCredentialResponse_insert(google::protobuf::Map<std::string, ::vc_api::v1::IssueVerifiableCredentialResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::vc_api::v1::IssueVerifiableCredentialResponse value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_vc_api_v1_IssueVerifiableCredentialResponse_get(const google::protobuf::Map<std::string, ::vc_api::v1::IssueVerifiableCredentialResponse>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::vc_api::v1::IssueVerifiableCredentialResponse** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::IssueVerifiableCredentialResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_vc_api_v1_IssueVerifiableCredentialResponse_remove(google::protobuf::Map<std::string, ::vc_api::v1::IssueVerifiableCredentialResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::vc_api::v1::IssueVerifiableCredentialResponse * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_vc_api_v1_IssueVerifiableCredentialResponse_iter(
      const google::protobuf::Map<std::string, ::vc_api::v1::IssueVerifiableCredentialResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_vc_api_v1_IssueVerifiableCredentialResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::vc_api::v1::IssueVerifiableCredentialResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::vc_api::v1::IssueVerifiableCredentialResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// vc_api.v1.CreatePresentationChallengeRequest
        // clang-format off
extern "C" {
void* __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeRequest_new() { return new ::vc_api::v1::CreatePresentationChallengeRequest(); }
void __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeRequest_delete(void* ptr) { delete static_cast<::vc_api::v1::CreatePresentationChallengeRequest*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeRequest_serialize(::vc_api::v1::CreatePresentationChallengeRequest* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeRequest_parse(::vc_api::v1::CreatePresentationChallengeRequest* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeRequest_copy_from(::vc_api::v1::CreatePresentationChallengeRequest* dst, const ::vc_api::v1::CreatePresentationChallengeRequest* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeRequest_repeated_len(google::protobuf::RepeatedPtrField<::vc_api::v1::CreatePresentationChallengeRequest>* field) {
  return field->size();
}
const ::vc_api::v1::CreatePresentationChallengeRequest& __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeRequest_repeated_get(
  google::protobuf::RepeatedPtrField<::vc_api::v1::CreatePresentationChallengeRequest>* field,
  size_t index) {
  return field->Get(index);
}
::vc_api::v1::CreatePresentationChallengeRequest* __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeRequest_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::vc_api::v1::CreatePresentationChallengeRequest>* field,
  size_t index) {
  return field->Mutable(index);
}
::vc_api::v1::CreatePresentationChallengeRequest* __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeRequest_repeated_add(google::protobuf::RepeatedPtrField<::vc_api::v1::CreatePresentationChallengeRequest>* field) {
  return field->Add();
}
void __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeRequest_repeated_clear(google::protobuf::RepeatedPtrField<::vc_api::v1::CreatePresentationChallengeRequest>* field) {
  field->Clear();
}
void __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeRequest_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::vc_api::v1::CreatePresentationChallengeRequest>& dst,
  const google::protobuf::RepeatedPtrField<::vc_api::v1::CreatePresentationChallengeRequest>& src) {
  dst = src;
}

void __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeRequest_clear_credential_types(::vc_api::v1::CreatePresentationChallengeRequest* msg) {
  msg->clear_credential_types();
}
google::protobuf::RepeatedField<int>* __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeRequest_get_mut_credential_types(::vc_api::v1::CreatePresentationChallengeRequest* msg) {
  return msg->mutable_credential_types();
}
const google::protobuf::RepeatedField<int>* __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeRequest_get_credential_types(
    const ::vc_api::v1::CreatePresentationChallengeRequest* msg) {
  return &msg->credential_types();
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::vc_api::v1::CreatePresentationChallengeRequest>* __rust_proto_thunk__Map_i32_vc_api_v1_CreatePresentationChallengeRequest_new() {
    return new google::protobuf::Map<int32_t, ::vc_api::v1::CreatePresentationChallengeRequest>();
  }
  void __rust_proto_thunk__Map_i32_vc_api_v1_CreatePresentationChallengeRequest_free(const google::protobuf::Map<int32_t, ::vc_api::v1::CreatePresentationChallengeRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_vc_api_v1_CreatePresentationChallengeRequest_clear(google::protobuf::Map<int32_t, ::vc_api::v1::CreatePresentationChallengeRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_vc_api_v1_CreatePresentationChallengeRequest_size(const google::protobuf::Map<int32_t, ::vc_api::v1::CreatePresentationChallengeRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_vc_api_v1_CreatePresentationChallengeRequest_insert(google::protobuf::Map<int32_t, ::vc_api::v1::CreatePresentationChallengeRequest> * m,
                          int32_t key, ::vc_api::v1::CreatePresentationChallengeRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_vc_api_v1_CreatePresentationChallengeRequest_get(const google::protobuf::Map<int32_t, ::vc_api::v1::CreatePresentationChallengeRequest>* m,
                       int32_t key, const ::vc_api::v1::CreatePresentationChallengeRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::CreatePresentationChallengeRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_vc_api_v1_CreatePresentationChallengeRequest_remove(google::protobuf::Map<int32_t, ::vc_api::v1::CreatePresentationChallengeRequest> * m,
                          int32_t key, ::vc_api::v1::CreatePresentationChallengeRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_vc_api_v1_CreatePresentationChallengeRequest_iter(
      const google::protobuf::Map<int32_t, ::vc_api::v1::CreatePresentationChallengeRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_vc_api_v1_CreatePresentationChallengeRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::vc_api::v1::CreatePresentationChallengeRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::vc_api::v1::CreatePresentationChallengeRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::vc_api::v1::CreatePresentationChallengeRequest>* __rust_proto_thunk__Map_u32_vc_api_v1_CreatePresentationChallengeRequest_new() {
    return new google::protobuf::Map<uint32_t, ::vc_api::v1::CreatePresentationChallengeRequest>();
  }
  void __rust_proto_thunk__Map_u32_vc_api_v1_CreatePresentationChallengeRequest_free(const google::protobuf::Map<uint32_t, ::vc_api::v1::CreatePresentationChallengeRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_vc_api_v1_CreatePresentationChallengeRequest_clear(google::protobuf::Map<uint32_t, ::vc_api::v1::CreatePresentationChallengeRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_vc_api_v1_CreatePresentationChallengeRequest_size(const google::protobuf::Map<uint32_t, ::vc_api::v1::CreatePresentationChallengeRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_vc_api_v1_CreatePresentationChallengeRequest_insert(google::protobuf::Map<uint32_t, ::vc_api::v1::CreatePresentationChallengeRequest> * m,
                          uint32_t key, ::vc_api::v1::CreatePresentationChallengeRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_vc_api_v1_CreatePresentationChallengeRequest_get(const google::protobuf::Map<uint32_t, ::vc_api::v1::CreatePresentationChallengeRequest>* m,
                       uint32_t key, const ::vc_api::v1::CreatePresentationChallengeRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::CreatePresentationChallengeRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_vc_api_v1_CreatePresentationChallengeRequest_remove(google::protobuf::Map<uint32_t, ::vc_api::v1::CreatePresentationChallengeRequest> * m,
                          uint32_t key, ::vc_api::v1::CreatePresentationChallengeRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_vc_api_v1_CreatePresentationChallengeRequest_iter(
      const google::protobuf::Map<uint32_t, ::vc_api::v1::CreatePresentationChallengeRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_vc_api_v1_CreatePresentationChallengeRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::vc_api::v1::CreatePresentationChallengeRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::vc_api::v1::CreatePresentationChallengeRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::vc_api::v1::CreatePresentationChallengeRequest>* __rust_proto_thunk__Map_i64_vc_api_v1_CreatePresentationChallengeRequest_new() {
    return new google::protobuf::Map<int64_t, ::vc_api::v1::CreatePresentationChallengeRequest>();
  }
  void __rust_proto_thunk__Map_i64_vc_api_v1_CreatePresentationChallengeRequest_free(const google::protobuf::Map<int64_t, ::vc_api::v1::CreatePresentationChallengeRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_vc_api_v1_CreatePresentationChallengeRequest_clear(google::protobuf::Map<int64_t, ::vc_api::v1::CreatePresentationChallengeRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_vc_api_v1_CreatePresentationChallengeRequest_size(const google::protobuf::Map<int64_t, ::vc_api::v1::CreatePresentationChallengeRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_vc_api_v1_CreatePresentationChallengeRequest_insert(google::protobuf::Map<int64_t, ::vc_api::v1::CreatePresentationChallengeRequest> * m,
                          int64_t key, ::vc_api::v1::CreatePresentationChallengeRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_vc_api_v1_CreatePresentationChallengeRequest_get(const google::protobuf::Map<int64_t, ::vc_api::v1::CreatePresentationChallengeRequest>* m,
                       int64_t key, const ::vc_api::v1::CreatePresentationChallengeRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::CreatePresentationChallengeRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_vc_api_v1_CreatePresentationChallengeRequest_remove(google::protobuf::Map<int64_t, ::vc_api::v1::CreatePresentationChallengeRequest> * m,
                          int64_t key, ::vc_api::v1::CreatePresentationChallengeRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_vc_api_v1_CreatePresentationChallengeRequest_iter(
      const google::protobuf::Map<int64_t, ::vc_api::v1::CreatePresentationChallengeRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_vc_api_v1_CreatePresentationChallengeRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::vc_api::v1::CreatePresentationChallengeRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::vc_api::v1::CreatePresentationChallengeRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::vc_api::v1::CreatePresentationChallengeRequest>* __rust_proto_thunk__Map_u64_vc_api_v1_CreatePresentationChallengeRequest_new() {
    return new google::protobuf::Map<uint64_t, ::vc_api::v1::CreatePresentationChallengeRequest>();
  }
  void __rust_proto_thunk__Map_u64_vc_api_v1_CreatePresentationChallengeRequest_free(const google::protobuf::Map<uint64_t, ::vc_api::v1::CreatePresentationChallengeRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_vc_api_v1_CreatePresentationChallengeRequest_clear(google::protobuf::Map<uint64_t, ::vc_api::v1::CreatePresentationChallengeRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_vc_api_v1_CreatePresentationChallengeRequest_size(const google::protobuf::Map<uint64_t, ::vc_api::v1::CreatePresentationChallengeRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_vc_api_v1_CreatePresentationChallengeRequest_insert(google::protobuf::Map<uint64_t, ::vc_api::v1::CreatePresentationChallengeRequest> * m,
                          uint64_t key, ::vc_api::v1::CreatePresentationChallengeRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_vc_api_v1_CreatePresentationChallengeRequest_get(const google::protobuf::Map<uint64_t, ::vc_api::v1::CreatePresentationChallengeRequest>* m,
                       uint64_t key, const ::vc_api::v1::CreatePresentationChallengeRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::CreatePresentationChallengeRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_vc_api_v1_CreatePresentationChallengeRequest_remove(google::protobuf::Map<uint64_t, ::vc_api::v1::CreatePresentationChallengeRequest> * m,
                          uint64_t key, ::vc_api::v1::CreatePresentationChallengeRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_vc_api_v1_CreatePresentationChallengeRequest_iter(
      const google::protobuf::Map<uint64_t, ::vc_api::v1::CreatePresentationChallengeRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_vc_api_v1_CreatePresentationChallengeRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::vc_api::v1::CreatePresentationChallengeRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::vc_api::v1::CreatePresentationChallengeRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::vc_api::v1::CreatePresentationChallengeRequest>* __rust_proto_thunk__Map_bool_vc_api_v1_CreatePresentationChallengeRequest_new() {
    return new google::protobuf::Map<bool, ::vc_api::v1::CreatePresentationChallengeRequest>();
  }
  void __rust_proto_thunk__Map_bool_vc_api_v1_CreatePresentationChallengeRequest_free(const google::protobuf::Map<bool, ::vc_api::v1::CreatePresentationChallengeRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_vc_api_v1_CreatePresentationChallengeRequest_clear(google::protobuf::Map<bool, ::vc_api::v1::CreatePresentationChallengeRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_vc_api_v1_CreatePresentationChallengeRequest_size(const google::protobuf::Map<bool, ::vc_api::v1::CreatePresentationChallengeRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_vc_api_v1_CreatePresentationChallengeRequest_insert(google::protobuf::Map<bool, ::vc_api::v1::CreatePresentationChallengeRequest> * m,
                          bool key, ::vc_api::v1::CreatePresentationChallengeRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_vc_api_v1_CreatePresentationChallengeRequest_get(const google::protobuf::Map<bool, ::vc_api::v1::CreatePresentationChallengeRequest>* m,
                       bool key, const ::vc_api::v1::CreatePresentationChallengeRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::CreatePresentationChallengeRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_vc_api_v1_CreatePresentationChallengeRequest_remove(google::protobuf::Map<bool, ::vc_api::v1::CreatePresentationChallengeRequest> * m,
                          bool key, ::vc_api::v1::CreatePresentationChallengeRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_vc_api_v1_CreatePresentationChallengeRequest_iter(
      const google::protobuf::Map<bool, ::vc_api::v1::CreatePresentationChallengeRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_vc_api_v1_CreatePresentationChallengeRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::vc_api::v1::CreatePresentationChallengeRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::vc_api::v1::CreatePresentationChallengeRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::vc_api::v1::CreatePresentationChallengeRequest>* __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreatePresentationChallengeRequest_new() {
    return new google::protobuf::Map<std::string, ::vc_api::v1::CreatePresentationChallengeRequest>();
  }
  void __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreatePresentationChallengeRequest_free(const google::protobuf::Map<std::string, ::vc_api::v1::CreatePresentationChallengeRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreatePresentationChallengeRequest_clear(google::protobuf::Map<std::string, ::vc_api::v1::CreatePresentationChallengeRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreatePresentationChallengeRequest_size(const google::protobuf::Map<std::string, ::vc_api::v1::CreatePresentationChallengeRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreatePresentationChallengeRequest_insert(google::protobuf::Map<std::string, ::vc_api::v1::CreatePresentationChallengeRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::vc_api::v1::CreatePresentationChallengeRequest value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreatePresentationChallengeRequest_get(const google::protobuf::Map<std::string, ::vc_api::v1::CreatePresentationChallengeRequest>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::vc_api::v1::CreatePresentationChallengeRequest** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::CreatePresentationChallengeRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreatePresentationChallengeRequest_remove(google::protobuf::Map<std::string, ::vc_api::v1::CreatePresentationChallengeRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::vc_api::v1::CreatePresentationChallengeRequest * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreatePresentationChallengeRequest_iter(
      const google::protobuf::Map<std::string, ::vc_api::v1::CreatePresentationChallengeRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreatePresentationChallengeRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::vc_api::v1::CreatePresentationChallengeRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::vc_api::v1::CreatePresentationChallengeRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// vc_api.v1.CreatePresentationChallengeResponse
        // clang-format off
extern "C" {
void* __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeResponse_new() { return new ::vc_api::v1::CreatePresentationChallengeResponse(); }
void __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeResponse_delete(void* ptr) { delete static_cast<::vc_api::v1::CreatePresentationChallengeResponse*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeResponse_serialize(::vc_api::v1::CreatePresentationChallengeResponse* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeResponse_parse(::vc_api::v1::CreatePresentationChallengeResponse* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeResponse_copy_from(::vc_api::v1::CreatePresentationChallengeResponse* dst, const ::vc_api::v1::CreatePresentationChallengeResponse* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeResponse_repeated_len(google::protobuf::RepeatedPtrField<::vc_api::v1::CreatePresentationChallengeResponse>* field) {
  return field->size();
}
const ::vc_api::v1::CreatePresentationChallengeResponse& __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeResponse_repeated_get(
  google::protobuf::RepeatedPtrField<::vc_api::v1::CreatePresentationChallengeResponse>* field,
  size_t index) {
  return field->Get(index);
}
::vc_api::v1::CreatePresentationChallengeResponse* __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeResponse_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::vc_api::v1::CreatePresentationChallengeResponse>* field,
  size_t index) {
  return field->Mutable(index);
}
::vc_api::v1::CreatePresentationChallengeResponse* __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeResponse_repeated_add(google::protobuf::RepeatedPtrField<::vc_api::v1::CreatePresentationChallengeResponse>* field) {
  return field->Add();
}
void __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeResponse_repeated_clear(google::protobuf::RepeatedPtrField<::vc_api::v1::CreatePresentationChallengeResponse>* field) {
  field->Clear();
}
void __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeResponse_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::vc_api::v1::CreatePresentationChallengeResponse>& dst,
  const google::protobuf::RepeatedPtrField<::vc_api::v1::CreatePresentationChallengeResponse>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeResponse_get_nonce(::vc_api::v1::CreatePresentationChallengeResponse* msg) {
  absl::string_view val = msg->nonce();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeResponse_set_nonce(::vc_api::v1::CreatePresentationChallengeResponse* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_nonce(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeResponse_get_endpoint(::vc_api::v1::CreatePresentationChallengeResponse* msg) {
  absl::string_view val = msg->endpoint();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeResponse_set_endpoint(::vc_api::v1::CreatePresentationChallengeResponse* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_endpoint(absl::string_view(s.ptr, s.len));
}

void __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeResponse_clear_credential_types(::vc_api::v1::CreatePresentationChallengeResponse* msg) {
  msg->clear_credential_types();
}
google::protobuf::RepeatedField<int>* __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeResponse_get_mut_credential_types(::vc_api::v1::CreatePresentationChallengeResponse* msg) {
  return msg->mutable_credential_types();
}
const google::protobuf::RepeatedField<int>* __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeResponse_get_credential_types(
    const ::vc_api::v1::CreatePresentationChallengeResponse* msg) {
  return &msg->credential_types();
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::vc_api::v1::CreatePresentationChallengeResponse>* __rust_proto_thunk__Map_i32_vc_api_v1_CreatePresentationChallengeResponse_new() {
    return new google::protobuf::Map<int32_t, ::vc_api::v1::CreatePresentationChallengeResponse>();
  }
  void __rust_proto_thunk__Map_i32_vc_api_v1_CreatePresentationChallengeResponse_free(const google::protobuf::Map<int32_t, ::vc_api::v1::CreatePresentationChallengeResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_vc_api_v1_CreatePresentationChallengeResponse_clear(google::protobuf::Map<int32_t, ::vc_api::v1::CreatePresentationChallengeResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_vc_api_v1_CreatePresentationChallengeResponse_size(const google::protobuf::Map<int32_t, ::vc_api::v1::CreatePresentationChallengeResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_vc_api_v1_CreatePresentationChallengeResponse_insert(google::protobuf::Map<int32_t, ::vc_api::v1::CreatePresentationChallengeResponse> * m,
                          int32_t key, ::vc_api::v1::CreatePresentationChallengeResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_vc_api_v1_CreatePresentationChallengeResponse_get(const google::protobuf::Map<int32_t, ::vc_api::v1::CreatePresentationChallengeResponse>* m,
                       int32_t key, const ::vc_api::v1::CreatePresentationChallengeResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::CreatePresentationChallengeResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_vc_api_v1_CreatePresentationChallengeResponse_remove(google::protobuf::Map<int32_t, ::vc_api::v1::CreatePresentationChallengeResponse> * m,
                          int32_t key, ::vc_api::v1::CreatePresentationChallengeResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_vc_api_v1_CreatePresentationChallengeResponse_iter(
      const google::protobuf::Map<int32_t, ::vc_api::v1::CreatePresentationChallengeResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_vc_api_v1_CreatePresentationChallengeResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::vc_api::v1::CreatePresentationChallengeResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::vc_api::v1::CreatePresentationChallengeResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::vc_api::v1::CreatePresentationChallengeResponse>* __rust_proto_thunk__Map_u32_vc_api_v1_CreatePresentationChallengeResponse_new() {
    return new google::protobuf::Map<uint32_t, ::vc_api::v1::CreatePresentationChallengeResponse>();
  }
  void __rust_proto_thunk__Map_u32_vc_api_v1_CreatePresentationChallengeResponse_free(const google::protobuf::Map<uint32_t, ::vc_api::v1::CreatePresentationChallengeResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_vc_api_v1_CreatePresentationChallengeResponse_clear(google::protobuf::Map<uint32_t, ::vc_api::v1::CreatePresentationChallengeResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_vc_api_v1_CreatePresentationChallengeResponse_size(const google::protobuf::Map<uint32_t, ::vc_api::v1::CreatePresentationChallengeResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_vc_api_v1_CreatePresentationChallengeResponse_insert(google::protobuf::Map<uint32_t, ::vc_api::v1::CreatePresentationChallengeResponse> * m,
                          uint32_t key, ::vc_api::v1::CreatePresentationChallengeResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_vc_api_v1_CreatePresentationChallengeResponse_get(const google::protobuf::Map<uint32_t, ::vc_api::v1::CreatePresentationChallengeResponse>* m,
                       uint32_t key, const ::vc_api::v1::CreatePresentationChallengeResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::CreatePresentationChallengeResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_vc_api_v1_CreatePresentationChallengeResponse_remove(google::protobuf::Map<uint32_t, ::vc_api::v1::CreatePresentationChallengeResponse> * m,
                          uint32_t key, ::vc_api::v1::CreatePresentationChallengeResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_vc_api_v1_CreatePresentationChallengeResponse_iter(
      const google::protobuf::Map<uint32_t, ::vc_api::v1::CreatePresentationChallengeResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_vc_api_v1_CreatePresentationChallengeResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::vc_api::v1::CreatePresentationChallengeResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::vc_api::v1::CreatePresentationChallengeResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::vc_api::v1::CreatePresentationChallengeResponse>* __rust_proto_thunk__Map_i64_vc_api_v1_CreatePresentationChallengeResponse_new() {
    return new google::protobuf::Map<int64_t, ::vc_api::v1::CreatePresentationChallengeResponse>();
  }
  void __rust_proto_thunk__Map_i64_vc_api_v1_CreatePresentationChallengeResponse_free(const google::protobuf::Map<int64_t, ::vc_api::v1::CreatePresentationChallengeResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_vc_api_v1_CreatePresentationChallengeResponse_clear(google::protobuf::Map<int64_t, ::vc_api::v1::CreatePresentationChallengeResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_vc_api_v1_CreatePresentationChallengeResponse_size(const google::protobuf::Map<int64_t, ::vc_api::v1::CreatePresentationChallengeResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_vc_api_v1_CreatePresentationChallengeResponse_insert(google::protobuf::Map<int64_t, ::vc_api::v1::CreatePresentationChallengeResponse> * m,
                          int64_t key, ::vc_api::v1::CreatePresentationChallengeResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_vc_api_v1_CreatePresentationChallengeResponse_get(const google::protobuf::Map<int64_t, ::vc_api::v1::CreatePresentationChallengeResponse>* m,
                       int64_t key, const ::vc_api::v1::CreatePresentationChallengeResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::CreatePresentationChallengeResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_vc_api_v1_CreatePresentationChallengeResponse_remove(google::protobuf::Map<int64_t, ::vc_api::v1::CreatePresentationChallengeResponse> * m,
                          int64_t key, ::vc_api::v1::CreatePresentationChallengeResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_vc_api_v1_CreatePresentationChallengeResponse_iter(
      const google::protobuf::Map<int64_t, ::vc_api::v1::CreatePresentationChallengeResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_vc_api_v1_CreatePresentationChallengeResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::vc_api::v1::CreatePresentationChallengeResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::vc_api::v1::CreatePresentationChallengeResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::vc_api::v1::CreatePresentationChallengeResponse>* __rust_proto_thunk__Map_u64_vc_api_v1_CreatePresentationChallengeResponse_new() {
    return new google::protobuf::Map<uint64_t, ::vc_api::v1::CreatePresentationChallengeResponse>();
  }
  void __rust_proto_thunk__Map_u64_vc_api_v1_CreatePresentationChallengeResponse_free(const google::protobuf::Map<uint64_t, ::vc_api::v1::CreatePresentationChallengeResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_vc_api_v1_CreatePresentationChallengeResponse_clear(google::protobuf::Map<uint64_t, ::vc_api::v1::CreatePresentationChallengeResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_vc_api_v1_CreatePresentationChallengeResponse_size(const google::protobuf::Map<uint64_t, ::vc_api::v1::CreatePresentationChallengeResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_vc_api_v1_CreatePresentationChallengeResponse_insert(google::protobuf::Map<uint64_t, ::vc_api::v1::CreatePresentationChallengeResponse> * m,
                          uint64_t key, ::vc_api::v1::CreatePresentationChallengeResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_vc_api_v1_CreatePresentationChallengeResponse_get(const google::protobuf::Map<uint64_t, ::vc_api::v1::CreatePresentationChallengeResponse>* m,
                       uint64_t key, const ::vc_api::v1::CreatePresentationChallengeResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::CreatePresentationChallengeResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_vc_api_v1_CreatePresentationChallengeResponse_remove(google::protobuf::Map<uint64_t, ::vc_api::v1::CreatePresentationChallengeResponse> * m,
                          uint64_t key, ::vc_api::v1::CreatePresentationChallengeResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_vc_api_v1_CreatePresentationChallengeResponse_iter(
      const google::protobuf::Map<uint64_t, ::vc_api::v1::CreatePresentationChallengeResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_vc_api_v1_CreatePresentationChallengeResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::vc_api::v1::CreatePresentationChallengeResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::vc_api::v1::CreatePresentationChallengeResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::vc_api::v1::CreatePresentationChallengeResponse>* __rust_proto_thunk__Map_bool_vc_api_v1_CreatePresentationChallengeResponse_new() {
    return new google::protobuf::Map<bool, ::vc_api::v1::CreatePresentationChallengeResponse>();
  }
  void __rust_proto_thunk__Map_bool_vc_api_v1_CreatePresentationChallengeResponse_free(const google::protobuf::Map<bool, ::vc_api::v1::CreatePresentationChallengeResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_vc_api_v1_CreatePresentationChallengeResponse_clear(google::protobuf::Map<bool, ::vc_api::v1::CreatePresentationChallengeResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_vc_api_v1_CreatePresentationChallengeResponse_size(const google::protobuf::Map<bool, ::vc_api::v1::CreatePresentationChallengeResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_vc_api_v1_CreatePresentationChallengeResponse_insert(google::protobuf::Map<bool, ::vc_api::v1::CreatePresentationChallengeResponse> * m,
                          bool key, ::vc_api::v1::CreatePresentationChallengeResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_vc_api_v1_CreatePresentationChallengeResponse_get(const google::protobuf::Map<bool, ::vc_api::v1::CreatePresentationChallengeResponse>* m,
                       bool key, const ::vc_api::v1::CreatePresentationChallengeResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::CreatePresentationChallengeResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_vc_api_v1_CreatePresentationChallengeResponse_remove(google::protobuf::Map<bool, ::vc_api::v1::CreatePresentationChallengeResponse> * m,
                          bool key, ::vc_api::v1::CreatePresentationChallengeResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_vc_api_v1_CreatePresentationChallengeResponse_iter(
      const google::protobuf::Map<bool, ::vc_api::v1::CreatePresentationChallengeResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_vc_api_v1_CreatePresentationChallengeResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::vc_api::v1::CreatePresentationChallengeResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::vc_api::v1::CreatePresentationChallengeResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::vc_api::v1::CreatePresentationChallengeResponse>* __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreatePresentationChallengeResponse_new() {
    return new google::protobuf::Map<std::string, ::vc_api::v1::CreatePresentationChallengeResponse>();
  }
  void __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreatePresentationChallengeResponse_free(const google::protobuf::Map<std::string, ::vc_api::v1::CreatePresentationChallengeResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreatePresentationChallengeResponse_clear(google::protobuf::Map<std::string, ::vc_api::v1::CreatePresentationChallengeResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreatePresentationChallengeResponse_size(const google::protobuf::Map<std::string, ::vc_api::v1::CreatePresentationChallengeResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreatePresentationChallengeResponse_insert(google::protobuf::Map<std::string, ::vc_api::v1::CreatePresentationChallengeResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::vc_api::v1::CreatePresentationChallengeResponse value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreatePresentationChallengeResponse_get(const google::protobuf::Map<std::string, ::vc_api::v1::CreatePresentationChallengeResponse>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::vc_api::v1::CreatePresentationChallengeResponse** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::CreatePresentationChallengeResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreatePresentationChallengeResponse_remove(google::protobuf::Map<std::string, ::vc_api::v1::CreatePresentationChallengeResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::vc_api::v1::CreatePresentationChallengeResponse * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreatePresentationChallengeResponse_iter(
      const google::protobuf::Map<std::string, ::vc_api::v1::CreatePresentationChallengeResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreatePresentationChallengeResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::vc_api::v1::CreatePresentationChallengeResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::vc_api::v1::CreatePresentationChallengeResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// vc_api.v1.PresentVerifiableCredentialRequest
        // clang-format off
extern "C" {
void* __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_new() { return new ::vc_api::v1::PresentVerifiableCredentialRequest(); }
void __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_delete(void* ptr) { delete static_cast<::vc_api::v1::PresentVerifiableCredentialRequest*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_serialize(::vc_api::v1::PresentVerifiableCredentialRequest* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_parse(::vc_api::v1::PresentVerifiableCredentialRequest* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_copy_from(::vc_api::v1::PresentVerifiableCredentialRequest* dst, const ::vc_api::v1::PresentVerifiableCredentialRequest* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_repeated_len(google::protobuf::RepeatedPtrField<::vc_api::v1::PresentVerifiableCredentialRequest>* field) {
  return field->size();
}
const ::vc_api::v1::PresentVerifiableCredentialRequest& __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_repeated_get(
  google::protobuf::RepeatedPtrField<::vc_api::v1::PresentVerifiableCredentialRequest>* field,
  size_t index) {
  return field->Get(index);
}
::vc_api::v1::PresentVerifiableCredentialRequest* __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::vc_api::v1::PresentVerifiableCredentialRequest>* field,
  size_t index) {
  return field->Mutable(index);
}
::vc_api::v1::PresentVerifiableCredentialRequest* __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_repeated_add(google::protobuf::RepeatedPtrField<::vc_api::v1::PresentVerifiableCredentialRequest>* field) {
  return field->Add();
}
void __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_repeated_clear(google::protobuf::RepeatedPtrField<::vc_api::v1::PresentVerifiableCredentialRequest>* field) {
  field->Clear();
}
void __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::vc_api::v1::PresentVerifiableCredentialRequest>& dst,
  const google::protobuf::RepeatedPtrField<::vc_api::v1::PresentVerifiableCredentialRequest>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_get_presentation(::vc_api::v1::PresentVerifiableCredentialRequest* msg) {
  absl::string_view val = msg->presentation();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_set_presentation(::vc_api::v1::PresentVerifiableCredentialRequest* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_presentation(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_get_nonce(::vc_api::v1::PresentVerifiableCredentialRequest* msg) {
  absl::string_view val = msg->nonce();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_set_nonce(::vc_api::v1::PresentVerifiableCredentialRequest* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_nonce(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_get_signature(::vc_api::v1::PresentVerifiableCredentialRequest* msg) {
  absl::string_view val = msg->signature();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_set_signature(::vc_api::v1::PresentVerifiableCredentialRequest* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_signature(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_get_did(::vc_api::v1::PresentVerifiableCredentialRequest* msg) {
  absl::string_view val = msg->did();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_set_did(::vc_api::v1::PresentVerifiableCredentialRequest* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_did(absl::string_view(s.ptr, s.len));
}

void __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_clear_credential_type(::vc_api::v1::PresentVerifiableCredentialRequest* msg) {
  msg->clear_credential_type();
}
google::protobuf::RepeatedField<int>* __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_get_mut_credential_type(::vc_api::v1::PresentVerifiableCredentialRequest* msg) {
  return msg->mutable_credential_type();
}
const google::protobuf::RepeatedField<int>* __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_get_credential_type(
    const ::vc_api::v1::PresentVerifiableCredentialRequest* msg) {
  return &msg->credential_type();
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::vc_api::v1::PresentVerifiableCredentialRequest>* __rust_proto_thunk__Map_i32_vc_api_v1_PresentVerifiableCredentialRequest_new() {
    return new google::protobuf::Map<int32_t, ::vc_api::v1::PresentVerifiableCredentialRequest>();
  }
  void __rust_proto_thunk__Map_i32_vc_api_v1_PresentVerifiableCredentialRequest_free(const google::protobuf::Map<int32_t, ::vc_api::v1::PresentVerifiableCredentialRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_vc_api_v1_PresentVerifiableCredentialRequest_clear(google::protobuf::Map<int32_t, ::vc_api::v1::PresentVerifiableCredentialRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_vc_api_v1_PresentVerifiableCredentialRequest_size(const google::protobuf::Map<int32_t, ::vc_api::v1::PresentVerifiableCredentialRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_vc_api_v1_PresentVerifiableCredentialRequest_insert(google::protobuf::Map<int32_t, ::vc_api::v1::PresentVerifiableCredentialRequest> * m,
                          int32_t key, ::vc_api::v1::PresentVerifiableCredentialRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_vc_api_v1_PresentVerifiableCredentialRequest_get(const google::protobuf::Map<int32_t, ::vc_api::v1::PresentVerifiableCredentialRequest>* m,
                       int32_t key, const ::vc_api::v1::PresentVerifiableCredentialRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::PresentVerifiableCredentialRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_vc_api_v1_PresentVerifiableCredentialRequest_remove(google::protobuf::Map<int32_t, ::vc_api::v1::PresentVerifiableCredentialRequest> * m,
                          int32_t key, ::vc_api::v1::PresentVerifiableCredentialRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_vc_api_v1_PresentVerifiableCredentialRequest_iter(
      const google::protobuf::Map<int32_t, ::vc_api::v1::PresentVerifiableCredentialRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_vc_api_v1_PresentVerifiableCredentialRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::vc_api::v1::PresentVerifiableCredentialRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::vc_api::v1::PresentVerifiableCredentialRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::vc_api::v1::PresentVerifiableCredentialRequest>* __rust_proto_thunk__Map_u32_vc_api_v1_PresentVerifiableCredentialRequest_new() {
    return new google::protobuf::Map<uint32_t, ::vc_api::v1::PresentVerifiableCredentialRequest>();
  }
  void __rust_proto_thunk__Map_u32_vc_api_v1_PresentVerifiableCredentialRequest_free(const google::protobuf::Map<uint32_t, ::vc_api::v1::PresentVerifiableCredentialRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_vc_api_v1_PresentVerifiableCredentialRequest_clear(google::protobuf::Map<uint32_t, ::vc_api::v1::PresentVerifiableCredentialRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_vc_api_v1_PresentVerifiableCredentialRequest_size(const google::protobuf::Map<uint32_t, ::vc_api::v1::PresentVerifiableCredentialRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_vc_api_v1_PresentVerifiableCredentialRequest_insert(google::protobuf::Map<uint32_t, ::vc_api::v1::PresentVerifiableCredentialRequest> * m,
                          uint32_t key, ::vc_api::v1::PresentVerifiableCredentialRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_vc_api_v1_PresentVerifiableCredentialRequest_get(const google::protobuf::Map<uint32_t, ::vc_api::v1::PresentVerifiableCredentialRequest>* m,
                       uint32_t key, const ::vc_api::v1::PresentVerifiableCredentialRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::PresentVerifiableCredentialRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_vc_api_v1_PresentVerifiableCredentialRequest_remove(google::protobuf::Map<uint32_t, ::vc_api::v1::PresentVerifiableCredentialRequest> * m,
                          uint32_t key, ::vc_api::v1::PresentVerifiableCredentialRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_vc_api_v1_PresentVerifiableCredentialRequest_iter(
      const google::protobuf::Map<uint32_t, ::vc_api::v1::PresentVerifiableCredentialRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_vc_api_v1_PresentVerifiableCredentialRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::vc_api::v1::PresentVerifiableCredentialRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::vc_api::v1::PresentVerifiableCredentialRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::vc_api::v1::PresentVerifiableCredentialRequest>* __rust_proto_thunk__Map_i64_vc_api_v1_PresentVerifiableCredentialRequest_new() {
    return new google::protobuf::Map<int64_t, ::vc_api::v1::PresentVerifiableCredentialRequest>();
  }
  void __rust_proto_thunk__Map_i64_vc_api_v1_PresentVerifiableCredentialRequest_free(const google::protobuf::Map<int64_t, ::vc_api::v1::PresentVerifiableCredentialRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_vc_api_v1_PresentVerifiableCredentialRequest_clear(google::protobuf::Map<int64_t, ::vc_api::v1::PresentVerifiableCredentialRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_vc_api_v1_PresentVerifiableCredentialRequest_size(const google::protobuf::Map<int64_t, ::vc_api::v1::PresentVerifiableCredentialRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_vc_api_v1_PresentVerifiableCredentialRequest_insert(google::protobuf::Map<int64_t, ::vc_api::v1::PresentVerifiableCredentialRequest> * m,
                          int64_t key, ::vc_api::v1::PresentVerifiableCredentialRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_vc_api_v1_PresentVerifiableCredentialRequest_get(const google::protobuf::Map<int64_t, ::vc_api::v1::PresentVerifiableCredentialRequest>* m,
                       int64_t key, const ::vc_api::v1::PresentVerifiableCredentialRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::PresentVerifiableCredentialRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_vc_api_v1_PresentVerifiableCredentialRequest_remove(google::protobuf::Map<int64_t, ::vc_api::v1::PresentVerifiableCredentialRequest> * m,
                          int64_t key, ::vc_api::v1::PresentVerifiableCredentialRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_vc_api_v1_PresentVerifiableCredentialRequest_iter(
      const google::protobuf::Map<int64_t, ::vc_api::v1::PresentVerifiableCredentialRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_vc_api_v1_PresentVerifiableCredentialRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::vc_api::v1::PresentVerifiableCredentialRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::vc_api::v1::PresentVerifiableCredentialRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::vc_api::v1::PresentVerifiableCredentialRequest>* __rust_proto_thunk__Map_u64_vc_api_v1_PresentVerifiableCredentialRequest_new() {
    return new google::protobuf::Map<uint64_t, ::vc_api::v1::PresentVerifiableCredentialRequest>();
  }
  void __rust_proto_thunk__Map_u64_vc_api_v1_PresentVerifiableCredentialRequest_free(const google::protobuf::Map<uint64_t, ::vc_api::v1::PresentVerifiableCredentialRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_vc_api_v1_PresentVerifiableCredentialRequest_clear(google::protobuf::Map<uint64_t, ::vc_api::v1::PresentVerifiableCredentialRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_vc_api_v1_PresentVerifiableCredentialRequest_size(const google::protobuf::Map<uint64_t, ::vc_api::v1::PresentVerifiableCredentialRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_vc_api_v1_PresentVerifiableCredentialRequest_insert(google::protobuf::Map<uint64_t, ::vc_api::v1::PresentVerifiableCredentialRequest> * m,
                          uint64_t key, ::vc_api::v1::PresentVerifiableCredentialRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_vc_api_v1_PresentVerifiableCredentialRequest_get(const google::protobuf::Map<uint64_t, ::vc_api::v1::PresentVerifiableCredentialRequest>* m,
                       uint64_t key, const ::vc_api::v1::PresentVerifiableCredentialRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::PresentVerifiableCredentialRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_vc_api_v1_PresentVerifiableCredentialRequest_remove(google::protobuf::Map<uint64_t, ::vc_api::v1::PresentVerifiableCredentialRequest> * m,
                          uint64_t key, ::vc_api::v1::PresentVerifiableCredentialRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_vc_api_v1_PresentVerifiableCredentialRequest_iter(
      const google::protobuf::Map<uint64_t, ::vc_api::v1::PresentVerifiableCredentialRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_vc_api_v1_PresentVerifiableCredentialRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::vc_api::v1::PresentVerifiableCredentialRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::vc_api::v1::PresentVerifiableCredentialRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::vc_api::v1::PresentVerifiableCredentialRequest>* __rust_proto_thunk__Map_bool_vc_api_v1_PresentVerifiableCredentialRequest_new() {
    return new google::protobuf::Map<bool, ::vc_api::v1::PresentVerifiableCredentialRequest>();
  }
  void __rust_proto_thunk__Map_bool_vc_api_v1_PresentVerifiableCredentialRequest_free(const google::protobuf::Map<bool, ::vc_api::v1::PresentVerifiableCredentialRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_vc_api_v1_PresentVerifiableCredentialRequest_clear(google::protobuf::Map<bool, ::vc_api::v1::PresentVerifiableCredentialRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_vc_api_v1_PresentVerifiableCredentialRequest_size(const google::protobuf::Map<bool, ::vc_api::v1::PresentVerifiableCredentialRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_vc_api_v1_PresentVerifiableCredentialRequest_insert(google::protobuf::Map<bool, ::vc_api::v1::PresentVerifiableCredentialRequest> * m,
                          bool key, ::vc_api::v1::PresentVerifiableCredentialRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_vc_api_v1_PresentVerifiableCredentialRequest_get(const google::protobuf::Map<bool, ::vc_api::v1::PresentVerifiableCredentialRequest>* m,
                       bool key, const ::vc_api::v1::PresentVerifiableCredentialRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::PresentVerifiableCredentialRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_vc_api_v1_PresentVerifiableCredentialRequest_remove(google::protobuf::Map<bool, ::vc_api::v1::PresentVerifiableCredentialRequest> * m,
                          bool key, ::vc_api::v1::PresentVerifiableCredentialRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_vc_api_v1_PresentVerifiableCredentialRequest_iter(
      const google::protobuf::Map<bool, ::vc_api::v1::PresentVerifiableCredentialRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_vc_api_v1_PresentVerifiableCredentialRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::vc_api::v1::PresentVerifiableCredentialRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::vc_api::v1::PresentVerifiableCredentialRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::vc_api::v1::PresentVerifiableCredentialRequest>* __rust_proto_thunk__Map_ProtoStr_vc_api_v1_PresentVerifiableCredentialRequest_new() {
    return new google::protobuf::Map<std::string, ::vc_api::v1::PresentVerifiableCredentialRequest>();
  }
  void __rust_proto_thunk__Map_ProtoStr_vc_api_v1_PresentVerifiableCredentialRequest_free(const google::protobuf::Map<std::string, ::vc_api::v1::PresentVerifiableCredentialRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_vc_api_v1_PresentVerifiableCredentialRequest_clear(google::protobuf::Map<std::string, ::vc_api::v1::PresentVerifiableCredentialRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_vc_api_v1_PresentVerifiableCredentialRequest_size(const google::protobuf::Map<std::string, ::vc_api::v1::PresentVerifiableCredentialRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_vc_api_v1_PresentVerifiableCredentialRequest_insert(google::protobuf::Map<std::string, ::vc_api::v1::PresentVerifiableCredentialRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::vc_api::v1::PresentVerifiableCredentialRequest value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_vc_api_v1_PresentVerifiableCredentialRequest_get(const google::protobuf::Map<std::string, ::vc_api::v1::PresentVerifiableCredentialRequest>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::vc_api::v1::PresentVerifiableCredentialRequest** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::PresentVerifiableCredentialRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_vc_api_v1_PresentVerifiableCredentialRequest_remove(google::protobuf::Map<std::string, ::vc_api::v1::PresentVerifiableCredentialRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::vc_api::v1::PresentVerifiableCredentialRequest * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_vc_api_v1_PresentVerifiableCredentialRequest_iter(
      const google::protobuf::Map<std::string, ::vc_api::v1::PresentVerifiableCredentialRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_vc_api_v1_PresentVerifiableCredentialRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::vc_api::v1::PresentVerifiableCredentialRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::vc_api::v1::PresentVerifiableCredentialRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// vc_api.v1.PresentVerifiableCredentialResponse
        // clang-format off
extern "C" {
void* __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialResponse_new() { return new ::vc_api::v1::PresentVerifiableCredentialResponse(); }
void __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialResponse_delete(void* ptr) { delete static_cast<::vc_api::v1::PresentVerifiableCredentialResponse*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialResponse_serialize(::vc_api::v1::PresentVerifiableCredentialResponse* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialResponse_parse(::vc_api::v1::PresentVerifiableCredentialResponse* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialResponse_copy_from(::vc_api::v1::PresentVerifiableCredentialResponse* dst, const ::vc_api::v1::PresentVerifiableCredentialResponse* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialResponse_repeated_len(google::protobuf::RepeatedPtrField<::vc_api::v1::PresentVerifiableCredentialResponse>* field) {
  return field->size();
}
const ::vc_api::v1::PresentVerifiableCredentialResponse& __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialResponse_repeated_get(
  google::protobuf::RepeatedPtrField<::vc_api::v1::PresentVerifiableCredentialResponse>* field,
  size_t index) {
  return field->Get(index);
}
::vc_api::v1::PresentVerifiableCredentialResponse* __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialResponse_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::vc_api::v1::PresentVerifiableCredentialResponse>* field,
  size_t index) {
  return field->Mutable(index);
}
::vc_api::v1::PresentVerifiableCredentialResponse* __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialResponse_repeated_add(google::protobuf::RepeatedPtrField<::vc_api::v1::PresentVerifiableCredentialResponse>* field) {
  return field->Add();
}
void __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialResponse_repeated_clear(google::protobuf::RepeatedPtrField<::vc_api::v1::PresentVerifiableCredentialResponse>* field) {
  field->Clear();
}
void __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialResponse_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::vc_api::v1::PresentVerifiableCredentialResponse>& dst,
  const google::protobuf::RepeatedPtrField<::vc_api::v1::PresentVerifiableCredentialResponse>& src) {
  dst = src;
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::vc_api::v1::PresentVerifiableCredentialResponse>* __rust_proto_thunk__Map_i32_vc_api_v1_PresentVerifiableCredentialResponse_new() {
    return new google::protobuf::Map<int32_t, ::vc_api::v1::PresentVerifiableCredentialResponse>();
  }
  void __rust_proto_thunk__Map_i32_vc_api_v1_PresentVerifiableCredentialResponse_free(const google::protobuf::Map<int32_t, ::vc_api::v1::PresentVerifiableCredentialResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_vc_api_v1_PresentVerifiableCredentialResponse_clear(google::protobuf::Map<int32_t, ::vc_api::v1::PresentVerifiableCredentialResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_vc_api_v1_PresentVerifiableCredentialResponse_size(const google::protobuf::Map<int32_t, ::vc_api::v1::PresentVerifiableCredentialResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_vc_api_v1_PresentVerifiableCredentialResponse_insert(google::protobuf::Map<int32_t, ::vc_api::v1::PresentVerifiableCredentialResponse> * m,
                          int32_t key, ::vc_api::v1::PresentVerifiableCredentialResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_vc_api_v1_PresentVerifiableCredentialResponse_get(const google::protobuf::Map<int32_t, ::vc_api::v1::PresentVerifiableCredentialResponse>* m,
                       int32_t key, const ::vc_api::v1::PresentVerifiableCredentialResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::PresentVerifiableCredentialResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_vc_api_v1_PresentVerifiableCredentialResponse_remove(google::protobuf::Map<int32_t, ::vc_api::v1::PresentVerifiableCredentialResponse> * m,
                          int32_t key, ::vc_api::v1::PresentVerifiableCredentialResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_vc_api_v1_PresentVerifiableCredentialResponse_iter(
      const google::protobuf::Map<int32_t, ::vc_api::v1::PresentVerifiableCredentialResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_vc_api_v1_PresentVerifiableCredentialResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::vc_api::v1::PresentVerifiableCredentialResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::vc_api::v1::PresentVerifiableCredentialResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::vc_api::v1::PresentVerifiableCredentialResponse>* __rust_proto_thunk__Map_u32_vc_api_v1_PresentVerifiableCredentialResponse_new() {
    return new google::protobuf::Map<uint32_t, ::vc_api::v1::PresentVerifiableCredentialResponse>();
  }
  void __rust_proto_thunk__Map_u32_vc_api_v1_PresentVerifiableCredentialResponse_free(const google::protobuf::Map<uint32_t, ::vc_api::v1::PresentVerifiableCredentialResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_vc_api_v1_PresentVerifiableCredentialResponse_clear(google::protobuf::Map<uint32_t, ::vc_api::v1::PresentVerifiableCredentialResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_vc_api_v1_PresentVerifiableCredentialResponse_size(const google::protobuf::Map<uint32_t, ::vc_api::v1::PresentVerifiableCredentialResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_vc_api_v1_PresentVerifiableCredentialResponse_insert(google::protobuf::Map<uint32_t, ::vc_api::v1::PresentVerifiableCredentialResponse> * m,
                          uint32_t key, ::vc_api::v1::PresentVerifiableCredentialResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_vc_api_v1_PresentVerifiableCredentialResponse_get(const google::protobuf::Map<uint32_t, ::vc_api::v1::PresentVerifiableCredentialResponse>* m,
                       uint32_t key, const ::vc_api::v1::PresentVerifiableCredentialResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::PresentVerifiableCredentialResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_vc_api_v1_PresentVerifiableCredentialResponse_remove(google::protobuf::Map<uint32_t, ::vc_api::v1::PresentVerifiableCredentialResponse> * m,
                          uint32_t key, ::vc_api::v1::PresentVerifiableCredentialResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_vc_api_v1_PresentVerifiableCredentialResponse_iter(
      const google::protobuf::Map<uint32_t, ::vc_api::v1::PresentVerifiableCredentialResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_vc_api_v1_PresentVerifiableCredentialResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::vc_api::v1::PresentVerifiableCredentialResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::vc_api::v1::PresentVerifiableCredentialResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::vc_api::v1::PresentVerifiableCredentialResponse>* __rust_proto_thunk__Map_i64_vc_api_v1_PresentVerifiableCredentialResponse_new() {
    return new google::protobuf::Map<int64_t, ::vc_api::v1::PresentVerifiableCredentialResponse>();
  }
  void __rust_proto_thunk__Map_i64_vc_api_v1_PresentVerifiableCredentialResponse_free(const google::protobuf::Map<int64_t, ::vc_api::v1::PresentVerifiableCredentialResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_vc_api_v1_PresentVerifiableCredentialResponse_clear(google::protobuf::Map<int64_t, ::vc_api::v1::PresentVerifiableCredentialResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_vc_api_v1_PresentVerifiableCredentialResponse_size(const google::protobuf::Map<int64_t, ::vc_api::v1::PresentVerifiableCredentialResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_vc_api_v1_PresentVerifiableCredentialResponse_insert(google::protobuf::Map<int64_t, ::vc_api::v1::PresentVerifiableCredentialResponse> * m,
                          int64_t key, ::vc_api::v1::PresentVerifiableCredentialResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_vc_api_v1_PresentVerifiableCredentialResponse_get(const google::protobuf::Map<int64_t, ::vc_api::v1::PresentVerifiableCredentialResponse>* m,
                       int64_t key, const ::vc_api::v1::PresentVerifiableCredentialResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::PresentVerifiableCredentialResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_vc_api_v1_PresentVerifiableCredentialResponse_remove(google::protobuf::Map<int64_t, ::vc_api::v1::PresentVerifiableCredentialResponse> * m,
                          int64_t key, ::vc_api::v1::PresentVerifiableCredentialResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_vc_api_v1_PresentVerifiableCredentialResponse_iter(
      const google::protobuf::Map<int64_t, ::vc_api::v1::PresentVerifiableCredentialResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_vc_api_v1_PresentVerifiableCredentialResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::vc_api::v1::PresentVerifiableCredentialResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::vc_api::v1::PresentVerifiableCredentialResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::vc_api::v1::PresentVerifiableCredentialResponse>* __rust_proto_thunk__Map_u64_vc_api_v1_PresentVerifiableCredentialResponse_new() {
    return new google::protobuf::Map<uint64_t, ::vc_api::v1::PresentVerifiableCredentialResponse>();
  }
  void __rust_proto_thunk__Map_u64_vc_api_v1_PresentVerifiableCredentialResponse_free(const google::protobuf::Map<uint64_t, ::vc_api::v1::PresentVerifiableCredentialResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_vc_api_v1_PresentVerifiableCredentialResponse_clear(google::protobuf::Map<uint64_t, ::vc_api::v1::PresentVerifiableCredentialResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_vc_api_v1_PresentVerifiableCredentialResponse_size(const google::protobuf::Map<uint64_t, ::vc_api::v1::PresentVerifiableCredentialResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_vc_api_v1_PresentVerifiableCredentialResponse_insert(google::protobuf::Map<uint64_t, ::vc_api::v1::PresentVerifiableCredentialResponse> * m,
                          uint64_t key, ::vc_api::v1::PresentVerifiableCredentialResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_vc_api_v1_PresentVerifiableCredentialResponse_get(const google::protobuf::Map<uint64_t, ::vc_api::v1::PresentVerifiableCredentialResponse>* m,
                       uint64_t key, const ::vc_api::v1::PresentVerifiableCredentialResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::PresentVerifiableCredentialResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_vc_api_v1_PresentVerifiableCredentialResponse_remove(google::protobuf::Map<uint64_t, ::vc_api::v1::PresentVerifiableCredentialResponse> * m,
                          uint64_t key, ::vc_api::v1::PresentVerifiableCredentialResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_vc_api_v1_PresentVerifiableCredentialResponse_iter(
      const google::protobuf::Map<uint64_t, ::vc_api::v1::PresentVerifiableCredentialResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_vc_api_v1_PresentVerifiableCredentialResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::vc_api::v1::PresentVerifiableCredentialResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::vc_api::v1::PresentVerifiableCredentialResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::vc_api::v1::PresentVerifiableCredentialResponse>* __rust_proto_thunk__Map_bool_vc_api_v1_PresentVerifiableCredentialResponse_new() {
    return new google::protobuf::Map<bool, ::vc_api::v1::PresentVerifiableCredentialResponse>();
  }
  void __rust_proto_thunk__Map_bool_vc_api_v1_PresentVerifiableCredentialResponse_free(const google::protobuf::Map<bool, ::vc_api::v1::PresentVerifiableCredentialResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_vc_api_v1_PresentVerifiableCredentialResponse_clear(google::protobuf::Map<bool, ::vc_api::v1::PresentVerifiableCredentialResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_vc_api_v1_PresentVerifiableCredentialResponse_size(const google::protobuf::Map<bool, ::vc_api::v1::PresentVerifiableCredentialResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_vc_api_v1_PresentVerifiableCredentialResponse_insert(google::protobuf::Map<bool, ::vc_api::v1::PresentVerifiableCredentialResponse> * m,
                          bool key, ::vc_api::v1::PresentVerifiableCredentialResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_vc_api_v1_PresentVerifiableCredentialResponse_get(const google::protobuf::Map<bool, ::vc_api::v1::PresentVerifiableCredentialResponse>* m,
                       bool key, const ::vc_api::v1::PresentVerifiableCredentialResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::PresentVerifiableCredentialResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_vc_api_v1_PresentVerifiableCredentialResponse_remove(google::protobuf::Map<bool, ::vc_api::v1::PresentVerifiableCredentialResponse> * m,
                          bool key, ::vc_api::v1::PresentVerifiableCredentialResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_vc_api_v1_PresentVerifiableCredentialResponse_iter(
      const google::protobuf::Map<bool, ::vc_api::v1::PresentVerifiableCredentialResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_vc_api_v1_PresentVerifiableCredentialResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::vc_api::v1::PresentVerifiableCredentialResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::vc_api::v1::PresentVerifiableCredentialResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::vc_api::v1::PresentVerifiableCredentialResponse>* __rust_proto_thunk__Map_ProtoStr_vc_api_v1_PresentVerifiableCredentialResponse_new() {
    return new google::protobuf::Map<std::string, ::vc_api::v1::PresentVerifiableCredentialResponse>();
  }
  void __rust_proto_thunk__Map_ProtoStr_vc_api_v1_PresentVerifiableCredentialResponse_free(const google::protobuf::Map<std::string, ::vc_api::v1::PresentVerifiableCredentialResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_vc_api_v1_PresentVerifiableCredentialResponse_clear(google::protobuf::Map<std::string, ::vc_api::v1::PresentVerifiableCredentialResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_vc_api_v1_PresentVerifiableCredentialResponse_size(const google::protobuf::Map<std::string, ::vc_api::v1::PresentVerifiableCredentialResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_vc_api_v1_PresentVerifiableCredentialResponse_insert(google::protobuf::Map<std::string, ::vc_api::v1::PresentVerifiableCredentialResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::vc_api::v1::PresentVerifiableCredentialResponse value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_vc_api_v1_PresentVerifiableCredentialResponse_get(const google::protobuf::Map<std::string, ::vc_api::v1::PresentVerifiableCredentialResponse>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::vc_api::v1::PresentVerifiableCredentialResponse** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::PresentVerifiableCredentialResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_vc_api_v1_PresentVerifiableCredentialResponse_remove(google::protobuf::Map<std::string, ::vc_api::v1::PresentVerifiableCredentialResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::vc_api::v1::PresentVerifiableCredentialResponse * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_vc_api_v1_PresentVerifiableCredentialResponse_iter(
      const google::protobuf::Map<std::string, ::vc_api::v1::PresentVerifiableCredentialResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_vc_api_v1_PresentVerifiableCredentialResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::vc_api::v1::PresentVerifiableCredentialResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::vc_api::v1::PresentVerifiableCredentialResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// vc_api.v1.WaitForCompletionRequest
        // clang-format off
extern "C" {
void* __rust_proto_thunk__vc_api_v1_WaitForCompletionRequest_new() { return new ::vc_api::v1::WaitForCompletionRequest(); }
void __rust_proto_thunk__vc_api_v1_WaitForCompletionRequest_delete(void* ptr) { delete static_cast<::vc_api::v1::WaitForCompletionRequest*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__vc_api_v1_WaitForCompletionRequest_serialize(::vc_api::v1::WaitForCompletionRequest* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__vc_api_v1_WaitForCompletionRequest_parse(::vc_api::v1::WaitForCompletionRequest* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__vc_api_v1_WaitForCompletionRequest_copy_from(::vc_api::v1::WaitForCompletionRequest* dst, const ::vc_api::v1::WaitForCompletionRequest* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__vc_api_v1_WaitForCompletionRequest_repeated_len(google::protobuf::RepeatedPtrField<::vc_api::v1::WaitForCompletionRequest>* field) {
  return field->size();
}
const ::vc_api::v1::WaitForCompletionRequest& __rust_proto_thunk__vc_api_v1_WaitForCompletionRequest_repeated_get(
  google::protobuf::RepeatedPtrField<::vc_api::v1::WaitForCompletionRequest>* field,
  size_t index) {
  return field->Get(index);
}
::vc_api::v1::WaitForCompletionRequest* __rust_proto_thunk__vc_api_v1_WaitForCompletionRequest_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::vc_api::v1::WaitForCompletionRequest>* field,
  size_t index) {
  return field->Mutable(index);
}
::vc_api::v1::WaitForCompletionRequest* __rust_proto_thunk__vc_api_v1_WaitForCompletionRequest_repeated_add(google::protobuf::RepeatedPtrField<::vc_api::v1::WaitForCompletionRequest>* field) {
  return field->Add();
}
void __rust_proto_thunk__vc_api_v1_WaitForCompletionRequest_repeated_clear(google::protobuf::RepeatedPtrField<::vc_api::v1::WaitForCompletionRequest>* field) {
  field->Clear();
}
void __rust_proto_thunk__vc_api_v1_WaitForCompletionRequest_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::vc_api::v1::WaitForCompletionRequest>& dst,
  const google::protobuf::RepeatedPtrField<::vc_api::v1::WaitForCompletionRequest>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__vc_api_v1_WaitForCompletionRequest_get_nonce(::vc_api::v1::WaitForCompletionRequest* msg) {
  absl::string_view val = msg->nonce();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__vc_api_v1_WaitForCompletionRequest_set_nonce(::vc_api::v1::WaitForCompletionRequest* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_nonce(absl::string_view(s.ptr, s.len));
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::vc_api::v1::WaitForCompletionRequest>* __rust_proto_thunk__Map_i32_vc_api_v1_WaitForCompletionRequest_new() {
    return new google::protobuf::Map<int32_t, ::vc_api::v1::WaitForCompletionRequest>();
  }
  void __rust_proto_thunk__Map_i32_vc_api_v1_WaitForCompletionRequest_free(const google::protobuf::Map<int32_t, ::vc_api::v1::WaitForCompletionRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_vc_api_v1_WaitForCompletionRequest_clear(google::protobuf::Map<int32_t, ::vc_api::v1::WaitForCompletionRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_vc_api_v1_WaitForCompletionRequest_size(const google::protobuf::Map<int32_t, ::vc_api::v1::WaitForCompletionRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_vc_api_v1_WaitForCompletionRequest_insert(google::protobuf::Map<int32_t, ::vc_api::v1::WaitForCompletionRequest> * m,
                          int32_t key, ::vc_api::v1::WaitForCompletionRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_vc_api_v1_WaitForCompletionRequest_get(const google::protobuf::Map<int32_t, ::vc_api::v1::WaitForCompletionRequest>* m,
                       int32_t key, const ::vc_api::v1::WaitForCompletionRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::WaitForCompletionRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_vc_api_v1_WaitForCompletionRequest_remove(google::protobuf::Map<int32_t, ::vc_api::v1::WaitForCompletionRequest> * m,
                          int32_t key, ::vc_api::v1::WaitForCompletionRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_vc_api_v1_WaitForCompletionRequest_iter(
      const google::protobuf::Map<int32_t, ::vc_api::v1::WaitForCompletionRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_vc_api_v1_WaitForCompletionRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::vc_api::v1::WaitForCompletionRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::vc_api::v1::WaitForCompletionRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::vc_api::v1::WaitForCompletionRequest>* __rust_proto_thunk__Map_u32_vc_api_v1_WaitForCompletionRequest_new() {
    return new google::protobuf::Map<uint32_t, ::vc_api::v1::WaitForCompletionRequest>();
  }
  void __rust_proto_thunk__Map_u32_vc_api_v1_WaitForCompletionRequest_free(const google::protobuf::Map<uint32_t, ::vc_api::v1::WaitForCompletionRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_vc_api_v1_WaitForCompletionRequest_clear(google::protobuf::Map<uint32_t, ::vc_api::v1::WaitForCompletionRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_vc_api_v1_WaitForCompletionRequest_size(const google::protobuf::Map<uint32_t, ::vc_api::v1::WaitForCompletionRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_vc_api_v1_WaitForCompletionRequest_insert(google::protobuf::Map<uint32_t, ::vc_api::v1::WaitForCompletionRequest> * m,
                          uint32_t key, ::vc_api::v1::WaitForCompletionRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_vc_api_v1_WaitForCompletionRequest_get(const google::protobuf::Map<uint32_t, ::vc_api::v1::WaitForCompletionRequest>* m,
                       uint32_t key, const ::vc_api::v1::WaitForCompletionRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::WaitForCompletionRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_vc_api_v1_WaitForCompletionRequest_remove(google::protobuf::Map<uint32_t, ::vc_api::v1::WaitForCompletionRequest> * m,
                          uint32_t key, ::vc_api::v1::WaitForCompletionRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_vc_api_v1_WaitForCompletionRequest_iter(
      const google::protobuf::Map<uint32_t, ::vc_api::v1::WaitForCompletionRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_vc_api_v1_WaitForCompletionRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::vc_api::v1::WaitForCompletionRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::vc_api::v1::WaitForCompletionRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::vc_api::v1::WaitForCompletionRequest>* __rust_proto_thunk__Map_i64_vc_api_v1_WaitForCompletionRequest_new() {
    return new google::protobuf::Map<int64_t, ::vc_api::v1::WaitForCompletionRequest>();
  }
  void __rust_proto_thunk__Map_i64_vc_api_v1_WaitForCompletionRequest_free(const google::protobuf::Map<int64_t, ::vc_api::v1::WaitForCompletionRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_vc_api_v1_WaitForCompletionRequest_clear(google::protobuf::Map<int64_t, ::vc_api::v1::WaitForCompletionRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_vc_api_v1_WaitForCompletionRequest_size(const google::protobuf::Map<int64_t, ::vc_api::v1::WaitForCompletionRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_vc_api_v1_WaitForCompletionRequest_insert(google::protobuf::Map<int64_t, ::vc_api::v1::WaitForCompletionRequest> * m,
                          int64_t key, ::vc_api::v1::WaitForCompletionRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_vc_api_v1_WaitForCompletionRequest_get(const google::protobuf::Map<int64_t, ::vc_api::v1::WaitForCompletionRequest>* m,
                       int64_t key, const ::vc_api::v1::WaitForCompletionRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::WaitForCompletionRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_vc_api_v1_WaitForCompletionRequest_remove(google::protobuf::Map<int64_t, ::vc_api::v1::WaitForCompletionRequest> * m,
                          int64_t key, ::vc_api::v1::WaitForCompletionRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_vc_api_v1_WaitForCompletionRequest_iter(
      const google::protobuf::Map<int64_t, ::vc_api::v1::WaitForCompletionRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_vc_api_v1_WaitForCompletionRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::vc_api::v1::WaitForCompletionRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::vc_api::v1::WaitForCompletionRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::vc_api::v1::WaitForCompletionRequest>* __rust_proto_thunk__Map_u64_vc_api_v1_WaitForCompletionRequest_new() {
    return new google::protobuf::Map<uint64_t, ::vc_api::v1::WaitForCompletionRequest>();
  }
  void __rust_proto_thunk__Map_u64_vc_api_v1_WaitForCompletionRequest_free(const google::protobuf::Map<uint64_t, ::vc_api::v1::WaitForCompletionRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_vc_api_v1_WaitForCompletionRequest_clear(google::protobuf::Map<uint64_t, ::vc_api::v1::WaitForCompletionRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_vc_api_v1_WaitForCompletionRequest_size(const google::protobuf::Map<uint64_t, ::vc_api::v1::WaitForCompletionRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_vc_api_v1_WaitForCompletionRequest_insert(google::protobuf::Map<uint64_t, ::vc_api::v1::WaitForCompletionRequest> * m,
                          uint64_t key, ::vc_api::v1::WaitForCompletionRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_vc_api_v1_WaitForCompletionRequest_get(const google::protobuf::Map<uint64_t, ::vc_api::v1::WaitForCompletionRequest>* m,
                       uint64_t key, const ::vc_api::v1::WaitForCompletionRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::WaitForCompletionRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_vc_api_v1_WaitForCompletionRequest_remove(google::protobuf::Map<uint64_t, ::vc_api::v1::WaitForCompletionRequest> * m,
                          uint64_t key, ::vc_api::v1::WaitForCompletionRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_vc_api_v1_WaitForCompletionRequest_iter(
      const google::protobuf::Map<uint64_t, ::vc_api::v1::WaitForCompletionRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_vc_api_v1_WaitForCompletionRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::vc_api::v1::WaitForCompletionRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::vc_api::v1::WaitForCompletionRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::vc_api::v1::WaitForCompletionRequest>* __rust_proto_thunk__Map_bool_vc_api_v1_WaitForCompletionRequest_new() {
    return new google::protobuf::Map<bool, ::vc_api::v1::WaitForCompletionRequest>();
  }
  void __rust_proto_thunk__Map_bool_vc_api_v1_WaitForCompletionRequest_free(const google::protobuf::Map<bool, ::vc_api::v1::WaitForCompletionRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_vc_api_v1_WaitForCompletionRequest_clear(google::protobuf::Map<bool, ::vc_api::v1::WaitForCompletionRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_vc_api_v1_WaitForCompletionRequest_size(const google::protobuf::Map<bool, ::vc_api::v1::WaitForCompletionRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_vc_api_v1_WaitForCompletionRequest_insert(google::protobuf::Map<bool, ::vc_api::v1::WaitForCompletionRequest> * m,
                          bool key, ::vc_api::v1::WaitForCompletionRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_vc_api_v1_WaitForCompletionRequest_get(const google::protobuf::Map<bool, ::vc_api::v1::WaitForCompletionRequest>* m,
                       bool key, const ::vc_api::v1::WaitForCompletionRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::WaitForCompletionRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_vc_api_v1_WaitForCompletionRequest_remove(google::protobuf::Map<bool, ::vc_api::v1::WaitForCompletionRequest> * m,
                          bool key, ::vc_api::v1::WaitForCompletionRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_vc_api_v1_WaitForCompletionRequest_iter(
      const google::protobuf::Map<bool, ::vc_api::v1::WaitForCompletionRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_vc_api_v1_WaitForCompletionRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::vc_api::v1::WaitForCompletionRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::vc_api::v1::WaitForCompletionRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::vc_api::v1::WaitForCompletionRequest>* __rust_proto_thunk__Map_ProtoStr_vc_api_v1_WaitForCompletionRequest_new() {
    return new google::protobuf::Map<std::string, ::vc_api::v1::WaitForCompletionRequest>();
  }
  void __rust_proto_thunk__Map_ProtoStr_vc_api_v1_WaitForCompletionRequest_free(const google::protobuf::Map<std::string, ::vc_api::v1::WaitForCompletionRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_vc_api_v1_WaitForCompletionRequest_clear(google::protobuf::Map<std::string, ::vc_api::v1::WaitForCompletionRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_vc_api_v1_WaitForCompletionRequest_size(const google::protobuf::Map<std::string, ::vc_api::v1::WaitForCompletionRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_vc_api_v1_WaitForCompletionRequest_insert(google::protobuf::Map<std::string, ::vc_api::v1::WaitForCompletionRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::vc_api::v1::WaitForCompletionRequest value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_vc_api_v1_WaitForCompletionRequest_get(const google::protobuf::Map<std::string, ::vc_api::v1::WaitForCompletionRequest>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::vc_api::v1::WaitForCompletionRequest** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::WaitForCompletionRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_vc_api_v1_WaitForCompletionRequest_remove(google::protobuf::Map<std::string, ::vc_api::v1::WaitForCompletionRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::vc_api::v1::WaitForCompletionRequest * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_vc_api_v1_WaitForCompletionRequest_iter(
      const google::protobuf::Map<std::string, ::vc_api::v1::WaitForCompletionRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_vc_api_v1_WaitForCompletionRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::vc_api::v1::WaitForCompletionRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::vc_api::v1::WaitForCompletionRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// vc_api.v1.WaitForCompletionResponse
        // clang-format off
extern "C" {
void* __rust_proto_thunk__vc_api_v1_WaitForCompletionResponse_new() { return new ::vc_api::v1::WaitForCompletionResponse(); }
void __rust_proto_thunk__vc_api_v1_WaitForCompletionResponse_delete(void* ptr) { delete static_cast<::vc_api::v1::WaitForCompletionResponse*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__vc_api_v1_WaitForCompletionResponse_serialize(::vc_api::v1::WaitForCompletionResponse* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__vc_api_v1_WaitForCompletionResponse_parse(::vc_api::v1::WaitForCompletionResponse* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__vc_api_v1_WaitForCompletionResponse_copy_from(::vc_api::v1::WaitForCompletionResponse* dst, const ::vc_api::v1::WaitForCompletionResponse* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__vc_api_v1_WaitForCompletionResponse_repeated_len(google::protobuf::RepeatedPtrField<::vc_api::v1::WaitForCompletionResponse>* field) {
  return field->size();
}
const ::vc_api::v1::WaitForCompletionResponse& __rust_proto_thunk__vc_api_v1_WaitForCompletionResponse_repeated_get(
  google::protobuf::RepeatedPtrField<::vc_api::v1::WaitForCompletionResponse>* field,
  size_t index) {
  return field->Get(index);
}
::vc_api::v1::WaitForCompletionResponse* __rust_proto_thunk__vc_api_v1_WaitForCompletionResponse_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::vc_api::v1::WaitForCompletionResponse>* field,
  size_t index) {
  return field->Mutable(index);
}
::vc_api::v1::WaitForCompletionResponse* __rust_proto_thunk__vc_api_v1_WaitForCompletionResponse_repeated_add(google::protobuf::RepeatedPtrField<::vc_api::v1::WaitForCompletionResponse>* field) {
  return field->Add();
}
void __rust_proto_thunk__vc_api_v1_WaitForCompletionResponse_repeated_clear(google::protobuf::RepeatedPtrField<::vc_api::v1::WaitForCompletionResponse>* field) {
  field->Clear();
}
void __rust_proto_thunk__vc_api_v1_WaitForCompletionResponse_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::vc_api::v1::WaitForCompletionResponse>& dst,
  const google::protobuf::RepeatedPtrField<::vc_api::v1::WaitForCompletionResponse>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__vc_api_v1_WaitForCompletionResponse_get_did(::vc_api::v1::WaitForCompletionResponse* msg) {
  absl::string_view val = msg->did();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__vc_api_v1_WaitForCompletionResponse_set_did(::vc_api::v1::WaitForCompletionResponse* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_did(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__vc_api_v1_WaitForCompletionResponse_get_data(::vc_api::v1::WaitForCompletionResponse* msg) {
  absl::string_view val = msg->data();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__vc_api_v1_WaitForCompletionResponse_set_data(::vc_api::v1::WaitForCompletionResponse* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_data(absl::string_view(s.ptr, s.len));
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::vc_api::v1::WaitForCompletionResponse>* __rust_proto_thunk__Map_i32_vc_api_v1_WaitForCompletionResponse_new() {
    return new google::protobuf::Map<int32_t, ::vc_api::v1::WaitForCompletionResponse>();
  }
  void __rust_proto_thunk__Map_i32_vc_api_v1_WaitForCompletionResponse_free(const google::protobuf::Map<int32_t, ::vc_api::v1::WaitForCompletionResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_vc_api_v1_WaitForCompletionResponse_clear(google::protobuf::Map<int32_t, ::vc_api::v1::WaitForCompletionResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_vc_api_v1_WaitForCompletionResponse_size(const google::protobuf::Map<int32_t, ::vc_api::v1::WaitForCompletionResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_vc_api_v1_WaitForCompletionResponse_insert(google::protobuf::Map<int32_t, ::vc_api::v1::WaitForCompletionResponse> * m,
                          int32_t key, ::vc_api::v1::WaitForCompletionResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_vc_api_v1_WaitForCompletionResponse_get(const google::protobuf::Map<int32_t, ::vc_api::v1::WaitForCompletionResponse>* m,
                       int32_t key, const ::vc_api::v1::WaitForCompletionResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::WaitForCompletionResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_vc_api_v1_WaitForCompletionResponse_remove(google::protobuf::Map<int32_t, ::vc_api::v1::WaitForCompletionResponse> * m,
                          int32_t key, ::vc_api::v1::WaitForCompletionResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_vc_api_v1_WaitForCompletionResponse_iter(
      const google::protobuf::Map<int32_t, ::vc_api::v1::WaitForCompletionResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_vc_api_v1_WaitForCompletionResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::vc_api::v1::WaitForCompletionResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::vc_api::v1::WaitForCompletionResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::vc_api::v1::WaitForCompletionResponse>* __rust_proto_thunk__Map_u32_vc_api_v1_WaitForCompletionResponse_new() {
    return new google::protobuf::Map<uint32_t, ::vc_api::v1::WaitForCompletionResponse>();
  }
  void __rust_proto_thunk__Map_u32_vc_api_v1_WaitForCompletionResponse_free(const google::protobuf::Map<uint32_t, ::vc_api::v1::WaitForCompletionResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_vc_api_v1_WaitForCompletionResponse_clear(google::protobuf::Map<uint32_t, ::vc_api::v1::WaitForCompletionResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_vc_api_v1_WaitForCompletionResponse_size(const google::protobuf::Map<uint32_t, ::vc_api::v1::WaitForCompletionResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_vc_api_v1_WaitForCompletionResponse_insert(google::protobuf::Map<uint32_t, ::vc_api::v1::WaitForCompletionResponse> * m,
                          uint32_t key, ::vc_api::v1::WaitForCompletionResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_vc_api_v1_WaitForCompletionResponse_get(const google::protobuf::Map<uint32_t, ::vc_api::v1::WaitForCompletionResponse>* m,
                       uint32_t key, const ::vc_api::v1::WaitForCompletionResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::WaitForCompletionResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_vc_api_v1_WaitForCompletionResponse_remove(google::protobuf::Map<uint32_t, ::vc_api::v1::WaitForCompletionResponse> * m,
                          uint32_t key, ::vc_api::v1::WaitForCompletionResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_vc_api_v1_WaitForCompletionResponse_iter(
      const google::protobuf::Map<uint32_t, ::vc_api::v1::WaitForCompletionResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_vc_api_v1_WaitForCompletionResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::vc_api::v1::WaitForCompletionResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::vc_api::v1::WaitForCompletionResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::vc_api::v1::WaitForCompletionResponse>* __rust_proto_thunk__Map_i64_vc_api_v1_WaitForCompletionResponse_new() {
    return new google::protobuf::Map<int64_t, ::vc_api::v1::WaitForCompletionResponse>();
  }
  void __rust_proto_thunk__Map_i64_vc_api_v1_WaitForCompletionResponse_free(const google::protobuf::Map<int64_t, ::vc_api::v1::WaitForCompletionResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_vc_api_v1_WaitForCompletionResponse_clear(google::protobuf::Map<int64_t, ::vc_api::v1::WaitForCompletionResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_vc_api_v1_WaitForCompletionResponse_size(const google::protobuf::Map<int64_t, ::vc_api::v1::WaitForCompletionResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_vc_api_v1_WaitForCompletionResponse_insert(google::protobuf::Map<int64_t, ::vc_api::v1::WaitForCompletionResponse> * m,
                          int64_t key, ::vc_api::v1::WaitForCompletionResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_vc_api_v1_WaitForCompletionResponse_get(const google::protobuf::Map<int64_t, ::vc_api::v1::WaitForCompletionResponse>* m,
                       int64_t key, const ::vc_api::v1::WaitForCompletionResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::WaitForCompletionResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_vc_api_v1_WaitForCompletionResponse_remove(google::protobuf::Map<int64_t, ::vc_api::v1::WaitForCompletionResponse> * m,
                          int64_t key, ::vc_api::v1::WaitForCompletionResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_vc_api_v1_WaitForCompletionResponse_iter(
      const google::protobuf::Map<int64_t, ::vc_api::v1::WaitForCompletionResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_vc_api_v1_WaitForCompletionResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::vc_api::v1::WaitForCompletionResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::vc_api::v1::WaitForCompletionResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::vc_api::v1::WaitForCompletionResponse>* __rust_proto_thunk__Map_u64_vc_api_v1_WaitForCompletionResponse_new() {
    return new google::protobuf::Map<uint64_t, ::vc_api::v1::WaitForCompletionResponse>();
  }
  void __rust_proto_thunk__Map_u64_vc_api_v1_WaitForCompletionResponse_free(const google::protobuf::Map<uint64_t, ::vc_api::v1::WaitForCompletionResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_vc_api_v1_WaitForCompletionResponse_clear(google::protobuf::Map<uint64_t, ::vc_api::v1::WaitForCompletionResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_vc_api_v1_WaitForCompletionResponse_size(const google::protobuf::Map<uint64_t, ::vc_api::v1::WaitForCompletionResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_vc_api_v1_WaitForCompletionResponse_insert(google::protobuf::Map<uint64_t, ::vc_api::v1::WaitForCompletionResponse> * m,
                          uint64_t key, ::vc_api::v1::WaitForCompletionResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_vc_api_v1_WaitForCompletionResponse_get(const google::protobuf::Map<uint64_t, ::vc_api::v1::WaitForCompletionResponse>* m,
                       uint64_t key, const ::vc_api::v1::WaitForCompletionResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::WaitForCompletionResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_vc_api_v1_WaitForCompletionResponse_remove(google::protobuf::Map<uint64_t, ::vc_api::v1::WaitForCompletionResponse> * m,
                          uint64_t key, ::vc_api::v1::WaitForCompletionResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_vc_api_v1_WaitForCompletionResponse_iter(
      const google::protobuf::Map<uint64_t, ::vc_api::v1::WaitForCompletionResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_vc_api_v1_WaitForCompletionResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::vc_api::v1::WaitForCompletionResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::vc_api::v1::WaitForCompletionResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::vc_api::v1::WaitForCompletionResponse>* __rust_proto_thunk__Map_bool_vc_api_v1_WaitForCompletionResponse_new() {
    return new google::protobuf::Map<bool, ::vc_api::v1::WaitForCompletionResponse>();
  }
  void __rust_proto_thunk__Map_bool_vc_api_v1_WaitForCompletionResponse_free(const google::protobuf::Map<bool, ::vc_api::v1::WaitForCompletionResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_vc_api_v1_WaitForCompletionResponse_clear(google::protobuf::Map<bool, ::vc_api::v1::WaitForCompletionResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_vc_api_v1_WaitForCompletionResponse_size(const google::protobuf::Map<bool, ::vc_api::v1::WaitForCompletionResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_vc_api_v1_WaitForCompletionResponse_insert(google::protobuf::Map<bool, ::vc_api::v1::WaitForCompletionResponse> * m,
                          bool key, ::vc_api::v1::WaitForCompletionResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_vc_api_v1_WaitForCompletionResponse_get(const google::protobuf::Map<bool, ::vc_api::v1::WaitForCompletionResponse>* m,
                       bool key, const ::vc_api::v1::WaitForCompletionResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::WaitForCompletionResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_vc_api_v1_WaitForCompletionResponse_remove(google::protobuf::Map<bool, ::vc_api::v1::WaitForCompletionResponse> * m,
                          bool key, ::vc_api::v1::WaitForCompletionResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_vc_api_v1_WaitForCompletionResponse_iter(
      const google::protobuf::Map<bool, ::vc_api::v1::WaitForCompletionResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_vc_api_v1_WaitForCompletionResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::vc_api::v1::WaitForCompletionResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::vc_api::v1::WaitForCompletionResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::vc_api::v1::WaitForCompletionResponse>* __rust_proto_thunk__Map_ProtoStr_vc_api_v1_WaitForCompletionResponse_new() {
    return new google::protobuf::Map<std::string, ::vc_api::v1::WaitForCompletionResponse>();
  }
  void __rust_proto_thunk__Map_ProtoStr_vc_api_v1_WaitForCompletionResponse_free(const google::protobuf::Map<std::string, ::vc_api::v1::WaitForCompletionResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_vc_api_v1_WaitForCompletionResponse_clear(google::protobuf::Map<std::string, ::vc_api::v1::WaitForCompletionResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_vc_api_v1_WaitForCompletionResponse_size(const google::protobuf::Map<std::string, ::vc_api::v1::WaitForCompletionResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_vc_api_v1_WaitForCompletionResponse_insert(google::protobuf::Map<std::string, ::vc_api::v1::WaitForCompletionResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::vc_api::v1::WaitForCompletionResponse value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_vc_api_v1_WaitForCompletionResponse_get(const google::protobuf::Map<std::string, ::vc_api::v1::WaitForCompletionResponse>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::vc_api::v1::WaitForCompletionResponse** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::WaitForCompletionResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_vc_api_v1_WaitForCompletionResponse_remove(google::protobuf::Map<std::string, ::vc_api::v1::WaitForCompletionResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::vc_api::v1::WaitForCompletionResponse * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_vc_api_v1_WaitForCompletionResponse_iter(
      const google::protobuf::Map<std::string, ::vc_api::v1::WaitForCompletionResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_vc_api_v1_WaitForCompletionResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::vc_api::v1::WaitForCompletionResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::vc_api::v1::WaitForCompletionResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// vc_api.v1.UpdateVerifiableCredentialStatusRequest
        // clang-format off
extern "C" {
void* __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusRequest_new() { return new ::vc_api::v1::UpdateVerifiableCredentialStatusRequest(); }
void __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusRequest_delete(void* ptr) { delete static_cast<::vc_api::v1::UpdateVerifiableCredentialStatusRequest*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusRequest_serialize(::vc_api::v1::UpdateVerifiableCredentialStatusRequest* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusRequest_parse(::vc_api::v1::UpdateVerifiableCredentialStatusRequest* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusRequest_copy_from(::vc_api::v1::UpdateVerifiableCredentialStatusRequest* dst, const ::vc_api::v1::UpdateVerifiableCredentialStatusRequest* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusRequest_repeated_len(google::protobuf::RepeatedPtrField<::vc_api::v1::UpdateVerifiableCredentialStatusRequest>* field) {
  return field->size();
}
const ::vc_api::v1::UpdateVerifiableCredentialStatusRequest& __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusRequest_repeated_get(
  google::protobuf::RepeatedPtrField<::vc_api::v1::UpdateVerifiableCredentialStatusRequest>* field,
  size_t index) {
  return field->Get(index);
}
::vc_api::v1::UpdateVerifiableCredentialStatusRequest* __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusRequest_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::vc_api::v1::UpdateVerifiableCredentialStatusRequest>* field,
  size_t index) {
  return field->Mutable(index);
}
::vc_api::v1::UpdateVerifiableCredentialStatusRequest* __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusRequest_repeated_add(google::protobuf::RepeatedPtrField<::vc_api::v1::UpdateVerifiableCredentialStatusRequest>* field) {
  return field->Add();
}
void __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusRequest_repeated_clear(google::protobuf::RepeatedPtrField<::vc_api::v1::UpdateVerifiableCredentialStatusRequest>* field) {
  field->Clear();
}
void __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusRequest_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::vc_api::v1::UpdateVerifiableCredentialStatusRequest>& dst,
  const google::protobuf::RepeatedPtrField<::vc_api::v1::UpdateVerifiableCredentialStatusRequest>& src) {
  dst = src;
}

::vc_api::v1::CredentialType __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusRequest_get_credential_type(::vc_api::v1::UpdateVerifiableCredentialStatusRequest* msg) { return msg->credential_type(); }
void __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusRequest_set_credential_type(::vc_api::v1::UpdateVerifiableCredentialStatusRequest* msg, ::vc_api::v1::CredentialType val) {
  msg->set_credential_type(val);
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusRequest_get_did(::vc_api::v1::UpdateVerifiableCredentialStatusRequest* msg) {
  absl::string_view val = msg->did();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusRequest_set_did(::vc_api::v1::UpdateVerifiableCredentialStatusRequest* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_did(absl::string_view(s.ptr, s.len));
}

::vc_api::v1::UpdateVerifiableCredentialStatusType __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusRequest_get_update_type(::vc_api::v1::UpdateVerifiableCredentialStatusRequest* msg) { return msg->update_type(); }
void __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusRequest_set_update_type(::vc_api::v1::UpdateVerifiableCredentialStatusRequest* msg, ::vc_api::v1::UpdateVerifiableCredentialStatusType val) {
  msg->set_update_type(val);
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::vc_api::v1::UpdateVerifiableCredentialStatusRequest>* __rust_proto_thunk__Map_i32_vc_api_v1_UpdateVerifiableCredentialStatusRequest_new() {
    return new google::protobuf::Map<int32_t, ::vc_api::v1::UpdateVerifiableCredentialStatusRequest>();
  }
  void __rust_proto_thunk__Map_i32_vc_api_v1_UpdateVerifiableCredentialStatusRequest_free(const google::protobuf::Map<int32_t, ::vc_api::v1::UpdateVerifiableCredentialStatusRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_vc_api_v1_UpdateVerifiableCredentialStatusRequest_clear(google::protobuf::Map<int32_t, ::vc_api::v1::UpdateVerifiableCredentialStatusRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_vc_api_v1_UpdateVerifiableCredentialStatusRequest_size(const google::protobuf::Map<int32_t, ::vc_api::v1::UpdateVerifiableCredentialStatusRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_vc_api_v1_UpdateVerifiableCredentialStatusRequest_insert(google::protobuf::Map<int32_t, ::vc_api::v1::UpdateVerifiableCredentialStatusRequest> * m,
                          int32_t key, ::vc_api::v1::UpdateVerifiableCredentialStatusRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_vc_api_v1_UpdateVerifiableCredentialStatusRequest_get(const google::protobuf::Map<int32_t, ::vc_api::v1::UpdateVerifiableCredentialStatusRequest>* m,
                       int32_t key, const ::vc_api::v1::UpdateVerifiableCredentialStatusRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::UpdateVerifiableCredentialStatusRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_vc_api_v1_UpdateVerifiableCredentialStatusRequest_remove(google::protobuf::Map<int32_t, ::vc_api::v1::UpdateVerifiableCredentialStatusRequest> * m,
                          int32_t key, ::vc_api::v1::UpdateVerifiableCredentialStatusRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_vc_api_v1_UpdateVerifiableCredentialStatusRequest_iter(
      const google::protobuf::Map<int32_t, ::vc_api::v1::UpdateVerifiableCredentialStatusRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_vc_api_v1_UpdateVerifiableCredentialStatusRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::vc_api::v1::UpdateVerifiableCredentialStatusRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::vc_api::v1::UpdateVerifiableCredentialStatusRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::vc_api::v1::UpdateVerifiableCredentialStatusRequest>* __rust_proto_thunk__Map_u32_vc_api_v1_UpdateVerifiableCredentialStatusRequest_new() {
    return new google::protobuf::Map<uint32_t, ::vc_api::v1::UpdateVerifiableCredentialStatusRequest>();
  }
  void __rust_proto_thunk__Map_u32_vc_api_v1_UpdateVerifiableCredentialStatusRequest_free(const google::protobuf::Map<uint32_t, ::vc_api::v1::UpdateVerifiableCredentialStatusRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_vc_api_v1_UpdateVerifiableCredentialStatusRequest_clear(google::protobuf::Map<uint32_t, ::vc_api::v1::UpdateVerifiableCredentialStatusRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_vc_api_v1_UpdateVerifiableCredentialStatusRequest_size(const google::protobuf::Map<uint32_t, ::vc_api::v1::UpdateVerifiableCredentialStatusRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_vc_api_v1_UpdateVerifiableCredentialStatusRequest_insert(google::protobuf::Map<uint32_t, ::vc_api::v1::UpdateVerifiableCredentialStatusRequest> * m,
                          uint32_t key, ::vc_api::v1::UpdateVerifiableCredentialStatusRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_vc_api_v1_UpdateVerifiableCredentialStatusRequest_get(const google::protobuf::Map<uint32_t, ::vc_api::v1::UpdateVerifiableCredentialStatusRequest>* m,
                       uint32_t key, const ::vc_api::v1::UpdateVerifiableCredentialStatusRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::UpdateVerifiableCredentialStatusRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_vc_api_v1_UpdateVerifiableCredentialStatusRequest_remove(google::protobuf::Map<uint32_t, ::vc_api::v1::UpdateVerifiableCredentialStatusRequest> * m,
                          uint32_t key, ::vc_api::v1::UpdateVerifiableCredentialStatusRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_vc_api_v1_UpdateVerifiableCredentialStatusRequest_iter(
      const google::protobuf::Map<uint32_t, ::vc_api::v1::UpdateVerifiableCredentialStatusRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_vc_api_v1_UpdateVerifiableCredentialStatusRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::vc_api::v1::UpdateVerifiableCredentialStatusRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::vc_api::v1::UpdateVerifiableCredentialStatusRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::vc_api::v1::UpdateVerifiableCredentialStatusRequest>* __rust_proto_thunk__Map_i64_vc_api_v1_UpdateVerifiableCredentialStatusRequest_new() {
    return new google::protobuf::Map<int64_t, ::vc_api::v1::UpdateVerifiableCredentialStatusRequest>();
  }
  void __rust_proto_thunk__Map_i64_vc_api_v1_UpdateVerifiableCredentialStatusRequest_free(const google::protobuf::Map<int64_t, ::vc_api::v1::UpdateVerifiableCredentialStatusRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_vc_api_v1_UpdateVerifiableCredentialStatusRequest_clear(google::protobuf::Map<int64_t, ::vc_api::v1::UpdateVerifiableCredentialStatusRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_vc_api_v1_UpdateVerifiableCredentialStatusRequest_size(const google::protobuf::Map<int64_t, ::vc_api::v1::UpdateVerifiableCredentialStatusRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_vc_api_v1_UpdateVerifiableCredentialStatusRequest_insert(google::protobuf::Map<int64_t, ::vc_api::v1::UpdateVerifiableCredentialStatusRequest> * m,
                          int64_t key, ::vc_api::v1::UpdateVerifiableCredentialStatusRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_vc_api_v1_UpdateVerifiableCredentialStatusRequest_get(const google::protobuf::Map<int64_t, ::vc_api::v1::UpdateVerifiableCredentialStatusRequest>* m,
                       int64_t key, const ::vc_api::v1::UpdateVerifiableCredentialStatusRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::UpdateVerifiableCredentialStatusRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_vc_api_v1_UpdateVerifiableCredentialStatusRequest_remove(google::protobuf::Map<int64_t, ::vc_api::v1::UpdateVerifiableCredentialStatusRequest> * m,
                          int64_t key, ::vc_api::v1::UpdateVerifiableCredentialStatusRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_vc_api_v1_UpdateVerifiableCredentialStatusRequest_iter(
      const google::protobuf::Map<int64_t, ::vc_api::v1::UpdateVerifiableCredentialStatusRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_vc_api_v1_UpdateVerifiableCredentialStatusRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::vc_api::v1::UpdateVerifiableCredentialStatusRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::vc_api::v1::UpdateVerifiableCredentialStatusRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::vc_api::v1::UpdateVerifiableCredentialStatusRequest>* __rust_proto_thunk__Map_u64_vc_api_v1_UpdateVerifiableCredentialStatusRequest_new() {
    return new google::protobuf::Map<uint64_t, ::vc_api::v1::UpdateVerifiableCredentialStatusRequest>();
  }
  void __rust_proto_thunk__Map_u64_vc_api_v1_UpdateVerifiableCredentialStatusRequest_free(const google::protobuf::Map<uint64_t, ::vc_api::v1::UpdateVerifiableCredentialStatusRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_vc_api_v1_UpdateVerifiableCredentialStatusRequest_clear(google::protobuf::Map<uint64_t, ::vc_api::v1::UpdateVerifiableCredentialStatusRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_vc_api_v1_UpdateVerifiableCredentialStatusRequest_size(const google::protobuf::Map<uint64_t, ::vc_api::v1::UpdateVerifiableCredentialStatusRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_vc_api_v1_UpdateVerifiableCredentialStatusRequest_insert(google::protobuf::Map<uint64_t, ::vc_api::v1::UpdateVerifiableCredentialStatusRequest> * m,
                          uint64_t key, ::vc_api::v1::UpdateVerifiableCredentialStatusRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_vc_api_v1_UpdateVerifiableCredentialStatusRequest_get(const google::protobuf::Map<uint64_t, ::vc_api::v1::UpdateVerifiableCredentialStatusRequest>* m,
                       uint64_t key, const ::vc_api::v1::UpdateVerifiableCredentialStatusRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::UpdateVerifiableCredentialStatusRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_vc_api_v1_UpdateVerifiableCredentialStatusRequest_remove(google::protobuf::Map<uint64_t, ::vc_api::v1::UpdateVerifiableCredentialStatusRequest> * m,
                          uint64_t key, ::vc_api::v1::UpdateVerifiableCredentialStatusRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_vc_api_v1_UpdateVerifiableCredentialStatusRequest_iter(
      const google::protobuf::Map<uint64_t, ::vc_api::v1::UpdateVerifiableCredentialStatusRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_vc_api_v1_UpdateVerifiableCredentialStatusRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::vc_api::v1::UpdateVerifiableCredentialStatusRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::vc_api::v1::UpdateVerifiableCredentialStatusRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::vc_api::v1::UpdateVerifiableCredentialStatusRequest>* __rust_proto_thunk__Map_bool_vc_api_v1_UpdateVerifiableCredentialStatusRequest_new() {
    return new google::protobuf::Map<bool, ::vc_api::v1::UpdateVerifiableCredentialStatusRequest>();
  }
  void __rust_proto_thunk__Map_bool_vc_api_v1_UpdateVerifiableCredentialStatusRequest_free(const google::protobuf::Map<bool, ::vc_api::v1::UpdateVerifiableCredentialStatusRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_vc_api_v1_UpdateVerifiableCredentialStatusRequest_clear(google::protobuf::Map<bool, ::vc_api::v1::UpdateVerifiableCredentialStatusRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_vc_api_v1_UpdateVerifiableCredentialStatusRequest_size(const google::protobuf::Map<bool, ::vc_api::v1::UpdateVerifiableCredentialStatusRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_vc_api_v1_UpdateVerifiableCredentialStatusRequest_insert(google::protobuf::Map<bool, ::vc_api::v1::UpdateVerifiableCredentialStatusRequest> * m,
                          bool key, ::vc_api::v1::UpdateVerifiableCredentialStatusRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_vc_api_v1_UpdateVerifiableCredentialStatusRequest_get(const google::protobuf::Map<bool, ::vc_api::v1::UpdateVerifiableCredentialStatusRequest>* m,
                       bool key, const ::vc_api::v1::UpdateVerifiableCredentialStatusRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::UpdateVerifiableCredentialStatusRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_vc_api_v1_UpdateVerifiableCredentialStatusRequest_remove(google::protobuf::Map<bool, ::vc_api::v1::UpdateVerifiableCredentialStatusRequest> * m,
                          bool key, ::vc_api::v1::UpdateVerifiableCredentialStatusRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_vc_api_v1_UpdateVerifiableCredentialStatusRequest_iter(
      const google::protobuf::Map<bool, ::vc_api::v1::UpdateVerifiableCredentialStatusRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_vc_api_v1_UpdateVerifiableCredentialStatusRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::vc_api::v1::UpdateVerifiableCredentialStatusRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::vc_api::v1::UpdateVerifiableCredentialStatusRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::vc_api::v1::UpdateVerifiableCredentialStatusRequest>* __rust_proto_thunk__Map_ProtoStr_vc_api_v1_UpdateVerifiableCredentialStatusRequest_new() {
    return new google::protobuf::Map<std::string, ::vc_api::v1::UpdateVerifiableCredentialStatusRequest>();
  }
  void __rust_proto_thunk__Map_ProtoStr_vc_api_v1_UpdateVerifiableCredentialStatusRequest_free(const google::protobuf::Map<std::string, ::vc_api::v1::UpdateVerifiableCredentialStatusRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_vc_api_v1_UpdateVerifiableCredentialStatusRequest_clear(google::protobuf::Map<std::string, ::vc_api::v1::UpdateVerifiableCredentialStatusRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_vc_api_v1_UpdateVerifiableCredentialStatusRequest_size(const google::protobuf::Map<std::string, ::vc_api::v1::UpdateVerifiableCredentialStatusRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_vc_api_v1_UpdateVerifiableCredentialStatusRequest_insert(google::protobuf::Map<std::string, ::vc_api::v1::UpdateVerifiableCredentialStatusRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::vc_api::v1::UpdateVerifiableCredentialStatusRequest value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_vc_api_v1_UpdateVerifiableCredentialStatusRequest_get(const google::protobuf::Map<std::string, ::vc_api::v1::UpdateVerifiableCredentialStatusRequest>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::vc_api::v1::UpdateVerifiableCredentialStatusRequest** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::UpdateVerifiableCredentialStatusRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_vc_api_v1_UpdateVerifiableCredentialStatusRequest_remove(google::protobuf::Map<std::string, ::vc_api::v1::UpdateVerifiableCredentialStatusRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::vc_api::v1::UpdateVerifiableCredentialStatusRequest * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_vc_api_v1_UpdateVerifiableCredentialStatusRequest_iter(
      const google::protobuf::Map<std::string, ::vc_api::v1::UpdateVerifiableCredentialStatusRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_vc_api_v1_UpdateVerifiableCredentialStatusRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::vc_api::v1::UpdateVerifiableCredentialStatusRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::vc_api::v1::UpdateVerifiableCredentialStatusRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// vc_api.v1.UpdateVerifiableCredentialStatusResponse
        // clang-format off
extern "C" {
void* __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusResponse_new() { return new ::vc_api::v1::UpdateVerifiableCredentialStatusResponse(); }
void __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusResponse_delete(void* ptr) { delete static_cast<::vc_api::v1::UpdateVerifiableCredentialStatusResponse*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusResponse_serialize(::vc_api::v1::UpdateVerifiableCredentialStatusResponse* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusResponse_parse(::vc_api::v1::UpdateVerifiableCredentialStatusResponse* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusResponse_copy_from(::vc_api::v1::UpdateVerifiableCredentialStatusResponse* dst, const ::vc_api::v1::UpdateVerifiableCredentialStatusResponse* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusResponse_repeated_len(google::protobuf::RepeatedPtrField<::vc_api::v1::UpdateVerifiableCredentialStatusResponse>* field) {
  return field->size();
}
const ::vc_api::v1::UpdateVerifiableCredentialStatusResponse& __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusResponse_repeated_get(
  google::protobuf::RepeatedPtrField<::vc_api::v1::UpdateVerifiableCredentialStatusResponse>* field,
  size_t index) {
  return field->Get(index);
}
::vc_api::v1::UpdateVerifiableCredentialStatusResponse* __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusResponse_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::vc_api::v1::UpdateVerifiableCredentialStatusResponse>* field,
  size_t index) {
  return field->Mutable(index);
}
::vc_api::v1::UpdateVerifiableCredentialStatusResponse* __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusResponse_repeated_add(google::protobuf::RepeatedPtrField<::vc_api::v1::UpdateVerifiableCredentialStatusResponse>* field) {
  return field->Add();
}
void __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusResponse_repeated_clear(google::protobuf::RepeatedPtrField<::vc_api::v1::UpdateVerifiableCredentialStatusResponse>* field) {
  field->Clear();
}
void __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusResponse_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::vc_api::v1::UpdateVerifiableCredentialStatusResponse>& dst,
  const google::protobuf::RepeatedPtrField<::vc_api::v1::UpdateVerifiableCredentialStatusResponse>& src) {
  dst = src;
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::vc_api::v1::UpdateVerifiableCredentialStatusResponse>* __rust_proto_thunk__Map_i32_vc_api_v1_UpdateVerifiableCredentialStatusResponse_new() {
    return new google::protobuf::Map<int32_t, ::vc_api::v1::UpdateVerifiableCredentialStatusResponse>();
  }
  void __rust_proto_thunk__Map_i32_vc_api_v1_UpdateVerifiableCredentialStatusResponse_free(const google::protobuf::Map<int32_t, ::vc_api::v1::UpdateVerifiableCredentialStatusResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_vc_api_v1_UpdateVerifiableCredentialStatusResponse_clear(google::protobuf::Map<int32_t, ::vc_api::v1::UpdateVerifiableCredentialStatusResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_vc_api_v1_UpdateVerifiableCredentialStatusResponse_size(const google::protobuf::Map<int32_t, ::vc_api::v1::UpdateVerifiableCredentialStatusResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_vc_api_v1_UpdateVerifiableCredentialStatusResponse_insert(google::protobuf::Map<int32_t, ::vc_api::v1::UpdateVerifiableCredentialStatusResponse> * m,
                          int32_t key, ::vc_api::v1::UpdateVerifiableCredentialStatusResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_vc_api_v1_UpdateVerifiableCredentialStatusResponse_get(const google::protobuf::Map<int32_t, ::vc_api::v1::UpdateVerifiableCredentialStatusResponse>* m,
                       int32_t key, const ::vc_api::v1::UpdateVerifiableCredentialStatusResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::UpdateVerifiableCredentialStatusResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_vc_api_v1_UpdateVerifiableCredentialStatusResponse_remove(google::protobuf::Map<int32_t, ::vc_api::v1::UpdateVerifiableCredentialStatusResponse> * m,
                          int32_t key, ::vc_api::v1::UpdateVerifiableCredentialStatusResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_vc_api_v1_UpdateVerifiableCredentialStatusResponse_iter(
      const google::protobuf::Map<int32_t, ::vc_api::v1::UpdateVerifiableCredentialStatusResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_vc_api_v1_UpdateVerifiableCredentialStatusResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::vc_api::v1::UpdateVerifiableCredentialStatusResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::vc_api::v1::UpdateVerifiableCredentialStatusResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::vc_api::v1::UpdateVerifiableCredentialStatusResponse>* __rust_proto_thunk__Map_u32_vc_api_v1_UpdateVerifiableCredentialStatusResponse_new() {
    return new google::protobuf::Map<uint32_t, ::vc_api::v1::UpdateVerifiableCredentialStatusResponse>();
  }
  void __rust_proto_thunk__Map_u32_vc_api_v1_UpdateVerifiableCredentialStatusResponse_free(const google::protobuf::Map<uint32_t, ::vc_api::v1::UpdateVerifiableCredentialStatusResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_vc_api_v1_UpdateVerifiableCredentialStatusResponse_clear(google::protobuf::Map<uint32_t, ::vc_api::v1::UpdateVerifiableCredentialStatusResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_vc_api_v1_UpdateVerifiableCredentialStatusResponse_size(const google::protobuf::Map<uint32_t, ::vc_api::v1::UpdateVerifiableCredentialStatusResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_vc_api_v1_UpdateVerifiableCredentialStatusResponse_insert(google::protobuf::Map<uint32_t, ::vc_api::v1::UpdateVerifiableCredentialStatusResponse> * m,
                          uint32_t key, ::vc_api::v1::UpdateVerifiableCredentialStatusResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_vc_api_v1_UpdateVerifiableCredentialStatusResponse_get(const google::protobuf::Map<uint32_t, ::vc_api::v1::UpdateVerifiableCredentialStatusResponse>* m,
                       uint32_t key, const ::vc_api::v1::UpdateVerifiableCredentialStatusResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::UpdateVerifiableCredentialStatusResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_vc_api_v1_UpdateVerifiableCredentialStatusResponse_remove(google::protobuf::Map<uint32_t, ::vc_api::v1::UpdateVerifiableCredentialStatusResponse> * m,
                          uint32_t key, ::vc_api::v1::UpdateVerifiableCredentialStatusResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_vc_api_v1_UpdateVerifiableCredentialStatusResponse_iter(
      const google::protobuf::Map<uint32_t, ::vc_api::v1::UpdateVerifiableCredentialStatusResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_vc_api_v1_UpdateVerifiableCredentialStatusResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::vc_api::v1::UpdateVerifiableCredentialStatusResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::vc_api::v1::UpdateVerifiableCredentialStatusResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::vc_api::v1::UpdateVerifiableCredentialStatusResponse>* __rust_proto_thunk__Map_i64_vc_api_v1_UpdateVerifiableCredentialStatusResponse_new() {
    return new google::protobuf::Map<int64_t, ::vc_api::v1::UpdateVerifiableCredentialStatusResponse>();
  }
  void __rust_proto_thunk__Map_i64_vc_api_v1_UpdateVerifiableCredentialStatusResponse_free(const google::protobuf::Map<int64_t, ::vc_api::v1::UpdateVerifiableCredentialStatusResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_vc_api_v1_UpdateVerifiableCredentialStatusResponse_clear(google::protobuf::Map<int64_t, ::vc_api::v1::UpdateVerifiableCredentialStatusResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_vc_api_v1_UpdateVerifiableCredentialStatusResponse_size(const google::protobuf::Map<int64_t, ::vc_api::v1::UpdateVerifiableCredentialStatusResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_vc_api_v1_UpdateVerifiableCredentialStatusResponse_insert(google::protobuf::Map<int64_t, ::vc_api::v1::UpdateVerifiableCredentialStatusResponse> * m,
                          int64_t key, ::vc_api::v1::UpdateVerifiableCredentialStatusResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_vc_api_v1_UpdateVerifiableCredentialStatusResponse_get(const google::protobuf::Map<int64_t, ::vc_api::v1::UpdateVerifiableCredentialStatusResponse>* m,
                       int64_t key, const ::vc_api::v1::UpdateVerifiableCredentialStatusResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::UpdateVerifiableCredentialStatusResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_vc_api_v1_UpdateVerifiableCredentialStatusResponse_remove(google::protobuf::Map<int64_t, ::vc_api::v1::UpdateVerifiableCredentialStatusResponse> * m,
                          int64_t key, ::vc_api::v1::UpdateVerifiableCredentialStatusResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_vc_api_v1_UpdateVerifiableCredentialStatusResponse_iter(
      const google::protobuf::Map<int64_t, ::vc_api::v1::UpdateVerifiableCredentialStatusResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_vc_api_v1_UpdateVerifiableCredentialStatusResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::vc_api::v1::UpdateVerifiableCredentialStatusResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::vc_api::v1::UpdateVerifiableCredentialStatusResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::vc_api::v1::UpdateVerifiableCredentialStatusResponse>* __rust_proto_thunk__Map_u64_vc_api_v1_UpdateVerifiableCredentialStatusResponse_new() {
    return new google::protobuf::Map<uint64_t, ::vc_api::v1::UpdateVerifiableCredentialStatusResponse>();
  }
  void __rust_proto_thunk__Map_u64_vc_api_v1_UpdateVerifiableCredentialStatusResponse_free(const google::protobuf::Map<uint64_t, ::vc_api::v1::UpdateVerifiableCredentialStatusResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_vc_api_v1_UpdateVerifiableCredentialStatusResponse_clear(google::protobuf::Map<uint64_t, ::vc_api::v1::UpdateVerifiableCredentialStatusResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_vc_api_v1_UpdateVerifiableCredentialStatusResponse_size(const google::protobuf::Map<uint64_t, ::vc_api::v1::UpdateVerifiableCredentialStatusResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_vc_api_v1_UpdateVerifiableCredentialStatusResponse_insert(google::protobuf::Map<uint64_t, ::vc_api::v1::UpdateVerifiableCredentialStatusResponse> * m,
                          uint64_t key, ::vc_api::v1::UpdateVerifiableCredentialStatusResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_vc_api_v1_UpdateVerifiableCredentialStatusResponse_get(const google::protobuf::Map<uint64_t, ::vc_api::v1::UpdateVerifiableCredentialStatusResponse>* m,
                       uint64_t key, const ::vc_api::v1::UpdateVerifiableCredentialStatusResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::UpdateVerifiableCredentialStatusResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_vc_api_v1_UpdateVerifiableCredentialStatusResponse_remove(google::protobuf::Map<uint64_t, ::vc_api::v1::UpdateVerifiableCredentialStatusResponse> * m,
                          uint64_t key, ::vc_api::v1::UpdateVerifiableCredentialStatusResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_vc_api_v1_UpdateVerifiableCredentialStatusResponse_iter(
      const google::protobuf::Map<uint64_t, ::vc_api::v1::UpdateVerifiableCredentialStatusResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_vc_api_v1_UpdateVerifiableCredentialStatusResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::vc_api::v1::UpdateVerifiableCredentialStatusResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::vc_api::v1::UpdateVerifiableCredentialStatusResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::vc_api::v1::UpdateVerifiableCredentialStatusResponse>* __rust_proto_thunk__Map_bool_vc_api_v1_UpdateVerifiableCredentialStatusResponse_new() {
    return new google::protobuf::Map<bool, ::vc_api::v1::UpdateVerifiableCredentialStatusResponse>();
  }
  void __rust_proto_thunk__Map_bool_vc_api_v1_UpdateVerifiableCredentialStatusResponse_free(const google::protobuf::Map<bool, ::vc_api::v1::UpdateVerifiableCredentialStatusResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_vc_api_v1_UpdateVerifiableCredentialStatusResponse_clear(google::protobuf::Map<bool, ::vc_api::v1::UpdateVerifiableCredentialStatusResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_vc_api_v1_UpdateVerifiableCredentialStatusResponse_size(const google::protobuf::Map<bool, ::vc_api::v1::UpdateVerifiableCredentialStatusResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_vc_api_v1_UpdateVerifiableCredentialStatusResponse_insert(google::protobuf::Map<bool, ::vc_api::v1::UpdateVerifiableCredentialStatusResponse> * m,
                          bool key, ::vc_api::v1::UpdateVerifiableCredentialStatusResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_vc_api_v1_UpdateVerifiableCredentialStatusResponse_get(const google::protobuf::Map<bool, ::vc_api::v1::UpdateVerifiableCredentialStatusResponse>* m,
                       bool key, const ::vc_api::v1::UpdateVerifiableCredentialStatusResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::UpdateVerifiableCredentialStatusResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_vc_api_v1_UpdateVerifiableCredentialStatusResponse_remove(google::protobuf::Map<bool, ::vc_api::v1::UpdateVerifiableCredentialStatusResponse> * m,
                          bool key, ::vc_api::v1::UpdateVerifiableCredentialStatusResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_vc_api_v1_UpdateVerifiableCredentialStatusResponse_iter(
      const google::protobuf::Map<bool, ::vc_api::v1::UpdateVerifiableCredentialStatusResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_vc_api_v1_UpdateVerifiableCredentialStatusResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::vc_api::v1::UpdateVerifiableCredentialStatusResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::vc_api::v1::UpdateVerifiableCredentialStatusResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::vc_api::v1::UpdateVerifiableCredentialStatusResponse>* __rust_proto_thunk__Map_ProtoStr_vc_api_v1_UpdateVerifiableCredentialStatusResponse_new() {
    return new google::protobuf::Map<std::string, ::vc_api::v1::UpdateVerifiableCredentialStatusResponse>();
  }
  void __rust_proto_thunk__Map_ProtoStr_vc_api_v1_UpdateVerifiableCredentialStatusResponse_free(const google::protobuf::Map<std::string, ::vc_api::v1::UpdateVerifiableCredentialStatusResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_vc_api_v1_UpdateVerifiableCredentialStatusResponse_clear(google::protobuf::Map<std::string, ::vc_api::v1::UpdateVerifiableCredentialStatusResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_vc_api_v1_UpdateVerifiableCredentialStatusResponse_size(const google::protobuf::Map<std::string, ::vc_api::v1::UpdateVerifiableCredentialStatusResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_vc_api_v1_UpdateVerifiableCredentialStatusResponse_insert(google::protobuf::Map<std::string, ::vc_api::v1::UpdateVerifiableCredentialStatusResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::vc_api::v1::UpdateVerifiableCredentialStatusResponse value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_vc_api_v1_UpdateVerifiableCredentialStatusResponse_get(const google::protobuf::Map<std::string, ::vc_api::v1::UpdateVerifiableCredentialStatusResponse>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::vc_api::v1::UpdateVerifiableCredentialStatusResponse** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::vc_api::v1::UpdateVerifiableCredentialStatusResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_vc_api_v1_UpdateVerifiableCredentialStatusResponse_remove(google::protobuf::Map<std::string, ::vc_api::v1::UpdateVerifiableCredentialStatusResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::vc_api::v1::UpdateVerifiableCredentialStatusResponse * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_vc_api_v1_UpdateVerifiableCredentialStatusResponse_iter(
      const google::protobuf::Map<std::string, ::vc_api::v1::UpdateVerifiableCredentialStatusResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_vc_api_v1_UpdateVerifiableCredentialStatusResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::vc_api::v1::UpdateVerifiableCredentialStatusResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::vc_api::v1::UpdateVerifiableCredentialStatusResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// vc_api.v1.CredentialType
extern "C" {
  __PB_RUST_EXPOSE_SCALAR_MAP_METHODS_FOR_VALUE_TYPE(
      ::vc_api::v1::CredentialType, vc_api_v1_CredentialType, ::vc_api::v1::CredentialType, value, cpp_value)
}

// vc_api.v1.UpdateVerifiableCredentialStatusType
extern "C" {
  __PB_RUST_EXPOSE_SCALAR_MAP_METHODS_FOR_VALUE_TYPE(
      ::vc_api::v1::UpdateVerifiableCredentialStatusType, vc_api_v1_UpdateVerifiableCredentialStatusType, ::vc_api::v1::UpdateVerifiableCredentialStatusType, value, cpp_value)
}

