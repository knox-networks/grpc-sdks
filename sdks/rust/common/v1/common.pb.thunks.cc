#include "common/v1/common.proto.h"
#include "google/protobuf/rust/cpp_kernel/cpp_api.h"
// common.DynamicVerifier
        // clang-format off
extern "C" {
void* __rust_proto_thunk__common_DynamicVerifier_new() { return new ::common::DynamicVerifier(); }
void __rust_proto_thunk__common_DynamicVerifier_delete(void* ptr) { delete static_cast<::common::DynamicVerifier*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__common_DynamicVerifier_serialize(::common::DynamicVerifier* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__common_DynamicVerifier_parse(::common::DynamicVerifier* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__common_DynamicVerifier_copy_from(::common::DynamicVerifier* dst, const ::common::DynamicVerifier* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__common_DynamicVerifier_repeated_len(google::protobuf::RepeatedPtrField<::common::DynamicVerifier>* field) {
  return field->size();
}
const ::common::DynamicVerifier& __rust_proto_thunk__common_DynamicVerifier_repeated_get(
  google::protobuf::RepeatedPtrField<::common::DynamicVerifier>* field,
  size_t index) {
  return field->Get(index);
}
::common::DynamicVerifier* __rust_proto_thunk__common_DynamicVerifier_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::common::DynamicVerifier>* field,
  size_t index) {
  return field->Mutable(index);
}
::common::DynamicVerifier* __rust_proto_thunk__common_DynamicVerifier_repeated_add(google::protobuf::RepeatedPtrField<::common::DynamicVerifier>* field) {
  return field->Add();
}
void __rust_proto_thunk__common_DynamicVerifier_repeated_clear(google::protobuf::RepeatedPtrField<::common::DynamicVerifier>* field) {
  field->Clear();
}
void __rust_proto_thunk__common_DynamicVerifier_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::common::DynamicVerifier>& dst,
  const google::protobuf::RepeatedPtrField<::common::DynamicVerifier>& src) {
  dst = src;
}

::common::SignatureSystem __rust_proto_thunk__common_DynamicVerifier_get_signature_system(::common::DynamicVerifier* msg) { return msg->signature_system(); }
void __rust_proto_thunk__common_DynamicVerifier_set_signature_system(::common::DynamicVerifier* msg, ::common::SignatureSystem val) {
  msg->set_signature_system(val);
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__common_DynamicVerifier_get_verifier(::common::DynamicVerifier* msg) {
  absl::string_view val = msg->verifier();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__common_DynamicVerifier_set_verifier(::common::DynamicVerifier* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_verifier(absl::string_view(s.ptr, s.len));
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::common::DynamicVerifier>* __rust_proto_thunk__Map_i32_common_DynamicVerifier_new() {
    return new google::protobuf::Map<int32_t, ::common::DynamicVerifier>();
  }
  void __rust_proto_thunk__Map_i32_common_DynamicVerifier_free(const google::protobuf::Map<int32_t, ::common::DynamicVerifier>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_common_DynamicVerifier_clear(google::protobuf::Map<int32_t, ::common::DynamicVerifier> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_common_DynamicVerifier_size(const google::protobuf::Map<int32_t, ::common::DynamicVerifier>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_common_DynamicVerifier_insert(google::protobuf::Map<int32_t, ::common::DynamicVerifier> * m,
                          int32_t key, ::common::DynamicVerifier value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_DynamicVerifier_get(const google::protobuf::Map<int32_t, ::common::DynamicVerifier>* m,
                       int32_t key, const ::common::DynamicVerifier** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::DynamicVerifier* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_DynamicVerifier_remove(google::protobuf::Map<int32_t, ::common::DynamicVerifier> * m,
                          int32_t key, ::common::DynamicVerifier * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_common_DynamicVerifier_iter(
      const google::protobuf::Map<int32_t, ::common::DynamicVerifier>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_common_DynamicVerifier_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::common::DynamicVerifier** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::common::DynamicVerifier>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::common::DynamicVerifier>* __rust_proto_thunk__Map_u32_common_DynamicVerifier_new() {
    return new google::protobuf::Map<uint32_t, ::common::DynamicVerifier>();
  }
  void __rust_proto_thunk__Map_u32_common_DynamicVerifier_free(const google::protobuf::Map<uint32_t, ::common::DynamicVerifier>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_common_DynamicVerifier_clear(google::protobuf::Map<uint32_t, ::common::DynamicVerifier> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_common_DynamicVerifier_size(const google::protobuf::Map<uint32_t, ::common::DynamicVerifier>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_common_DynamicVerifier_insert(google::protobuf::Map<uint32_t, ::common::DynamicVerifier> * m,
                          uint32_t key, ::common::DynamicVerifier value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_DynamicVerifier_get(const google::protobuf::Map<uint32_t, ::common::DynamicVerifier>* m,
                       uint32_t key, const ::common::DynamicVerifier** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::DynamicVerifier* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_DynamicVerifier_remove(google::protobuf::Map<uint32_t, ::common::DynamicVerifier> * m,
                          uint32_t key, ::common::DynamicVerifier * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_common_DynamicVerifier_iter(
      const google::protobuf::Map<uint32_t, ::common::DynamicVerifier>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_common_DynamicVerifier_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::common::DynamicVerifier** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::common::DynamicVerifier>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::common::DynamicVerifier>* __rust_proto_thunk__Map_i64_common_DynamicVerifier_new() {
    return new google::protobuf::Map<int64_t, ::common::DynamicVerifier>();
  }
  void __rust_proto_thunk__Map_i64_common_DynamicVerifier_free(const google::protobuf::Map<int64_t, ::common::DynamicVerifier>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_common_DynamicVerifier_clear(google::protobuf::Map<int64_t, ::common::DynamicVerifier> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_common_DynamicVerifier_size(const google::protobuf::Map<int64_t, ::common::DynamicVerifier>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_common_DynamicVerifier_insert(google::protobuf::Map<int64_t, ::common::DynamicVerifier> * m,
                          int64_t key, ::common::DynamicVerifier value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_DynamicVerifier_get(const google::protobuf::Map<int64_t, ::common::DynamicVerifier>* m,
                       int64_t key, const ::common::DynamicVerifier** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::DynamicVerifier* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_DynamicVerifier_remove(google::protobuf::Map<int64_t, ::common::DynamicVerifier> * m,
                          int64_t key, ::common::DynamicVerifier * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_common_DynamicVerifier_iter(
      const google::protobuf::Map<int64_t, ::common::DynamicVerifier>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_common_DynamicVerifier_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::common::DynamicVerifier** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::common::DynamicVerifier>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::common::DynamicVerifier>* __rust_proto_thunk__Map_u64_common_DynamicVerifier_new() {
    return new google::protobuf::Map<uint64_t, ::common::DynamicVerifier>();
  }
  void __rust_proto_thunk__Map_u64_common_DynamicVerifier_free(const google::protobuf::Map<uint64_t, ::common::DynamicVerifier>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_common_DynamicVerifier_clear(google::protobuf::Map<uint64_t, ::common::DynamicVerifier> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_common_DynamicVerifier_size(const google::protobuf::Map<uint64_t, ::common::DynamicVerifier>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_common_DynamicVerifier_insert(google::protobuf::Map<uint64_t, ::common::DynamicVerifier> * m,
                          uint64_t key, ::common::DynamicVerifier value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_DynamicVerifier_get(const google::protobuf::Map<uint64_t, ::common::DynamicVerifier>* m,
                       uint64_t key, const ::common::DynamicVerifier** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::DynamicVerifier* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_DynamicVerifier_remove(google::protobuf::Map<uint64_t, ::common::DynamicVerifier> * m,
                          uint64_t key, ::common::DynamicVerifier * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_common_DynamicVerifier_iter(
      const google::protobuf::Map<uint64_t, ::common::DynamicVerifier>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_common_DynamicVerifier_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::common::DynamicVerifier** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::common::DynamicVerifier>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::common::DynamicVerifier>* __rust_proto_thunk__Map_bool_common_DynamicVerifier_new() {
    return new google::protobuf::Map<bool, ::common::DynamicVerifier>();
  }
  void __rust_proto_thunk__Map_bool_common_DynamicVerifier_free(const google::protobuf::Map<bool, ::common::DynamicVerifier>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_common_DynamicVerifier_clear(google::protobuf::Map<bool, ::common::DynamicVerifier> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_common_DynamicVerifier_size(const google::protobuf::Map<bool, ::common::DynamicVerifier>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_common_DynamicVerifier_insert(google::protobuf::Map<bool, ::common::DynamicVerifier> * m,
                          bool key, ::common::DynamicVerifier value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_DynamicVerifier_get(const google::protobuf::Map<bool, ::common::DynamicVerifier>* m,
                       bool key, const ::common::DynamicVerifier** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::DynamicVerifier* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_DynamicVerifier_remove(google::protobuf::Map<bool, ::common::DynamicVerifier> * m,
                          bool key, ::common::DynamicVerifier * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_common_DynamicVerifier_iter(
      const google::protobuf::Map<bool, ::common::DynamicVerifier>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_common_DynamicVerifier_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::common::DynamicVerifier** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::common::DynamicVerifier>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::common::DynamicVerifier>* __rust_proto_thunk__Map_ProtoStr_common_DynamicVerifier_new() {
    return new google::protobuf::Map<std::string, ::common::DynamicVerifier>();
  }
  void __rust_proto_thunk__Map_ProtoStr_common_DynamicVerifier_free(const google::protobuf::Map<std::string, ::common::DynamicVerifier>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_common_DynamicVerifier_clear(google::protobuf::Map<std::string, ::common::DynamicVerifier> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_common_DynamicVerifier_size(const google::protobuf::Map<std::string, ::common::DynamicVerifier>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_DynamicVerifier_insert(google::protobuf::Map<std::string, ::common::DynamicVerifier> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::DynamicVerifier value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_DynamicVerifier_get(const google::protobuf::Map<std::string, ::common::DynamicVerifier>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::common::DynamicVerifier** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::common::DynamicVerifier* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_DynamicVerifier_remove(google::protobuf::Map<std::string, ::common::DynamicVerifier> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::DynamicVerifier * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_common_DynamicVerifier_iter(
      const google::protobuf::Map<std::string, ::common::DynamicVerifier>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_common_DynamicVerifier_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::common::DynamicVerifier** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::common::DynamicVerifier>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// common.Amount
        // clang-format off
extern "C" {
void* __rust_proto_thunk__common_Amount_new() { return new ::common::Amount(); }
void __rust_proto_thunk__common_Amount_delete(void* ptr) { delete static_cast<::common::Amount*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__common_Amount_serialize(::common::Amount* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__common_Amount_parse(::common::Amount* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__common_Amount_copy_from(::common::Amount* dst, const ::common::Amount* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__common_Amount_repeated_len(google::protobuf::RepeatedPtrField<::common::Amount>* field) {
  return field->size();
}
const ::common::Amount& __rust_proto_thunk__common_Amount_repeated_get(
  google::protobuf::RepeatedPtrField<::common::Amount>* field,
  size_t index) {
  return field->Get(index);
}
::common::Amount* __rust_proto_thunk__common_Amount_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::common::Amount>* field,
  size_t index) {
  return field->Mutable(index);
}
::common::Amount* __rust_proto_thunk__common_Amount_repeated_add(google::protobuf::RepeatedPtrField<::common::Amount>* field) {
  return field->Add();
}
void __rust_proto_thunk__common_Amount_repeated_clear(google::protobuf::RepeatedPtrField<::common::Amount>* field) {
  field->Clear();
}
void __rust_proto_thunk__common_Amount_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::common::Amount>& dst,
  const google::protobuf::RepeatedPtrField<::common::Amount>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__common_Amount_get_currency_code(::common::Amount* msg) {
  absl::string_view val = msg->currency_code();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__common_Amount_set_currency_code(::common::Amount* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_currency_code(absl::string_view(s.ptr, s.len));
}

::int64_t __rust_proto_thunk__common_Amount_get_amount(::common::Amount* msg) { return msg->amount(); }
void __rust_proto_thunk__common_Amount_set_amount(::common::Amount* msg, ::int64_t val) {
  msg->set_amount(val);
}

::uint32_t __rust_proto_thunk__common_Amount_get_decimals(::common::Amount* msg) { return msg->decimals(); }
void __rust_proto_thunk__common_Amount_set_decimals(::common::Amount* msg, ::uint32_t val) {
  msg->set_decimals(val);
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::common::Amount>* __rust_proto_thunk__Map_i32_common_Amount_new() {
    return new google::protobuf::Map<int32_t, ::common::Amount>();
  }
  void __rust_proto_thunk__Map_i32_common_Amount_free(const google::protobuf::Map<int32_t, ::common::Amount>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_common_Amount_clear(google::protobuf::Map<int32_t, ::common::Amount> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_common_Amount_size(const google::protobuf::Map<int32_t, ::common::Amount>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_common_Amount_insert(google::protobuf::Map<int32_t, ::common::Amount> * m,
                          int32_t key, ::common::Amount value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_Amount_get(const google::protobuf::Map<int32_t, ::common::Amount>* m,
                       int32_t key, const ::common::Amount** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::Amount* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_Amount_remove(google::protobuf::Map<int32_t, ::common::Amount> * m,
                          int32_t key, ::common::Amount * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_common_Amount_iter(
      const google::protobuf::Map<int32_t, ::common::Amount>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_common_Amount_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::common::Amount** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::common::Amount>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::common::Amount>* __rust_proto_thunk__Map_u32_common_Amount_new() {
    return new google::protobuf::Map<uint32_t, ::common::Amount>();
  }
  void __rust_proto_thunk__Map_u32_common_Amount_free(const google::protobuf::Map<uint32_t, ::common::Amount>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_common_Amount_clear(google::protobuf::Map<uint32_t, ::common::Amount> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_common_Amount_size(const google::protobuf::Map<uint32_t, ::common::Amount>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_common_Amount_insert(google::protobuf::Map<uint32_t, ::common::Amount> * m,
                          uint32_t key, ::common::Amount value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_Amount_get(const google::protobuf::Map<uint32_t, ::common::Amount>* m,
                       uint32_t key, const ::common::Amount** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::Amount* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_Amount_remove(google::protobuf::Map<uint32_t, ::common::Amount> * m,
                          uint32_t key, ::common::Amount * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_common_Amount_iter(
      const google::protobuf::Map<uint32_t, ::common::Amount>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_common_Amount_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::common::Amount** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::common::Amount>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::common::Amount>* __rust_proto_thunk__Map_i64_common_Amount_new() {
    return new google::protobuf::Map<int64_t, ::common::Amount>();
  }
  void __rust_proto_thunk__Map_i64_common_Amount_free(const google::protobuf::Map<int64_t, ::common::Amount>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_common_Amount_clear(google::protobuf::Map<int64_t, ::common::Amount> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_common_Amount_size(const google::protobuf::Map<int64_t, ::common::Amount>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_common_Amount_insert(google::protobuf::Map<int64_t, ::common::Amount> * m,
                          int64_t key, ::common::Amount value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_Amount_get(const google::protobuf::Map<int64_t, ::common::Amount>* m,
                       int64_t key, const ::common::Amount** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::Amount* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_Amount_remove(google::protobuf::Map<int64_t, ::common::Amount> * m,
                          int64_t key, ::common::Amount * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_common_Amount_iter(
      const google::protobuf::Map<int64_t, ::common::Amount>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_common_Amount_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::common::Amount** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::common::Amount>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::common::Amount>* __rust_proto_thunk__Map_u64_common_Amount_new() {
    return new google::protobuf::Map<uint64_t, ::common::Amount>();
  }
  void __rust_proto_thunk__Map_u64_common_Amount_free(const google::protobuf::Map<uint64_t, ::common::Amount>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_common_Amount_clear(google::protobuf::Map<uint64_t, ::common::Amount> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_common_Amount_size(const google::protobuf::Map<uint64_t, ::common::Amount>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_common_Amount_insert(google::protobuf::Map<uint64_t, ::common::Amount> * m,
                          uint64_t key, ::common::Amount value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_Amount_get(const google::protobuf::Map<uint64_t, ::common::Amount>* m,
                       uint64_t key, const ::common::Amount** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::Amount* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_Amount_remove(google::protobuf::Map<uint64_t, ::common::Amount> * m,
                          uint64_t key, ::common::Amount * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_common_Amount_iter(
      const google::protobuf::Map<uint64_t, ::common::Amount>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_common_Amount_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::common::Amount** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::common::Amount>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::common::Amount>* __rust_proto_thunk__Map_bool_common_Amount_new() {
    return new google::protobuf::Map<bool, ::common::Amount>();
  }
  void __rust_proto_thunk__Map_bool_common_Amount_free(const google::protobuf::Map<bool, ::common::Amount>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_common_Amount_clear(google::protobuf::Map<bool, ::common::Amount> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_common_Amount_size(const google::protobuf::Map<bool, ::common::Amount>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_common_Amount_insert(google::protobuf::Map<bool, ::common::Amount> * m,
                          bool key, ::common::Amount value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_Amount_get(const google::protobuf::Map<bool, ::common::Amount>* m,
                       bool key, const ::common::Amount** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::Amount* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_Amount_remove(google::protobuf::Map<bool, ::common::Amount> * m,
                          bool key, ::common::Amount * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_common_Amount_iter(
      const google::protobuf::Map<bool, ::common::Amount>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_common_Amount_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::common::Amount** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::common::Amount>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::common::Amount>* __rust_proto_thunk__Map_ProtoStr_common_Amount_new() {
    return new google::protobuf::Map<std::string, ::common::Amount>();
  }
  void __rust_proto_thunk__Map_ProtoStr_common_Amount_free(const google::protobuf::Map<std::string, ::common::Amount>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_common_Amount_clear(google::protobuf::Map<std::string, ::common::Amount> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_common_Amount_size(const google::protobuf::Map<std::string, ::common::Amount>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_Amount_insert(google::protobuf::Map<std::string, ::common::Amount> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::Amount value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_Amount_get(const google::protobuf::Map<std::string, ::common::Amount>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::common::Amount** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::common::Amount* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_Amount_remove(google::protobuf::Map<std::string, ::common::Amount> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::Amount * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_common_Amount_iter(
      const google::protobuf::Map<std::string, ::common::Amount>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_common_Amount_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::common::Amount** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::common::Amount>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// common.Distribution
        // clang-format off
extern "C" {
void* __rust_proto_thunk__common_Distribution_new() { return new ::common::Distribution(); }
void __rust_proto_thunk__common_Distribution_delete(void* ptr) { delete static_cast<::common::Distribution*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__common_Distribution_serialize(::common::Distribution* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__common_Distribution_parse(::common::Distribution* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__common_Distribution_copy_from(::common::Distribution* dst, const ::common::Distribution* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__common_Distribution_repeated_len(google::protobuf::RepeatedPtrField<::common::Distribution>* field) {
  return field->size();
}
const ::common::Distribution& __rust_proto_thunk__common_Distribution_repeated_get(
  google::protobuf::RepeatedPtrField<::common::Distribution>* field,
  size_t index) {
  return field->Get(index);
}
::common::Distribution* __rust_proto_thunk__common_Distribution_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::common::Distribution>* field,
  size_t index) {
  return field->Mutable(index);
}
::common::Distribution* __rust_proto_thunk__common_Distribution_repeated_add(google::protobuf::RepeatedPtrField<::common::Distribution>* field) {
  return field->Add();
}
void __rust_proto_thunk__common_Distribution_repeated_clear(google::protobuf::RepeatedPtrField<::common::Distribution>* field) {
  field->Clear();
}
void __rust_proto_thunk__common_Distribution_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::common::Distribution>& dst,
  const google::protobuf::RepeatedPtrField<::common::Distribution>& src) {
  dst = src;
}

const void* __rust_proto_thunk__common_Distribution_get_value(const ::common::Distribution* msg) {
  return &msg->value();
}
void* __rust_proto_thunk__common_Distribution_get_mut_value(::common::Distribution* msg) { return msg->mutable_value(); }


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::common::Distribution>* __rust_proto_thunk__Map_i32_common_Distribution_new() {
    return new google::protobuf::Map<int32_t, ::common::Distribution>();
  }
  void __rust_proto_thunk__Map_i32_common_Distribution_free(const google::protobuf::Map<int32_t, ::common::Distribution>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_common_Distribution_clear(google::protobuf::Map<int32_t, ::common::Distribution> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_common_Distribution_size(const google::protobuf::Map<int32_t, ::common::Distribution>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_common_Distribution_insert(google::protobuf::Map<int32_t, ::common::Distribution> * m,
                          int32_t key, ::common::Distribution value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_Distribution_get(const google::protobuf::Map<int32_t, ::common::Distribution>* m,
                       int32_t key, const ::common::Distribution** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::Distribution* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_Distribution_remove(google::protobuf::Map<int32_t, ::common::Distribution> * m,
                          int32_t key, ::common::Distribution * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_common_Distribution_iter(
      const google::protobuf::Map<int32_t, ::common::Distribution>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_common_Distribution_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::common::Distribution** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::common::Distribution>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::common::Distribution>* __rust_proto_thunk__Map_u32_common_Distribution_new() {
    return new google::protobuf::Map<uint32_t, ::common::Distribution>();
  }
  void __rust_proto_thunk__Map_u32_common_Distribution_free(const google::protobuf::Map<uint32_t, ::common::Distribution>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_common_Distribution_clear(google::protobuf::Map<uint32_t, ::common::Distribution> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_common_Distribution_size(const google::protobuf::Map<uint32_t, ::common::Distribution>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_common_Distribution_insert(google::protobuf::Map<uint32_t, ::common::Distribution> * m,
                          uint32_t key, ::common::Distribution value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_Distribution_get(const google::protobuf::Map<uint32_t, ::common::Distribution>* m,
                       uint32_t key, const ::common::Distribution** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::Distribution* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_Distribution_remove(google::protobuf::Map<uint32_t, ::common::Distribution> * m,
                          uint32_t key, ::common::Distribution * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_common_Distribution_iter(
      const google::protobuf::Map<uint32_t, ::common::Distribution>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_common_Distribution_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::common::Distribution** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::common::Distribution>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::common::Distribution>* __rust_proto_thunk__Map_i64_common_Distribution_new() {
    return new google::protobuf::Map<int64_t, ::common::Distribution>();
  }
  void __rust_proto_thunk__Map_i64_common_Distribution_free(const google::protobuf::Map<int64_t, ::common::Distribution>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_common_Distribution_clear(google::protobuf::Map<int64_t, ::common::Distribution> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_common_Distribution_size(const google::protobuf::Map<int64_t, ::common::Distribution>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_common_Distribution_insert(google::protobuf::Map<int64_t, ::common::Distribution> * m,
                          int64_t key, ::common::Distribution value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_Distribution_get(const google::protobuf::Map<int64_t, ::common::Distribution>* m,
                       int64_t key, const ::common::Distribution** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::Distribution* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_Distribution_remove(google::protobuf::Map<int64_t, ::common::Distribution> * m,
                          int64_t key, ::common::Distribution * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_common_Distribution_iter(
      const google::protobuf::Map<int64_t, ::common::Distribution>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_common_Distribution_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::common::Distribution** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::common::Distribution>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::common::Distribution>* __rust_proto_thunk__Map_u64_common_Distribution_new() {
    return new google::protobuf::Map<uint64_t, ::common::Distribution>();
  }
  void __rust_proto_thunk__Map_u64_common_Distribution_free(const google::protobuf::Map<uint64_t, ::common::Distribution>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_common_Distribution_clear(google::protobuf::Map<uint64_t, ::common::Distribution> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_common_Distribution_size(const google::protobuf::Map<uint64_t, ::common::Distribution>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_common_Distribution_insert(google::protobuf::Map<uint64_t, ::common::Distribution> * m,
                          uint64_t key, ::common::Distribution value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_Distribution_get(const google::protobuf::Map<uint64_t, ::common::Distribution>* m,
                       uint64_t key, const ::common::Distribution** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::Distribution* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_Distribution_remove(google::protobuf::Map<uint64_t, ::common::Distribution> * m,
                          uint64_t key, ::common::Distribution * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_common_Distribution_iter(
      const google::protobuf::Map<uint64_t, ::common::Distribution>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_common_Distribution_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::common::Distribution** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::common::Distribution>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::common::Distribution>* __rust_proto_thunk__Map_bool_common_Distribution_new() {
    return new google::protobuf::Map<bool, ::common::Distribution>();
  }
  void __rust_proto_thunk__Map_bool_common_Distribution_free(const google::protobuf::Map<bool, ::common::Distribution>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_common_Distribution_clear(google::protobuf::Map<bool, ::common::Distribution> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_common_Distribution_size(const google::protobuf::Map<bool, ::common::Distribution>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_common_Distribution_insert(google::protobuf::Map<bool, ::common::Distribution> * m,
                          bool key, ::common::Distribution value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_Distribution_get(const google::protobuf::Map<bool, ::common::Distribution>* m,
                       bool key, const ::common::Distribution** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::Distribution* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_Distribution_remove(google::protobuf::Map<bool, ::common::Distribution> * m,
                          bool key, ::common::Distribution * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_common_Distribution_iter(
      const google::protobuf::Map<bool, ::common::Distribution>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_common_Distribution_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::common::Distribution** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::common::Distribution>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::common::Distribution>* __rust_proto_thunk__Map_ProtoStr_common_Distribution_new() {
    return new google::protobuf::Map<std::string, ::common::Distribution>();
  }
  void __rust_proto_thunk__Map_ProtoStr_common_Distribution_free(const google::protobuf::Map<std::string, ::common::Distribution>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_common_Distribution_clear(google::protobuf::Map<std::string, ::common::Distribution> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_common_Distribution_size(const google::protobuf::Map<std::string, ::common::Distribution>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_Distribution_insert(google::protobuf::Map<std::string, ::common::Distribution> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::Distribution value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_Distribution_get(const google::protobuf::Map<std::string, ::common::Distribution>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::common::Distribution** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::common::Distribution* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_Distribution_remove(google::protobuf::Map<std::string, ::common::Distribution> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::Distribution * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_common_Distribution_iter(
      const google::protobuf::Map<std::string, ::common::Distribution>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_common_Distribution_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::common::Distribution** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::common::Distribution>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// common.AuthenticationRequest
        // clang-format off
extern "C" {
void* __rust_proto_thunk__common_AuthenticationRequest_new() { return new ::common::AuthenticationRequest(); }
void __rust_proto_thunk__common_AuthenticationRequest_delete(void* ptr) { delete static_cast<::common::AuthenticationRequest*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__common_AuthenticationRequest_serialize(::common::AuthenticationRequest* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__common_AuthenticationRequest_parse(::common::AuthenticationRequest* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__common_AuthenticationRequest_copy_from(::common::AuthenticationRequest* dst, const ::common::AuthenticationRequest* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__common_AuthenticationRequest_repeated_len(google::protobuf::RepeatedPtrField<::common::AuthenticationRequest>* field) {
  return field->size();
}
const ::common::AuthenticationRequest& __rust_proto_thunk__common_AuthenticationRequest_repeated_get(
  google::protobuf::RepeatedPtrField<::common::AuthenticationRequest>* field,
  size_t index) {
  return field->Get(index);
}
::common::AuthenticationRequest* __rust_proto_thunk__common_AuthenticationRequest_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::common::AuthenticationRequest>* field,
  size_t index) {
  return field->Mutable(index);
}
::common::AuthenticationRequest* __rust_proto_thunk__common_AuthenticationRequest_repeated_add(google::protobuf::RepeatedPtrField<::common::AuthenticationRequest>* field) {
  return field->Add();
}
void __rust_proto_thunk__common_AuthenticationRequest_repeated_clear(google::protobuf::RepeatedPtrField<::common::AuthenticationRequest>* field) {
  field->Clear();
}
void __rust_proto_thunk__common_AuthenticationRequest_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::common::AuthenticationRequest>& dst,
  const google::protobuf::RepeatedPtrField<::common::AuthenticationRequest>& src) {
  dst = src;
}

const void* __rust_proto_thunk__common_AuthenticationRequest_get_verifier(::common::AuthenticationRequest* msg) {
  return static_cast<const void*>(&msg->verifier());
}
void* __rust_proto_thunk__common_AuthenticationRequest_get_mut_verifier(::common::AuthenticationRequest* msg) {
  return static_cast<void*>(msg->mutable_verifier());
}
void __rust_proto_thunk__common_AuthenticationRequest_clear_verifier(::common::AuthenticationRequest* msg) { msg->clear_verifier(); }
bool __rust_proto_thunk__common_AuthenticationRequest_has_verifier(::common::AuthenticationRequest* msg) { return msg->has_verifier(); }

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__common_AuthenticationRequest_get_connection_id(::common::AuthenticationRequest* msg) {
  absl::string_view val = msg->connection_id();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__common_AuthenticationRequest_set_connection_id(::common::AuthenticationRequest* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_connection_id(absl::string_view(s.ptr, s.len));
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::common::AuthenticationRequest>* __rust_proto_thunk__Map_i32_common_AuthenticationRequest_new() {
    return new google::protobuf::Map<int32_t, ::common::AuthenticationRequest>();
  }
  void __rust_proto_thunk__Map_i32_common_AuthenticationRequest_free(const google::protobuf::Map<int32_t, ::common::AuthenticationRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_common_AuthenticationRequest_clear(google::protobuf::Map<int32_t, ::common::AuthenticationRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_common_AuthenticationRequest_size(const google::protobuf::Map<int32_t, ::common::AuthenticationRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_common_AuthenticationRequest_insert(google::protobuf::Map<int32_t, ::common::AuthenticationRequest> * m,
                          int32_t key, ::common::AuthenticationRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_AuthenticationRequest_get(const google::protobuf::Map<int32_t, ::common::AuthenticationRequest>* m,
                       int32_t key, const ::common::AuthenticationRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::AuthenticationRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_AuthenticationRequest_remove(google::protobuf::Map<int32_t, ::common::AuthenticationRequest> * m,
                          int32_t key, ::common::AuthenticationRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_common_AuthenticationRequest_iter(
      const google::protobuf::Map<int32_t, ::common::AuthenticationRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_common_AuthenticationRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::common::AuthenticationRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::common::AuthenticationRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::common::AuthenticationRequest>* __rust_proto_thunk__Map_u32_common_AuthenticationRequest_new() {
    return new google::protobuf::Map<uint32_t, ::common::AuthenticationRequest>();
  }
  void __rust_proto_thunk__Map_u32_common_AuthenticationRequest_free(const google::protobuf::Map<uint32_t, ::common::AuthenticationRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_common_AuthenticationRequest_clear(google::protobuf::Map<uint32_t, ::common::AuthenticationRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_common_AuthenticationRequest_size(const google::protobuf::Map<uint32_t, ::common::AuthenticationRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_common_AuthenticationRequest_insert(google::protobuf::Map<uint32_t, ::common::AuthenticationRequest> * m,
                          uint32_t key, ::common::AuthenticationRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_AuthenticationRequest_get(const google::protobuf::Map<uint32_t, ::common::AuthenticationRequest>* m,
                       uint32_t key, const ::common::AuthenticationRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::AuthenticationRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_AuthenticationRequest_remove(google::protobuf::Map<uint32_t, ::common::AuthenticationRequest> * m,
                          uint32_t key, ::common::AuthenticationRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_common_AuthenticationRequest_iter(
      const google::protobuf::Map<uint32_t, ::common::AuthenticationRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_common_AuthenticationRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::common::AuthenticationRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::common::AuthenticationRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::common::AuthenticationRequest>* __rust_proto_thunk__Map_i64_common_AuthenticationRequest_new() {
    return new google::protobuf::Map<int64_t, ::common::AuthenticationRequest>();
  }
  void __rust_proto_thunk__Map_i64_common_AuthenticationRequest_free(const google::protobuf::Map<int64_t, ::common::AuthenticationRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_common_AuthenticationRequest_clear(google::protobuf::Map<int64_t, ::common::AuthenticationRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_common_AuthenticationRequest_size(const google::protobuf::Map<int64_t, ::common::AuthenticationRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_common_AuthenticationRequest_insert(google::protobuf::Map<int64_t, ::common::AuthenticationRequest> * m,
                          int64_t key, ::common::AuthenticationRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_AuthenticationRequest_get(const google::protobuf::Map<int64_t, ::common::AuthenticationRequest>* m,
                       int64_t key, const ::common::AuthenticationRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::AuthenticationRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_AuthenticationRequest_remove(google::protobuf::Map<int64_t, ::common::AuthenticationRequest> * m,
                          int64_t key, ::common::AuthenticationRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_common_AuthenticationRequest_iter(
      const google::protobuf::Map<int64_t, ::common::AuthenticationRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_common_AuthenticationRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::common::AuthenticationRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::common::AuthenticationRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::common::AuthenticationRequest>* __rust_proto_thunk__Map_u64_common_AuthenticationRequest_new() {
    return new google::protobuf::Map<uint64_t, ::common::AuthenticationRequest>();
  }
  void __rust_proto_thunk__Map_u64_common_AuthenticationRequest_free(const google::protobuf::Map<uint64_t, ::common::AuthenticationRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_common_AuthenticationRequest_clear(google::protobuf::Map<uint64_t, ::common::AuthenticationRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_common_AuthenticationRequest_size(const google::protobuf::Map<uint64_t, ::common::AuthenticationRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_common_AuthenticationRequest_insert(google::protobuf::Map<uint64_t, ::common::AuthenticationRequest> * m,
                          uint64_t key, ::common::AuthenticationRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_AuthenticationRequest_get(const google::protobuf::Map<uint64_t, ::common::AuthenticationRequest>* m,
                       uint64_t key, const ::common::AuthenticationRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::AuthenticationRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_AuthenticationRequest_remove(google::protobuf::Map<uint64_t, ::common::AuthenticationRequest> * m,
                          uint64_t key, ::common::AuthenticationRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_common_AuthenticationRequest_iter(
      const google::protobuf::Map<uint64_t, ::common::AuthenticationRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_common_AuthenticationRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::common::AuthenticationRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::common::AuthenticationRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::common::AuthenticationRequest>* __rust_proto_thunk__Map_bool_common_AuthenticationRequest_new() {
    return new google::protobuf::Map<bool, ::common::AuthenticationRequest>();
  }
  void __rust_proto_thunk__Map_bool_common_AuthenticationRequest_free(const google::protobuf::Map<bool, ::common::AuthenticationRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_common_AuthenticationRequest_clear(google::protobuf::Map<bool, ::common::AuthenticationRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_common_AuthenticationRequest_size(const google::protobuf::Map<bool, ::common::AuthenticationRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_common_AuthenticationRequest_insert(google::protobuf::Map<bool, ::common::AuthenticationRequest> * m,
                          bool key, ::common::AuthenticationRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_AuthenticationRequest_get(const google::protobuf::Map<bool, ::common::AuthenticationRequest>* m,
                       bool key, const ::common::AuthenticationRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::AuthenticationRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_AuthenticationRequest_remove(google::protobuf::Map<bool, ::common::AuthenticationRequest> * m,
                          bool key, ::common::AuthenticationRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_common_AuthenticationRequest_iter(
      const google::protobuf::Map<bool, ::common::AuthenticationRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_common_AuthenticationRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::common::AuthenticationRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::common::AuthenticationRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::common::AuthenticationRequest>* __rust_proto_thunk__Map_ProtoStr_common_AuthenticationRequest_new() {
    return new google::protobuf::Map<std::string, ::common::AuthenticationRequest>();
  }
  void __rust_proto_thunk__Map_ProtoStr_common_AuthenticationRequest_free(const google::protobuf::Map<std::string, ::common::AuthenticationRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_common_AuthenticationRequest_clear(google::protobuf::Map<std::string, ::common::AuthenticationRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_common_AuthenticationRequest_size(const google::protobuf::Map<std::string, ::common::AuthenticationRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_AuthenticationRequest_insert(google::protobuf::Map<std::string, ::common::AuthenticationRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::AuthenticationRequest value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_AuthenticationRequest_get(const google::protobuf::Map<std::string, ::common::AuthenticationRequest>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::common::AuthenticationRequest** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::common::AuthenticationRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_AuthenticationRequest_remove(google::protobuf::Map<std::string, ::common::AuthenticationRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::AuthenticationRequest * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_common_AuthenticationRequest_iter(
      const google::protobuf::Map<std::string, ::common::AuthenticationRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_common_AuthenticationRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::common::AuthenticationRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::common::AuthenticationRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// common.AuthenticationResponse
        // clang-format off
extern "C" {
void* __rust_proto_thunk__common_AuthenticationResponse_new() { return new ::common::AuthenticationResponse(); }
void __rust_proto_thunk__common_AuthenticationResponse_delete(void* ptr) { delete static_cast<::common::AuthenticationResponse*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__common_AuthenticationResponse_serialize(::common::AuthenticationResponse* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__common_AuthenticationResponse_parse(::common::AuthenticationResponse* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__common_AuthenticationResponse_copy_from(::common::AuthenticationResponse* dst, const ::common::AuthenticationResponse* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__common_AuthenticationResponse_repeated_len(google::protobuf::RepeatedPtrField<::common::AuthenticationResponse>* field) {
  return field->size();
}
const ::common::AuthenticationResponse& __rust_proto_thunk__common_AuthenticationResponse_repeated_get(
  google::protobuf::RepeatedPtrField<::common::AuthenticationResponse>* field,
  size_t index) {
  return field->Get(index);
}
::common::AuthenticationResponse* __rust_proto_thunk__common_AuthenticationResponse_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::common::AuthenticationResponse>* field,
  size_t index) {
  return field->Mutable(index);
}
::common::AuthenticationResponse* __rust_proto_thunk__common_AuthenticationResponse_repeated_add(google::protobuf::RepeatedPtrField<::common::AuthenticationResponse>* field) {
  return field->Add();
}
void __rust_proto_thunk__common_AuthenticationResponse_repeated_clear(google::protobuf::RepeatedPtrField<::common::AuthenticationResponse>* field) {
  field->Clear();
}
void __rust_proto_thunk__common_AuthenticationResponse_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::common::AuthenticationResponse>& dst,
  const google::protobuf::RepeatedPtrField<::common::AuthenticationResponse>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__common_AuthenticationResponse_get_challenge(::common::AuthenticationResponse* msg) {
  absl::string_view val = msg->challenge();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__common_AuthenticationResponse_set_challenge(::common::AuthenticationResponse* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_challenge(absl::string_view(s.ptr, s.len));
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::common::AuthenticationResponse>* __rust_proto_thunk__Map_i32_common_AuthenticationResponse_new() {
    return new google::protobuf::Map<int32_t, ::common::AuthenticationResponse>();
  }
  void __rust_proto_thunk__Map_i32_common_AuthenticationResponse_free(const google::protobuf::Map<int32_t, ::common::AuthenticationResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_common_AuthenticationResponse_clear(google::protobuf::Map<int32_t, ::common::AuthenticationResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_common_AuthenticationResponse_size(const google::protobuf::Map<int32_t, ::common::AuthenticationResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_common_AuthenticationResponse_insert(google::protobuf::Map<int32_t, ::common::AuthenticationResponse> * m,
                          int32_t key, ::common::AuthenticationResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_AuthenticationResponse_get(const google::protobuf::Map<int32_t, ::common::AuthenticationResponse>* m,
                       int32_t key, const ::common::AuthenticationResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::AuthenticationResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_AuthenticationResponse_remove(google::protobuf::Map<int32_t, ::common::AuthenticationResponse> * m,
                          int32_t key, ::common::AuthenticationResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_common_AuthenticationResponse_iter(
      const google::protobuf::Map<int32_t, ::common::AuthenticationResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_common_AuthenticationResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::common::AuthenticationResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::common::AuthenticationResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::common::AuthenticationResponse>* __rust_proto_thunk__Map_u32_common_AuthenticationResponse_new() {
    return new google::protobuf::Map<uint32_t, ::common::AuthenticationResponse>();
  }
  void __rust_proto_thunk__Map_u32_common_AuthenticationResponse_free(const google::protobuf::Map<uint32_t, ::common::AuthenticationResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_common_AuthenticationResponse_clear(google::protobuf::Map<uint32_t, ::common::AuthenticationResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_common_AuthenticationResponse_size(const google::protobuf::Map<uint32_t, ::common::AuthenticationResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_common_AuthenticationResponse_insert(google::protobuf::Map<uint32_t, ::common::AuthenticationResponse> * m,
                          uint32_t key, ::common::AuthenticationResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_AuthenticationResponse_get(const google::protobuf::Map<uint32_t, ::common::AuthenticationResponse>* m,
                       uint32_t key, const ::common::AuthenticationResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::AuthenticationResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_AuthenticationResponse_remove(google::protobuf::Map<uint32_t, ::common::AuthenticationResponse> * m,
                          uint32_t key, ::common::AuthenticationResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_common_AuthenticationResponse_iter(
      const google::protobuf::Map<uint32_t, ::common::AuthenticationResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_common_AuthenticationResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::common::AuthenticationResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::common::AuthenticationResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::common::AuthenticationResponse>* __rust_proto_thunk__Map_i64_common_AuthenticationResponse_new() {
    return new google::protobuf::Map<int64_t, ::common::AuthenticationResponse>();
  }
  void __rust_proto_thunk__Map_i64_common_AuthenticationResponse_free(const google::protobuf::Map<int64_t, ::common::AuthenticationResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_common_AuthenticationResponse_clear(google::protobuf::Map<int64_t, ::common::AuthenticationResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_common_AuthenticationResponse_size(const google::protobuf::Map<int64_t, ::common::AuthenticationResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_common_AuthenticationResponse_insert(google::protobuf::Map<int64_t, ::common::AuthenticationResponse> * m,
                          int64_t key, ::common::AuthenticationResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_AuthenticationResponse_get(const google::protobuf::Map<int64_t, ::common::AuthenticationResponse>* m,
                       int64_t key, const ::common::AuthenticationResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::AuthenticationResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_AuthenticationResponse_remove(google::protobuf::Map<int64_t, ::common::AuthenticationResponse> * m,
                          int64_t key, ::common::AuthenticationResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_common_AuthenticationResponse_iter(
      const google::protobuf::Map<int64_t, ::common::AuthenticationResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_common_AuthenticationResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::common::AuthenticationResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::common::AuthenticationResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::common::AuthenticationResponse>* __rust_proto_thunk__Map_u64_common_AuthenticationResponse_new() {
    return new google::protobuf::Map<uint64_t, ::common::AuthenticationResponse>();
  }
  void __rust_proto_thunk__Map_u64_common_AuthenticationResponse_free(const google::protobuf::Map<uint64_t, ::common::AuthenticationResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_common_AuthenticationResponse_clear(google::protobuf::Map<uint64_t, ::common::AuthenticationResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_common_AuthenticationResponse_size(const google::protobuf::Map<uint64_t, ::common::AuthenticationResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_common_AuthenticationResponse_insert(google::protobuf::Map<uint64_t, ::common::AuthenticationResponse> * m,
                          uint64_t key, ::common::AuthenticationResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_AuthenticationResponse_get(const google::protobuf::Map<uint64_t, ::common::AuthenticationResponse>* m,
                       uint64_t key, const ::common::AuthenticationResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::AuthenticationResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_AuthenticationResponse_remove(google::protobuf::Map<uint64_t, ::common::AuthenticationResponse> * m,
                          uint64_t key, ::common::AuthenticationResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_common_AuthenticationResponse_iter(
      const google::protobuf::Map<uint64_t, ::common::AuthenticationResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_common_AuthenticationResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::common::AuthenticationResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::common::AuthenticationResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::common::AuthenticationResponse>* __rust_proto_thunk__Map_bool_common_AuthenticationResponse_new() {
    return new google::protobuf::Map<bool, ::common::AuthenticationResponse>();
  }
  void __rust_proto_thunk__Map_bool_common_AuthenticationResponse_free(const google::protobuf::Map<bool, ::common::AuthenticationResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_common_AuthenticationResponse_clear(google::protobuf::Map<bool, ::common::AuthenticationResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_common_AuthenticationResponse_size(const google::protobuf::Map<bool, ::common::AuthenticationResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_common_AuthenticationResponse_insert(google::protobuf::Map<bool, ::common::AuthenticationResponse> * m,
                          bool key, ::common::AuthenticationResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_AuthenticationResponse_get(const google::protobuf::Map<bool, ::common::AuthenticationResponse>* m,
                       bool key, const ::common::AuthenticationResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::AuthenticationResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_AuthenticationResponse_remove(google::protobuf::Map<bool, ::common::AuthenticationResponse> * m,
                          bool key, ::common::AuthenticationResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_common_AuthenticationResponse_iter(
      const google::protobuf::Map<bool, ::common::AuthenticationResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_common_AuthenticationResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::common::AuthenticationResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::common::AuthenticationResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::common::AuthenticationResponse>* __rust_proto_thunk__Map_ProtoStr_common_AuthenticationResponse_new() {
    return new google::protobuf::Map<std::string, ::common::AuthenticationResponse>();
  }
  void __rust_proto_thunk__Map_ProtoStr_common_AuthenticationResponse_free(const google::protobuf::Map<std::string, ::common::AuthenticationResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_common_AuthenticationResponse_clear(google::protobuf::Map<std::string, ::common::AuthenticationResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_common_AuthenticationResponse_size(const google::protobuf::Map<std::string, ::common::AuthenticationResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_AuthenticationResponse_insert(google::protobuf::Map<std::string, ::common::AuthenticationResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::AuthenticationResponse value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_AuthenticationResponse_get(const google::protobuf::Map<std::string, ::common::AuthenticationResponse>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::common::AuthenticationResponse** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::common::AuthenticationResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_AuthenticationResponse_remove(google::protobuf::Map<std::string, ::common::AuthenticationResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::AuthenticationResponse * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_common_AuthenticationResponse_iter(
      const google::protobuf::Map<std::string, ::common::AuthenticationResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_common_AuthenticationResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::common::AuthenticationResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::common::AuthenticationResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// common.OneConnection
        // clang-format off
extern "C" {
void* __rust_proto_thunk__common_OneConnection_new() { return new ::common::OneConnection(); }
void __rust_proto_thunk__common_OneConnection_delete(void* ptr) { delete static_cast<::common::OneConnection*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__common_OneConnection_serialize(::common::OneConnection* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__common_OneConnection_parse(::common::OneConnection* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__common_OneConnection_copy_from(::common::OneConnection* dst, const ::common::OneConnection* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__common_OneConnection_repeated_len(google::protobuf::RepeatedPtrField<::common::OneConnection>* field) {
  return field->size();
}
const ::common::OneConnection& __rust_proto_thunk__common_OneConnection_repeated_get(
  google::protobuf::RepeatedPtrField<::common::OneConnection>* field,
  size_t index) {
  return field->Get(index);
}
::common::OneConnection* __rust_proto_thunk__common_OneConnection_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::common::OneConnection>* field,
  size_t index) {
  return field->Mutable(index);
}
::common::OneConnection* __rust_proto_thunk__common_OneConnection_repeated_add(google::protobuf::RepeatedPtrField<::common::OneConnection>* field) {
  return field->Add();
}
void __rust_proto_thunk__common_OneConnection_repeated_clear(google::protobuf::RepeatedPtrField<::common::OneConnection>* field) {
  field->Clear();
}
void __rust_proto_thunk__common_OneConnection_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::common::OneConnection>& dst,
  const google::protobuf::RepeatedPtrField<::common::OneConnection>& src) {
  dst = src;
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::common::OneConnection>* __rust_proto_thunk__Map_i32_common_OneConnection_new() {
    return new google::protobuf::Map<int32_t, ::common::OneConnection>();
  }
  void __rust_proto_thunk__Map_i32_common_OneConnection_free(const google::protobuf::Map<int32_t, ::common::OneConnection>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_common_OneConnection_clear(google::protobuf::Map<int32_t, ::common::OneConnection> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_common_OneConnection_size(const google::protobuf::Map<int32_t, ::common::OneConnection>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_common_OneConnection_insert(google::protobuf::Map<int32_t, ::common::OneConnection> * m,
                          int32_t key, ::common::OneConnection value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_OneConnection_get(const google::protobuf::Map<int32_t, ::common::OneConnection>* m,
                       int32_t key, const ::common::OneConnection** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::OneConnection* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_OneConnection_remove(google::protobuf::Map<int32_t, ::common::OneConnection> * m,
                          int32_t key, ::common::OneConnection * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_common_OneConnection_iter(
      const google::protobuf::Map<int32_t, ::common::OneConnection>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_common_OneConnection_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::common::OneConnection** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::common::OneConnection>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::common::OneConnection>* __rust_proto_thunk__Map_u32_common_OneConnection_new() {
    return new google::protobuf::Map<uint32_t, ::common::OneConnection>();
  }
  void __rust_proto_thunk__Map_u32_common_OneConnection_free(const google::protobuf::Map<uint32_t, ::common::OneConnection>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_common_OneConnection_clear(google::protobuf::Map<uint32_t, ::common::OneConnection> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_common_OneConnection_size(const google::protobuf::Map<uint32_t, ::common::OneConnection>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_common_OneConnection_insert(google::protobuf::Map<uint32_t, ::common::OneConnection> * m,
                          uint32_t key, ::common::OneConnection value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_OneConnection_get(const google::protobuf::Map<uint32_t, ::common::OneConnection>* m,
                       uint32_t key, const ::common::OneConnection** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::OneConnection* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_OneConnection_remove(google::protobuf::Map<uint32_t, ::common::OneConnection> * m,
                          uint32_t key, ::common::OneConnection * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_common_OneConnection_iter(
      const google::protobuf::Map<uint32_t, ::common::OneConnection>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_common_OneConnection_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::common::OneConnection** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::common::OneConnection>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::common::OneConnection>* __rust_proto_thunk__Map_i64_common_OneConnection_new() {
    return new google::protobuf::Map<int64_t, ::common::OneConnection>();
  }
  void __rust_proto_thunk__Map_i64_common_OneConnection_free(const google::protobuf::Map<int64_t, ::common::OneConnection>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_common_OneConnection_clear(google::protobuf::Map<int64_t, ::common::OneConnection> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_common_OneConnection_size(const google::protobuf::Map<int64_t, ::common::OneConnection>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_common_OneConnection_insert(google::protobuf::Map<int64_t, ::common::OneConnection> * m,
                          int64_t key, ::common::OneConnection value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_OneConnection_get(const google::protobuf::Map<int64_t, ::common::OneConnection>* m,
                       int64_t key, const ::common::OneConnection** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::OneConnection* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_OneConnection_remove(google::protobuf::Map<int64_t, ::common::OneConnection> * m,
                          int64_t key, ::common::OneConnection * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_common_OneConnection_iter(
      const google::protobuf::Map<int64_t, ::common::OneConnection>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_common_OneConnection_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::common::OneConnection** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::common::OneConnection>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::common::OneConnection>* __rust_proto_thunk__Map_u64_common_OneConnection_new() {
    return new google::protobuf::Map<uint64_t, ::common::OneConnection>();
  }
  void __rust_proto_thunk__Map_u64_common_OneConnection_free(const google::protobuf::Map<uint64_t, ::common::OneConnection>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_common_OneConnection_clear(google::protobuf::Map<uint64_t, ::common::OneConnection> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_common_OneConnection_size(const google::protobuf::Map<uint64_t, ::common::OneConnection>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_common_OneConnection_insert(google::protobuf::Map<uint64_t, ::common::OneConnection> * m,
                          uint64_t key, ::common::OneConnection value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_OneConnection_get(const google::protobuf::Map<uint64_t, ::common::OneConnection>* m,
                       uint64_t key, const ::common::OneConnection** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::OneConnection* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_OneConnection_remove(google::protobuf::Map<uint64_t, ::common::OneConnection> * m,
                          uint64_t key, ::common::OneConnection * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_common_OneConnection_iter(
      const google::protobuf::Map<uint64_t, ::common::OneConnection>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_common_OneConnection_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::common::OneConnection** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::common::OneConnection>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::common::OneConnection>* __rust_proto_thunk__Map_bool_common_OneConnection_new() {
    return new google::protobuf::Map<bool, ::common::OneConnection>();
  }
  void __rust_proto_thunk__Map_bool_common_OneConnection_free(const google::protobuf::Map<bool, ::common::OneConnection>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_common_OneConnection_clear(google::protobuf::Map<bool, ::common::OneConnection> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_common_OneConnection_size(const google::protobuf::Map<bool, ::common::OneConnection>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_common_OneConnection_insert(google::protobuf::Map<bool, ::common::OneConnection> * m,
                          bool key, ::common::OneConnection value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_OneConnection_get(const google::protobuf::Map<bool, ::common::OneConnection>* m,
                       bool key, const ::common::OneConnection** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::OneConnection* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_OneConnection_remove(google::protobuf::Map<bool, ::common::OneConnection> * m,
                          bool key, ::common::OneConnection * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_common_OneConnection_iter(
      const google::protobuf::Map<bool, ::common::OneConnection>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_common_OneConnection_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::common::OneConnection** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::common::OneConnection>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::common::OneConnection>* __rust_proto_thunk__Map_ProtoStr_common_OneConnection_new() {
    return new google::protobuf::Map<std::string, ::common::OneConnection>();
  }
  void __rust_proto_thunk__Map_ProtoStr_common_OneConnection_free(const google::protobuf::Map<std::string, ::common::OneConnection>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_common_OneConnection_clear(google::protobuf::Map<std::string, ::common::OneConnection> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_common_OneConnection_size(const google::protobuf::Map<std::string, ::common::OneConnection>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_OneConnection_insert(google::protobuf::Map<std::string, ::common::OneConnection> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::OneConnection value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_OneConnection_get(const google::protobuf::Map<std::string, ::common::OneConnection>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::common::OneConnection** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::common::OneConnection* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_OneConnection_remove(google::protobuf::Map<std::string, ::common::OneConnection> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::OneConnection * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_common_OneConnection_iter(
      const google::protobuf::Map<std::string, ::common::OneConnection>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_common_OneConnection_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::common::OneConnection** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::common::OneConnection>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// common.ManyVerifierConnection
        // clang-format off
extern "C" {
void* __rust_proto_thunk__common_ManyVerifierConnection_new() { return new ::common::ManyVerifierConnection(); }
void __rust_proto_thunk__common_ManyVerifierConnection_delete(void* ptr) { delete static_cast<::common::ManyVerifierConnection*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__common_ManyVerifierConnection_serialize(::common::ManyVerifierConnection* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__common_ManyVerifierConnection_parse(::common::ManyVerifierConnection* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__common_ManyVerifierConnection_copy_from(::common::ManyVerifierConnection* dst, const ::common::ManyVerifierConnection* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__common_ManyVerifierConnection_repeated_len(google::protobuf::RepeatedPtrField<::common::ManyVerifierConnection>* field) {
  return field->size();
}
const ::common::ManyVerifierConnection& __rust_proto_thunk__common_ManyVerifierConnection_repeated_get(
  google::protobuf::RepeatedPtrField<::common::ManyVerifierConnection>* field,
  size_t index) {
  return field->Get(index);
}
::common::ManyVerifierConnection* __rust_proto_thunk__common_ManyVerifierConnection_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::common::ManyVerifierConnection>* field,
  size_t index) {
  return field->Mutable(index);
}
::common::ManyVerifierConnection* __rust_proto_thunk__common_ManyVerifierConnection_repeated_add(google::protobuf::RepeatedPtrField<::common::ManyVerifierConnection>* field) {
  return field->Add();
}
void __rust_proto_thunk__common_ManyVerifierConnection_repeated_clear(google::protobuf::RepeatedPtrField<::common::ManyVerifierConnection>* field) {
  field->Clear();
}
void __rust_proto_thunk__common_ManyVerifierConnection_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::common::ManyVerifierConnection>& dst,
  const google::protobuf::RepeatedPtrField<::common::ManyVerifierConnection>& src) {
  dst = src;
}

void __rust_proto_thunk__common_ManyVerifierConnection_clear_associated(::common::ManyVerifierConnection* msg) {
  msg->clear_associated();
}
google::protobuf::RepeatedPtrField<::common::DynamicVerifier>* __rust_proto_thunk__common_ManyVerifierConnection_get_mut_associated(::common::ManyVerifierConnection* msg) {
  return msg->mutable_associated();
}
const google::protobuf::RepeatedPtrField<::common::DynamicVerifier>* __rust_proto_thunk__common_ManyVerifierConnection_get_associated(
    const ::common::ManyVerifierConnection* msg) {
  return &msg->associated();
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::common::ManyVerifierConnection>* __rust_proto_thunk__Map_i32_common_ManyVerifierConnection_new() {
    return new google::protobuf::Map<int32_t, ::common::ManyVerifierConnection>();
  }
  void __rust_proto_thunk__Map_i32_common_ManyVerifierConnection_free(const google::protobuf::Map<int32_t, ::common::ManyVerifierConnection>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_common_ManyVerifierConnection_clear(google::protobuf::Map<int32_t, ::common::ManyVerifierConnection> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_common_ManyVerifierConnection_size(const google::protobuf::Map<int32_t, ::common::ManyVerifierConnection>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_common_ManyVerifierConnection_insert(google::protobuf::Map<int32_t, ::common::ManyVerifierConnection> * m,
                          int32_t key, ::common::ManyVerifierConnection value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_ManyVerifierConnection_get(const google::protobuf::Map<int32_t, ::common::ManyVerifierConnection>* m,
                       int32_t key, const ::common::ManyVerifierConnection** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::ManyVerifierConnection* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_ManyVerifierConnection_remove(google::protobuf::Map<int32_t, ::common::ManyVerifierConnection> * m,
                          int32_t key, ::common::ManyVerifierConnection * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_common_ManyVerifierConnection_iter(
      const google::protobuf::Map<int32_t, ::common::ManyVerifierConnection>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_common_ManyVerifierConnection_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::common::ManyVerifierConnection** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::common::ManyVerifierConnection>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::common::ManyVerifierConnection>* __rust_proto_thunk__Map_u32_common_ManyVerifierConnection_new() {
    return new google::protobuf::Map<uint32_t, ::common::ManyVerifierConnection>();
  }
  void __rust_proto_thunk__Map_u32_common_ManyVerifierConnection_free(const google::protobuf::Map<uint32_t, ::common::ManyVerifierConnection>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_common_ManyVerifierConnection_clear(google::protobuf::Map<uint32_t, ::common::ManyVerifierConnection> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_common_ManyVerifierConnection_size(const google::protobuf::Map<uint32_t, ::common::ManyVerifierConnection>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_common_ManyVerifierConnection_insert(google::protobuf::Map<uint32_t, ::common::ManyVerifierConnection> * m,
                          uint32_t key, ::common::ManyVerifierConnection value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_ManyVerifierConnection_get(const google::protobuf::Map<uint32_t, ::common::ManyVerifierConnection>* m,
                       uint32_t key, const ::common::ManyVerifierConnection** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::ManyVerifierConnection* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_ManyVerifierConnection_remove(google::protobuf::Map<uint32_t, ::common::ManyVerifierConnection> * m,
                          uint32_t key, ::common::ManyVerifierConnection * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_common_ManyVerifierConnection_iter(
      const google::protobuf::Map<uint32_t, ::common::ManyVerifierConnection>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_common_ManyVerifierConnection_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::common::ManyVerifierConnection** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::common::ManyVerifierConnection>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::common::ManyVerifierConnection>* __rust_proto_thunk__Map_i64_common_ManyVerifierConnection_new() {
    return new google::protobuf::Map<int64_t, ::common::ManyVerifierConnection>();
  }
  void __rust_proto_thunk__Map_i64_common_ManyVerifierConnection_free(const google::protobuf::Map<int64_t, ::common::ManyVerifierConnection>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_common_ManyVerifierConnection_clear(google::protobuf::Map<int64_t, ::common::ManyVerifierConnection> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_common_ManyVerifierConnection_size(const google::protobuf::Map<int64_t, ::common::ManyVerifierConnection>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_common_ManyVerifierConnection_insert(google::protobuf::Map<int64_t, ::common::ManyVerifierConnection> * m,
                          int64_t key, ::common::ManyVerifierConnection value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_ManyVerifierConnection_get(const google::protobuf::Map<int64_t, ::common::ManyVerifierConnection>* m,
                       int64_t key, const ::common::ManyVerifierConnection** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::ManyVerifierConnection* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_ManyVerifierConnection_remove(google::protobuf::Map<int64_t, ::common::ManyVerifierConnection> * m,
                          int64_t key, ::common::ManyVerifierConnection * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_common_ManyVerifierConnection_iter(
      const google::protobuf::Map<int64_t, ::common::ManyVerifierConnection>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_common_ManyVerifierConnection_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::common::ManyVerifierConnection** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::common::ManyVerifierConnection>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::common::ManyVerifierConnection>* __rust_proto_thunk__Map_u64_common_ManyVerifierConnection_new() {
    return new google::protobuf::Map<uint64_t, ::common::ManyVerifierConnection>();
  }
  void __rust_proto_thunk__Map_u64_common_ManyVerifierConnection_free(const google::protobuf::Map<uint64_t, ::common::ManyVerifierConnection>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_common_ManyVerifierConnection_clear(google::protobuf::Map<uint64_t, ::common::ManyVerifierConnection> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_common_ManyVerifierConnection_size(const google::protobuf::Map<uint64_t, ::common::ManyVerifierConnection>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_common_ManyVerifierConnection_insert(google::protobuf::Map<uint64_t, ::common::ManyVerifierConnection> * m,
                          uint64_t key, ::common::ManyVerifierConnection value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_ManyVerifierConnection_get(const google::protobuf::Map<uint64_t, ::common::ManyVerifierConnection>* m,
                       uint64_t key, const ::common::ManyVerifierConnection** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::ManyVerifierConnection* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_ManyVerifierConnection_remove(google::protobuf::Map<uint64_t, ::common::ManyVerifierConnection> * m,
                          uint64_t key, ::common::ManyVerifierConnection * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_common_ManyVerifierConnection_iter(
      const google::protobuf::Map<uint64_t, ::common::ManyVerifierConnection>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_common_ManyVerifierConnection_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::common::ManyVerifierConnection** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::common::ManyVerifierConnection>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::common::ManyVerifierConnection>* __rust_proto_thunk__Map_bool_common_ManyVerifierConnection_new() {
    return new google::protobuf::Map<bool, ::common::ManyVerifierConnection>();
  }
  void __rust_proto_thunk__Map_bool_common_ManyVerifierConnection_free(const google::protobuf::Map<bool, ::common::ManyVerifierConnection>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_common_ManyVerifierConnection_clear(google::protobuf::Map<bool, ::common::ManyVerifierConnection> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_common_ManyVerifierConnection_size(const google::protobuf::Map<bool, ::common::ManyVerifierConnection>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_common_ManyVerifierConnection_insert(google::protobuf::Map<bool, ::common::ManyVerifierConnection> * m,
                          bool key, ::common::ManyVerifierConnection value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_ManyVerifierConnection_get(const google::protobuf::Map<bool, ::common::ManyVerifierConnection>* m,
                       bool key, const ::common::ManyVerifierConnection** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::ManyVerifierConnection* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_ManyVerifierConnection_remove(google::protobuf::Map<bool, ::common::ManyVerifierConnection> * m,
                          bool key, ::common::ManyVerifierConnection * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_common_ManyVerifierConnection_iter(
      const google::protobuf::Map<bool, ::common::ManyVerifierConnection>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_common_ManyVerifierConnection_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::common::ManyVerifierConnection** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::common::ManyVerifierConnection>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::common::ManyVerifierConnection>* __rust_proto_thunk__Map_ProtoStr_common_ManyVerifierConnection_new() {
    return new google::protobuf::Map<std::string, ::common::ManyVerifierConnection>();
  }
  void __rust_proto_thunk__Map_ProtoStr_common_ManyVerifierConnection_free(const google::protobuf::Map<std::string, ::common::ManyVerifierConnection>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_common_ManyVerifierConnection_clear(google::protobuf::Map<std::string, ::common::ManyVerifierConnection> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_common_ManyVerifierConnection_size(const google::protobuf::Map<std::string, ::common::ManyVerifierConnection>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_ManyVerifierConnection_insert(google::protobuf::Map<std::string, ::common::ManyVerifierConnection> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::ManyVerifierConnection value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_ManyVerifierConnection_get(const google::protobuf::Map<std::string, ::common::ManyVerifierConnection>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::common::ManyVerifierConnection** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::common::ManyVerifierConnection* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_ManyVerifierConnection_remove(google::protobuf::Map<std::string, ::common::ManyVerifierConnection> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::ManyVerifierConnection * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_common_ManyVerifierConnection_iter(
      const google::protobuf::Map<std::string, ::common::ManyVerifierConnection>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_common_ManyVerifierConnection_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::common::ManyVerifierConnection** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::common::ManyVerifierConnection>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// common.ManyUetrConnection
        // clang-format off
extern "C" {
void* __rust_proto_thunk__common_ManyUetrConnection_new() { return new ::common::ManyUetrConnection(); }
void __rust_proto_thunk__common_ManyUetrConnection_delete(void* ptr) { delete static_cast<::common::ManyUetrConnection*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__common_ManyUetrConnection_serialize(::common::ManyUetrConnection* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__common_ManyUetrConnection_parse(::common::ManyUetrConnection* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__common_ManyUetrConnection_copy_from(::common::ManyUetrConnection* dst, const ::common::ManyUetrConnection* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__common_ManyUetrConnection_repeated_len(google::protobuf::RepeatedPtrField<::common::ManyUetrConnection>* field) {
  return field->size();
}
const ::common::ManyUetrConnection& __rust_proto_thunk__common_ManyUetrConnection_repeated_get(
  google::protobuf::RepeatedPtrField<::common::ManyUetrConnection>* field,
  size_t index) {
  return field->Get(index);
}
::common::ManyUetrConnection* __rust_proto_thunk__common_ManyUetrConnection_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::common::ManyUetrConnection>* field,
  size_t index) {
  return field->Mutable(index);
}
::common::ManyUetrConnection* __rust_proto_thunk__common_ManyUetrConnection_repeated_add(google::protobuf::RepeatedPtrField<::common::ManyUetrConnection>* field) {
  return field->Add();
}
void __rust_proto_thunk__common_ManyUetrConnection_repeated_clear(google::protobuf::RepeatedPtrField<::common::ManyUetrConnection>* field) {
  field->Clear();
}
void __rust_proto_thunk__common_ManyUetrConnection_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::common::ManyUetrConnection>& dst,
  const google::protobuf::RepeatedPtrField<::common::ManyUetrConnection>& src) {
  dst = src;
}

void __rust_proto_thunk__common_ManyUetrConnection_clear_associated(::common::ManyUetrConnection* msg) {
  msg->clear_associated();
}
google::protobuf::RepeatedPtrField<std::string>* __rust_proto_thunk__common_ManyUetrConnection_get_mut_associated(::common::ManyUetrConnection* msg) {
  return msg->mutable_associated();
}
const google::protobuf::RepeatedPtrField<std::string>* __rust_proto_thunk__common_ManyUetrConnection_get_associated(
    const ::common::ManyUetrConnection* msg) {
  return &msg->associated();
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::common::ManyUetrConnection>* __rust_proto_thunk__Map_i32_common_ManyUetrConnection_new() {
    return new google::protobuf::Map<int32_t, ::common::ManyUetrConnection>();
  }
  void __rust_proto_thunk__Map_i32_common_ManyUetrConnection_free(const google::protobuf::Map<int32_t, ::common::ManyUetrConnection>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_common_ManyUetrConnection_clear(google::protobuf::Map<int32_t, ::common::ManyUetrConnection> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_common_ManyUetrConnection_size(const google::protobuf::Map<int32_t, ::common::ManyUetrConnection>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_common_ManyUetrConnection_insert(google::protobuf::Map<int32_t, ::common::ManyUetrConnection> * m,
                          int32_t key, ::common::ManyUetrConnection value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_ManyUetrConnection_get(const google::protobuf::Map<int32_t, ::common::ManyUetrConnection>* m,
                       int32_t key, const ::common::ManyUetrConnection** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::ManyUetrConnection* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_ManyUetrConnection_remove(google::protobuf::Map<int32_t, ::common::ManyUetrConnection> * m,
                          int32_t key, ::common::ManyUetrConnection * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_common_ManyUetrConnection_iter(
      const google::protobuf::Map<int32_t, ::common::ManyUetrConnection>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_common_ManyUetrConnection_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::common::ManyUetrConnection** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::common::ManyUetrConnection>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::common::ManyUetrConnection>* __rust_proto_thunk__Map_u32_common_ManyUetrConnection_new() {
    return new google::protobuf::Map<uint32_t, ::common::ManyUetrConnection>();
  }
  void __rust_proto_thunk__Map_u32_common_ManyUetrConnection_free(const google::protobuf::Map<uint32_t, ::common::ManyUetrConnection>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_common_ManyUetrConnection_clear(google::protobuf::Map<uint32_t, ::common::ManyUetrConnection> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_common_ManyUetrConnection_size(const google::protobuf::Map<uint32_t, ::common::ManyUetrConnection>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_common_ManyUetrConnection_insert(google::protobuf::Map<uint32_t, ::common::ManyUetrConnection> * m,
                          uint32_t key, ::common::ManyUetrConnection value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_ManyUetrConnection_get(const google::protobuf::Map<uint32_t, ::common::ManyUetrConnection>* m,
                       uint32_t key, const ::common::ManyUetrConnection** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::ManyUetrConnection* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_ManyUetrConnection_remove(google::protobuf::Map<uint32_t, ::common::ManyUetrConnection> * m,
                          uint32_t key, ::common::ManyUetrConnection * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_common_ManyUetrConnection_iter(
      const google::protobuf::Map<uint32_t, ::common::ManyUetrConnection>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_common_ManyUetrConnection_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::common::ManyUetrConnection** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::common::ManyUetrConnection>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::common::ManyUetrConnection>* __rust_proto_thunk__Map_i64_common_ManyUetrConnection_new() {
    return new google::protobuf::Map<int64_t, ::common::ManyUetrConnection>();
  }
  void __rust_proto_thunk__Map_i64_common_ManyUetrConnection_free(const google::protobuf::Map<int64_t, ::common::ManyUetrConnection>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_common_ManyUetrConnection_clear(google::protobuf::Map<int64_t, ::common::ManyUetrConnection> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_common_ManyUetrConnection_size(const google::protobuf::Map<int64_t, ::common::ManyUetrConnection>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_common_ManyUetrConnection_insert(google::protobuf::Map<int64_t, ::common::ManyUetrConnection> * m,
                          int64_t key, ::common::ManyUetrConnection value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_ManyUetrConnection_get(const google::protobuf::Map<int64_t, ::common::ManyUetrConnection>* m,
                       int64_t key, const ::common::ManyUetrConnection** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::ManyUetrConnection* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_ManyUetrConnection_remove(google::protobuf::Map<int64_t, ::common::ManyUetrConnection> * m,
                          int64_t key, ::common::ManyUetrConnection * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_common_ManyUetrConnection_iter(
      const google::protobuf::Map<int64_t, ::common::ManyUetrConnection>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_common_ManyUetrConnection_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::common::ManyUetrConnection** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::common::ManyUetrConnection>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::common::ManyUetrConnection>* __rust_proto_thunk__Map_u64_common_ManyUetrConnection_new() {
    return new google::protobuf::Map<uint64_t, ::common::ManyUetrConnection>();
  }
  void __rust_proto_thunk__Map_u64_common_ManyUetrConnection_free(const google::protobuf::Map<uint64_t, ::common::ManyUetrConnection>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_common_ManyUetrConnection_clear(google::protobuf::Map<uint64_t, ::common::ManyUetrConnection> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_common_ManyUetrConnection_size(const google::protobuf::Map<uint64_t, ::common::ManyUetrConnection>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_common_ManyUetrConnection_insert(google::protobuf::Map<uint64_t, ::common::ManyUetrConnection> * m,
                          uint64_t key, ::common::ManyUetrConnection value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_ManyUetrConnection_get(const google::protobuf::Map<uint64_t, ::common::ManyUetrConnection>* m,
                       uint64_t key, const ::common::ManyUetrConnection** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::ManyUetrConnection* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_ManyUetrConnection_remove(google::protobuf::Map<uint64_t, ::common::ManyUetrConnection> * m,
                          uint64_t key, ::common::ManyUetrConnection * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_common_ManyUetrConnection_iter(
      const google::protobuf::Map<uint64_t, ::common::ManyUetrConnection>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_common_ManyUetrConnection_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::common::ManyUetrConnection** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::common::ManyUetrConnection>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::common::ManyUetrConnection>* __rust_proto_thunk__Map_bool_common_ManyUetrConnection_new() {
    return new google::protobuf::Map<bool, ::common::ManyUetrConnection>();
  }
  void __rust_proto_thunk__Map_bool_common_ManyUetrConnection_free(const google::protobuf::Map<bool, ::common::ManyUetrConnection>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_common_ManyUetrConnection_clear(google::protobuf::Map<bool, ::common::ManyUetrConnection> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_common_ManyUetrConnection_size(const google::protobuf::Map<bool, ::common::ManyUetrConnection>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_common_ManyUetrConnection_insert(google::protobuf::Map<bool, ::common::ManyUetrConnection> * m,
                          bool key, ::common::ManyUetrConnection value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_ManyUetrConnection_get(const google::protobuf::Map<bool, ::common::ManyUetrConnection>* m,
                       bool key, const ::common::ManyUetrConnection** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::ManyUetrConnection* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_ManyUetrConnection_remove(google::protobuf::Map<bool, ::common::ManyUetrConnection> * m,
                          bool key, ::common::ManyUetrConnection * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_common_ManyUetrConnection_iter(
      const google::protobuf::Map<bool, ::common::ManyUetrConnection>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_common_ManyUetrConnection_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::common::ManyUetrConnection** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::common::ManyUetrConnection>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::common::ManyUetrConnection>* __rust_proto_thunk__Map_ProtoStr_common_ManyUetrConnection_new() {
    return new google::protobuf::Map<std::string, ::common::ManyUetrConnection>();
  }
  void __rust_proto_thunk__Map_ProtoStr_common_ManyUetrConnection_free(const google::protobuf::Map<std::string, ::common::ManyUetrConnection>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_common_ManyUetrConnection_clear(google::protobuf::Map<std::string, ::common::ManyUetrConnection> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_common_ManyUetrConnection_size(const google::protobuf::Map<std::string, ::common::ManyUetrConnection>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_ManyUetrConnection_insert(google::protobuf::Map<std::string, ::common::ManyUetrConnection> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::ManyUetrConnection value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_ManyUetrConnection_get(const google::protobuf::Map<std::string, ::common::ManyUetrConnection>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::common::ManyUetrConnection** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::common::ManyUetrConnection* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_ManyUetrConnection_remove(google::protobuf::Map<std::string, ::common::ManyUetrConnection> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::ManyUetrConnection * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_common_ManyUetrConnection_iter(
      const google::protobuf::Map<std::string, ::common::ManyUetrConnection>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_common_ManyUetrConnection_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::common::ManyUetrConnection** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::common::ManyUetrConnection>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// common.AssociatedVerifierConnection
        // clang-format off
extern "C" {
void* __rust_proto_thunk__common_AssociatedVerifierConnection_new() { return new ::common::AssociatedVerifierConnection(); }
void __rust_proto_thunk__common_AssociatedVerifierConnection_delete(void* ptr) { delete static_cast<::common::AssociatedVerifierConnection*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__common_AssociatedVerifierConnection_serialize(::common::AssociatedVerifierConnection* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__common_AssociatedVerifierConnection_parse(::common::AssociatedVerifierConnection* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__common_AssociatedVerifierConnection_copy_from(::common::AssociatedVerifierConnection* dst, const ::common::AssociatedVerifierConnection* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__common_AssociatedVerifierConnection_repeated_len(google::protobuf::RepeatedPtrField<::common::AssociatedVerifierConnection>* field) {
  return field->size();
}
const ::common::AssociatedVerifierConnection& __rust_proto_thunk__common_AssociatedVerifierConnection_repeated_get(
  google::protobuf::RepeatedPtrField<::common::AssociatedVerifierConnection>* field,
  size_t index) {
  return field->Get(index);
}
::common::AssociatedVerifierConnection* __rust_proto_thunk__common_AssociatedVerifierConnection_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::common::AssociatedVerifierConnection>* field,
  size_t index) {
  return field->Mutable(index);
}
::common::AssociatedVerifierConnection* __rust_proto_thunk__common_AssociatedVerifierConnection_repeated_add(google::protobuf::RepeatedPtrField<::common::AssociatedVerifierConnection>* field) {
  return field->Add();
}
void __rust_proto_thunk__common_AssociatedVerifierConnection_repeated_clear(google::protobuf::RepeatedPtrField<::common::AssociatedVerifierConnection>* field) {
  field->Clear();
}
void __rust_proto_thunk__common_AssociatedVerifierConnection_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::common::AssociatedVerifierConnection>& dst,
  const google::protobuf::RepeatedPtrField<::common::AssociatedVerifierConnection>& src) {
  dst = src;
}

const void* __rust_proto_thunk__common_AssociatedVerifierConnection_get_primary_verifier(::common::AssociatedVerifierConnection* msg) {
  return static_cast<const void*>(&msg->primary_verifier());
}
void* __rust_proto_thunk__common_AssociatedVerifierConnection_get_mut_primary_verifier(::common::AssociatedVerifierConnection* msg) {
  return static_cast<void*>(msg->mutable_primary_verifier());
}
void __rust_proto_thunk__common_AssociatedVerifierConnection_clear_primary_verifier(::common::AssociatedVerifierConnection* msg) { msg->clear_primary_verifier(); }
bool __rust_proto_thunk__common_AssociatedVerifierConnection_has_primary_verifier(::common::AssociatedVerifierConnection* msg) { return msg->has_primary_verifier(); }

const void* __rust_proto_thunk__common_AssociatedVerifierConnection_get_associated_verifier(::common::AssociatedVerifierConnection* msg) {
  return static_cast<const void*>(&msg->associated_verifier());
}
void* __rust_proto_thunk__common_AssociatedVerifierConnection_get_mut_associated_verifier(::common::AssociatedVerifierConnection* msg) {
  return static_cast<void*>(msg->mutable_associated_verifier());
}
void __rust_proto_thunk__common_AssociatedVerifierConnection_clear_associated_verifier(::common::AssociatedVerifierConnection* msg) { msg->clear_associated_verifier(); }
bool __rust_proto_thunk__common_AssociatedVerifierConnection_has_associated_verifier(::common::AssociatedVerifierConnection* msg) { return msg->has_associated_verifier(); }


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::common::AssociatedVerifierConnection>* __rust_proto_thunk__Map_i32_common_AssociatedVerifierConnection_new() {
    return new google::protobuf::Map<int32_t, ::common::AssociatedVerifierConnection>();
  }
  void __rust_proto_thunk__Map_i32_common_AssociatedVerifierConnection_free(const google::protobuf::Map<int32_t, ::common::AssociatedVerifierConnection>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_common_AssociatedVerifierConnection_clear(google::protobuf::Map<int32_t, ::common::AssociatedVerifierConnection> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_common_AssociatedVerifierConnection_size(const google::protobuf::Map<int32_t, ::common::AssociatedVerifierConnection>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_common_AssociatedVerifierConnection_insert(google::protobuf::Map<int32_t, ::common::AssociatedVerifierConnection> * m,
                          int32_t key, ::common::AssociatedVerifierConnection value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_AssociatedVerifierConnection_get(const google::protobuf::Map<int32_t, ::common::AssociatedVerifierConnection>* m,
                       int32_t key, const ::common::AssociatedVerifierConnection** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::AssociatedVerifierConnection* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_AssociatedVerifierConnection_remove(google::protobuf::Map<int32_t, ::common::AssociatedVerifierConnection> * m,
                          int32_t key, ::common::AssociatedVerifierConnection * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_common_AssociatedVerifierConnection_iter(
      const google::protobuf::Map<int32_t, ::common::AssociatedVerifierConnection>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_common_AssociatedVerifierConnection_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::common::AssociatedVerifierConnection** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::common::AssociatedVerifierConnection>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::common::AssociatedVerifierConnection>* __rust_proto_thunk__Map_u32_common_AssociatedVerifierConnection_new() {
    return new google::protobuf::Map<uint32_t, ::common::AssociatedVerifierConnection>();
  }
  void __rust_proto_thunk__Map_u32_common_AssociatedVerifierConnection_free(const google::protobuf::Map<uint32_t, ::common::AssociatedVerifierConnection>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_common_AssociatedVerifierConnection_clear(google::protobuf::Map<uint32_t, ::common::AssociatedVerifierConnection> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_common_AssociatedVerifierConnection_size(const google::protobuf::Map<uint32_t, ::common::AssociatedVerifierConnection>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_common_AssociatedVerifierConnection_insert(google::protobuf::Map<uint32_t, ::common::AssociatedVerifierConnection> * m,
                          uint32_t key, ::common::AssociatedVerifierConnection value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_AssociatedVerifierConnection_get(const google::protobuf::Map<uint32_t, ::common::AssociatedVerifierConnection>* m,
                       uint32_t key, const ::common::AssociatedVerifierConnection** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::AssociatedVerifierConnection* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_AssociatedVerifierConnection_remove(google::protobuf::Map<uint32_t, ::common::AssociatedVerifierConnection> * m,
                          uint32_t key, ::common::AssociatedVerifierConnection * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_common_AssociatedVerifierConnection_iter(
      const google::protobuf::Map<uint32_t, ::common::AssociatedVerifierConnection>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_common_AssociatedVerifierConnection_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::common::AssociatedVerifierConnection** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::common::AssociatedVerifierConnection>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::common::AssociatedVerifierConnection>* __rust_proto_thunk__Map_i64_common_AssociatedVerifierConnection_new() {
    return new google::protobuf::Map<int64_t, ::common::AssociatedVerifierConnection>();
  }
  void __rust_proto_thunk__Map_i64_common_AssociatedVerifierConnection_free(const google::protobuf::Map<int64_t, ::common::AssociatedVerifierConnection>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_common_AssociatedVerifierConnection_clear(google::protobuf::Map<int64_t, ::common::AssociatedVerifierConnection> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_common_AssociatedVerifierConnection_size(const google::protobuf::Map<int64_t, ::common::AssociatedVerifierConnection>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_common_AssociatedVerifierConnection_insert(google::protobuf::Map<int64_t, ::common::AssociatedVerifierConnection> * m,
                          int64_t key, ::common::AssociatedVerifierConnection value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_AssociatedVerifierConnection_get(const google::protobuf::Map<int64_t, ::common::AssociatedVerifierConnection>* m,
                       int64_t key, const ::common::AssociatedVerifierConnection** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::AssociatedVerifierConnection* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_AssociatedVerifierConnection_remove(google::protobuf::Map<int64_t, ::common::AssociatedVerifierConnection> * m,
                          int64_t key, ::common::AssociatedVerifierConnection * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_common_AssociatedVerifierConnection_iter(
      const google::protobuf::Map<int64_t, ::common::AssociatedVerifierConnection>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_common_AssociatedVerifierConnection_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::common::AssociatedVerifierConnection** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::common::AssociatedVerifierConnection>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::common::AssociatedVerifierConnection>* __rust_proto_thunk__Map_u64_common_AssociatedVerifierConnection_new() {
    return new google::protobuf::Map<uint64_t, ::common::AssociatedVerifierConnection>();
  }
  void __rust_proto_thunk__Map_u64_common_AssociatedVerifierConnection_free(const google::protobuf::Map<uint64_t, ::common::AssociatedVerifierConnection>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_common_AssociatedVerifierConnection_clear(google::protobuf::Map<uint64_t, ::common::AssociatedVerifierConnection> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_common_AssociatedVerifierConnection_size(const google::protobuf::Map<uint64_t, ::common::AssociatedVerifierConnection>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_common_AssociatedVerifierConnection_insert(google::protobuf::Map<uint64_t, ::common::AssociatedVerifierConnection> * m,
                          uint64_t key, ::common::AssociatedVerifierConnection value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_AssociatedVerifierConnection_get(const google::protobuf::Map<uint64_t, ::common::AssociatedVerifierConnection>* m,
                       uint64_t key, const ::common::AssociatedVerifierConnection** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::AssociatedVerifierConnection* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_AssociatedVerifierConnection_remove(google::protobuf::Map<uint64_t, ::common::AssociatedVerifierConnection> * m,
                          uint64_t key, ::common::AssociatedVerifierConnection * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_common_AssociatedVerifierConnection_iter(
      const google::protobuf::Map<uint64_t, ::common::AssociatedVerifierConnection>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_common_AssociatedVerifierConnection_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::common::AssociatedVerifierConnection** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::common::AssociatedVerifierConnection>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::common::AssociatedVerifierConnection>* __rust_proto_thunk__Map_bool_common_AssociatedVerifierConnection_new() {
    return new google::protobuf::Map<bool, ::common::AssociatedVerifierConnection>();
  }
  void __rust_proto_thunk__Map_bool_common_AssociatedVerifierConnection_free(const google::protobuf::Map<bool, ::common::AssociatedVerifierConnection>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_common_AssociatedVerifierConnection_clear(google::protobuf::Map<bool, ::common::AssociatedVerifierConnection> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_common_AssociatedVerifierConnection_size(const google::protobuf::Map<bool, ::common::AssociatedVerifierConnection>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_common_AssociatedVerifierConnection_insert(google::protobuf::Map<bool, ::common::AssociatedVerifierConnection> * m,
                          bool key, ::common::AssociatedVerifierConnection value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_AssociatedVerifierConnection_get(const google::protobuf::Map<bool, ::common::AssociatedVerifierConnection>* m,
                       bool key, const ::common::AssociatedVerifierConnection** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::AssociatedVerifierConnection* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_AssociatedVerifierConnection_remove(google::protobuf::Map<bool, ::common::AssociatedVerifierConnection> * m,
                          bool key, ::common::AssociatedVerifierConnection * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_common_AssociatedVerifierConnection_iter(
      const google::protobuf::Map<bool, ::common::AssociatedVerifierConnection>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_common_AssociatedVerifierConnection_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::common::AssociatedVerifierConnection** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::common::AssociatedVerifierConnection>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::common::AssociatedVerifierConnection>* __rust_proto_thunk__Map_ProtoStr_common_AssociatedVerifierConnection_new() {
    return new google::protobuf::Map<std::string, ::common::AssociatedVerifierConnection>();
  }
  void __rust_proto_thunk__Map_ProtoStr_common_AssociatedVerifierConnection_free(const google::protobuf::Map<std::string, ::common::AssociatedVerifierConnection>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_common_AssociatedVerifierConnection_clear(google::protobuf::Map<std::string, ::common::AssociatedVerifierConnection> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_common_AssociatedVerifierConnection_size(const google::protobuf::Map<std::string, ::common::AssociatedVerifierConnection>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_AssociatedVerifierConnection_insert(google::protobuf::Map<std::string, ::common::AssociatedVerifierConnection> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::AssociatedVerifierConnection value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_AssociatedVerifierConnection_get(const google::protobuf::Map<std::string, ::common::AssociatedVerifierConnection>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::common::AssociatedVerifierConnection** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::common::AssociatedVerifierConnection* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_AssociatedVerifierConnection_remove(google::protobuf::Map<std::string, ::common::AssociatedVerifierConnection> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::AssociatedVerifierConnection * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_common_AssociatedVerifierConnection_iter(
      const google::protobuf::Map<std::string, ::common::AssociatedVerifierConnection>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_common_AssociatedVerifierConnection_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::common::AssociatedVerifierConnection** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::common::AssociatedVerifierConnection>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// common.ConnectionRequest
        // clang-format off
extern "C" {
void* __rust_proto_thunk__common_ConnectionRequest_new() { return new ::common::ConnectionRequest(); }
void __rust_proto_thunk__common_ConnectionRequest_delete(void* ptr) { delete static_cast<::common::ConnectionRequest*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__common_ConnectionRequest_serialize(::common::ConnectionRequest* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__common_ConnectionRequest_parse(::common::ConnectionRequest* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__common_ConnectionRequest_copy_from(::common::ConnectionRequest* dst, const ::common::ConnectionRequest* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__common_ConnectionRequest_repeated_len(google::protobuf::RepeatedPtrField<::common::ConnectionRequest>* field) {
  return field->size();
}
const ::common::ConnectionRequest& __rust_proto_thunk__common_ConnectionRequest_repeated_get(
  google::protobuf::RepeatedPtrField<::common::ConnectionRequest>* field,
  size_t index) {
  return field->Get(index);
}
::common::ConnectionRequest* __rust_proto_thunk__common_ConnectionRequest_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::common::ConnectionRequest>* field,
  size_t index) {
  return field->Mutable(index);
}
::common::ConnectionRequest* __rust_proto_thunk__common_ConnectionRequest_repeated_add(google::protobuf::RepeatedPtrField<::common::ConnectionRequest>* field) {
  return field->Add();
}
void __rust_proto_thunk__common_ConnectionRequest_repeated_clear(google::protobuf::RepeatedPtrField<::common::ConnectionRequest>* field) {
  field->Clear();
}
void __rust_proto_thunk__common_ConnectionRequest_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::common::ConnectionRequest>& dst,
  const google::protobuf::RepeatedPtrField<::common::ConnectionRequest>& src) {
  dst = src;
}

const void* __rust_proto_thunk__common_ConnectionRequest_get_one(::common::ConnectionRequest* msg) {
  return static_cast<const void*>(&msg->one());
}
void* __rust_proto_thunk__common_ConnectionRequest_get_mut_one(::common::ConnectionRequest* msg) {
  return static_cast<void*>(msg->mutable_one());
}
void __rust_proto_thunk__common_ConnectionRequest_clear_one(::common::ConnectionRequest* msg) { msg->clear_one(); }
bool __rust_proto_thunk__common_ConnectionRequest_has_one(::common::ConnectionRequest* msg) { return msg->has_one(); }

const void* __rust_proto_thunk__common_ConnectionRequest_get_many_verifier(::common::ConnectionRequest* msg) {
  return static_cast<const void*>(&msg->many_verifier());
}
void* __rust_proto_thunk__common_ConnectionRequest_get_mut_many_verifier(::common::ConnectionRequest* msg) {
  return static_cast<void*>(msg->mutable_many_verifier());
}
void __rust_proto_thunk__common_ConnectionRequest_clear_many_verifier(::common::ConnectionRequest* msg) { msg->clear_many_verifier(); }
bool __rust_proto_thunk__common_ConnectionRequest_has_many_verifier(::common::ConnectionRequest* msg) { return msg->has_many_verifier(); }

const void* __rust_proto_thunk__common_ConnectionRequest_get_many_uetr(::common::ConnectionRequest* msg) {
  return static_cast<const void*>(&msg->many_uetr());
}
void* __rust_proto_thunk__common_ConnectionRequest_get_mut_many_uetr(::common::ConnectionRequest* msg) {
  return static_cast<void*>(msg->mutable_many_uetr());
}
void __rust_proto_thunk__common_ConnectionRequest_clear_many_uetr(::common::ConnectionRequest* msg) { msg->clear_many_uetr(); }
bool __rust_proto_thunk__common_ConnectionRequest_has_many_uetr(::common::ConnectionRequest* msg) { return msg->has_many_uetr(); }

const void* __rust_proto_thunk__common_ConnectionRequest_get_signature(::common::ConnectionRequest* msg) {
  return static_cast<const void*>(&msg->signature());
}
void* __rust_proto_thunk__common_ConnectionRequest_get_mut_signature(::common::ConnectionRequest* msg) {
  return static_cast<void*>(msg->mutable_signature());
}
void __rust_proto_thunk__common_ConnectionRequest_clear_signature(::common::ConnectionRequest* msg) { msg->clear_signature(); }
bool __rust_proto_thunk__common_ConnectionRequest_has_signature(::common::ConnectionRequest* msg) { return msg->has_signature(); }

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__common_ConnectionRequest_get_connection_id(::common::ConnectionRequest* msg) {
  absl::string_view val = msg->connection_id();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__common_ConnectionRequest_set_connection_id(::common::ConnectionRequest* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_connection_id(absl::string_view(s.ptr, s.len));
}


::common::ConnectionRequest::RequestTypeCase __rust_proto_thunk__common_ConnectionRequest_case_request_type(::common::ConnectionRequest* msg) {
  return msg->request_type_case();
}
}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::common::ConnectionRequest>* __rust_proto_thunk__Map_i32_common_ConnectionRequest_new() {
    return new google::protobuf::Map<int32_t, ::common::ConnectionRequest>();
  }
  void __rust_proto_thunk__Map_i32_common_ConnectionRequest_free(const google::protobuf::Map<int32_t, ::common::ConnectionRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_common_ConnectionRequest_clear(google::protobuf::Map<int32_t, ::common::ConnectionRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_common_ConnectionRequest_size(const google::protobuf::Map<int32_t, ::common::ConnectionRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_common_ConnectionRequest_insert(google::protobuf::Map<int32_t, ::common::ConnectionRequest> * m,
                          int32_t key, ::common::ConnectionRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_ConnectionRequest_get(const google::protobuf::Map<int32_t, ::common::ConnectionRequest>* m,
                       int32_t key, const ::common::ConnectionRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::ConnectionRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_ConnectionRequest_remove(google::protobuf::Map<int32_t, ::common::ConnectionRequest> * m,
                          int32_t key, ::common::ConnectionRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_common_ConnectionRequest_iter(
      const google::protobuf::Map<int32_t, ::common::ConnectionRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_common_ConnectionRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::common::ConnectionRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::common::ConnectionRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::common::ConnectionRequest>* __rust_proto_thunk__Map_u32_common_ConnectionRequest_new() {
    return new google::protobuf::Map<uint32_t, ::common::ConnectionRequest>();
  }
  void __rust_proto_thunk__Map_u32_common_ConnectionRequest_free(const google::protobuf::Map<uint32_t, ::common::ConnectionRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_common_ConnectionRequest_clear(google::protobuf::Map<uint32_t, ::common::ConnectionRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_common_ConnectionRequest_size(const google::protobuf::Map<uint32_t, ::common::ConnectionRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_common_ConnectionRequest_insert(google::protobuf::Map<uint32_t, ::common::ConnectionRequest> * m,
                          uint32_t key, ::common::ConnectionRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_ConnectionRequest_get(const google::protobuf::Map<uint32_t, ::common::ConnectionRequest>* m,
                       uint32_t key, const ::common::ConnectionRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::ConnectionRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_ConnectionRequest_remove(google::protobuf::Map<uint32_t, ::common::ConnectionRequest> * m,
                          uint32_t key, ::common::ConnectionRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_common_ConnectionRequest_iter(
      const google::protobuf::Map<uint32_t, ::common::ConnectionRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_common_ConnectionRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::common::ConnectionRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::common::ConnectionRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::common::ConnectionRequest>* __rust_proto_thunk__Map_i64_common_ConnectionRequest_new() {
    return new google::protobuf::Map<int64_t, ::common::ConnectionRequest>();
  }
  void __rust_proto_thunk__Map_i64_common_ConnectionRequest_free(const google::protobuf::Map<int64_t, ::common::ConnectionRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_common_ConnectionRequest_clear(google::protobuf::Map<int64_t, ::common::ConnectionRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_common_ConnectionRequest_size(const google::protobuf::Map<int64_t, ::common::ConnectionRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_common_ConnectionRequest_insert(google::protobuf::Map<int64_t, ::common::ConnectionRequest> * m,
                          int64_t key, ::common::ConnectionRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_ConnectionRequest_get(const google::protobuf::Map<int64_t, ::common::ConnectionRequest>* m,
                       int64_t key, const ::common::ConnectionRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::ConnectionRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_ConnectionRequest_remove(google::protobuf::Map<int64_t, ::common::ConnectionRequest> * m,
                          int64_t key, ::common::ConnectionRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_common_ConnectionRequest_iter(
      const google::protobuf::Map<int64_t, ::common::ConnectionRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_common_ConnectionRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::common::ConnectionRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::common::ConnectionRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::common::ConnectionRequest>* __rust_proto_thunk__Map_u64_common_ConnectionRequest_new() {
    return new google::protobuf::Map<uint64_t, ::common::ConnectionRequest>();
  }
  void __rust_proto_thunk__Map_u64_common_ConnectionRequest_free(const google::protobuf::Map<uint64_t, ::common::ConnectionRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_common_ConnectionRequest_clear(google::protobuf::Map<uint64_t, ::common::ConnectionRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_common_ConnectionRequest_size(const google::protobuf::Map<uint64_t, ::common::ConnectionRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_common_ConnectionRequest_insert(google::protobuf::Map<uint64_t, ::common::ConnectionRequest> * m,
                          uint64_t key, ::common::ConnectionRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_ConnectionRequest_get(const google::protobuf::Map<uint64_t, ::common::ConnectionRequest>* m,
                       uint64_t key, const ::common::ConnectionRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::ConnectionRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_ConnectionRequest_remove(google::protobuf::Map<uint64_t, ::common::ConnectionRequest> * m,
                          uint64_t key, ::common::ConnectionRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_common_ConnectionRequest_iter(
      const google::protobuf::Map<uint64_t, ::common::ConnectionRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_common_ConnectionRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::common::ConnectionRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::common::ConnectionRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::common::ConnectionRequest>* __rust_proto_thunk__Map_bool_common_ConnectionRequest_new() {
    return new google::protobuf::Map<bool, ::common::ConnectionRequest>();
  }
  void __rust_proto_thunk__Map_bool_common_ConnectionRequest_free(const google::protobuf::Map<bool, ::common::ConnectionRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_common_ConnectionRequest_clear(google::protobuf::Map<bool, ::common::ConnectionRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_common_ConnectionRequest_size(const google::protobuf::Map<bool, ::common::ConnectionRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_common_ConnectionRequest_insert(google::protobuf::Map<bool, ::common::ConnectionRequest> * m,
                          bool key, ::common::ConnectionRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_ConnectionRequest_get(const google::protobuf::Map<bool, ::common::ConnectionRequest>* m,
                       bool key, const ::common::ConnectionRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::ConnectionRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_ConnectionRequest_remove(google::protobuf::Map<bool, ::common::ConnectionRequest> * m,
                          bool key, ::common::ConnectionRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_common_ConnectionRequest_iter(
      const google::protobuf::Map<bool, ::common::ConnectionRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_common_ConnectionRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::common::ConnectionRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::common::ConnectionRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::common::ConnectionRequest>* __rust_proto_thunk__Map_ProtoStr_common_ConnectionRequest_new() {
    return new google::protobuf::Map<std::string, ::common::ConnectionRequest>();
  }
  void __rust_proto_thunk__Map_ProtoStr_common_ConnectionRequest_free(const google::protobuf::Map<std::string, ::common::ConnectionRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_common_ConnectionRequest_clear(google::protobuf::Map<std::string, ::common::ConnectionRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_common_ConnectionRequest_size(const google::protobuf::Map<std::string, ::common::ConnectionRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_ConnectionRequest_insert(google::protobuf::Map<std::string, ::common::ConnectionRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::ConnectionRequest value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_ConnectionRequest_get(const google::protobuf::Map<std::string, ::common::ConnectionRequest>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::common::ConnectionRequest** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::common::ConnectionRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_ConnectionRequest_remove(google::protobuf::Map<std::string, ::common::ConnectionRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::ConnectionRequest * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_common_ConnectionRequest_iter(
      const google::protobuf::Map<std::string, ::common::ConnectionRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_common_ConnectionRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::common::ConnectionRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::common::ConnectionRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// common.AuthenticationSignature
        // clang-format off
extern "C" {
void* __rust_proto_thunk__common_AuthenticationSignature_new() { return new ::common::AuthenticationSignature(); }
void __rust_proto_thunk__common_AuthenticationSignature_delete(void* ptr) { delete static_cast<::common::AuthenticationSignature*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__common_AuthenticationSignature_serialize(::common::AuthenticationSignature* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__common_AuthenticationSignature_parse(::common::AuthenticationSignature* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__common_AuthenticationSignature_copy_from(::common::AuthenticationSignature* dst, const ::common::AuthenticationSignature* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__common_AuthenticationSignature_repeated_len(google::protobuf::RepeatedPtrField<::common::AuthenticationSignature>* field) {
  return field->size();
}
const ::common::AuthenticationSignature& __rust_proto_thunk__common_AuthenticationSignature_repeated_get(
  google::protobuf::RepeatedPtrField<::common::AuthenticationSignature>* field,
  size_t index) {
  return field->Get(index);
}
::common::AuthenticationSignature* __rust_proto_thunk__common_AuthenticationSignature_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::common::AuthenticationSignature>* field,
  size_t index) {
  return field->Mutable(index);
}
::common::AuthenticationSignature* __rust_proto_thunk__common_AuthenticationSignature_repeated_add(google::protobuf::RepeatedPtrField<::common::AuthenticationSignature>* field) {
  return field->Add();
}
void __rust_proto_thunk__common_AuthenticationSignature_repeated_clear(google::protobuf::RepeatedPtrField<::common::AuthenticationSignature>* field) {
  field->Clear();
}
void __rust_proto_thunk__common_AuthenticationSignature_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::common::AuthenticationSignature>& dst,
  const google::protobuf::RepeatedPtrField<::common::AuthenticationSignature>& src) {
  dst = src;
}

const void* __rust_proto_thunk__common_AuthenticationSignature_get_verifier(::common::AuthenticationSignature* msg) {
  return static_cast<const void*>(&msg->verifier());
}
void* __rust_proto_thunk__common_AuthenticationSignature_get_mut_verifier(::common::AuthenticationSignature* msg) {
  return static_cast<void*>(msg->mutable_verifier());
}
void __rust_proto_thunk__common_AuthenticationSignature_clear_verifier(::common::AuthenticationSignature* msg) { msg->clear_verifier(); }
bool __rust_proto_thunk__common_AuthenticationSignature_has_verifier(::common::AuthenticationSignature* msg) { return msg->has_verifier(); }

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__common_AuthenticationSignature_get_signature(::common::AuthenticationSignature* msg) {
  absl::string_view val = msg->signature();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__common_AuthenticationSignature_set_signature(::common::AuthenticationSignature* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_signature(absl::string_view(s.ptr, s.len));
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::common::AuthenticationSignature>* __rust_proto_thunk__Map_i32_common_AuthenticationSignature_new() {
    return new google::protobuf::Map<int32_t, ::common::AuthenticationSignature>();
  }
  void __rust_proto_thunk__Map_i32_common_AuthenticationSignature_free(const google::protobuf::Map<int32_t, ::common::AuthenticationSignature>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_common_AuthenticationSignature_clear(google::protobuf::Map<int32_t, ::common::AuthenticationSignature> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_common_AuthenticationSignature_size(const google::protobuf::Map<int32_t, ::common::AuthenticationSignature>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_common_AuthenticationSignature_insert(google::protobuf::Map<int32_t, ::common::AuthenticationSignature> * m,
                          int32_t key, ::common::AuthenticationSignature value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_AuthenticationSignature_get(const google::protobuf::Map<int32_t, ::common::AuthenticationSignature>* m,
                       int32_t key, const ::common::AuthenticationSignature** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::AuthenticationSignature* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_AuthenticationSignature_remove(google::protobuf::Map<int32_t, ::common::AuthenticationSignature> * m,
                          int32_t key, ::common::AuthenticationSignature * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_common_AuthenticationSignature_iter(
      const google::protobuf::Map<int32_t, ::common::AuthenticationSignature>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_common_AuthenticationSignature_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::common::AuthenticationSignature** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::common::AuthenticationSignature>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::common::AuthenticationSignature>* __rust_proto_thunk__Map_u32_common_AuthenticationSignature_new() {
    return new google::protobuf::Map<uint32_t, ::common::AuthenticationSignature>();
  }
  void __rust_proto_thunk__Map_u32_common_AuthenticationSignature_free(const google::protobuf::Map<uint32_t, ::common::AuthenticationSignature>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_common_AuthenticationSignature_clear(google::protobuf::Map<uint32_t, ::common::AuthenticationSignature> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_common_AuthenticationSignature_size(const google::protobuf::Map<uint32_t, ::common::AuthenticationSignature>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_common_AuthenticationSignature_insert(google::protobuf::Map<uint32_t, ::common::AuthenticationSignature> * m,
                          uint32_t key, ::common::AuthenticationSignature value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_AuthenticationSignature_get(const google::protobuf::Map<uint32_t, ::common::AuthenticationSignature>* m,
                       uint32_t key, const ::common::AuthenticationSignature** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::AuthenticationSignature* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_AuthenticationSignature_remove(google::protobuf::Map<uint32_t, ::common::AuthenticationSignature> * m,
                          uint32_t key, ::common::AuthenticationSignature * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_common_AuthenticationSignature_iter(
      const google::protobuf::Map<uint32_t, ::common::AuthenticationSignature>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_common_AuthenticationSignature_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::common::AuthenticationSignature** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::common::AuthenticationSignature>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::common::AuthenticationSignature>* __rust_proto_thunk__Map_i64_common_AuthenticationSignature_new() {
    return new google::protobuf::Map<int64_t, ::common::AuthenticationSignature>();
  }
  void __rust_proto_thunk__Map_i64_common_AuthenticationSignature_free(const google::protobuf::Map<int64_t, ::common::AuthenticationSignature>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_common_AuthenticationSignature_clear(google::protobuf::Map<int64_t, ::common::AuthenticationSignature> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_common_AuthenticationSignature_size(const google::protobuf::Map<int64_t, ::common::AuthenticationSignature>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_common_AuthenticationSignature_insert(google::protobuf::Map<int64_t, ::common::AuthenticationSignature> * m,
                          int64_t key, ::common::AuthenticationSignature value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_AuthenticationSignature_get(const google::protobuf::Map<int64_t, ::common::AuthenticationSignature>* m,
                       int64_t key, const ::common::AuthenticationSignature** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::AuthenticationSignature* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_AuthenticationSignature_remove(google::protobuf::Map<int64_t, ::common::AuthenticationSignature> * m,
                          int64_t key, ::common::AuthenticationSignature * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_common_AuthenticationSignature_iter(
      const google::protobuf::Map<int64_t, ::common::AuthenticationSignature>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_common_AuthenticationSignature_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::common::AuthenticationSignature** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::common::AuthenticationSignature>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::common::AuthenticationSignature>* __rust_proto_thunk__Map_u64_common_AuthenticationSignature_new() {
    return new google::protobuf::Map<uint64_t, ::common::AuthenticationSignature>();
  }
  void __rust_proto_thunk__Map_u64_common_AuthenticationSignature_free(const google::protobuf::Map<uint64_t, ::common::AuthenticationSignature>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_common_AuthenticationSignature_clear(google::protobuf::Map<uint64_t, ::common::AuthenticationSignature> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_common_AuthenticationSignature_size(const google::protobuf::Map<uint64_t, ::common::AuthenticationSignature>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_common_AuthenticationSignature_insert(google::protobuf::Map<uint64_t, ::common::AuthenticationSignature> * m,
                          uint64_t key, ::common::AuthenticationSignature value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_AuthenticationSignature_get(const google::protobuf::Map<uint64_t, ::common::AuthenticationSignature>* m,
                       uint64_t key, const ::common::AuthenticationSignature** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::AuthenticationSignature* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_AuthenticationSignature_remove(google::protobuf::Map<uint64_t, ::common::AuthenticationSignature> * m,
                          uint64_t key, ::common::AuthenticationSignature * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_common_AuthenticationSignature_iter(
      const google::protobuf::Map<uint64_t, ::common::AuthenticationSignature>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_common_AuthenticationSignature_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::common::AuthenticationSignature** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::common::AuthenticationSignature>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::common::AuthenticationSignature>* __rust_proto_thunk__Map_bool_common_AuthenticationSignature_new() {
    return new google::protobuf::Map<bool, ::common::AuthenticationSignature>();
  }
  void __rust_proto_thunk__Map_bool_common_AuthenticationSignature_free(const google::protobuf::Map<bool, ::common::AuthenticationSignature>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_common_AuthenticationSignature_clear(google::protobuf::Map<bool, ::common::AuthenticationSignature> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_common_AuthenticationSignature_size(const google::protobuf::Map<bool, ::common::AuthenticationSignature>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_common_AuthenticationSignature_insert(google::protobuf::Map<bool, ::common::AuthenticationSignature> * m,
                          bool key, ::common::AuthenticationSignature value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_AuthenticationSignature_get(const google::protobuf::Map<bool, ::common::AuthenticationSignature>* m,
                       bool key, const ::common::AuthenticationSignature** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::AuthenticationSignature* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_AuthenticationSignature_remove(google::protobuf::Map<bool, ::common::AuthenticationSignature> * m,
                          bool key, ::common::AuthenticationSignature * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_common_AuthenticationSignature_iter(
      const google::protobuf::Map<bool, ::common::AuthenticationSignature>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_common_AuthenticationSignature_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::common::AuthenticationSignature** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::common::AuthenticationSignature>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::common::AuthenticationSignature>* __rust_proto_thunk__Map_ProtoStr_common_AuthenticationSignature_new() {
    return new google::protobuf::Map<std::string, ::common::AuthenticationSignature>();
  }
  void __rust_proto_thunk__Map_ProtoStr_common_AuthenticationSignature_free(const google::protobuf::Map<std::string, ::common::AuthenticationSignature>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_common_AuthenticationSignature_clear(google::protobuf::Map<std::string, ::common::AuthenticationSignature> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_common_AuthenticationSignature_size(const google::protobuf::Map<std::string, ::common::AuthenticationSignature>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_AuthenticationSignature_insert(google::protobuf::Map<std::string, ::common::AuthenticationSignature> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::AuthenticationSignature value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_AuthenticationSignature_get(const google::protobuf::Map<std::string, ::common::AuthenticationSignature>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::common::AuthenticationSignature** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::common::AuthenticationSignature* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_AuthenticationSignature_remove(google::protobuf::Map<std::string, ::common::AuthenticationSignature> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::AuthenticationSignature * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_common_AuthenticationSignature_iter(
      const google::protobuf::Map<std::string, ::common::AuthenticationSignature>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_common_AuthenticationSignature_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::common::AuthenticationSignature** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::common::AuthenticationSignature>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// common.AuthorizedSignatureResponse
        // clang-format off
extern "C" {
void* __rust_proto_thunk__common_AuthorizedSignatureResponse_new() { return new ::common::AuthorizedSignatureResponse(); }
void __rust_proto_thunk__common_AuthorizedSignatureResponse_delete(void* ptr) { delete static_cast<::common::AuthorizedSignatureResponse*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__common_AuthorizedSignatureResponse_serialize(::common::AuthorizedSignatureResponse* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__common_AuthorizedSignatureResponse_parse(::common::AuthorizedSignatureResponse* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__common_AuthorizedSignatureResponse_copy_from(::common::AuthorizedSignatureResponse* dst, const ::common::AuthorizedSignatureResponse* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__common_AuthorizedSignatureResponse_repeated_len(google::protobuf::RepeatedPtrField<::common::AuthorizedSignatureResponse>* field) {
  return field->size();
}
const ::common::AuthorizedSignatureResponse& __rust_proto_thunk__common_AuthorizedSignatureResponse_repeated_get(
  google::protobuf::RepeatedPtrField<::common::AuthorizedSignatureResponse>* field,
  size_t index) {
  return field->Get(index);
}
::common::AuthorizedSignatureResponse* __rust_proto_thunk__common_AuthorizedSignatureResponse_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::common::AuthorizedSignatureResponse>* field,
  size_t index) {
  return field->Mutable(index);
}
::common::AuthorizedSignatureResponse* __rust_proto_thunk__common_AuthorizedSignatureResponse_repeated_add(google::protobuf::RepeatedPtrField<::common::AuthorizedSignatureResponse>* field) {
  return field->Add();
}
void __rust_proto_thunk__common_AuthorizedSignatureResponse_repeated_clear(google::protobuf::RepeatedPtrField<::common::AuthorizedSignatureResponse>* field) {
  field->Clear();
}
void __rust_proto_thunk__common_AuthorizedSignatureResponse_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::common::AuthorizedSignatureResponse>& dst,
  const google::protobuf::RepeatedPtrField<::common::AuthorizedSignatureResponse>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__common_AuthorizedSignatureResponse_get_promissory_file(::common::AuthorizedSignatureResponse* msg) {
  absl::string_view val = msg->promissory_file();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__common_AuthorizedSignatureResponse_set_promissory_file(::common::AuthorizedSignatureResponse* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_promissory_file(absl::string_view(s.ptr, s.len));
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::common::AuthorizedSignatureResponse>* __rust_proto_thunk__Map_i32_common_AuthorizedSignatureResponse_new() {
    return new google::protobuf::Map<int32_t, ::common::AuthorizedSignatureResponse>();
  }
  void __rust_proto_thunk__Map_i32_common_AuthorizedSignatureResponse_free(const google::protobuf::Map<int32_t, ::common::AuthorizedSignatureResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_common_AuthorizedSignatureResponse_clear(google::protobuf::Map<int32_t, ::common::AuthorizedSignatureResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_common_AuthorizedSignatureResponse_size(const google::protobuf::Map<int32_t, ::common::AuthorizedSignatureResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_common_AuthorizedSignatureResponse_insert(google::protobuf::Map<int32_t, ::common::AuthorizedSignatureResponse> * m,
                          int32_t key, ::common::AuthorizedSignatureResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_AuthorizedSignatureResponse_get(const google::protobuf::Map<int32_t, ::common::AuthorizedSignatureResponse>* m,
                       int32_t key, const ::common::AuthorizedSignatureResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::AuthorizedSignatureResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_AuthorizedSignatureResponse_remove(google::protobuf::Map<int32_t, ::common::AuthorizedSignatureResponse> * m,
                          int32_t key, ::common::AuthorizedSignatureResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_common_AuthorizedSignatureResponse_iter(
      const google::protobuf::Map<int32_t, ::common::AuthorizedSignatureResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_common_AuthorizedSignatureResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::common::AuthorizedSignatureResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::common::AuthorizedSignatureResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::common::AuthorizedSignatureResponse>* __rust_proto_thunk__Map_u32_common_AuthorizedSignatureResponse_new() {
    return new google::protobuf::Map<uint32_t, ::common::AuthorizedSignatureResponse>();
  }
  void __rust_proto_thunk__Map_u32_common_AuthorizedSignatureResponse_free(const google::protobuf::Map<uint32_t, ::common::AuthorizedSignatureResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_common_AuthorizedSignatureResponse_clear(google::protobuf::Map<uint32_t, ::common::AuthorizedSignatureResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_common_AuthorizedSignatureResponse_size(const google::protobuf::Map<uint32_t, ::common::AuthorizedSignatureResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_common_AuthorizedSignatureResponse_insert(google::protobuf::Map<uint32_t, ::common::AuthorizedSignatureResponse> * m,
                          uint32_t key, ::common::AuthorizedSignatureResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_AuthorizedSignatureResponse_get(const google::protobuf::Map<uint32_t, ::common::AuthorizedSignatureResponse>* m,
                       uint32_t key, const ::common::AuthorizedSignatureResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::AuthorizedSignatureResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_AuthorizedSignatureResponse_remove(google::protobuf::Map<uint32_t, ::common::AuthorizedSignatureResponse> * m,
                          uint32_t key, ::common::AuthorizedSignatureResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_common_AuthorizedSignatureResponse_iter(
      const google::protobuf::Map<uint32_t, ::common::AuthorizedSignatureResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_common_AuthorizedSignatureResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::common::AuthorizedSignatureResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::common::AuthorizedSignatureResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::common::AuthorizedSignatureResponse>* __rust_proto_thunk__Map_i64_common_AuthorizedSignatureResponse_new() {
    return new google::protobuf::Map<int64_t, ::common::AuthorizedSignatureResponse>();
  }
  void __rust_proto_thunk__Map_i64_common_AuthorizedSignatureResponse_free(const google::protobuf::Map<int64_t, ::common::AuthorizedSignatureResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_common_AuthorizedSignatureResponse_clear(google::protobuf::Map<int64_t, ::common::AuthorizedSignatureResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_common_AuthorizedSignatureResponse_size(const google::protobuf::Map<int64_t, ::common::AuthorizedSignatureResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_common_AuthorizedSignatureResponse_insert(google::protobuf::Map<int64_t, ::common::AuthorizedSignatureResponse> * m,
                          int64_t key, ::common::AuthorizedSignatureResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_AuthorizedSignatureResponse_get(const google::protobuf::Map<int64_t, ::common::AuthorizedSignatureResponse>* m,
                       int64_t key, const ::common::AuthorizedSignatureResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::AuthorizedSignatureResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_AuthorizedSignatureResponse_remove(google::protobuf::Map<int64_t, ::common::AuthorizedSignatureResponse> * m,
                          int64_t key, ::common::AuthorizedSignatureResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_common_AuthorizedSignatureResponse_iter(
      const google::protobuf::Map<int64_t, ::common::AuthorizedSignatureResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_common_AuthorizedSignatureResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::common::AuthorizedSignatureResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::common::AuthorizedSignatureResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::common::AuthorizedSignatureResponse>* __rust_proto_thunk__Map_u64_common_AuthorizedSignatureResponse_new() {
    return new google::protobuf::Map<uint64_t, ::common::AuthorizedSignatureResponse>();
  }
  void __rust_proto_thunk__Map_u64_common_AuthorizedSignatureResponse_free(const google::protobuf::Map<uint64_t, ::common::AuthorizedSignatureResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_common_AuthorizedSignatureResponse_clear(google::protobuf::Map<uint64_t, ::common::AuthorizedSignatureResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_common_AuthorizedSignatureResponse_size(const google::protobuf::Map<uint64_t, ::common::AuthorizedSignatureResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_common_AuthorizedSignatureResponse_insert(google::protobuf::Map<uint64_t, ::common::AuthorizedSignatureResponse> * m,
                          uint64_t key, ::common::AuthorizedSignatureResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_AuthorizedSignatureResponse_get(const google::protobuf::Map<uint64_t, ::common::AuthorizedSignatureResponse>* m,
                       uint64_t key, const ::common::AuthorizedSignatureResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::AuthorizedSignatureResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_AuthorizedSignatureResponse_remove(google::protobuf::Map<uint64_t, ::common::AuthorizedSignatureResponse> * m,
                          uint64_t key, ::common::AuthorizedSignatureResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_common_AuthorizedSignatureResponse_iter(
      const google::protobuf::Map<uint64_t, ::common::AuthorizedSignatureResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_common_AuthorizedSignatureResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::common::AuthorizedSignatureResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::common::AuthorizedSignatureResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::common::AuthorizedSignatureResponse>* __rust_proto_thunk__Map_bool_common_AuthorizedSignatureResponse_new() {
    return new google::protobuf::Map<bool, ::common::AuthorizedSignatureResponse>();
  }
  void __rust_proto_thunk__Map_bool_common_AuthorizedSignatureResponse_free(const google::protobuf::Map<bool, ::common::AuthorizedSignatureResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_common_AuthorizedSignatureResponse_clear(google::protobuf::Map<bool, ::common::AuthorizedSignatureResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_common_AuthorizedSignatureResponse_size(const google::protobuf::Map<bool, ::common::AuthorizedSignatureResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_common_AuthorizedSignatureResponse_insert(google::protobuf::Map<bool, ::common::AuthorizedSignatureResponse> * m,
                          bool key, ::common::AuthorizedSignatureResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_AuthorizedSignatureResponse_get(const google::protobuf::Map<bool, ::common::AuthorizedSignatureResponse>* m,
                       bool key, const ::common::AuthorizedSignatureResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::AuthorizedSignatureResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_AuthorizedSignatureResponse_remove(google::protobuf::Map<bool, ::common::AuthorizedSignatureResponse> * m,
                          bool key, ::common::AuthorizedSignatureResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_common_AuthorizedSignatureResponse_iter(
      const google::protobuf::Map<bool, ::common::AuthorizedSignatureResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_common_AuthorizedSignatureResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::common::AuthorizedSignatureResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::common::AuthorizedSignatureResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::common::AuthorizedSignatureResponse>* __rust_proto_thunk__Map_ProtoStr_common_AuthorizedSignatureResponse_new() {
    return new google::protobuf::Map<std::string, ::common::AuthorizedSignatureResponse>();
  }
  void __rust_proto_thunk__Map_ProtoStr_common_AuthorizedSignatureResponse_free(const google::protobuf::Map<std::string, ::common::AuthorizedSignatureResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_common_AuthorizedSignatureResponse_clear(google::protobuf::Map<std::string, ::common::AuthorizedSignatureResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_common_AuthorizedSignatureResponse_size(const google::protobuf::Map<std::string, ::common::AuthorizedSignatureResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_AuthorizedSignatureResponse_insert(google::protobuf::Map<std::string, ::common::AuthorizedSignatureResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::AuthorizedSignatureResponse value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_AuthorizedSignatureResponse_get(const google::protobuf::Map<std::string, ::common::AuthorizedSignatureResponse>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::common::AuthorizedSignatureResponse** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::common::AuthorizedSignatureResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_AuthorizedSignatureResponse_remove(google::protobuf::Map<std::string, ::common::AuthorizedSignatureResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::AuthorizedSignatureResponse * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_common_AuthorizedSignatureResponse_iter(
      const google::protobuf::Map<std::string, ::common::AuthorizedSignatureResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_common_AuthorizedSignatureResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::common::AuthorizedSignatureResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::common::AuthorizedSignatureResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// common.Media
        // clang-format off
extern "C" {
void* __rust_proto_thunk__common_Media_new() { return new ::common::Media(); }
void __rust_proto_thunk__common_Media_delete(void* ptr) { delete static_cast<::common::Media*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__common_Media_serialize(::common::Media* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__common_Media_parse(::common::Media* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__common_Media_copy_from(::common::Media* dst, const ::common::Media* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__common_Media_repeated_len(google::protobuf::RepeatedPtrField<::common::Media>* field) {
  return field->size();
}
const ::common::Media& __rust_proto_thunk__common_Media_repeated_get(
  google::protobuf::RepeatedPtrField<::common::Media>* field,
  size_t index) {
  return field->Get(index);
}
::common::Media* __rust_proto_thunk__common_Media_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::common::Media>* field,
  size_t index) {
  return field->Mutable(index);
}
::common::Media* __rust_proto_thunk__common_Media_repeated_add(google::protobuf::RepeatedPtrField<::common::Media>* field) {
  return field->Add();
}
void __rust_proto_thunk__common_Media_repeated_clear(google::protobuf::RepeatedPtrField<::common::Media>* field) {
  field->Clear();
}
void __rust_proto_thunk__common_Media_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::common::Media>& dst,
  const google::protobuf::RepeatedPtrField<::common::Media>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__common_Media_get_mime_type(::common::Media* msg) {
  absl::string_view val = msg->mime_type();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__common_Media_set_mime_type(::common::Media* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_mime_type(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__common_Media_get_data(::common::Media* msg) {
  absl::string_view val = msg->data();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__common_Media_set_data(::common::Media* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_data(absl::string_view(s.ptr, s.len));
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::common::Media>* __rust_proto_thunk__Map_i32_common_Media_new() {
    return new google::protobuf::Map<int32_t, ::common::Media>();
  }
  void __rust_proto_thunk__Map_i32_common_Media_free(const google::protobuf::Map<int32_t, ::common::Media>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_common_Media_clear(google::protobuf::Map<int32_t, ::common::Media> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_common_Media_size(const google::protobuf::Map<int32_t, ::common::Media>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_common_Media_insert(google::protobuf::Map<int32_t, ::common::Media> * m,
                          int32_t key, ::common::Media value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_Media_get(const google::protobuf::Map<int32_t, ::common::Media>* m,
                       int32_t key, const ::common::Media** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::Media* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_Media_remove(google::protobuf::Map<int32_t, ::common::Media> * m,
                          int32_t key, ::common::Media * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_common_Media_iter(
      const google::protobuf::Map<int32_t, ::common::Media>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_common_Media_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::common::Media** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::common::Media>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::common::Media>* __rust_proto_thunk__Map_u32_common_Media_new() {
    return new google::protobuf::Map<uint32_t, ::common::Media>();
  }
  void __rust_proto_thunk__Map_u32_common_Media_free(const google::protobuf::Map<uint32_t, ::common::Media>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_common_Media_clear(google::protobuf::Map<uint32_t, ::common::Media> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_common_Media_size(const google::protobuf::Map<uint32_t, ::common::Media>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_common_Media_insert(google::protobuf::Map<uint32_t, ::common::Media> * m,
                          uint32_t key, ::common::Media value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_Media_get(const google::protobuf::Map<uint32_t, ::common::Media>* m,
                       uint32_t key, const ::common::Media** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::Media* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_Media_remove(google::protobuf::Map<uint32_t, ::common::Media> * m,
                          uint32_t key, ::common::Media * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_common_Media_iter(
      const google::protobuf::Map<uint32_t, ::common::Media>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_common_Media_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::common::Media** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::common::Media>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::common::Media>* __rust_proto_thunk__Map_i64_common_Media_new() {
    return new google::protobuf::Map<int64_t, ::common::Media>();
  }
  void __rust_proto_thunk__Map_i64_common_Media_free(const google::protobuf::Map<int64_t, ::common::Media>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_common_Media_clear(google::protobuf::Map<int64_t, ::common::Media> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_common_Media_size(const google::protobuf::Map<int64_t, ::common::Media>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_common_Media_insert(google::protobuf::Map<int64_t, ::common::Media> * m,
                          int64_t key, ::common::Media value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_Media_get(const google::protobuf::Map<int64_t, ::common::Media>* m,
                       int64_t key, const ::common::Media** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::Media* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_Media_remove(google::protobuf::Map<int64_t, ::common::Media> * m,
                          int64_t key, ::common::Media * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_common_Media_iter(
      const google::protobuf::Map<int64_t, ::common::Media>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_common_Media_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::common::Media** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::common::Media>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::common::Media>* __rust_proto_thunk__Map_u64_common_Media_new() {
    return new google::protobuf::Map<uint64_t, ::common::Media>();
  }
  void __rust_proto_thunk__Map_u64_common_Media_free(const google::protobuf::Map<uint64_t, ::common::Media>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_common_Media_clear(google::protobuf::Map<uint64_t, ::common::Media> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_common_Media_size(const google::protobuf::Map<uint64_t, ::common::Media>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_common_Media_insert(google::protobuf::Map<uint64_t, ::common::Media> * m,
                          uint64_t key, ::common::Media value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_Media_get(const google::protobuf::Map<uint64_t, ::common::Media>* m,
                       uint64_t key, const ::common::Media** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::Media* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_Media_remove(google::protobuf::Map<uint64_t, ::common::Media> * m,
                          uint64_t key, ::common::Media * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_common_Media_iter(
      const google::protobuf::Map<uint64_t, ::common::Media>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_common_Media_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::common::Media** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::common::Media>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::common::Media>* __rust_proto_thunk__Map_bool_common_Media_new() {
    return new google::protobuf::Map<bool, ::common::Media>();
  }
  void __rust_proto_thunk__Map_bool_common_Media_free(const google::protobuf::Map<bool, ::common::Media>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_common_Media_clear(google::protobuf::Map<bool, ::common::Media> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_common_Media_size(const google::protobuf::Map<bool, ::common::Media>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_common_Media_insert(google::protobuf::Map<bool, ::common::Media> * m,
                          bool key, ::common::Media value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_Media_get(const google::protobuf::Map<bool, ::common::Media>* m,
                       bool key, const ::common::Media** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::Media* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_Media_remove(google::protobuf::Map<bool, ::common::Media> * m,
                          bool key, ::common::Media * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_common_Media_iter(
      const google::protobuf::Map<bool, ::common::Media>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_common_Media_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::common::Media** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::common::Media>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::common::Media>* __rust_proto_thunk__Map_ProtoStr_common_Media_new() {
    return new google::protobuf::Map<std::string, ::common::Media>();
  }
  void __rust_proto_thunk__Map_ProtoStr_common_Media_free(const google::protobuf::Map<std::string, ::common::Media>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_common_Media_clear(google::protobuf::Map<std::string, ::common::Media> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_common_Media_size(const google::protobuf::Map<std::string, ::common::Media>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_Media_insert(google::protobuf::Map<std::string, ::common::Media> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::Media value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_Media_get(const google::protobuf::Map<std::string, ::common::Media>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::common::Media** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::common::Media* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_Media_remove(google::protobuf::Map<std::string, ::common::Media> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::Media * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_common_Media_iter(
      const google::protobuf::Map<std::string, ::common::Media>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_common_Media_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::common::Media** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::common::Media>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// common.Role
        // clang-format off
extern "C" {
void* __rust_proto_thunk__common_Role_new() { return new ::common::Role(); }
void __rust_proto_thunk__common_Role_delete(void* ptr) { delete static_cast<::common::Role*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__common_Role_serialize(::common::Role* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__common_Role_parse(::common::Role* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__common_Role_copy_from(::common::Role* dst, const ::common::Role* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__common_Role_repeated_len(google::protobuf::RepeatedPtrField<::common::Role>* field) {
  return field->size();
}
const ::common::Role& __rust_proto_thunk__common_Role_repeated_get(
  google::protobuf::RepeatedPtrField<::common::Role>* field,
  size_t index) {
  return field->Get(index);
}
::common::Role* __rust_proto_thunk__common_Role_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::common::Role>* field,
  size_t index) {
  return field->Mutable(index);
}
::common::Role* __rust_proto_thunk__common_Role_repeated_add(google::protobuf::RepeatedPtrField<::common::Role>* field) {
  return field->Add();
}
void __rust_proto_thunk__common_Role_repeated_clear(google::protobuf::RepeatedPtrField<::common::Role>* field) {
  field->Clear();
}
void __rust_proto_thunk__common_Role_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::common::Role>& dst,
  const google::protobuf::RepeatedPtrField<::common::Role>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__common_Role_get_verifier(::common::Role* msg) {
  absl::string_view val = msg->verifier();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__common_Role_set_verifier(::common::Role* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_verifier(absl::string_view(s.ptr, s.len));
}

bool __rust_proto_thunk__common_Role_has_authorized(::common::Role* msg) {
  return msg->has_authorized();
}
void __rust_proto_thunk__common_Role_clear_authorized(::common::Role* msg) { msg->clear_authorized(); }
bool __rust_proto_thunk__common_Role_get_authorized(::common::Role* msg) { return msg->authorized(); }
void __rust_proto_thunk__common_Role_set_authorized(::common::Role* msg, bool val) {
  msg->set_authorized(val);
}

const void* __rust_proto_thunk__common_Role_get_issue_permissions(::common::Role* msg) {
  return static_cast<const void*>(&msg->issue_permissions());
}
void* __rust_proto_thunk__common_Role_get_mut_issue_permissions(::common::Role* msg) {
  return static_cast<void*>(msg->mutable_issue_permissions());
}
void __rust_proto_thunk__common_Role_clear_issue_permissions(::common::Role* msg) { msg->clear_issue_permissions(); }
bool __rust_proto_thunk__common_Role_has_issue_permissions(::common::Role* msg) { return msg->has_issue_permissions(); }


::common::Role::RoleEnumCase __rust_proto_thunk__common_Role_case_role_enum(::common::Role* msg) {
  return msg->role_enum_case();
}
}  // extern "C"
// clang-format on

        // clang-format off
extern "C" {
void* __rust_proto_thunk__common_Role_IssuePermissions_new() { return new ::common::Role_IssuePermissions(); }
void __rust_proto_thunk__common_Role_IssuePermissions_delete(void* ptr) { delete static_cast<::common::Role_IssuePermissions*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__common_Role_IssuePermissions_serialize(::common::Role_IssuePermissions* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__common_Role_IssuePermissions_parse(::common::Role_IssuePermissions* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__common_Role_IssuePermissions_copy_from(::common::Role_IssuePermissions* dst, const ::common::Role_IssuePermissions* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__common_Role_IssuePermissions_repeated_len(google::protobuf::RepeatedPtrField<::common::Role_IssuePermissions>* field) {
  return field->size();
}
const ::common::Role_IssuePermissions& __rust_proto_thunk__common_Role_IssuePermissions_repeated_get(
  google::protobuf::RepeatedPtrField<::common::Role_IssuePermissions>* field,
  size_t index) {
  return field->Get(index);
}
::common::Role_IssuePermissions* __rust_proto_thunk__common_Role_IssuePermissions_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::common::Role_IssuePermissions>* field,
  size_t index) {
  return field->Mutable(index);
}
::common::Role_IssuePermissions* __rust_proto_thunk__common_Role_IssuePermissions_repeated_add(google::protobuf::RepeatedPtrField<::common::Role_IssuePermissions>* field) {
  return field->Add();
}
void __rust_proto_thunk__common_Role_IssuePermissions_repeated_clear(google::protobuf::RepeatedPtrField<::common::Role_IssuePermissions>* field) {
  field->Clear();
}
void __rust_proto_thunk__common_Role_IssuePermissions_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::common::Role_IssuePermissions>& dst,
  const google::protobuf::RepeatedPtrField<::common::Role_IssuePermissions>& src) {
  dst = src;
}

void __rust_proto_thunk__common_Role_IssuePermissions_clear_inner(::common::Role_IssuePermissions* msg) {
  msg->clear_inner();
}
google::protobuf::RepeatedPtrField<::common::Amount>* __rust_proto_thunk__common_Role_IssuePermissions_get_mut_inner(::common::Role_IssuePermissions* msg) {
  return msg->mutable_inner();
}
const google::protobuf::RepeatedPtrField<::common::Amount>* __rust_proto_thunk__common_Role_IssuePermissions_get_inner(
    const ::common::Role_IssuePermissions* msg) {
  return &msg->inner();
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::common::Role_IssuePermissions>* __rust_proto_thunk__Map_i32_common_Role_IssuePermissions_new() {
    return new google::protobuf::Map<int32_t, ::common::Role_IssuePermissions>();
  }
  void __rust_proto_thunk__Map_i32_common_Role_IssuePermissions_free(const google::protobuf::Map<int32_t, ::common::Role_IssuePermissions>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_common_Role_IssuePermissions_clear(google::protobuf::Map<int32_t, ::common::Role_IssuePermissions> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_common_Role_IssuePermissions_size(const google::protobuf::Map<int32_t, ::common::Role_IssuePermissions>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_common_Role_IssuePermissions_insert(google::protobuf::Map<int32_t, ::common::Role_IssuePermissions> * m,
                          int32_t key, ::common::Role_IssuePermissions value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_Role_IssuePermissions_get(const google::protobuf::Map<int32_t, ::common::Role_IssuePermissions>* m,
                       int32_t key, const ::common::Role_IssuePermissions** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::Role_IssuePermissions* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_Role_IssuePermissions_remove(google::protobuf::Map<int32_t, ::common::Role_IssuePermissions> * m,
                          int32_t key, ::common::Role_IssuePermissions * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_common_Role_IssuePermissions_iter(
      const google::protobuf::Map<int32_t, ::common::Role_IssuePermissions>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_common_Role_IssuePermissions_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::common::Role_IssuePermissions** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::common::Role_IssuePermissions>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::common::Role_IssuePermissions>* __rust_proto_thunk__Map_u32_common_Role_IssuePermissions_new() {
    return new google::protobuf::Map<uint32_t, ::common::Role_IssuePermissions>();
  }
  void __rust_proto_thunk__Map_u32_common_Role_IssuePermissions_free(const google::protobuf::Map<uint32_t, ::common::Role_IssuePermissions>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_common_Role_IssuePermissions_clear(google::protobuf::Map<uint32_t, ::common::Role_IssuePermissions> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_common_Role_IssuePermissions_size(const google::protobuf::Map<uint32_t, ::common::Role_IssuePermissions>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_common_Role_IssuePermissions_insert(google::protobuf::Map<uint32_t, ::common::Role_IssuePermissions> * m,
                          uint32_t key, ::common::Role_IssuePermissions value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_Role_IssuePermissions_get(const google::protobuf::Map<uint32_t, ::common::Role_IssuePermissions>* m,
                       uint32_t key, const ::common::Role_IssuePermissions** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::Role_IssuePermissions* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_Role_IssuePermissions_remove(google::protobuf::Map<uint32_t, ::common::Role_IssuePermissions> * m,
                          uint32_t key, ::common::Role_IssuePermissions * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_common_Role_IssuePermissions_iter(
      const google::protobuf::Map<uint32_t, ::common::Role_IssuePermissions>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_common_Role_IssuePermissions_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::common::Role_IssuePermissions** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::common::Role_IssuePermissions>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::common::Role_IssuePermissions>* __rust_proto_thunk__Map_i64_common_Role_IssuePermissions_new() {
    return new google::protobuf::Map<int64_t, ::common::Role_IssuePermissions>();
  }
  void __rust_proto_thunk__Map_i64_common_Role_IssuePermissions_free(const google::protobuf::Map<int64_t, ::common::Role_IssuePermissions>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_common_Role_IssuePermissions_clear(google::protobuf::Map<int64_t, ::common::Role_IssuePermissions> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_common_Role_IssuePermissions_size(const google::protobuf::Map<int64_t, ::common::Role_IssuePermissions>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_common_Role_IssuePermissions_insert(google::protobuf::Map<int64_t, ::common::Role_IssuePermissions> * m,
                          int64_t key, ::common::Role_IssuePermissions value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_Role_IssuePermissions_get(const google::protobuf::Map<int64_t, ::common::Role_IssuePermissions>* m,
                       int64_t key, const ::common::Role_IssuePermissions** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::Role_IssuePermissions* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_Role_IssuePermissions_remove(google::protobuf::Map<int64_t, ::common::Role_IssuePermissions> * m,
                          int64_t key, ::common::Role_IssuePermissions * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_common_Role_IssuePermissions_iter(
      const google::protobuf::Map<int64_t, ::common::Role_IssuePermissions>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_common_Role_IssuePermissions_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::common::Role_IssuePermissions** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::common::Role_IssuePermissions>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::common::Role_IssuePermissions>* __rust_proto_thunk__Map_u64_common_Role_IssuePermissions_new() {
    return new google::protobuf::Map<uint64_t, ::common::Role_IssuePermissions>();
  }
  void __rust_proto_thunk__Map_u64_common_Role_IssuePermissions_free(const google::protobuf::Map<uint64_t, ::common::Role_IssuePermissions>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_common_Role_IssuePermissions_clear(google::protobuf::Map<uint64_t, ::common::Role_IssuePermissions> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_common_Role_IssuePermissions_size(const google::protobuf::Map<uint64_t, ::common::Role_IssuePermissions>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_common_Role_IssuePermissions_insert(google::protobuf::Map<uint64_t, ::common::Role_IssuePermissions> * m,
                          uint64_t key, ::common::Role_IssuePermissions value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_Role_IssuePermissions_get(const google::protobuf::Map<uint64_t, ::common::Role_IssuePermissions>* m,
                       uint64_t key, const ::common::Role_IssuePermissions** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::Role_IssuePermissions* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_Role_IssuePermissions_remove(google::protobuf::Map<uint64_t, ::common::Role_IssuePermissions> * m,
                          uint64_t key, ::common::Role_IssuePermissions * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_common_Role_IssuePermissions_iter(
      const google::protobuf::Map<uint64_t, ::common::Role_IssuePermissions>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_common_Role_IssuePermissions_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::common::Role_IssuePermissions** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::common::Role_IssuePermissions>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::common::Role_IssuePermissions>* __rust_proto_thunk__Map_bool_common_Role_IssuePermissions_new() {
    return new google::protobuf::Map<bool, ::common::Role_IssuePermissions>();
  }
  void __rust_proto_thunk__Map_bool_common_Role_IssuePermissions_free(const google::protobuf::Map<bool, ::common::Role_IssuePermissions>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_common_Role_IssuePermissions_clear(google::protobuf::Map<bool, ::common::Role_IssuePermissions> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_common_Role_IssuePermissions_size(const google::protobuf::Map<bool, ::common::Role_IssuePermissions>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_common_Role_IssuePermissions_insert(google::protobuf::Map<bool, ::common::Role_IssuePermissions> * m,
                          bool key, ::common::Role_IssuePermissions value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_Role_IssuePermissions_get(const google::protobuf::Map<bool, ::common::Role_IssuePermissions>* m,
                       bool key, const ::common::Role_IssuePermissions** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::Role_IssuePermissions* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_Role_IssuePermissions_remove(google::protobuf::Map<bool, ::common::Role_IssuePermissions> * m,
                          bool key, ::common::Role_IssuePermissions * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_common_Role_IssuePermissions_iter(
      const google::protobuf::Map<bool, ::common::Role_IssuePermissions>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_common_Role_IssuePermissions_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::common::Role_IssuePermissions** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::common::Role_IssuePermissions>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::common::Role_IssuePermissions>* __rust_proto_thunk__Map_ProtoStr_common_Role_IssuePermissions_new() {
    return new google::protobuf::Map<std::string, ::common::Role_IssuePermissions>();
  }
  void __rust_proto_thunk__Map_ProtoStr_common_Role_IssuePermissions_free(const google::protobuf::Map<std::string, ::common::Role_IssuePermissions>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_common_Role_IssuePermissions_clear(google::protobuf::Map<std::string, ::common::Role_IssuePermissions> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_common_Role_IssuePermissions_size(const google::protobuf::Map<std::string, ::common::Role_IssuePermissions>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_Role_IssuePermissions_insert(google::protobuf::Map<std::string, ::common::Role_IssuePermissions> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::Role_IssuePermissions value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_Role_IssuePermissions_get(const google::protobuf::Map<std::string, ::common::Role_IssuePermissions>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::common::Role_IssuePermissions** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::common::Role_IssuePermissions* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_Role_IssuePermissions_remove(google::protobuf::Map<std::string, ::common::Role_IssuePermissions> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::Role_IssuePermissions * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_common_Role_IssuePermissions_iter(
      const google::protobuf::Map<std::string, ::common::Role_IssuePermissions>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_common_Role_IssuePermissions_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::common::Role_IssuePermissions** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::common::Role_IssuePermissions>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int32_t, ::common::Role>* __rust_proto_thunk__Map_i32_common_Role_new() {
    return new google::protobuf::Map<int32_t, ::common::Role>();
  }
  void __rust_proto_thunk__Map_i32_common_Role_free(const google::protobuf::Map<int32_t, ::common::Role>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_common_Role_clear(google::protobuf::Map<int32_t, ::common::Role> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_common_Role_size(const google::protobuf::Map<int32_t, ::common::Role>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_common_Role_insert(google::protobuf::Map<int32_t, ::common::Role> * m,
                          int32_t key, ::common::Role value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_Role_get(const google::protobuf::Map<int32_t, ::common::Role>* m,
                       int32_t key, const ::common::Role** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::Role* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_Role_remove(google::protobuf::Map<int32_t, ::common::Role> * m,
                          int32_t key, ::common::Role * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_common_Role_iter(
      const google::protobuf::Map<int32_t, ::common::Role>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_common_Role_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::common::Role** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::common::Role>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::common::Role>* __rust_proto_thunk__Map_u32_common_Role_new() {
    return new google::protobuf::Map<uint32_t, ::common::Role>();
  }
  void __rust_proto_thunk__Map_u32_common_Role_free(const google::protobuf::Map<uint32_t, ::common::Role>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_common_Role_clear(google::protobuf::Map<uint32_t, ::common::Role> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_common_Role_size(const google::protobuf::Map<uint32_t, ::common::Role>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_common_Role_insert(google::protobuf::Map<uint32_t, ::common::Role> * m,
                          uint32_t key, ::common::Role value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_Role_get(const google::protobuf::Map<uint32_t, ::common::Role>* m,
                       uint32_t key, const ::common::Role** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::Role* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_Role_remove(google::protobuf::Map<uint32_t, ::common::Role> * m,
                          uint32_t key, ::common::Role * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_common_Role_iter(
      const google::protobuf::Map<uint32_t, ::common::Role>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_common_Role_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::common::Role** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::common::Role>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::common::Role>* __rust_proto_thunk__Map_i64_common_Role_new() {
    return new google::protobuf::Map<int64_t, ::common::Role>();
  }
  void __rust_proto_thunk__Map_i64_common_Role_free(const google::protobuf::Map<int64_t, ::common::Role>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_common_Role_clear(google::protobuf::Map<int64_t, ::common::Role> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_common_Role_size(const google::protobuf::Map<int64_t, ::common::Role>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_common_Role_insert(google::protobuf::Map<int64_t, ::common::Role> * m,
                          int64_t key, ::common::Role value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_Role_get(const google::protobuf::Map<int64_t, ::common::Role>* m,
                       int64_t key, const ::common::Role** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::Role* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_Role_remove(google::protobuf::Map<int64_t, ::common::Role> * m,
                          int64_t key, ::common::Role * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_common_Role_iter(
      const google::protobuf::Map<int64_t, ::common::Role>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_common_Role_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::common::Role** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::common::Role>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::common::Role>* __rust_proto_thunk__Map_u64_common_Role_new() {
    return new google::protobuf::Map<uint64_t, ::common::Role>();
  }
  void __rust_proto_thunk__Map_u64_common_Role_free(const google::protobuf::Map<uint64_t, ::common::Role>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_common_Role_clear(google::protobuf::Map<uint64_t, ::common::Role> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_common_Role_size(const google::protobuf::Map<uint64_t, ::common::Role>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_common_Role_insert(google::protobuf::Map<uint64_t, ::common::Role> * m,
                          uint64_t key, ::common::Role value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_Role_get(const google::protobuf::Map<uint64_t, ::common::Role>* m,
                       uint64_t key, const ::common::Role** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::Role* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_Role_remove(google::protobuf::Map<uint64_t, ::common::Role> * m,
                          uint64_t key, ::common::Role * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_common_Role_iter(
      const google::protobuf::Map<uint64_t, ::common::Role>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_common_Role_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::common::Role** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::common::Role>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::common::Role>* __rust_proto_thunk__Map_bool_common_Role_new() {
    return new google::protobuf::Map<bool, ::common::Role>();
  }
  void __rust_proto_thunk__Map_bool_common_Role_free(const google::protobuf::Map<bool, ::common::Role>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_common_Role_clear(google::protobuf::Map<bool, ::common::Role> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_common_Role_size(const google::protobuf::Map<bool, ::common::Role>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_common_Role_insert(google::protobuf::Map<bool, ::common::Role> * m,
                          bool key, ::common::Role value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_Role_get(const google::protobuf::Map<bool, ::common::Role>* m,
                       bool key, const ::common::Role** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::Role* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_Role_remove(google::protobuf::Map<bool, ::common::Role> * m,
                          bool key, ::common::Role * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_common_Role_iter(
      const google::protobuf::Map<bool, ::common::Role>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_common_Role_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::common::Role** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::common::Role>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::common::Role>* __rust_proto_thunk__Map_ProtoStr_common_Role_new() {
    return new google::protobuf::Map<std::string, ::common::Role>();
  }
  void __rust_proto_thunk__Map_ProtoStr_common_Role_free(const google::protobuf::Map<std::string, ::common::Role>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_common_Role_clear(google::protobuf::Map<std::string, ::common::Role> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_common_Role_size(const google::protobuf::Map<std::string, ::common::Role>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_Role_insert(google::protobuf::Map<std::string, ::common::Role> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::Role value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_Role_get(const google::protobuf::Map<std::string, ::common::Role>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::common::Role** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::common::Role* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_Role_remove(google::protobuf::Map<std::string, ::common::Role> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::Role * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_common_Role_iter(
      const google::protobuf::Map<std::string, ::common::Role>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_common_Role_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::common::Role** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::common::Role>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// common.GetVerifierRequest
        // clang-format off
extern "C" {
void* __rust_proto_thunk__common_GetVerifierRequest_new() { return new ::common::GetVerifierRequest(); }
void __rust_proto_thunk__common_GetVerifierRequest_delete(void* ptr) { delete static_cast<::common::GetVerifierRequest*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__common_GetVerifierRequest_serialize(::common::GetVerifierRequest* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__common_GetVerifierRequest_parse(::common::GetVerifierRequest* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__common_GetVerifierRequest_copy_from(::common::GetVerifierRequest* dst, const ::common::GetVerifierRequest* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__common_GetVerifierRequest_repeated_len(google::protobuf::RepeatedPtrField<::common::GetVerifierRequest>* field) {
  return field->size();
}
const ::common::GetVerifierRequest& __rust_proto_thunk__common_GetVerifierRequest_repeated_get(
  google::protobuf::RepeatedPtrField<::common::GetVerifierRequest>* field,
  size_t index) {
  return field->Get(index);
}
::common::GetVerifierRequest* __rust_proto_thunk__common_GetVerifierRequest_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::common::GetVerifierRequest>* field,
  size_t index) {
  return field->Mutable(index);
}
::common::GetVerifierRequest* __rust_proto_thunk__common_GetVerifierRequest_repeated_add(google::protobuf::RepeatedPtrField<::common::GetVerifierRequest>* field) {
  return field->Add();
}
void __rust_proto_thunk__common_GetVerifierRequest_repeated_clear(google::protobuf::RepeatedPtrField<::common::GetVerifierRequest>* field) {
  field->Clear();
}
void __rust_proto_thunk__common_GetVerifierRequest_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::common::GetVerifierRequest>& dst,
  const google::protobuf::RepeatedPtrField<::common::GetVerifierRequest>& src) {
  dst = src;
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::common::GetVerifierRequest>* __rust_proto_thunk__Map_i32_common_GetVerifierRequest_new() {
    return new google::protobuf::Map<int32_t, ::common::GetVerifierRequest>();
  }
  void __rust_proto_thunk__Map_i32_common_GetVerifierRequest_free(const google::protobuf::Map<int32_t, ::common::GetVerifierRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_common_GetVerifierRequest_clear(google::protobuf::Map<int32_t, ::common::GetVerifierRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_common_GetVerifierRequest_size(const google::protobuf::Map<int32_t, ::common::GetVerifierRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_common_GetVerifierRequest_insert(google::protobuf::Map<int32_t, ::common::GetVerifierRequest> * m,
                          int32_t key, ::common::GetVerifierRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_GetVerifierRequest_get(const google::protobuf::Map<int32_t, ::common::GetVerifierRequest>* m,
                       int32_t key, const ::common::GetVerifierRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::GetVerifierRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_GetVerifierRequest_remove(google::protobuf::Map<int32_t, ::common::GetVerifierRequest> * m,
                          int32_t key, ::common::GetVerifierRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_common_GetVerifierRequest_iter(
      const google::protobuf::Map<int32_t, ::common::GetVerifierRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_common_GetVerifierRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::common::GetVerifierRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::common::GetVerifierRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::common::GetVerifierRequest>* __rust_proto_thunk__Map_u32_common_GetVerifierRequest_new() {
    return new google::protobuf::Map<uint32_t, ::common::GetVerifierRequest>();
  }
  void __rust_proto_thunk__Map_u32_common_GetVerifierRequest_free(const google::protobuf::Map<uint32_t, ::common::GetVerifierRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_common_GetVerifierRequest_clear(google::protobuf::Map<uint32_t, ::common::GetVerifierRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_common_GetVerifierRequest_size(const google::protobuf::Map<uint32_t, ::common::GetVerifierRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_common_GetVerifierRequest_insert(google::protobuf::Map<uint32_t, ::common::GetVerifierRequest> * m,
                          uint32_t key, ::common::GetVerifierRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_GetVerifierRequest_get(const google::protobuf::Map<uint32_t, ::common::GetVerifierRequest>* m,
                       uint32_t key, const ::common::GetVerifierRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::GetVerifierRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_GetVerifierRequest_remove(google::protobuf::Map<uint32_t, ::common::GetVerifierRequest> * m,
                          uint32_t key, ::common::GetVerifierRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_common_GetVerifierRequest_iter(
      const google::protobuf::Map<uint32_t, ::common::GetVerifierRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_common_GetVerifierRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::common::GetVerifierRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::common::GetVerifierRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::common::GetVerifierRequest>* __rust_proto_thunk__Map_i64_common_GetVerifierRequest_new() {
    return new google::protobuf::Map<int64_t, ::common::GetVerifierRequest>();
  }
  void __rust_proto_thunk__Map_i64_common_GetVerifierRequest_free(const google::protobuf::Map<int64_t, ::common::GetVerifierRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_common_GetVerifierRequest_clear(google::protobuf::Map<int64_t, ::common::GetVerifierRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_common_GetVerifierRequest_size(const google::protobuf::Map<int64_t, ::common::GetVerifierRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_common_GetVerifierRequest_insert(google::protobuf::Map<int64_t, ::common::GetVerifierRequest> * m,
                          int64_t key, ::common::GetVerifierRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_GetVerifierRequest_get(const google::protobuf::Map<int64_t, ::common::GetVerifierRequest>* m,
                       int64_t key, const ::common::GetVerifierRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::GetVerifierRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_GetVerifierRequest_remove(google::protobuf::Map<int64_t, ::common::GetVerifierRequest> * m,
                          int64_t key, ::common::GetVerifierRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_common_GetVerifierRequest_iter(
      const google::protobuf::Map<int64_t, ::common::GetVerifierRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_common_GetVerifierRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::common::GetVerifierRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::common::GetVerifierRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::common::GetVerifierRequest>* __rust_proto_thunk__Map_u64_common_GetVerifierRequest_new() {
    return new google::protobuf::Map<uint64_t, ::common::GetVerifierRequest>();
  }
  void __rust_proto_thunk__Map_u64_common_GetVerifierRequest_free(const google::protobuf::Map<uint64_t, ::common::GetVerifierRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_common_GetVerifierRequest_clear(google::protobuf::Map<uint64_t, ::common::GetVerifierRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_common_GetVerifierRequest_size(const google::protobuf::Map<uint64_t, ::common::GetVerifierRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_common_GetVerifierRequest_insert(google::protobuf::Map<uint64_t, ::common::GetVerifierRequest> * m,
                          uint64_t key, ::common::GetVerifierRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_GetVerifierRequest_get(const google::protobuf::Map<uint64_t, ::common::GetVerifierRequest>* m,
                       uint64_t key, const ::common::GetVerifierRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::GetVerifierRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_GetVerifierRequest_remove(google::protobuf::Map<uint64_t, ::common::GetVerifierRequest> * m,
                          uint64_t key, ::common::GetVerifierRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_common_GetVerifierRequest_iter(
      const google::protobuf::Map<uint64_t, ::common::GetVerifierRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_common_GetVerifierRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::common::GetVerifierRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::common::GetVerifierRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::common::GetVerifierRequest>* __rust_proto_thunk__Map_bool_common_GetVerifierRequest_new() {
    return new google::protobuf::Map<bool, ::common::GetVerifierRequest>();
  }
  void __rust_proto_thunk__Map_bool_common_GetVerifierRequest_free(const google::protobuf::Map<bool, ::common::GetVerifierRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_common_GetVerifierRequest_clear(google::protobuf::Map<bool, ::common::GetVerifierRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_common_GetVerifierRequest_size(const google::protobuf::Map<bool, ::common::GetVerifierRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_common_GetVerifierRequest_insert(google::protobuf::Map<bool, ::common::GetVerifierRequest> * m,
                          bool key, ::common::GetVerifierRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_GetVerifierRequest_get(const google::protobuf::Map<bool, ::common::GetVerifierRequest>* m,
                       bool key, const ::common::GetVerifierRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::GetVerifierRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_GetVerifierRequest_remove(google::protobuf::Map<bool, ::common::GetVerifierRequest> * m,
                          bool key, ::common::GetVerifierRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_common_GetVerifierRequest_iter(
      const google::protobuf::Map<bool, ::common::GetVerifierRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_common_GetVerifierRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::common::GetVerifierRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::common::GetVerifierRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::common::GetVerifierRequest>* __rust_proto_thunk__Map_ProtoStr_common_GetVerifierRequest_new() {
    return new google::protobuf::Map<std::string, ::common::GetVerifierRequest>();
  }
  void __rust_proto_thunk__Map_ProtoStr_common_GetVerifierRequest_free(const google::protobuf::Map<std::string, ::common::GetVerifierRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_common_GetVerifierRequest_clear(google::protobuf::Map<std::string, ::common::GetVerifierRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_common_GetVerifierRequest_size(const google::protobuf::Map<std::string, ::common::GetVerifierRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_GetVerifierRequest_insert(google::protobuf::Map<std::string, ::common::GetVerifierRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::GetVerifierRequest value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_GetVerifierRequest_get(const google::protobuf::Map<std::string, ::common::GetVerifierRequest>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::common::GetVerifierRequest** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::common::GetVerifierRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_GetVerifierRequest_remove(google::protobuf::Map<std::string, ::common::GetVerifierRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::GetVerifierRequest * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_common_GetVerifierRequest_iter(
      const google::protobuf::Map<std::string, ::common::GetVerifierRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_common_GetVerifierRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::common::GetVerifierRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::common::GetVerifierRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// common.GetVerifierResponse
        // clang-format off
extern "C" {
void* __rust_proto_thunk__common_GetVerifierResponse_new() { return new ::common::GetVerifierResponse(); }
void __rust_proto_thunk__common_GetVerifierResponse_delete(void* ptr) { delete static_cast<::common::GetVerifierResponse*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__common_GetVerifierResponse_serialize(::common::GetVerifierResponse* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__common_GetVerifierResponse_parse(::common::GetVerifierResponse* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__common_GetVerifierResponse_copy_from(::common::GetVerifierResponse* dst, const ::common::GetVerifierResponse* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__common_GetVerifierResponse_repeated_len(google::protobuf::RepeatedPtrField<::common::GetVerifierResponse>* field) {
  return field->size();
}
const ::common::GetVerifierResponse& __rust_proto_thunk__common_GetVerifierResponse_repeated_get(
  google::protobuf::RepeatedPtrField<::common::GetVerifierResponse>* field,
  size_t index) {
  return field->Get(index);
}
::common::GetVerifierResponse* __rust_proto_thunk__common_GetVerifierResponse_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::common::GetVerifierResponse>* field,
  size_t index) {
  return field->Mutable(index);
}
::common::GetVerifierResponse* __rust_proto_thunk__common_GetVerifierResponse_repeated_add(google::protobuf::RepeatedPtrField<::common::GetVerifierResponse>* field) {
  return field->Add();
}
void __rust_proto_thunk__common_GetVerifierResponse_repeated_clear(google::protobuf::RepeatedPtrField<::common::GetVerifierResponse>* field) {
  field->Clear();
}
void __rust_proto_thunk__common_GetVerifierResponse_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::common::GetVerifierResponse>& dst,
  const google::protobuf::RepeatedPtrField<::common::GetVerifierResponse>& src) {
  dst = src;
}

const void* __rust_proto_thunk__common_GetVerifierResponse_get_bytes(::common::GetVerifierResponse* msg) {
  return static_cast<const void*>(&msg->bytes());
}
void* __rust_proto_thunk__common_GetVerifierResponse_get_mut_bytes(::common::GetVerifierResponse* msg) {
  return static_cast<void*>(msg->mutable_bytes());
}
void __rust_proto_thunk__common_GetVerifierResponse_clear_bytes(::common::GetVerifierResponse* msg) { msg->clear_bytes(); }
bool __rust_proto_thunk__common_GetVerifierResponse_has_bytes(::common::GetVerifierResponse* msg) { return msg->has_bytes(); }

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__common_GetVerifierResponse_get_multibase(::common::GetVerifierResponse* msg) {
  absl::string_view val = msg->multibase();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__common_GetVerifierResponse_set_multibase(::common::GetVerifierResponse* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_multibase(absl::string_view(s.ptr, s.len));
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::common::GetVerifierResponse>* __rust_proto_thunk__Map_i32_common_GetVerifierResponse_new() {
    return new google::protobuf::Map<int32_t, ::common::GetVerifierResponse>();
  }
  void __rust_proto_thunk__Map_i32_common_GetVerifierResponse_free(const google::protobuf::Map<int32_t, ::common::GetVerifierResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_common_GetVerifierResponse_clear(google::protobuf::Map<int32_t, ::common::GetVerifierResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_common_GetVerifierResponse_size(const google::protobuf::Map<int32_t, ::common::GetVerifierResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_common_GetVerifierResponse_insert(google::protobuf::Map<int32_t, ::common::GetVerifierResponse> * m,
                          int32_t key, ::common::GetVerifierResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_GetVerifierResponse_get(const google::protobuf::Map<int32_t, ::common::GetVerifierResponse>* m,
                       int32_t key, const ::common::GetVerifierResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::GetVerifierResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_GetVerifierResponse_remove(google::protobuf::Map<int32_t, ::common::GetVerifierResponse> * m,
                          int32_t key, ::common::GetVerifierResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_common_GetVerifierResponse_iter(
      const google::protobuf::Map<int32_t, ::common::GetVerifierResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_common_GetVerifierResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::common::GetVerifierResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::common::GetVerifierResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::common::GetVerifierResponse>* __rust_proto_thunk__Map_u32_common_GetVerifierResponse_new() {
    return new google::protobuf::Map<uint32_t, ::common::GetVerifierResponse>();
  }
  void __rust_proto_thunk__Map_u32_common_GetVerifierResponse_free(const google::protobuf::Map<uint32_t, ::common::GetVerifierResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_common_GetVerifierResponse_clear(google::protobuf::Map<uint32_t, ::common::GetVerifierResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_common_GetVerifierResponse_size(const google::protobuf::Map<uint32_t, ::common::GetVerifierResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_common_GetVerifierResponse_insert(google::protobuf::Map<uint32_t, ::common::GetVerifierResponse> * m,
                          uint32_t key, ::common::GetVerifierResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_GetVerifierResponse_get(const google::protobuf::Map<uint32_t, ::common::GetVerifierResponse>* m,
                       uint32_t key, const ::common::GetVerifierResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::GetVerifierResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_GetVerifierResponse_remove(google::protobuf::Map<uint32_t, ::common::GetVerifierResponse> * m,
                          uint32_t key, ::common::GetVerifierResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_common_GetVerifierResponse_iter(
      const google::protobuf::Map<uint32_t, ::common::GetVerifierResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_common_GetVerifierResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::common::GetVerifierResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::common::GetVerifierResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::common::GetVerifierResponse>* __rust_proto_thunk__Map_i64_common_GetVerifierResponse_new() {
    return new google::protobuf::Map<int64_t, ::common::GetVerifierResponse>();
  }
  void __rust_proto_thunk__Map_i64_common_GetVerifierResponse_free(const google::protobuf::Map<int64_t, ::common::GetVerifierResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_common_GetVerifierResponse_clear(google::protobuf::Map<int64_t, ::common::GetVerifierResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_common_GetVerifierResponse_size(const google::protobuf::Map<int64_t, ::common::GetVerifierResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_common_GetVerifierResponse_insert(google::protobuf::Map<int64_t, ::common::GetVerifierResponse> * m,
                          int64_t key, ::common::GetVerifierResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_GetVerifierResponse_get(const google::protobuf::Map<int64_t, ::common::GetVerifierResponse>* m,
                       int64_t key, const ::common::GetVerifierResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::GetVerifierResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_GetVerifierResponse_remove(google::protobuf::Map<int64_t, ::common::GetVerifierResponse> * m,
                          int64_t key, ::common::GetVerifierResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_common_GetVerifierResponse_iter(
      const google::protobuf::Map<int64_t, ::common::GetVerifierResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_common_GetVerifierResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::common::GetVerifierResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::common::GetVerifierResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::common::GetVerifierResponse>* __rust_proto_thunk__Map_u64_common_GetVerifierResponse_new() {
    return new google::protobuf::Map<uint64_t, ::common::GetVerifierResponse>();
  }
  void __rust_proto_thunk__Map_u64_common_GetVerifierResponse_free(const google::protobuf::Map<uint64_t, ::common::GetVerifierResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_common_GetVerifierResponse_clear(google::protobuf::Map<uint64_t, ::common::GetVerifierResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_common_GetVerifierResponse_size(const google::protobuf::Map<uint64_t, ::common::GetVerifierResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_common_GetVerifierResponse_insert(google::protobuf::Map<uint64_t, ::common::GetVerifierResponse> * m,
                          uint64_t key, ::common::GetVerifierResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_GetVerifierResponse_get(const google::protobuf::Map<uint64_t, ::common::GetVerifierResponse>* m,
                       uint64_t key, const ::common::GetVerifierResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::GetVerifierResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_GetVerifierResponse_remove(google::protobuf::Map<uint64_t, ::common::GetVerifierResponse> * m,
                          uint64_t key, ::common::GetVerifierResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_common_GetVerifierResponse_iter(
      const google::protobuf::Map<uint64_t, ::common::GetVerifierResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_common_GetVerifierResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::common::GetVerifierResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::common::GetVerifierResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::common::GetVerifierResponse>* __rust_proto_thunk__Map_bool_common_GetVerifierResponse_new() {
    return new google::protobuf::Map<bool, ::common::GetVerifierResponse>();
  }
  void __rust_proto_thunk__Map_bool_common_GetVerifierResponse_free(const google::protobuf::Map<bool, ::common::GetVerifierResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_common_GetVerifierResponse_clear(google::protobuf::Map<bool, ::common::GetVerifierResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_common_GetVerifierResponse_size(const google::protobuf::Map<bool, ::common::GetVerifierResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_common_GetVerifierResponse_insert(google::protobuf::Map<bool, ::common::GetVerifierResponse> * m,
                          bool key, ::common::GetVerifierResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_GetVerifierResponse_get(const google::protobuf::Map<bool, ::common::GetVerifierResponse>* m,
                       bool key, const ::common::GetVerifierResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::GetVerifierResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_GetVerifierResponse_remove(google::protobuf::Map<bool, ::common::GetVerifierResponse> * m,
                          bool key, ::common::GetVerifierResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_common_GetVerifierResponse_iter(
      const google::protobuf::Map<bool, ::common::GetVerifierResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_common_GetVerifierResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::common::GetVerifierResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::common::GetVerifierResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::common::GetVerifierResponse>* __rust_proto_thunk__Map_ProtoStr_common_GetVerifierResponse_new() {
    return new google::protobuf::Map<std::string, ::common::GetVerifierResponse>();
  }
  void __rust_proto_thunk__Map_ProtoStr_common_GetVerifierResponse_free(const google::protobuf::Map<std::string, ::common::GetVerifierResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_common_GetVerifierResponse_clear(google::protobuf::Map<std::string, ::common::GetVerifierResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_common_GetVerifierResponse_size(const google::protobuf::Map<std::string, ::common::GetVerifierResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_GetVerifierResponse_insert(google::protobuf::Map<std::string, ::common::GetVerifierResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::GetVerifierResponse value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_GetVerifierResponse_get(const google::protobuf::Map<std::string, ::common::GetVerifierResponse>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::common::GetVerifierResponse** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::common::GetVerifierResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_GetVerifierResponse_remove(google::protobuf::Map<std::string, ::common::GetVerifierResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::GetVerifierResponse * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_common_GetVerifierResponse_iter(
      const google::protobuf::Map<std::string, ::common::GetVerifierResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_common_GetVerifierResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::common::GetVerifierResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::common::GetVerifierResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// common.GetServiceTypeRequest
        // clang-format off
extern "C" {
void* __rust_proto_thunk__common_GetServiceTypeRequest_new() { return new ::common::GetServiceTypeRequest(); }
void __rust_proto_thunk__common_GetServiceTypeRequest_delete(void* ptr) { delete static_cast<::common::GetServiceTypeRequest*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__common_GetServiceTypeRequest_serialize(::common::GetServiceTypeRequest* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__common_GetServiceTypeRequest_parse(::common::GetServiceTypeRequest* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__common_GetServiceTypeRequest_copy_from(::common::GetServiceTypeRequest* dst, const ::common::GetServiceTypeRequest* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__common_GetServiceTypeRequest_repeated_len(google::protobuf::RepeatedPtrField<::common::GetServiceTypeRequest>* field) {
  return field->size();
}
const ::common::GetServiceTypeRequest& __rust_proto_thunk__common_GetServiceTypeRequest_repeated_get(
  google::protobuf::RepeatedPtrField<::common::GetServiceTypeRequest>* field,
  size_t index) {
  return field->Get(index);
}
::common::GetServiceTypeRequest* __rust_proto_thunk__common_GetServiceTypeRequest_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::common::GetServiceTypeRequest>* field,
  size_t index) {
  return field->Mutable(index);
}
::common::GetServiceTypeRequest* __rust_proto_thunk__common_GetServiceTypeRequest_repeated_add(google::protobuf::RepeatedPtrField<::common::GetServiceTypeRequest>* field) {
  return field->Add();
}
void __rust_proto_thunk__common_GetServiceTypeRequest_repeated_clear(google::protobuf::RepeatedPtrField<::common::GetServiceTypeRequest>* field) {
  field->Clear();
}
void __rust_proto_thunk__common_GetServiceTypeRequest_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::common::GetServiceTypeRequest>& dst,
  const google::protobuf::RepeatedPtrField<::common::GetServiceTypeRequest>& src) {
  dst = src;
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::common::GetServiceTypeRequest>* __rust_proto_thunk__Map_i32_common_GetServiceTypeRequest_new() {
    return new google::protobuf::Map<int32_t, ::common::GetServiceTypeRequest>();
  }
  void __rust_proto_thunk__Map_i32_common_GetServiceTypeRequest_free(const google::protobuf::Map<int32_t, ::common::GetServiceTypeRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_common_GetServiceTypeRequest_clear(google::protobuf::Map<int32_t, ::common::GetServiceTypeRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_common_GetServiceTypeRequest_size(const google::protobuf::Map<int32_t, ::common::GetServiceTypeRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_common_GetServiceTypeRequest_insert(google::protobuf::Map<int32_t, ::common::GetServiceTypeRequest> * m,
                          int32_t key, ::common::GetServiceTypeRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_GetServiceTypeRequest_get(const google::protobuf::Map<int32_t, ::common::GetServiceTypeRequest>* m,
                       int32_t key, const ::common::GetServiceTypeRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::GetServiceTypeRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_GetServiceTypeRequest_remove(google::protobuf::Map<int32_t, ::common::GetServiceTypeRequest> * m,
                          int32_t key, ::common::GetServiceTypeRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_common_GetServiceTypeRequest_iter(
      const google::protobuf::Map<int32_t, ::common::GetServiceTypeRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_common_GetServiceTypeRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::common::GetServiceTypeRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::common::GetServiceTypeRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::common::GetServiceTypeRequest>* __rust_proto_thunk__Map_u32_common_GetServiceTypeRequest_new() {
    return new google::protobuf::Map<uint32_t, ::common::GetServiceTypeRequest>();
  }
  void __rust_proto_thunk__Map_u32_common_GetServiceTypeRequest_free(const google::protobuf::Map<uint32_t, ::common::GetServiceTypeRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_common_GetServiceTypeRequest_clear(google::protobuf::Map<uint32_t, ::common::GetServiceTypeRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_common_GetServiceTypeRequest_size(const google::protobuf::Map<uint32_t, ::common::GetServiceTypeRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_common_GetServiceTypeRequest_insert(google::protobuf::Map<uint32_t, ::common::GetServiceTypeRequest> * m,
                          uint32_t key, ::common::GetServiceTypeRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_GetServiceTypeRequest_get(const google::protobuf::Map<uint32_t, ::common::GetServiceTypeRequest>* m,
                       uint32_t key, const ::common::GetServiceTypeRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::GetServiceTypeRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_GetServiceTypeRequest_remove(google::protobuf::Map<uint32_t, ::common::GetServiceTypeRequest> * m,
                          uint32_t key, ::common::GetServiceTypeRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_common_GetServiceTypeRequest_iter(
      const google::protobuf::Map<uint32_t, ::common::GetServiceTypeRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_common_GetServiceTypeRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::common::GetServiceTypeRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::common::GetServiceTypeRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::common::GetServiceTypeRequest>* __rust_proto_thunk__Map_i64_common_GetServiceTypeRequest_new() {
    return new google::protobuf::Map<int64_t, ::common::GetServiceTypeRequest>();
  }
  void __rust_proto_thunk__Map_i64_common_GetServiceTypeRequest_free(const google::protobuf::Map<int64_t, ::common::GetServiceTypeRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_common_GetServiceTypeRequest_clear(google::protobuf::Map<int64_t, ::common::GetServiceTypeRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_common_GetServiceTypeRequest_size(const google::protobuf::Map<int64_t, ::common::GetServiceTypeRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_common_GetServiceTypeRequest_insert(google::protobuf::Map<int64_t, ::common::GetServiceTypeRequest> * m,
                          int64_t key, ::common::GetServiceTypeRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_GetServiceTypeRequest_get(const google::protobuf::Map<int64_t, ::common::GetServiceTypeRequest>* m,
                       int64_t key, const ::common::GetServiceTypeRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::GetServiceTypeRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_GetServiceTypeRequest_remove(google::protobuf::Map<int64_t, ::common::GetServiceTypeRequest> * m,
                          int64_t key, ::common::GetServiceTypeRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_common_GetServiceTypeRequest_iter(
      const google::protobuf::Map<int64_t, ::common::GetServiceTypeRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_common_GetServiceTypeRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::common::GetServiceTypeRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::common::GetServiceTypeRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::common::GetServiceTypeRequest>* __rust_proto_thunk__Map_u64_common_GetServiceTypeRequest_new() {
    return new google::protobuf::Map<uint64_t, ::common::GetServiceTypeRequest>();
  }
  void __rust_proto_thunk__Map_u64_common_GetServiceTypeRequest_free(const google::protobuf::Map<uint64_t, ::common::GetServiceTypeRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_common_GetServiceTypeRequest_clear(google::protobuf::Map<uint64_t, ::common::GetServiceTypeRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_common_GetServiceTypeRequest_size(const google::protobuf::Map<uint64_t, ::common::GetServiceTypeRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_common_GetServiceTypeRequest_insert(google::protobuf::Map<uint64_t, ::common::GetServiceTypeRequest> * m,
                          uint64_t key, ::common::GetServiceTypeRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_GetServiceTypeRequest_get(const google::protobuf::Map<uint64_t, ::common::GetServiceTypeRequest>* m,
                       uint64_t key, const ::common::GetServiceTypeRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::GetServiceTypeRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_GetServiceTypeRequest_remove(google::protobuf::Map<uint64_t, ::common::GetServiceTypeRequest> * m,
                          uint64_t key, ::common::GetServiceTypeRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_common_GetServiceTypeRequest_iter(
      const google::protobuf::Map<uint64_t, ::common::GetServiceTypeRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_common_GetServiceTypeRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::common::GetServiceTypeRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::common::GetServiceTypeRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::common::GetServiceTypeRequest>* __rust_proto_thunk__Map_bool_common_GetServiceTypeRequest_new() {
    return new google::protobuf::Map<bool, ::common::GetServiceTypeRequest>();
  }
  void __rust_proto_thunk__Map_bool_common_GetServiceTypeRequest_free(const google::protobuf::Map<bool, ::common::GetServiceTypeRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_common_GetServiceTypeRequest_clear(google::protobuf::Map<bool, ::common::GetServiceTypeRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_common_GetServiceTypeRequest_size(const google::protobuf::Map<bool, ::common::GetServiceTypeRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_common_GetServiceTypeRequest_insert(google::protobuf::Map<bool, ::common::GetServiceTypeRequest> * m,
                          bool key, ::common::GetServiceTypeRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_GetServiceTypeRequest_get(const google::protobuf::Map<bool, ::common::GetServiceTypeRequest>* m,
                       bool key, const ::common::GetServiceTypeRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::GetServiceTypeRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_GetServiceTypeRequest_remove(google::protobuf::Map<bool, ::common::GetServiceTypeRequest> * m,
                          bool key, ::common::GetServiceTypeRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_common_GetServiceTypeRequest_iter(
      const google::protobuf::Map<bool, ::common::GetServiceTypeRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_common_GetServiceTypeRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::common::GetServiceTypeRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::common::GetServiceTypeRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::common::GetServiceTypeRequest>* __rust_proto_thunk__Map_ProtoStr_common_GetServiceTypeRequest_new() {
    return new google::protobuf::Map<std::string, ::common::GetServiceTypeRequest>();
  }
  void __rust_proto_thunk__Map_ProtoStr_common_GetServiceTypeRequest_free(const google::protobuf::Map<std::string, ::common::GetServiceTypeRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_common_GetServiceTypeRequest_clear(google::protobuf::Map<std::string, ::common::GetServiceTypeRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_common_GetServiceTypeRequest_size(const google::protobuf::Map<std::string, ::common::GetServiceTypeRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_GetServiceTypeRequest_insert(google::protobuf::Map<std::string, ::common::GetServiceTypeRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::GetServiceTypeRequest value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_GetServiceTypeRequest_get(const google::protobuf::Map<std::string, ::common::GetServiceTypeRequest>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::common::GetServiceTypeRequest** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::common::GetServiceTypeRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_GetServiceTypeRequest_remove(google::protobuf::Map<std::string, ::common::GetServiceTypeRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::GetServiceTypeRequest * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_common_GetServiceTypeRequest_iter(
      const google::protobuf::Map<std::string, ::common::GetServiceTypeRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_common_GetServiceTypeRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::common::GetServiceTypeRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::common::GetServiceTypeRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// common.GetServiceTypeResponse
        // clang-format off
extern "C" {
void* __rust_proto_thunk__common_GetServiceTypeResponse_new() { return new ::common::GetServiceTypeResponse(); }
void __rust_proto_thunk__common_GetServiceTypeResponse_delete(void* ptr) { delete static_cast<::common::GetServiceTypeResponse*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__common_GetServiceTypeResponse_serialize(::common::GetServiceTypeResponse* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__common_GetServiceTypeResponse_parse(::common::GetServiceTypeResponse* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__common_GetServiceTypeResponse_copy_from(::common::GetServiceTypeResponse* dst, const ::common::GetServiceTypeResponse* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__common_GetServiceTypeResponse_repeated_len(google::protobuf::RepeatedPtrField<::common::GetServiceTypeResponse>* field) {
  return field->size();
}
const ::common::GetServiceTypeResponse& __rust_proto_thunk__common_GetServiceTypeResponse_repeated_get(
  google::protobuf::RepeatedPtrField<::common::GetServiceTypeResponse>* field,
  size_t index) {
  return field->Get(index);
}
::common::GetServiceTypeResponse* __rust_proto_thunk__common_GetServiceTypeResponse_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::common::GetServiceTypeResponse>* field,
  size_t index) {
  return field->Mutable(index);
}
::common::GetServiceTypeResponse* __rust_proto_thunk__common_GetServiceTypeResponse_repeated_add(google::protobuf::RepeatedPtrField<::common::GetServiceTypeResponse>* field) {
  return field->Add();
}
void __rust_proto_thunk__common_GetServiceTypeResponse_repeated_clear(google::protobuf::RepeatedPtrField<::common::GetServiceTypeResponse>* field) {
  field->Clear();
}
void __rust_proto_thunk__common_GetServiceTypeResponse_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::common::GetServiceTypeResponse>& dst,
  const google::protobuf::RepeatedPtrField<::common::GetServiceTypeResponse>& src) {
  dst = src;
}

::common::ServiceType __rust_proto_thunk__common_GetServiceTypeResponse_get_service_type(::common::GetServiceTypeResponse* msg) { return msg->service_type(); }
void __rust_proto_thunk__common_GetServiceTypeResponse_set_service_type(::common::GetServiceTypeResponse* msg, ::common::ServiceType val) {
  msg->set_service_type(val);
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::common::GetServiceTypeResponse>* __rust_proto_thunk__Map_i32_common_GetServiceTypeResponse_new() {
    return new google::protobuf::Map<int32_t, ::common::GetServiceTypeResponse>();
  }
  void __rust_proto_thunk__Map_i32_common_GetServiceTypeResponse_free(const google::protobuf::Map<int32_t, ::common::GetServiceTypeResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_common_GetServiceTypeResponse_clear(google::protobuf::Map<int32_t, ::common::GetServiceTypeResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_common_GetServiceTypeResponse_size(const google::protobuf::Map<int32_t, ::common::GetServiceTypeResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_common_GetServiceTypeResponse_insert(google::protobuf::Map<int32_t, ::common::GetServiceTypeResponse> * m,
                          int32_t key, ::common::GetServiceTypeResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_GetServiceTypeResponse_get(const google::protobuf::Map<int32_t, ::common::GetServiceTypeResponse>* m,
                       int32_t key, const ::common::GetServiceTypeResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::GetServiceTypeResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_GetServiceTypeResponse_remove(google::protobuf::Map<int32_t, ::common::GetServiceTypeResponse> * m,
                          int32_t key, ::common::GetServiceTypeResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_common_GetServiceTypeResponse_iter(
      const google::protobuf::Map<int32_t, ::common::GetServiceTypeResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_common_GetServiceTypeResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::common::GetServiceTypeResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::common::GetServiceTypeResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::common::GetServiceTypeResponse>* __rust_proto_thunk__Map_u32_common_GetServiceTypeResponse_new() {
    return new google::protobuf::Map<uint32_t, ::common::GetServiceTypeResponse>();
  }
  void __rust_proto_thunk__Map_u32_common_GetServiceTypeResponse_free(const google::protobuf::Map<uint32_t, ::common::GetServiceTypeResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_common_GetServiceTypeResponse_clear(google::protobuf::Map<uint32_t, ::common::GetServiceTypeResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_common_GetServiceTypeResponse_size(const google::protobuf::Map<uint32_t, ::common::GetServiceTypeResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_common_GetServiceTypeResponse_insert(google::protobuf::Map<uint32_t, ::common::GetServiceTypeResponse> * m,
                          uint32_t key, ::common::GetServiceTypeResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_GetServiceTypeResponse_get(const google::protobuf::Map<uint32_t, ::common::GetServiceTypeResponse>* m,
                       uint32_t key, const ::common::GetServiceTypeResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::GetServiceTypeResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_GetServiceTypeResponse_remove(google::protobuf::Map<uint32_t, ::common::GetServiceTypeResponse> * m,
                          uint32_t key, ::common::GetServiceTypeResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_common_GetServiceTypeResponse_iter(
      const google::protobuf::Map<uint32_t, ::common::GetServiceTypeResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_common_GetServiceTypeResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::common::GetServiceTypeResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::common::GetServiceTypeResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::common::GetServiceTypeResponse>* __rust_proto_thunk__Map_i64_common_GetServiceTypeResponse_new() {
    return new google::protobuf::Map<int64_t, ::common::GetServiceTypeResponse>();
  }
  void __rust_proto_thunk__Map_i64_common_GetServiceTypeResponse_free(const google::protobuf::Map<int64_t, ::common::GetServiceTypeResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_common_GetServiceTypeResponse_clear(google::protobuf::Map<int64_t, ::common::GetServiceTypeResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_common_GetServiceTypeResponse_size(const google::protobuf::Map<int64_t, ::common::GetServiceTypeResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_common_GetServiceTypeResponse_insert(google::protobuf::Map<int64_t, ::common::GetServiceTypeResponse> * m,
                          int64_t key, ::common::GetServiceTypeResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_GetServiceTypeResponse_get(const google::protobuf::Map<int64_t, ::common::GetServiceTypeResponse>* m,
                       int64_t key, const ::common::GetServiceTypeResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::GetServiceTypeResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_GetServiceTypeResponse_remove(google::protobuf::Map<int64_t, ::common::GetServiceTypeResponse> * m,
                          int64_t key, ::common::GetServiceTypeResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_common_GetServiceTypeResponse_iter(
      const google::protobuf::Map<int64_t, ::common::GetServiceTypeResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_common_GetServiceTypeResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::common::GetServiceTypeResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::common::GetServiceTypeResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::common::GetServiceTypeResponse>* __rust_proto_thunk__Map_u64_common_GetServiceTypeResponse_new() {
    return new google::protobuf::Map<uint64_t, ::common::GetServiceTypeResponse>();
  }
  void __rust_proto_thunk__Map_u64_common_GetServiceTypeResponse_free(const google::protobuf::Map<uint64_t, ::common::GetServiceTypeResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_common_GetServiceTypeResponse_clear(google::protobuf::Map<uint64_t, ::common::GetServiceTypeResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_common_GetServiceTypeResponse_size(const google::protobuf::Map<uint64_t, ::common::GetServiceTypeResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_common_GetServiceTypeResponse_insert(google::protobuf::Map<uint64_t, ::common::GetServiceTypeResponse> * m,
                          uint64_t key, ::common::GetServiceTypeResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_GetServiceTypeResponse_get(const google::protobuf::Map<uint64_t, ::common::GetServiceTypeResponse>* m,
                       uint64_t key, const ::common::GetServiceTypeResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::GetServiceTypeResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_GetServiceTypeResponse_remove(google::protobuf::Map<uint64_t, ::common::GetServiceTypeResponse> * m,
                          uint64_t key, ::common::GetServiceTypeResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_common_GetServiceTypeResponse_iter(
      const google::protobuf::Map<uint64_t, ::common::GetServiceTypeResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_common_GetServiceTypeResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::common::GetServiceTypeResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::common::GetServiceTypeResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::common::GetServiceTypeResponse>* __rust_proto_thunk__Map_bool_common_GetServiceTypeResponse_new() {
    return new google::protobuf::Map<bool, ::common::GetServiceTypeResponse>();
  }
  void __rust_proto_thunk__Map_bool_common_GetServiceTypeResponse_free(const google::protobuf::Map<bool, ::common::GetServiceTypeResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_common_GetServiceTypeResponse_clear(google::protobuf::Map<bool, ::common::GetServiceTypeResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_common_GetServiceTypeResponse_size(const google::protobuf::Map<bool, ::common::GetServiceTypeResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_common_GetServiceTypeResponse_insert(google::protobuf::Map<bool, ::common::GetServiceTypeResponse> * m,
                          bool key, ::common::GetServiceTypeResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_GetServiceTypeResponse_get(const google::protobuf::Map<bool, ::common::GetServiceTypeResponse>* m,
                       bool key, const ::common::GetServiceTypeResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::GetServiceTypeResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_GetServiceTypeResponse_remove(google::protobuf::Map<bool, ::common::GetServiceTypeResponse> * m,
                          bool key, ::common::GetServiceTypeResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_common_GetServiceTypeResponse_iter(
      const google::protobuf::Map<bool, ::common::GetServiceTypeResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_common_GetServiceTypeResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::common::GetServiceTypeResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::common::GetServiceTypeResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::common::GetServiceTypeResponse>* __rust_proto_thunk__Map_ProtoStr_common_GetServiceTypeResponse_new() {
    return new google::protobuf::Map<std::string, ::common::GetServiceTypeResponse>();
  }
  void __rust_proto_thunk__Map_ProtoStr_common_GetServiceTypeResponse_free(const google::protobuf::Map<std::string, ::common::GetServiceTypeResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_common_GetServiceTypeResponse_clear(google::protobuf::Map<std::string, ::common::GetServiceTypeResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_common_GetServiceTypeResponse_size(const google::protobuf::Map<std::string, ::common::GetServiceTypeResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_GetServiceTypeResponse_insert(google::protobuf::Map<std::string, ::common::GetServiceTypeResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::GetServiceTypeResponse value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_GetServiceTypeResponse_get(const google::protobuf::Map<std::string, ::common::GetServiceTypeResponse>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::common::GetServiceTypeResponse** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::common::GetServiceTypeResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_GetServiceTypeResponse_remove(google::protobuf::Map<std::string, ::common::GetServiceTypeResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::GetServiceTypeResponse * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_common_GetServiceTypeResponse_iter(
      const google::protobuf::Map<std::string, ::common::GetServiceTypeResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_common_GetServiceTypeResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::common::GetServiceTypeResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::common::GetServiceTypeResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// common.GetInstanceRequest
        // clang-format off
extern "C" {
void* __rust_proto_thunk__common_GetInstanceRequest_new() { return new ::common::GetInstanceRequest(); }
void __rust_proto_thunk__common_GetInstanceRequest_delete(void* ptr) { delete static_cast<::common::GetInstanceRequest*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__common_GetInstanceRequest_serialize(::common::GetInstanceRequest* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__common_GetInstanceRequest_parse(::common::GetInstanceRequest* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__common_GetInstanceRequest_copy_from(::common::GetInstanceRequest* dst, const ::common::GetInstanceRequest* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__common_GetInstanceRequest_repeated_len(google::protobuf::RepeatedPtrField<::common::GetInstanceRequest>* field) {
  return field->size();
}
const ::common::GetInstanceRequest& __rust_proto_thunk__common_GetInstanceRequest_repeated_get(
  google::protobuf::RepeatedPtrField<::common::GetInstanceRequest>* field,
  size_t index) {
  return field->Get(index);
}
::common::GetInstanceRequest* __rust_proto_thunk__common_GetInstanceRequest_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::common::GetInstanceRequest>* field,
  size_t index) {
  return field->Mutable(index);
}
::common::GetInstanceRequest* __rust_proto_thunk__common_GetInstanceRequest_repeated_add(google::protobuf::RepeatedPtrField<::common::GetInstanceRequest>* field) {
  return field->Add();
}
void __rust_proto_thunk__common_GetInstanceRequest_repeated_clear(google::protobuf::RepeatedPtrField<::common::GetInstanceRequest>* field) {
  field->Clear();
}
void __rust_proto_thunk__common_GetInstanceRequest_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::common::GetInstanceRequest>& dst,
  const google::protobuf::RepeatedPtrField<::common::GetInstanceRequest>& src) {
  dst = src;
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::common::GetInstanceRequest>* __rust_proto_thunk__Map_i32_common_GetInstanceRequest_new() {
    return new google::protobuf::Map<int32_t, ::common::GetInstanceRequest>();
  }
  void __rust_proto_thunk__Map_i32_common_GetInstanceRequest_free(const google::protobuf::Map<int32_t, ::common::GetInstanceRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_common_GetInstanceRequest_clear(google::protobuf::Map<int32_t, ::common::GetInstanceRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_common_GetInstanceRequest_size(const google::protobuf::Map<int32_t, ::common::GetInstanceRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_common_GetInstanceRequest_insert(google::protobuf::Map<int32_t, ::common::GetInstanceRequest> * m,
                          int32_t key, ::common::GetInstanceRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_GetInstanceRequest_get(const google::protobuf::Map<int32_t, ::common::GetInstanceRequest>* m,
                       int32_t key, const ::common::GetInstanceRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::GetInstanceRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_GetInstanceRequest_remove(google::protobuf::Map<int32_t, ::common::GetInstanceRequest> * m,
                          int32_t key, ::common::GetInstanceRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_common_GetInstanceRequest_iter(
      const google::protobuf::Map<int32_t, ::common::GetInstanceRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_common_GetInstanceRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::common::GetInstanceRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::common::GetInstanceRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::common::GetInstanceRequest>* __rust_proto_thunk__Map_u32_common_GetInstanceRequest_new() {
    return new google::protobuf::Map<uint32_t, ::common::GetInstanceRequest>();
  }
  void __rust_proto_thunk__Map_u32_common_GetInstanceRequest_free(const google::protobuf::Map<uint32_t, ::common::GetInstanceRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_common_GetInstanceRequest_clear(google::protobuf::Map<uint32_t, ::common::GetInstanceRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_common_GetInstanceRequest_size(const google::protobuf::Map<uint32_t, ::common::GetInstanceRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_common_GetInstanceRequest_insert(google::protobuf::Map<uint32_t, ::common::GetInstanceRequest> * m,
                          uint32_t key, ::common::GetInstanceRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_GetInstanceRequest_get(const google::protobuf::Map<uint32_t, ::common::GetInstanceRequest>* m,
                       uint32_t key, const ::common::GetInstanceRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::GetInstanceRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_GetInstanceRequest_remove(google::protobuf::Map<uint32_t, ::common::GetInstanceRequest> * m,
                          uint32_t key, ::common::GetInstanceRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_common_GetInstanceRequest_iter(
      const google::protobuf::Map<uint32_t, ::common::GetInstanceRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_common_GetInstanceRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::common::GetInstanceRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::common::GetInstanceRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::common::GetInstanceRequest>* __rust_proto_thunk__Map_i64_common_GetInstanceRequest_new() {
    return new google::protobuf::Map<int64_t, ::common::GetInstanceRequest>();
  }
  void __rust_proto_thunk__Map_i64_common_GetInstanceRequest_free(const google::protobuf::Map<int64_t, ::common::GetInstanceRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_common_GetInstanceRequest_clear(google::protobuf::Map<int64_t, ::common::GetInstanceRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_common_GetInstanceRequest_size(const google::protobuf::Map<int64_t, ::common::GetInstanceRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_common_GetInstanceRequest_insert(google::protobuf::Map<int64_t, ::common::GetInstanceRequest> * m,
                          int64_t key, ::common::GetInstanceRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_GetInstanceRequest_get(const google::protobuf::Map<int64_t, ::common::GetInstanceRequest>* m,
                       int64_t key, const ::common::GetInstanceRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::GetInstanceRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_GetInstanceRequest_remove(google::protobuf::Map<int64_t, ::common::GetInstanceRequest> * m,
                          int64_t key, ::common::GetInstanceRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_common_GetInstanceRequest_iter(
      const google::protobuf::Map<int64_t, ::common::GetInstanceRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_common_GetInstanceRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::common::GetInstanceRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::common::GetInstanceRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::common::GetInstanceRequest>* __rust_proto_thunk__Map_u64_common_GetInstanceRequest_new() {
    return new google::protobuf::Map<uint64_t, ::common::GetInstanceRequest>();
  }
  void __rust_proto_thunk__Map_u64_common_GetInstanceRequest_free(const google::protobuf::Map<uint64_t, ::common::GetInstanceRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_common_GetInstanceRequest_clear(google::protobuf::Map<uint64_t, ::common::GetInstanceRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_common_GetInstanceRequest_size(const google::protobuf::Map<uint64_t, ::common::GetInstanceRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_common_GetInstanceRequest_insert(google::protobuf::Map<uint64_t, ::common::GetInstanceRequest> * m,
                          uint64_t key, ::common::GetInstanceRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_GetInstanceRequest_get(const google::protobuf::Map<uint64_t, ::common::GetInstanceRequest>* m,
                       uint64_t key, const ::common::GetInstanceRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::GetInstanceRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_GetInstanceRequest_remove(google::protobuf::Map<uint64_t, ::common::GetInstanceRequest> * m,
                          uint64_t key, ::common::GetInstanceRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_common_GetInstanceRequest_iter(
      const google::protobuf::Map<uint64_t, ::common::GetInstanceRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_common_GetInstanceRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::common::GetInstanceRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::common::GetInstanceRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::common::GetInstanceRequest>* __rust_proto_thunk__Map_bool_common_GetInstanceRequest_new() {
    return new google::protobuf::Map<bool, ::common::GetInstanceRequest>();
  }
  void __rust_proto_thunk__Map_bool_common_GetInstanceRequest_free(const google::protobuf::Map<bool, ::common::GetInstanceRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_common_GetInstanceRequest_clear(google::protobuf::Map<bool, ::common::GetInstanceRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_common_GetInstanceRequest_size(const google::protobuf::Map<bool, ::common::GetInstanceRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_common_GetInstanceRequest_insert(google::protobuf::Map<bool, ::common::GetInstanceRequest> * m,
                          bool key, ::common::GetInstanceRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_GetInstanceRequest_get(const google::protobuf::Map<bool, ::common::GetInstanceRequest>* m,
                       bool key, const ::common::GetInstanceRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::GetInstanceRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_GetInstanceRequest_remove(google::protobuf::Map<bool, ::common::GetInstanceRequest> * m,
                          bool key, ::common::GetInstanceRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_common_GetInstanceRequest_iter(
      const google::protobuf::Map<bool, ::common::GetInstanceRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_common_GetInstanceRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::common::GetInstanceRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::common::GetInstanceRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::common::GetInstanceRequest>* __rust_proto_thunk__Map_ProtoStr_common_GetInstanceRequest_new() {
    return new google::protobuf::Map<std::string, ::common::GetInstanceRequest>();
  }
  void __rust_proto_thunk__Map_ProtoStr_common_GetInstanceRequest_free(const google::protobuf::Map<std::string, ::common::GetInstanceRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_common_GetInstanceRequest_clear(google::protobuf::Map<std::string, ::common::GetInstanceRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_common_GetInstanceRequest_size(const google::protobuf::Map<std::string, ::common::GetInstanceRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_GetInstanceRequest_insert(google::protobuf::Map<std::string, ::common::GetInstanceRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::GetInstanceRequest value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_GetInstanceRequest_get(const google::protobuf::Map<std::string, ::common::GetInstanceRequest>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::common::GetInstanceRequest** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::common::GetInstanceRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_GetInstanceRequest_remove(google::protobuf::Map<std::string, ::common::GetInstanceRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::GetInstanceRequest * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_common_GetInstanceRequest_iter(
      const google::protobuf::Map<std::string, ::common::GetInstanceRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_common_GetInstanceRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::common::GetInstanceRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::common::GetInstanceRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// common.GetInstanceResponse
        // clang-format off
extern "C" {
void* __rust_proto_thunk__common_GetInstanceResponse_new() { return new ::common::GetInstanceResponse(); }
void __rust_proto_thunk__common_GetInstanceResponse_delete(void* ptr) { delete static_cast<::common::GetInstanceResponse*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__common_GetInstanceResponse_serialize(::common::GetInstanceResponse* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__common_GetInstanceResponse_parse(::common::GetInstanceResponse* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__common_GetInstanceResponse_copy_from(::common::GetInstanceResponse* dst, const ::common::GetInstanceResponse* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__common_GetInstanceResponse_repeated_len(google::protobuf::RepeatedPtrField<::common::GetInstanceResponse>* field) {
  return field->size();
}
const ::common::GetInstanceResponse& __rust_proto_thunk__common_GetInstanceResponse_repeated_get(
  google::protobuf::RepeatedPtrField<::common::GetInstanceResponse>* field,
  size_t index) {
  return field->Get(index);
}
::common::GetInstanceResponse* __rust_proto_thunk__common_GetInstanceResponse_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::common::GetInstanceResponse>* field,
  size_t index) {
  return field->Mutable(index);
}
::common::GetInstanceResponse* __rust_proto_thunk__common_GetInstanceResponse_repeated_add(google::protobuf::RepeatedPtrField<::common::GetInstanceResponse>* field) {
  return field->Add();
}
void __rust_proto_thunk__common_GetInstanceResponse_repeated_clear(google::protobuf::RepeatedPtrField<::common::GetInstanceResponse>* field) {
  field->Clear();
}
void __rust_proto_thunk__common_GetInstanceResponse_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::common::GetInstanceResponse>& dst,
  const google::protobuf::RepeatedPtrField<::common::GetInstanceResponse>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__common_GetInstanceResponse_get_instance_id(::common::GetInstanceResponse* msg) {
  absl::string_view val = msg->instance_id();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__common_GetInstanceResponse_set_instance_id(::common::GetInstanceResponse* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_instance_id(absl::string_view(s.ptr, s.len));
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::common::GetInstanceResponse>* __rust_proto_thunk__Map_i32_common_GetInstanceResponse_new() {
    return new google::protobuf::Map<int32_t, ::common::GetInstanceResponse>();
  }
  void __rust_proto_thunk__Map_i32_common_GetInstanceResponse_free(const google::protobuf::Map<int32_t, ::common::GetInstanceResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_common_GetInstanceResponse_clear(google::protobuf::Map<int32_t, ::common::GetInstanceResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_common_GetInstanceResponse_size(const google::protobuf::Map<int32_t, ::common::GetInstanceResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_common_GetInstanceResponse_insert(google::protobuf::Map<int32_t, ::common::GetInstanceResponse> * m,
                          int32_t key, ::common::GetInstanceResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_GetInstanceResponse_get(const google::protobuf::Map<int32_t, ::common::GetInstanceResponse>* m,
                       int32_t key, const ::common::GetInstanceResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::GetInstanceResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_GetInstanceResponse_remove(google::protobuf::Map<int32_t, ::common::GetInstanceResponse> * m,
                          int32_t key, ::common::GetInstanceResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_common_GetInstanceResponse_iter(
      const google::protobuf::Map<int32_t, ::common::GetInstanceResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_common_GetInstanceResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::common::GetInstanceResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::common::GetInstanceResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::common::GetInstanceResponse>* __rust_proto_thunk__Map_u32_common_GetInstanceResponse_new() {
    return new google::protobuf::Map<uint32_t, ::common::GetInstanceResponse>();
  }
  void __rust_proto_thunk__Map_u32_common_GetInstanceResponse_free(const google::protobuf::Map<uint32_t, ::common::GetInstanceResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_common_GetInstanceResponse_clear(google::protobuf::Map<uint32_t, ::common::GetInstanceResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_common_GetInstanceResponse_size(const google::protobuf::Map<uint32_t, ::common::GetInstanceResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_common_GetInstanceResponse_insert(google::protobuf::Map<uint32_t, ::common::GetInstanceResponse> * m,
                          uint32_t key, ::common::GetInstanceResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_GetInstanceResponse_get(const google::protobuf::Map<uint32_t, ::common::GetInstanceResponse>* m,
                       uint32_t key, const ::common::GetInstanceResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::GetInstanceResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_GetInstanceResponse_remove(google::protobuf::Map<uint32_t, ::common::GetInstanceResponse> * m,
                          uint32_t key, ::common::GetInstanceResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_common_GetInstanceResponse_iter(
      const google::protobuf::Map<uint32_t, ::common::GetInstanceResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_common_GetInstanceResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::common::GetInstanceResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::common::GetInstanceResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::common::GetInstanceResponse>* __rust_proto_thunk__Map_i64_common_GetInstanceResponse_new() {
    return new google::protobuf::Map<int64_t, ::common::GetInstanceResponse>();
  }
  void __rust_proto_thunk__Map_i64_common_GetInstanceResponse_free(const google::protobuf::Map<int64_t, ::common::GetInstanceResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_common_GetInstanceResponse_clear(google::protobuf::Map<int64_t, ::common::GetInstanceResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_common_GetInstanceResponse_size(const google::protobuf::Map<int64_t, ::common::GetInstanceResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_common_GetInstanceResponse_insert(google::protobuf::Map<int64_t, ::common::GetInstanceResponse> * m,
                          int64_t key, ::common::GetInstanceResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_GetInstanceResponse_get(const google::protobuf::Map<int64_t, ::common::GetInstanceResponse>* m,
                       int64_t key, const ::common::GetInstanceResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::GetInstanceResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_GetInstanceResponse_remove(google::protobuf::Map<int64_t, ::common::GetInstanceResponse> * m,
                          int64_t key, ::common::GetInstanceResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_common_GetInstanceResponse_iter(
      const google::protobuf::Map<int64_t, ::common::GetInstanceResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_common_GetInstanceResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::common::GetInstanceResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::common::GetInstanceResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::common::GetInstanceResponse>* __rust_proto_thunk__Map_u64_common_GetInstanceResponse_new() {
    return new google::protobuf::Map<uint64_t, ::common::GetInstanceResponse>();
  }
  void __rust_proto_thunk__Map_u64_common_GetInstanceResponse_free(const google::protobuf::Map<uint64_t, ::common::GetInstanceResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_common_GetInstanceResponse_clear(google::protobuf::Map<uint64_t, ::common::GetInstanceResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_common_GetInstanceResponse_size(const google::protobuf::Map<uint64_t, ::common::GetInstanceResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_common_GetInstanceResponse_insert(google::protobuf::Map<uint64_t, ::common::GetInstanceResponse> * m,
                          uint64_t key, ::common::GetInstanceResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_GetInstanceResponse_get(const google::protobuf::Map<uint64_t, ::common::GetInstanceResponse>* m,
                       uint64_t key, const ::common::GetInstanceResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::GetInstanceResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_GetInstanceResponse_remove(google::protobuf::Map<uint64_t, ::common::GetInstanceResponse> * m,
                          uint64_t key, ::common::GetInstanceResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_common_GetInstanceResponse_iter(
      const google::protobuf::Map<uint64_t, ::common::GetInstanceResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_common_GetInstanceResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::common::GetInstanceResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::common::GetInstanceResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::common::GetInstanceResponse>* __rust_proto_thunk__Map_bool_common_GetInstanceResponse_new() {
    return new google::protobuf::Map<bool, ::common::GetInstanceResponse>();
  }
  void __rust_proto_thunk__Map_bool_common_GetInstanceResponse_free(const google::protobuf::Map<bool, ::common::GetInstanceResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_common_GetInstanceResponse_clear(google::protobuf::Map<bool, ::common::GetInstanceResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_common_GetInstanceResponse_size(const google::protobuf::Map<bool, ::common::GetInstanceResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_common_GetInstanceResponse_insert(google::protobuf::Map<bool, ::common::GetInstanceResponse> * m,
                          bool key, ::common::GetInstanceResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_GetInstanceResponse_get(const google::protobuf::Map<bool, ::common::GetInstanceResponse>* m,
                       bool key, const ::common::GetInstanceResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::GetInstanceResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_GetInstanceResponse_remove(google::protobuf::Map<bool, ::common::GetInstanceResponse> * m,
                          bool key, ::common::GetInstanceResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_common_GetInstanceResponse_iter(
      const google::protobuf::Map<bool, ::common::GetInstanceResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_common_GetInstanceResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::common::GetInstanceResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::common::GetInstanceResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::common::GetInstanceResponse>* __rust_proto_thunk__Map_ProtoStr_common_GetInstanceResponse_new() {
    return new google::protobuf::Map<std::string, ::common::GetInstanceResponse>();
  }
  void __rust_proto_thunk__Map_ProtoStr_common_GetInstanceResponse_free(const google::protobuf::Map<std::string, ::common::GetInstanceResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_common_GetInstanceResponse_clear(google::protobuf::Map<std::string, ::common::GetInstanceResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_common_GetInstanceResponse_size(const google::protobuf::Map<std::string, ::common::GetInstanceResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_GetInstanceResponse_insert(google::protobuf::Map<std::string, ::common::GetInstanceResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::GetInstanceResponse value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_GetInstanceResponse_get(const google::protobuf::Map<std::string, ::common::GetInstanceResponse>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::common::GetInstanceResponse** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::common::GetInstanceResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_GetInstanceResponse_remove(google::protobuf::Map<std::string, ::common::GetInstanceResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::GetInstanceResponse * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_common_GetInstanceResponse_iter(
      const google::protobuf::Map<std::string, ::common::GetInstanceResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_common_GetInstanceResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::common::GetInstanceResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::common::GetInstanceResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// common.StartFundsChangeRequest
        // clang-format off
extern "C" {
void* __rust_proto_thunk__common_StartFundsChangeRequest_new() { return new ::common::StartFundsChangeRequest(); }
void __rust_proto_thunk__common_StartFundsChangeRequest_delete(void* ptr) { delete static_cast<::common::StartFundsChangeRequest*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__common_StartFundsChangeRequest_serialize(::common::StartFundsChangeRequest* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__common_StartFundsChangeRequest_parse(::common::StartFundsChangeRequest* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__common_StartFundsChangeRequest_copy_from(::common::StartFundsChangeRequest* dst, const ::common::StartFundsChangeRequest* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__common_StartFundsChangeRequest_repeated_len(google::protobuf::RepeatedPtrField<::common::StartFundsChangeRequest>* field) {
  return field->size();
}
const ::common::StartFundsChangeRequest& __rust_proto_thunk__common_StartFundsChangeRequest_repeated_get(
  google::protobuf::RepeatedPtrField<::common::StartFundsChangeRequest>* field,
  size_t index) {
  return field->Get(index);
}
::common::StartFundsChangeRequest* __rust_proto_thunk__common_StartFundsChangeRequest_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::common::StartFundsChangeRequest>* field,
  size_t index) {
  return field->Mutable(index);
}
::common::StartFundsChangeRequest* __rust_proto_thunk__common_StartFundsChangeRequest_repeated_add(google::protobuf::RepeatedPtrField<::common::StartFundsChangeRequest>* field) {
  return field->Add();
}
void __rust_proto_thunk__common_StartFundsChangeRequest_repeated_clear(google::protobuf::RepeatedPtrField<::common::StartFundsChangeRequest>* field) {
  field->Clear();
}
void __rust_proto_thunk__common_StartFundsChangeRequest_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::common::StartFundsChangeRequest>& dst,
  const google::protobuf::RepeatedPtrField<::common::StartFundsChangeRequest>& src) {
  dst = src;
}

const void* __rust_proto_thunk__common_StartFundsChangeRequest_get_amount(::common::StartFundsChangeRequest* msg) {
  return static_cast<const void*>(&msg->amount());
}
void* __rust_proto_thunk__common_StartFundsChangeRequest_get_mut_amount(::common::StartFundsChangeRequest* msg) {
  return static_cast<void*>(msg->mutable_amount());
}
void __rust_proto_thunk__common_StartFundsChangeRequest_clear_amount(::common::StartFundsChangeRequest* msg) { msg->clear_amount(); }
bool __rust_proto_thunk__common_StartFundsChangeRequest_has_amount(::common::StartFundsChangeRequest* msg) { return msg->has_amount(); }

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__common_StartFundsChangeRequest_get_promissory(::common::StartFundsChangeRequest* msg) {
  absl::string_view val = msg->promissory();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__common_StartFundsChangeRequest_set_promissory(::common::StartFundsChangeRequest* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_promissory(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__common_StartFundsChangeRequest_get_user_reference(::common::StartFundsChangeRequest* msg) {
  absl::string_view val = msg->user_reference();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__common_StartFundsChangeRequest_set_user_reference(::common::StartFundsChangeRequest* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_user_reference(absl::string_view(s.ptr, s.len));
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::common::StartFundsChangeRequest>* __rust_proto_thunk__Map_i32_common_StartFundsChangeRequest_new() {
    return new google::protobuf::Map<int32_t, ::common::StartFundsChangeRequest>();
  }
  void __rust_proto_thunk__Map_i32_common_StartFundsChangeRequest_free(const google::protobuf::Map<int32_t, ::common::StartFundsChangeRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_common_StartFundsChangeRequest_clear(google::protobuf::Map<int32_t, ::common::StartFundsChangeRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_common_StartFundsChangeRequest_size(const google::protobuf::Map<int32_t, ::common::StartFundsChangeRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_common_StartFundsChangeRequest_insert(google::protobuf::Map<int32_t, ::common::StartFundsChangeRequest> * m,
                          int32_t key, ::common::StartFundsChangeRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_StartFundsChangeRequest_get(const google::protobuf::Map<int32_t, ::common::StartFundsChangeRequest>* m,
                       int32_t key, const ::common::StartFundsChangeRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::StartFundsChangeRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_StartFundsChangeRequest_remove(google::protobuf::Map<int32_t, ::common::StartFundsChangeRequest> * m,
                          int32_t key, ::common::StartFundsChangeRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_common_StartFundsChangeRequest_iter(
      const google::protobuf::Map<int32_t, ::common::StartFundsChangeRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_common_StartFundsChangeRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::common::StartFundsChangeRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::common::StartFundsChangeRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::common::StartFundsChangeRequest>* __rust_proto_thunk__Map_u32_common_StartFundsChangeRequest_new() {
    return new google::protobuf::Map<uint32_t, ::common::StartFundsChangeRequest>();
  }
  void __rust_proto_thunk__Map_u32_common_StartFundsChangeRequest_free(const google::protobuf::Map<uint32_t, ::common::StartFundsChangeRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_common_StartFundsChangeRequest_clear(google::protobuf::Map<uint32_t, ::common::StartFundsChangeRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_common_StartFundsChangeRequest_size(const google::protobuf::Map<uint32_t, ::common::StartFundsChangeRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_common_StartFundsChangeRequest_insert(google::protobuf::Map<uint32_t, ::common::StartFundsChangeRequest> * m,
                          uint32_t key, ::common::StartFundsChangeRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_StartFundsChangeRequest_get(const google::protobuf::Map<uint32_t, ::common::StartFundsChangeRequest>* m,
                       uint32_t key, const ::common::StartFundsChangeRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::StartFundsChangeRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_StartFundsChangeRequest_remove(google::protobuf::Map<uint32_t, ::common::StartFundsChangeRequest> * m,
                          uint32_t key, ::common::StartFundsChangeRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_common_StartFundsChangeRequest_iter(
      const google::protobuf::Map<uint32_t, ::common::StartFundsChangeRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_common_StartFundsChangeRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::common::StartFundsChangeRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::common::StartFundsChangeRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::common::StartFundsChangeRequest>* __rust_proto_thunk__Map_i64_common_StartFundsChangeRequest_new() {
    return new google::protobuf::Map<int64_t, ::common::StartFundsChangeRequest>();
  }
  void __rust_proto_thunk__Map_i64_common_StartFundsChangeRequest_free(const google::protobuf::Map<int64_t, ::common::StartFundsChangeRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_common_StartFundsChangeRequest_clear(google::protobuf::Map<int64_t, ::common::StartFundsChangeRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_common_StartFundsChangeRequest_size(const google::protobuf::Map<int64_t, ::common::StartFundsChangeRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_common_StartFundsChangeRequest_insert(google::protobuf::Map<int64_t, ::common::StartFundsChangeRequest> * m,
                          int64_t key, ::common::StartFundsChangeRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_StartFundsChangeRequest_get(const google::protobuf::Map<int64_t, ::common::StartFundsChangeRequest>* m,
                       int64_t key, const ::common::StartFundsChangeRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::StartFundsChangeRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_StartFundsChangeRequest_remove(google::protobuf::Map<int64_t, ::common::StartFundsChangeRequest> * m,
                          int64_t key, ::common::StartFundsChangeRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_common_StartFundsChangeRequest_iter(
      const google::protobuf::Map<int64_t, ::common::StartFundsChangeRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_common_StartFundsChangeRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::common::StartFundsChangeRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::common::StartFundsChangeRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::common::StartFundsChangeRequest>* __rust_proto_thunk__Map_u64_common_StartFundsChangeRequest_new() {
    return new google::protobuf::Map<uint64_t, ::common::StartFundsChangeRequest>();
  }
  void __rust_proto_thunk__Map_u64_common_StartFundsChangeRequest_free(const google::protobuf::Map<uint64_t, ::common::StartFundsChangeRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_common_StartFundsChangeRequest_clear(google::protobuf::Map<uint64_t, ::common::StartFundsChangeRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_common_StartFundsChangeRequest_size(const google::protobuf::Map<uint64_t, ::common::StartFundsChangeRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_common_StartFundsChangeRequest_insert(google::protobuf::Map<uint64_t, ::common::StartFundsChangeRequest> * m,
                          uint64_t key, ::common::StartFundsChangeRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_StartFundsChangeRequest_get(const google::protobuf::Map<uint64_t, ::common::StartFundsChangeRequest>* m,
                       uint64_t key, const ::common::StartFundsChangeRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::StartFundsChangeRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_StartFundsChangeRequest_remove(google::protobuf::Map<uint64_t, ::common::StartFundsChangeRequest> * m,
                          uint64_t key, ::common::StartFundsChangeRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_common_StartFundsChangeRequest_iter(
      const google::protobuf::Map<uint64_t, ::common::StartFundsChangeRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_common_StartFundsChangeRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::common::StartFundsChangeRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::common::StartFundsChangeRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::common::StartFundsChangeRequest>* __rust_proto_thunk__Map_bool_common_StartFundsChangeRequest_new() {
    return new google::protobuf::Map<bool, ::common::StartFundsChangeRequest>();
  }
  void __rust_proto_thunk__Map_bool_common_StartFundsChangeRequest_free(const google::protobuf::Map<bool, ::common::StartFundsChangeRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_common_StartFundsChangeRequest_clear(google::protobuf::Map<bool, ::common::StartFundsChangeRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_common_StartFundsChangeRequest_size(const google::protobuf::Map<bool, ::common::StartFundsChangeRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_common_StartFundsChangeRequest_insert(google::protobuf::Map<bool, ::common::StartFundsChangeRequest> * m,
                          bool key, ::common::StartFundsChangeRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_StartFundsChangeRequest_get(const google::protobuf::Map<bool, ::common::StartFundsChangeRequest>* m,
                       bool key, const ::common::StartFundsChangeRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::StartFundsChangeRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_StartFundsChangeRequest_remove(google::protobuf::Map<bool, ::common::StartFundsChangeRequest> * m,
                          bool key, ::common::StartFundsChangeRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_common_StartFundsChangeRequest_iter(
      const google::protobuf::Map<bool, ::common::StartFundsChangeRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_common_StartFundsChangeRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::common::StartFundsChangeRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::common::StartFundsChangeRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::common::StartFundsChangeRequest>* __rust_proto_thunk__Map_ProtoStr_common_StartFundsChangeRequest_new() {
    return new google::protobuf::Map<std::string, ::common::StartFundsChangeRequest>();
  }
  void __rust_proto_thunk__Map_ProtoStr_common_StartFundsChangeRequest_free(const google::protobuf::Map<std::string, ::common::StartFundsChangeRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_common_StartFundsChangeRequest_clear(google::protobuf::Map<std::string, ::common::StartFundsChangeRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_common_StartFundsChangeRequest_size(const google::protobuf::Map<std::string, ::common::StartFundsChangeRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_StartFundsChangeRequest_insert(google::protobuf::Map<std::string, ::common::StartFundsChangeRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::StartFundsChangeRequest value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_StartFundsChangeRequest_get(const google::protobuf::Map<std::string, ::common::StartFundsChangeRequest>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::common::StartFundsChangeRequest** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::common::StartFundsChangeRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_StartFundsChangeRequest_remove(google::protobuf::Map<std::string, ::common::StartFundsChangeRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::StartFundsChangeRequest * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_common_StartFundsChangeRequest_iter(
      const google::protobuf::Map<std::string, ::common::StartFundsChangeRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_common_StartFundsChangeRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::common::StartFundsChangeRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::common::StartFundsChangeRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// common.StartFundsChangeResponse
        // clang-format off
extern "C" {
void* __rust_proto_thunk__common_StartFundsChangeResponse_new() { return new ::common::StartFundsChangeResponse(); }
void __rust_proto_thunk__common_StartFundsChangeResponse_delete(void* ptr) { delete static_cast<::common::StartFundsChangeResponse*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__common_StartFundsChangeResponse_serialize(::common::StartFundsChangeResponse* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__common_StartFundsChangeResponse_parse(::common::StartFundsChangeResponse* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__common_StartFundsChangeResponse_copy_from(::common::StartFundsChangeResponse* dst, const ::common::StartFundsChangeResponse* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__common_StartFundsChangeResponse_repeated_len(google::protobuf::RepeatedPtrField<::common::StartFundsChangeResponse>* field) {
  return field->size();
}
const ::common::StartFundsChangeResponse& __rust_proto_thunk__common_StartFundsChangeResponse_repeated_get(
  google::protobuf::RepeatedPtrField<::common::StartFundsChangeResponse>* field,
  size_t index) {
  return field->Get(index);
}
::common::StartFundsChangeResponse* __rust_proto_thunk__common_StartFundsChangeResponse_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::common::StartFundsChangeResponse>* field,
  size_t index) {
  return field->Mutable(index);
}
::common::StartFundsChangeResponse* __rust_proto_thunk__common_StartFundsChangeResponse_repeated_add(google::protobuf::RepeatedPtrField<::common::StartFundsChangeResponse>* field) {
  return field->Add();
}
void __rust_proto_thunk__common_StartFundsChangeResponse_repeated_clear(google::protobuf::RepeatedPtrField<::common::StartFundsChangeResponse>* field) {
  field->Clear();
}
void __rust_proto_thunk__common_StartFundsChangeResponse_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::common::StartFundsChangeResponse>& dst,
  const google::protobuf::RepeatedPtrField<::common::StartFundsChangeResponse>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__common_StartFundsChangeResponse_get_uetr(::common::StartFundsChangeResponse* msg) {
  absl::string_view val = msg->uetr();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__common_StartFundsChangeResponse_set_uetr(::common::StartFundsChangeResponse* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_uetr(absl::string_view(s.ptr, s.len));
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::common::StartFundsChangeResponse>* __rust_proto_thunk__Map_i32_common_StartFundsChangeResponse_new() {
    return new google::protobuf::Map<int32_t, ::common::StartFundsChangeResponse>();
  }
  void __rust_proto_thunk__Map_i32_common_StartFundsChangeResponse_free(const google::protobuf::Map<int32_t, ::common::StartFundsChangeResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_common_StartFundsChangeResponse_clear(google::protobuf::Map<int32_t, ::common::StartFundsChangeResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_common_StartFundsChangeResponse_size(const google::protobuf::Map<int32_t, ::common::StartFundsChangeResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_common_StartFundsChangeResponse_insert(google::protobuf::Map<int32_t, ::common::StartFundsChangeResponse> * m,
                          int32_t key, ::common::StartFundsChangeResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_StartFundsChangeResponse_get(const google::protobuf::Map<int32_t, ::common::StartFundsChangeResponse>* m,
                       int32_t key, const ::common::StartFundsChangeResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::StartFundsChangeResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_StartFundsChangeResponse_remove(google::protobuf::Map<int32_t, ::common::StartFundsChangeResponse> * m,
                          int32_t key, ::common::StartFundsChangeResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_common_StartFundsChangeResponse_iter(
      const google::protobuf::Map<int32_t, ::common::StartFundsChangeResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_common_StartFundsChangeResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::common::StartFundsChangeResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::common::StartFundsChangeResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::common::StartFundsChangeResponse>* __rust_proto_thunk__Map_u32_common_StartFundsChangeResponse_new() {
    return new google::protobuf::Map<uint32_t, ::common::StartFundsChangeResponse>();
  }
  void __rust_proto_thunk__Map_u32_common_StartFundsChangeResponse_free(const google::protobuf::Map<uint32_t, ::common::StartFundsChangeResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_common_StartFundsChangeResponse_clear(google::protobuf::Map<uint32_t, ::common::StartFundsChangeResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_common_StartFundsChangeResponse_size(const google::protobuf::Map<uint32_t, ::common::StartFundsChangeResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_common_StartFundsChangeResponse_insert(google::protobuf::Map<uint32_t, ::common::StartFundsChangeResponse> * m,
                          uint32_t key, ::common::StartFundsChangeResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_StartFundsChangeResponse_get(const google::protobuf::Map<uint32_t, ::common::StartFundsChangeResponse>* m,
                       uint32_t key, const ::common::StartFundsChangeResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::StartFundsChangeResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_StartFundsChangeResponse_remove(google::protobuf::Map<uint32_t, ::common::StartFundsChangeResponse> * m,
                          uint32_t key, ::common::StartFundsChangeResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_common_StartFundsChangeResponse_iter(
      const google::protobuf::Map<uint32_t, ::common::StartFundsChangeResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_common_StartFundsChangeResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::common::StartFundsChangeResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::common::StartFundsChangeResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::common::StartFundsChangeResponse>* __rust_proto_thunk__Map_i64_common_StartFundsChangeResponse_new() {
    return new google::protobuf::Map<int64_t, ::common::StartFundsChangeResponse>();
  }
  void __rust_proto_thunk__Map_i64_common_StartFundsChangeResponse_free(const google::protobuf::Map<int64_t, ::common::StartFundsChangeResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_common_StartFundsChangeResponse_clear(google::protobuf::Map<int64_t, ::common::StartFundsChangeResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_common_StartFundsChangeResponse_size(const google::protobuf::Map<int64_t, ::common::StartFundsChangeResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_common_StartFundsChangeResponse_insert(google::protobuf::Map<int64_t, ::common::StartFundsChangeResponse> * m,
                          int64_t key, ::common::StartFundsChangeResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_StartFundsChangeResponse_get(const google::protobuf::Map<int64_t, ::common::StartFundsChangeResponse>* m,
                       int64_t key, const ::common::StartFundsChangeResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::StartFundsChangeResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_StartFundsChangeResponse_remove(google::protobuf::Map<int64_t, ::common::StartFundsChangeResponse> * m,
                          int64_t key, ::common::StartFundsChangeResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_common_StartFundsChangeResponse_iter(
      const google::protobuf::Map<int64_t, ::common::StartFundsChangeResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_common_StartFundsChangeResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::common::StartFundsChangeResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::common::StartFundsChangeResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::common::StartFundsChangeResponse>* __rust_proto_thunk__Map_u64_common_StartFundsChangeResponse_new() {
    return new google::protobuf::Map<uint64_t, ::common::StartFundsChangeResponse>();
  }
  void __rust_proto_thunk__Map_u64_common_StartFundsChangeResponse_free(const google::protobuf::Map<uint64_t, ::common::StartFundsChangeResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_common_StartFundsChangeResponse_clear(google::protobuf::Map<uint64_t, ::common::StartFundsChangeResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_common_StartFundsChangeResponse_size(const google::protobuf::Map<uint64_t, ::common::StartFundsChangeResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_common_StartFundsChangeResponse_insert(google::protobuf::Map<uint64_t, ::common::StartFundsChangeResponse> * m,
                          uint64_t key, ::common::StartFundsChangeResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_StartFundsChangeResponse_get(const google::protobuf::Map<uint64_t, ::common::StartFundsChangeResponse>* m,
                       uint64_t key, const ::common::StartFundsChangeResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::StartFundsChangeResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_StartFundsChangeResponse_remove(google::protobuf::Map<uint64_t, ::common::StartFundsChangeResponse> * m,
                          uint64_t key, ::common::StartFundsChangeResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_common_StartFundsChangeResponse_iter(
      const google::protobuf::Map<uint64_t, ::common::StartFundsChangeResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_common_StartFundsChangeResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::common::StartFundsChangeResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::common::StartFundsChangeResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::common::StartFundsChangeResponse>* __rust_proto_thunk__Map_bool_common_StartFundsChangeResponse_new() {
    return new google::protobuf::Map<bool, ::common::StartFundsChangeResponse>();
  }
  void __rust_proto_thunk__Map_bool_common_StartFundsChangeResponse_free(const google::protobuf::Map<bool, ::common::StartFundsChangeResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_common_StartFundsChangeResponse_clear(google::protobuf::Map<bool, ::common::StartFundsChangeResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_common_StartFundsChangeResponse_size(const google::protobuf::Map<bool, ::common::StartFundsChangeResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_common_StartFundsChangeResponse_insert(google::protobuf::Map<bool, ::common::StartFundsChangeResponse> * m,
                          bool key, ::common::StartFundsChangeResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_StartFundsChangeResponse_get(const google::protobuf::Map<bool, ::common::StartFundsChangeResponse>* m,
                       bool key, const ::common::StartFundsChangeResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::StartFundsChangeResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_StartFundsChangeResponse_remove(google::protobuf::Map<bool, ::common::StartFundsChangeResponse> * m,
                          bool key, ::common::StartFundsChangeResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_common_StartFundsChangeResponse_iter(
      const google::protobuf::Map<bool, ::common::StartFundsChangeResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_common_StartFundsChangeResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::common::StartFundsChangeResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::common::StartFundsChangeResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::common::StartFundsChangeResponse>* __rust_proto_thunk__Map_ProtoStr_common_StartFundsChangeResponse_new() {
    return new google::protobuf::Map<std::string, ::common::StartFundsChangeResponse>();
  }
  void __rust_proto_thunk__Map_ProtoStr_common_StartFundsChangeResponse_free(const google::protobuf::Map<std::string, ::common::StartFundsChangeResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_common_StartFundsChangeResponse_clear(google::protobuf::Map<std::string, ::common::StartFundsChangeResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_common_StartFundsChangeResponse_size(const google::protobuf::Map<std::string, ::common::StartFundsChangeResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_StartFundsChangeResponse_insert(google::protobuf::Map<std::string, ::common::StartFundsChangeResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::StartFundsChangeResponse value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_StartFundsChangeResponse_get(const google::protobuf::Map<std::string, ::common::StartFundsChangeResponse>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::common::StartFundsChangeResponse** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::common::StartFundsChangeResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_StartFundsChangeResponse_remove(google::protobuf::Map<std::string, ::common::StartFundsChangeResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::StartFundsChangeResponse * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_common_StartFundsChangeResponse_iter(
      const google::protobuf::Map<std::string, ::common::StartFundsChangeResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_common_StartFundsChangeResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::common::StartFundsChangeResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::common::StartFundsChangeResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// common.FilterItem
        // clang-format off
extern "C" {
void* __rust_proto_thunk__common_FilterItem_new() { return new ::common::FilterItem(); }
void __rust_proto_thunk__common_FilterItem_delete(void* ptr) { delete static_cast<::common::FilterItem*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__common_FilterItem_serialize(::common::FilterItem* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__common_FilterItem_parse(::common::FilterItem* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__common_FilterItem_copy_from(::common::FilterItem* dst, const ::common::FilterItem* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__common_FilterItem_repeated_len(google::protobuf::RepeatedPtrField<::common::FilterItem>* field) {
  return field->size();
}
const ::common::FilterItem& __rust_proto_thunk__common_FilterItem_repeated_get(
  google::protobuf::RepeatedPtrField<::common::FilterItem>* field,
  size_t index) {
  return field->Get(index);
}
::common::FilterItem* __rust_proto_thunk__common_FilterItem_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::common::FilterItem>* field,
  size_t index) {
  return field->Mutable(index);
}
::common::FilterItem* __rust_proto_thunk__common_FilterItem_repeated_add(google::protobuf::RepeatedPtrField<::common::FilterItem>* field) {
  return field->Add();
}
void __rust_proto_thunk__common_FilterItem_repeated_clear(google::protobuf::RepeatedPtrField<::common::FilterItem>* field) {
  field->Clear();
}
void __rust_proto_thunk__common_FilterItem_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::common::FilterItem>& dst,
  const google::protobuf::RepeatedPtrField<::common::FilterItem>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__common_FilterItem_get_field(::common::FilterItem* msg) {
  absl::string_view val = msg->field();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__common_FilterItem_set_field(::common::FilterItem* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_field(absl::string_view(s.ptr, s.len));
}

::common::FilterItem_FilterOperator __rust_proto_thunk__common_FilterItem_get_operator(::common::FilterItem* msg) { return msg->operator_(); }
void __rust_proto_thunk__common_FilterItem_set_operator(::common::FilterItem* msg, ::common::FilterItem_FilterOperator val) {
  msg->set_operator_(val);
}

bool __rust_proto_thunk__common_FilterItem_has_str_value(::common::FilterItem* msg) {
  return msg->has_str_value();
}
void __rust_proto_thunk__common_FilterItem_clear_str_value(::common::FilterItem* msg) { msg->clear_str_value(); }
::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__common_FilterItem_get_str_value(::common::FilterItem* msg) {
  absl::string_view val = msg->str_value();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__common_FilterItem_set_str_value(::common::FilterItem* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_str_value(absl::string_view(s.ptr, s.len));
}

bool __rust_proto_thunk__common_FilterItem_has_int_value(::common::FilterItem* msg) {
  return msg->has_int_value();
}
void __rust_proto_thunk__common_FilterItem_clear_int_value(::common::FilterItem* msg) { msg->clear_int_value(); }
::int64_t __rust_proto_thunk__common_FilterItem_get_int_value(::common::FilterItem* msg) { return msg->int_value(); }
void __rust_proto_thunk__common_FilterItem_set_int_value(::common::FilterItem* msg, ::int64_t val) {
  msg->set_int_value(val);
}

bool __rust_proto_thunk__common_FilterItem_has_bool_value(::common::FilterItem* msg) {
  return msg->has_bool_value();
}
void __rust_proto_thunk__common_FilterItem_clear_bool_value(::common::FilterItem* msg) { msg->clear_bool_value(); }
bool __rust_proto_thunk__common_FilterItem_get_bool_value(::common::FilterItem* msg) { return msg->bool_value(); }
void __rust_proto_thunk__common_FilterItem_set_bool_value(::common::FilterItem* msg, bool val) {
  msg->set_bool_value(val);
}

bool __rust_proto_thunk__common_FilterItem_has_rfc3339_datetime_value(::common::FilterItem* msg) {
  return msg->has_rfc3339_datetime_value();
}
void __rust_proto_thunk__common_FilterItem_clear_rfc3339_datetime_value(::common::FilterItem* msg) { msg->clear_rfc3339_datetime_value(); }
::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__common_FilterItem_get_rfc3339_datetime_value(::common::FilterItem* msg) {
  absl::string_view val = msg->rfc3339_datetime_value();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__common_FilterItem_set_rfc3339_datetime_value(::common::FilterItem* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_rfc3339_datetime_value(absl::string_view(s.ptr, s.len));
}

bool __rust_proto_thunk__common_FilterItem_has_uuid_value(::common::FilterItem* msg) {
  return msg->has_uuid_value();
}
void __rust_proto_thunk__common_FilterItem_clear_uuid_value(::common::FilterItem* msg) { msg->clear_uuid_value(); }
::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__common_FilterItem_get_uuid_value(::common::FilterItem* msg) {
  absl::string_view val = msg->uuid_value();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__common_FilterItem_set_uuid_value(::common::FilterItem* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_uuid_value(absl::string_view(s.ptr, s.len));
}


::common::FilterItem::ValueCase __rust_proto_thunk__common_FilterItem_case_value(::common::FilterItem* msg) {
  return msg->value_case();
}
}  // extern "C"
// clang-format on

extern "C" {
  __PB_RUST_EXPOSE_SCALAR_MAP_METHODS_FOR_VALUE_TYPE(
      ::common::FilterItem_FilterOperator, common_FilterItem_FilterOperator, ::common::FilterItem_FilterOperator, value, cpp_value)
}
extern "C" {
  const google::protobuf::Map<int32_t, ::common::FilterItem>* __rust_proto_thunk__Map_i32_common_FilterItem_new() {
    return new google::protobuf::Map<int32_t, ::common::FilterItem>();
  }
  void __rust_proto_thunk__Map_i32_common_FilterItem_free(const google::protobuf::Map<int32_t, ::common::FilterItem>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_common_FilterItem_clear(google::protobuf::Map<int32_t, ::common::FilterItem> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_common_FilterItem_size(const google::protobuf::Map<int32_t, ::common::FilterItem>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_common_FilterItem_insert(google::protobuf::Map<int32_t, ::common::FilterItem> * m,
                          int32_t key, ::common::FilterItem value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_FilterItem_get(const google::protobuf::Map<int32_t, ::common::FilterItem>* m,
                       int32_t key, const ::common::FilterItem** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::FilterItem* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_FilterItem_remove(google::protobuf::Map<int32_t, ::common::FilterItem> * m,
                          int32_t key, ::common::FilterItem * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_common_FilterItem_iter(
      const google::protobuf::Map<int32_t, ::common::FilterItem>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_common_FilterItem_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::common::FilterItem** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::common::FilterItem>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::common::FilterItem>* __rust_proto_thunk__Map_u32_common_FilterItem_new() {
    return new google::protobuf::Map<uint32_t, ::common::FilterItem>();
  }
  void __rust_proto_thunk__Map_u32_common_FilterItem_free(const google::protobuf::Map<uint32_t, ::common::FilterItem>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_common_FilterItem_clear(google::protobuf::Map<uint32_t, ::common::FilterItem> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_common_FilterItem_size(const google::protobuf::Map<uint32_t, ::common::FilterItem>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_common_FilterItem_insert(google::protobuf::Map<uint32_t, ::common::FilterItem> * m,
                          uint32_t key, ::common::FilterItem value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_FilterItem_get(const google::protobuf::Map<uint32_t, ::common::FilterItem>* m,
                       uint32_t key, const ::common::FilterItem** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::FilterItem* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_FilterItem_remove(google::protobuf::Map<uint32_t, ::common::FilterItem> * m,
                          uint32_t key, ::common::FilterItem * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_common_FilterItem_iter(
      const google::protobuf::Map<uint32_t, ::common::FilterItem>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_common_FilterItem_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::common::FilterItem** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::common::FilterItem>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::common::FilterItem>* __rust_proto_thunk__Map_i64_common_FilterItem_new() {
    return new google::protobuf::Map<int64_t, ::common::FilterItem>();
  }
  void __rust_proto_thunk__Map_i64_common_FilterItem_free(const google::protobuf::Map<int64_t, ::common::FilterItem>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_common_FilterItem_clear(google::protobuf::Map<int64_t, ::common::FilterItem> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_common_FilterItem_size(const google::protobuf::Map<int64_t, ::common::FilterItem>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_common_FilterItem_insert(google::protobuf::Map<int64_t, ::common::FilterItem> * m,
                          int64_t key, ::common::FilterItem value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_FilterItem_get(const google::protobuf::Map<int64_t, ::common::FilterItem>* m,
                       int64_t key, const ::common::FilterItem** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::FilterItem* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_FilterItem_remove(google::protobuf::Map<int64_t, ::common::FilterItem> * m,
                          int64_t key, ::common::FilterItem * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_common_FilterItem_iter(
      const google::protobuf::Map<int64_t, ::common::FilterItem>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_common_FilterItem_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::common::FilterItem** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::common::FilterItem>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::common::FilterItem>* __rust_proto_thunk__Map_u64_common_FilterItem_new() {
    return new google::protobuf::Map<uint64_t, ::common::FilterItem>();
  }
  void __rust_proto_thunk__Map_u64_common_FilterItem_free(const google::protobuf::Map<uint64_t, ::common::FilterItem>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_common_FilterItem_clear(google::protobuf::Map<uint64_t, ::common::FilterItem> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_common_FilterItem_size(const google::protobuf::Map<uint64_t, ::common::FilterItem>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_common_FilterItem_insert(google::protobuf::Map<uint64_t, ::common::FilterItem> * m,
                          uint64_t key, ::common::FilterItem value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_FilterItem_get(const google::protobuf::Map<uint64_t, ::common::FilterItem>* m,
                       uint64_t key, const ::common::FilterItem** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::FilterItem* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_FilterItem_remove(google::protobuf::Map<uint64_t, ::common::FilterItem> * m,
                          uint64_t key, ::common::FilterItem * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_common_FilterItem_iter(
      const google::protobuf::Map<uint64_t, ::common::FilterItem>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_common_FilterItem_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::common::FilterItem** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::common::FilterItem>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::common::FilterItem>* __rust_proto_thunk__Map_bool_common_FilterItem_new() {
    return new google::protobuf::Map<bool, ::common::FilterItem>();
  }
  void __rust_proto_thunk__Map_bool_common_FilterItem_free(const google::protobuf::Map<bool, ::common::FilterItem>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_common_FilterItem_clear(google::protobuf::Map<bool, ::common::FilterItem> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_common_FilterItem_size(const google::protobuf::Map<bool, ::common::FilterItem>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_common_FilterItem_insert(google::protobuf::Map<bool, ::common::FilterItem> * m,
                          bool key, ::common::FilterItem value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_FilterItem_get(const google::protobuf::Map<bool, ::common::FilterItem>* m,
                       bool key, const ::common::FilterItem** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::FilterItem* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_FilterItem_remove(google::protobuf::Map<bool, ::common::FilterItem> * m,
                          bool key, ::common::FilterItem * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_common_FilterItem_iter(
      const google::protobuf::Map<bool, ::common::FilterItem>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_common_FilterItem_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::common::FilterItem** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::common::FilterItem>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::common::FilterItem>* __rust_proto_thunk__Map_ProtoStr_common_FilterItem_new() {
    return new google::protobuf::Map<std::string, ::common::FilterItem>();
  }
  void __rust_proto_thunk__Map_ProtoStr_common_FilterItem_free(const google::protobuf::Map<std::string, ::common::FilterItem>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_common_FilterItem_clear(google::protobuf::Map<std::string, ::common::FilterItem> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_common_FilterItem_size(const google::protobuf::Map<std::string, ::common::FilterItem>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_FilterItem_insert(google::protobuf::Map<std::string, ::common::FilterItem> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::FilterItem value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_FilterItem_get(const google::protobuf::Map<std::string, ::common::FilterItem>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::common::FilterItem** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::common::FilterItem* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_FilterItem_remove(google::protobuf::Map<std::string, ::common::FilterItem> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::FilterItem * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_common_FilterItem_iter(
      const google::protobuf::Map<std::string, ::common::FilterItem>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_common_FilterItem_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::common::FilterItem** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::common::FilterItem>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// common.Filter
        // clang-format off
extern "C" {
void* __rust_proto_thunk__common_Filter_new() { return new ::common::Filter(); }
void __rust_proto_thunk__common_Filter_delete(void* ptr) { delete static_cast<::common::Filter*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__common_Filter_serialize(::common::Filter* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__common_Filter_parse(::common::Filter* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__common_Filter_copy_from(::common::Filter* dst, const ::common::Filter* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__common_Filter_repeated_len(google::protobuf::RepeatedPtrField<::common::Filter>* field) {
  return field->size();
}
const ::common::Filter& __rust_proto_thunk__common_Filter_repeated_get(
  google::protobuf::RepeatedPtrField<::common::Filter>* field,
  size_t index) {
  return field->Get(index);
}
::common::Filter* __rust_proto_thunk__common_Filter_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::common::Filter>* field,
  size_t index) {
  return field->Mutable(index);
}
::common::Filter* __rust_proto_thunk__common_Filter_repeated_add(google::protobuf::RepeatedPtrField<::common::Filter>* field) {
  return field->Add();
}
void __rust_proto_thunk__common_Filter_repeated_clear(google::protobuf::RepeatedPtrField<::common::Filter>* field) {
  field->Clear();
}
void __rust_proto_thunk__common_Filter_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::common::Filter>& dst,
  const google::protobuf::RepeatedPtrField<::common::Filter>& src) {
  dst = src;
}

void __rust_proto_thunk__common_Filter_clear_filters(::common::Filter* msg) {
  msg->clear_filters();
}
google::protobuf::RepeatedPtrField<::common::FilterItem>* __rust_proto_thunk__common_Filter_get_mut_filters(::common::Filter* msg) {
  return msg->mutable_filters();
}
const google::protobuf::RepeatedPtrField<::common::FilterItem>* __rust_proto_thunk__common_Filter_get_filters(
    const ::common::Filter* msg) {
  return &msg->filters();
}

::common::LogicalOperator __rust_proto_thunk__common_Filter_get_operator(::common::Filter* msg) { return msg->operator_(); }
void __rust_proto_thunk__common_Filter_set_operator(::common::Filter* msg, ::common::LogicalOperator val) {
  msg->set_operator_(val);
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::common::Filter>* __rust_proto_thunk__Map_i32_common_Filter_new() {
    return new google::protobuf::Map<int32_t, ::common::Filter>();
  }
  void __rust_proto_thunk__Map_i32_common_Filter_free(const google::protobuf::Map<int32_t, ::common::Filter>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_common_Filter_clear(google::protobuf::Map<int32_t, ::common::Filter> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_common_Filter_size(const google::protobuf::Map<int32_t, ::common::Filter>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_common_Filter_insert(google::protobuf::Map<int32_t, ::common::Filter> * m,
                          int32_t key, ::common::Filter value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_Filter_get(const google::protobuf::Map<int32_t, ::common::Filter>* m,
                       int32_t key, const ::common::Filter** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::Filter* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_Filter_remove(google::protobuf::Map<int32_t, ::common::Filter> * m,
                          int32_t key, ::common::Filter * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_common_Filter_iter(
      const google::protobuf::Map<int32_t, ::common::Filter>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_common_Filter_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::common::Filter** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::common::Filter>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::common::Filter>* __rust_proto_thunk__Map_u32_common_Filter_new() {
    return new google::protobuf::Map<uint32_t, ::common::Filter>();
  }
  void __rust_proto_thunk__Map_u32_common_Filter_free(const google::protobuf::Map<uint32_t, ::common::Filter>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_common_Filter_clear(google::protobuf::Map<uint32_t, ::common::Filter> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_common_Filter_size(const google::protobuf::Map<uint32_t, ::common::Filter>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_common_Filter_insert(google::protobuf::Map<uint32_t, ::common::Filter> * m,
                          uint32_t key, ::common::Filter value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_Filter_get(const google::protobuf::Map<uint32_t, ::common::Filter>* m,
                       uint32_t key, const ::common::Filter** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::Filter* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_Filter_remove(google::protobuf::Map<uint32_t, ::common::Filter> * m,
                          uint32_t key, ::common::Filter * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_common_Filter_iter(
      const google::protobuf::Map<uint32_t, ::common::Filter>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_common_Filter_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::common::Filter** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::common::Filter>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::common::Filter>* __rust_proto_thunk__Map_i64_common_Filter_new() {
    return new google::protobuf::Map<int64_t, ::common::Filter>();
  }
  void __rust_proto_thunk__Map_i64_common_Filter_free(const google::protobuf::Map<int64_t, ::common::Filter>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_common_Filter_clear(google::protobuf::Map<int64_t, ::common::Filter> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_common_Filter_size(const google::protobuf::Map<int64_t, ::common::Filter>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_common_Filter_insert(google::protobuf::Map<int64_t, ::common::Filter> * m,
                          int64_t key, ::common::Filter value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_Filter_get(const google::protobuf::Map<int64_t, ::common::Filter>* m,
                       int64_t key, const ::common::Filter** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::Filter* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_Filter_remove(google::protobuf::Map<int64_t, ::common::Filter> * m,
                          int64_t key, ::common::Filter * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_common_Filter_iter(
      const google::protobuf::Map<int64_t, ::common::Filter>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_common_Filter_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::common::Filter** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::common::Filter>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::common::Filter>* __rust_proto_thunk__Map_u64_common_Filter_new() {
    return new google::protobuf::Map<uint64_t, ::common::Filter>();
  }
  void __rust_proto_thunk__Map_u64_common_Filter_free(const google::protobuf::Map<uint64_t, ::common::Filter>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_common_Filter_clear(google::protobuf::Map<uint64_t, ::common::Filter> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_common_Filter_size(const google::protobuf::Map<uint64_t, ::common::Filter>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_common_Filter_insert(google::protobuf::Map<uint64_t, ::common::Filter> * m,
                          uint64_t key, ::common::Filter value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_Filter_get(const google::protobuf::Map<uint64_t, ::common::Filter>* m,
                       uint64_t key, const ::common::Filter** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::Filter* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_Filter_remove(google::protobuf::Map<uint64_t, ::common::Filter> * m,
                          uint64_t key, ::common::Filter * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_common_Filter_iter(
      const google::protobuf::Map<uint64_t, ::common::Filter>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_common_Filter_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::common::Filter** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::common::Filter>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::common::Filter>* __rust_proto_thunk__Map_bool_common_Filter_new() {
    return new google::protobuf::Map<bool, ::common::Filter>();
  }
  void __rust_proto_thunk__Map_bool_common_Filter_free(const google::protobuf::Map<bool, ::common::Filter>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_common_Filter_clear(google::protobuf::Map<bool, ::common::Filter> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_common_Filter_size(const google::protobuf::Map<bool, ::common::Filter>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_common_Filter_insert(google::protobuf::Map<bool, ::common::Filter> * m,
                          bool key, ::common::Filter value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_Filter_get(const google::protobuf::Map<bool, ::common::Filter>* m,
                       bool key, const ::common::Filter** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::Filter* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_Filter_remove(google::protobuf::Map<bool, ::common::Filter> * m,
                          bool key, ::common::Filter * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_common_Filter_iter(
      const google::protobuf::Map<bool, ::common::Filter>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_common_Filter_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::common::Filter** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::common::Filter>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::common::Filter>* __rust_proto_thunk__Map_ProtoStr_common_Filter_new() {
    return new google::protobuf::Map<std::string, ::common::Filter>();
  }
  void __rust_proto_thunk__Map_ProtoStr_common_Filter_free(const google::protobuf::Map<std::string, ::common::Filter>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_common_Filter_clear(google::protobuf::Map<std::string, ::common::Filter> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_common_Filter_size(const google::protobuf::Map<std::string, ::common::Filter>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_Filter_insert(google::protobuf::Map<std::string, ::common::Filter> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::Filter value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_Filter_get(const google::protobuf::Map<std::string, ::common::Filter>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::common::Filter** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::common::Filter* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_Filter_remove(google::protobuf::Map<std::string, ::common::Filter> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::Filter * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_common_Filter_iter(
      const google::protobuf::Map<std::string, ::common::Filter>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_common_Filter_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::common::Filter** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::common::Filter>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// common.SortItem
        // clang-format off
extern "C" {
void* __rust_proto_thunk__common_SortItem_new() { return new ::common::SortItem(); }
void __rust_proto_thunk__common_SortItem_delete(void* ptr) { delete static_cast<::common::SortItem*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__common_SortItem_serialize(::common::SortItem* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__common_SortItem_parse(::common::SortItem* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__common_SortItem_copy_from(::common::SortItem* dst, const ::common::SortItem* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__common_SortItem_repeated_len(google::protobuf::RepeatedPtrField<::common::SortItem>* field) {
  return field->size();
}
const ::common::SortItem& __rust_proto_thunk__common_SortItem_repeated_get(
  google::protobuf::RepeatedPtrField<::common::SortItem>* field,
  size_t index) {
  return field->Get(index);
}
::common::SortItem* __rust_proto_thunk__common_SortItem_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::common::SortItem>* field,
  size_t index) {
  return field->Mutable(index);
}
::common::SortItem* __rust_proto_thunk__common_SortItem_repeated_add(google::protobuf::RepeatedPtrField<::common::SortItem>* field) {
  return field->Add();
}
void __rust_proto_thunk__common_SortItem_repeated_clear(google::protobuf::RepeatedPtrField<::common::SortItem>* field) {
  field->Clear();
}
void __rust_proto_thunk__common_SortItem_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::common::SortItem>& dst,
  const google::protobuf::RepeatedPtrField<::common::SortItem>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__common_SortItem_get_field(::common::SortItem* msg) {
  absl::string_view val = msg->field();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__common_SortItem_set_field(::common::SortItem* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_field(absl::string_view(s.ptr, s.len));
}

::common::SortDirection __rust_proto_thunk__common_SortItem_get_direction(::common::SortItem* msg) { return msg->direction(); }
void __rust_proto_thunk__common_SortItem_set_direction(::common::SortItem* msg, ::common::SortDirection val) {
  msg->set_direction(val);
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::common::SortItem>* __rust_proto_thunk__Map_i32_common_SortItem_new() {
    return new google::protobuf::Map<int32_t, ::common::SortItem>();
  }
  void __rust_proto_thunk__Map_i32_common_SortItem_free(const google::protobuf::Map<int32_t, ::common::SortItem>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_common_SortItem_clear(google::protobuf::Map<int32_t, ::common::SortItem> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_common_SortItem_size(const google::protobuf::Map<int32_t, ::common::SortItem>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_common_SortItem_insert(google::protobuf::Map<int32_t, ::common::SortItem> * m,
                          int32_t key, ::common::SortItem value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_SortItem_get(const google::protobuf::Map<int32_t, ::common::SortItem>* m,
                       int32_t key, const ::common::SortItem** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::SortItem* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_SortItem_remove(google::protobuf::Map<int32_t, ::common::SortItem> * m,
                          int32_t key, ::common::SortItem * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_common_SortItem_iter(
      const google::protobuf::Map<int32_t, ::common::SortItem>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_common_SortItem_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::common::SortItem** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::common::SortItem>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::common::SortItem>* __rust_proto_thunk__Map_u32_common_SortItem_new() {
    return new google::protobuf::Map<uint32_t, ::common::SortItem>();
  }
  void __rust_proto_thunk__Map_u32_common_SortItem_free(const google::protobuf::Map<uint32_t, ::common::SortItem>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_common_SortItem_clear(google::protobuf::Map<uint32_t, ::common::SortItem> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_common_SortItem_size(const google::protobuf::Map<uint32_t, ::common::SortItem>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_common_SortItem_insert(google::protobuf::Map<uint32_t, ::common::SortItem> * m,
                          uint32_t key, ::common::SortItem value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_SortItem_get(const google::protobuf::Map<uint32_t, ::common::SortItem>* m,
                       uint32_t key, const ::common::SortItem** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::SortItem* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_SortItem_remove(google::protobuf::Map<uint32_t, ::common::SortItem> * m,
                          uint32_t key, ::common::SortItem * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_common_SortItem_iter(
      const google::protobuf::Map<uint32_t, ::common::SortItem>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_common_SortItem_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::common::SortItem** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::common::SortItem>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::common::SortItem>* __rust_proto_thunk__Map_i64_common_SortItem_new() {
    return new google::protobuf::Map<int64_t, ::common::SortItem>();
  }
  void __rust_proto_thunk__Map_i64_common_SortItem_free(const google::protobuf::Map<int64_t, ::common::SortItem>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_common_SortItem_clear(google::protobuf::Map<int64_t, ::common::SortItem> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_common_SortItem_size(const google::protobuf::Map<int64_t, ::common::SortItem>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_common_SortItem_insert(google::protobuf::Map<int64_t, ::common::SortItem> * m,
                          int64_t key, ::common::SortItem value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_SortItem_get(const google::protobuf::Map<int64_t, ::common::SortItem>* m,
                       int64_t key, const ::common::SortItem** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::SortItem* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_SortItem_remove(google::protobuf::Map<int64_t, ::common::SortItem> * m,
                          int64_t key, ::common::SortItem * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_common_SortItem_iter(
      const google::protobuf::Map<int64_t, ::common::SortItem>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_common_SortItem_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::common::SortItem** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::common::SortItem>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::common::SortItem>* __rust_proto_thunk__Map_u64_common_SortItem_new() {
    return new google::protobuf::Map<uint64_t, ::common::SortItem>();
  }
  void __rust_proto_thunk__Map_u64_common_SortItem_free(const google::protobuf::Map<uint64_t, ::common::SortItem>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_common_SortItem_clear(google::protobuf::Map<uint64_t, ::common::SortItem> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_common_SortItem_size(const google::protobuf::Map<uint64_t, ::common::SortItem>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_common_SortItem_insert(google::protobuf::Map<uint64_t, ::common::SortItem> * m,
                          uint64_t key, ::common::SortItem value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_SortItem_get(const google::protobuf::Map<uint64_t, ::common::SortItem>* m,
                       uint64_t key, const ::common::SortItem** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::SortItem* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_SortItem_remove(google::protobuf::Map<uint64_t, ::common::SortItem> * m,
                          uint64_t key, ::common::SortItem * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_common_SortItem_iter(
      const google::protobuf::Map<uint64_t, ::common::SortItem>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_common_SortItem_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::common::SortItem** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::common::SortItem>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::common::SortItem>* __rust_proto_thunk__Map_bool_common_SortItem_new() {
    return new google::protobuf::Map<bool, ::common::SortItem>();
  }
  void __rust_proto_thunk__Map_bool_common_SortItem_free(const google::protobuf::Map<bool, ::common::SortItem>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_common_SortItem_clear(google::protobuf::Map<bool, ::common::SortItem> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_common_SortItem_size(const google::protobuf::Map<bool, ::common::SortItem>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_common_SortItem_insert(google::protobuf::Map<bool, ::common::SortItem> * m,
                          bool key, ::common::SortItem value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_SortItem_get(const google::protobuf::Map<bool, ::common::SortItem>* m,
                       bool key, const ::common::SortItem** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::SortItem* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_SortItem_remove(google::protobuf::Map<bool, ::common::SortItem> * m,
                          bool key, ::common::SortItem * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_common_SortItem_iter(
      const google::protobuf::Map<bool, ::common::SortItem>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_common_SortItem_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::common::SortItem** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::common::SortItem>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::common::SortItem>* __rust_proto_thunk__Map_ProtoStr_common_SortItem_new() {
    return new google::protobuf::Map<std::string, ::common::SortItem>();
  }
  void __rust_proto_thunk__Map_ProtoStr_common_SortItem_free(const google::protobuf::Map<std::string, ::common::SortItem>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_common_SortItem_clear(google::protobuf::Map<std::string, ::common::SortItem> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_common_SortItem_size(const google::protobuf::Map<std::string, ::common::SortItem>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_SortItem_insert(google::protobuf::Map<std::string, ::common::SortItem> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::SortItem value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_SortItem_get(const google::protobuf::Map<std::string, ::common::SortItem>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::common::SortItem** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::common::SortItem* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_SortItem_remove(google::protobuf::Map<std::string, ::common::SortItem> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::SortItem * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_common_SortItem_iter(
      const google::protobuf::Map<std::string, ::common::SortItem>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_common_SortItem_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::common::SortItem** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::common::SortItem>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// common.SortBy
        // clang-format off
extern "C" {
void* __rust_proto_thunk__common_SortBy_new() { return new ::common::SortBy(); }
void __rust_proto_thunk__common_SortBy_delete(void* ptr) { delete static_cast<::common::SortBy*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__common_SortBy_serialize(::common::SortBy* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__common_SortBy_parse(::common::SortBy* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__common_SortBy_copy_from(::common::SortBy* dst, const ::common::SortBy* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__common_SortBy_repeated_len(google::protobuf::RepeatedPtrField<::common::SortBy>* field) {
  return field->size();
}
const ::common::SortBy& __rust_proto_thunk__common_SortBy_repeated_get(
  google::protobuf::RepeatedPtrField<::common::SortBy>* field,
  size_t index) {
  return field->Get(index);
}
::common::SortBy* __rust_proto_thunk__common_SortBy_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::common::SortBy>* field,
  size_t index) {
  return field->Mutable(index);
}
::common::SortBy* __rust_proto_thunk__common_SortBy_repeated_add(google::protobuf::RepeatedPtrField<::common::SortBy>* field) {
  return field->Add();
}
void __rust_proto_thunk__common_SortBy_repeated_clear(google::protobuf::RepeatedPtrField<::common::SortBy>* field) {
  field->Clear();
}
void __rust_proto_thunk__common_SortBy_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::common::SortBy>& dst,
  const google::protobuf::RepeatedPtrField<::common::SortBy>& src) {
  dst = src;
}

void __rust_proto_thunk__common_SortBy_clear_items(::common::SortBy* msg) {
  msg->clear_items();
}
google::protobuf::RepeatedPtrField<::common::SortItem>* __rust_proto_thunk__common_SortBy_get_mut_items(::common::SortBy* msg) {
  return msg->mutable_items();
}
const google::protobuf::RepeatedPtrField<::common::SortItem>* __rust_proto_thunk__common_SortBy_get_items(
    const ::common::SortBy* msg) {
  return &msg->items();
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::common::SortBy>* __rust_proto_thunk__Map_i32_common_SortBy_new() {
    return new google::protobuf::Map<int32_t, ::common::SortBy>();
  }
  void __rust_proto_thunk__Map_i32_common_SortBy_free(const google::protobuf::Map<int32_t, ::common::SortBy>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_common_SortBy_clear(google::protobuf::Map<int32_t, ::common::SortBy> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_common_SortBy_size(const google::protobuf::Map<int32_t, ::common::SortBy>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_common_SortBy_insert(google::protobuf::Map<int32_t, ::common::SortBy> * m,
                          int32_t key, ::common::SortBy value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_SortBy_get(const google::protobuf::Map<int32_t, ::common::SortBy>* m,
                       int32_t key, const ::common::SortBy** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::SortBy* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_SortBy_remove(google::protobuf::Map<int32_t, ::common::SortBy> * m,
                          int32_t key, ::common::SortBy * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_common_SortBy_iter(
      const google::protobuf::Map<int32_t, ::common::SortBy>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_common_SortBy_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::common::SortBy** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::common::SortBy>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::common::SortBy>* __rust_proto_thunk__Map_u32_common_SortBy_new() {
    return new google::protobuf::Map<uint32_t, ::common::SortBy>();
  }
  void __rust_proto_thunk__Map_u32_common_SortBy_free(const google::protobuf::Map<uint32_t, ::common::SortBy>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_common_SortBy_clear(google::protobuf::Map<uint32_t, ::common::SortBy> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_common_SortBy_size(const google::protobuf::Map<uint32_t, ::common::SortBy>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_common_SortBy_insert(google::protobuf::Map<uint32_t, ::common::SortBy> * m,
                          uint32_t key, ::common::SortBy value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_SortBy_get(const google::protobuf::Map<uint32_t, ::common::SortBy>* m,
                       uint32_t key, const ::common::SortBy** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::SortBy* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_SortBy_remove(google::protobuf::Map<uint32_t, ::common::SortBy> * m,
                          uint32_t key, ::common::SortBy * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_common_SortBy_iter(
      const google::protobuf::Map<uint32_t, ::common::SortBy>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_common_SortBy_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::common::SortBy** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::common::SortBy>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::common::SortBy>* __rust_proto_thunk__Map_i64_common_SortBy_new() {
    return new google::protobuf::Map<int64_t, ::common::SortBy>();
  }
  void __rust_proto_thunk__Map_i64_common_SortBy_free(const google::protobuf::Map<int64_t, ::common::SortBy>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_common_SortBy_clear(google::protobuf::Map<int64_t, ::common::SortBy> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_common_SortBy_size(const google::protobuf::Map<int64_t, ::common::SortBy>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_common_SortBy_insert(google::protobuf::Map<int64_t, ::common::SortBy> * m,
                          int64_t key, ::common::SortBy value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_SortBy_get(const google::protobuf::Map<int64_t, ::common::SortBy>* m,
                       int64_t key, const ::common::SortBy** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::SortBy* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_SortBy_remove(google::protobuf::Map<int64_t, ::common::SortBy> * m,
                          int64_t key, ::common::SortBy * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_common_SortBy_iter(
      const google::protobuf::Map<int64_t, ::common::SortBy>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_common_SortBy_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::common::SortBy** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::common::SortBy>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::common::SortBy>* __rust_proto_thunk__Map_u64_common_SortBy_new() {
    return new google::protobuf::Map<uint64_t, ::common::SortBy>();
  }
  void __rust_proto_thunk__Map_u64_common_SortBy_free(const google::protobuf::Map<uint64_t, ::common::SortBy>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_common_SortBy_clear(google::protobuf::Map<uint64_t, ::common::SortBy> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_common_SortBy_size(const google::protobuf::Map<uint64_t, ::common::SortBy>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_common_SortBy_insert(google::protobuf::Map<uint64_t, ::common::SortBy> * m,
                          uint64_t key, ::common::SortBy value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_SortBy_get(const google::protobuf::Map<uint64_t, ::common::SortBy>* m,
                       uint64_t key, const ::common::SortBy** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::SortBy* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_SortBy_remove(google::protobuf::Map<uint64_t, ::common::SortBy> * m,
                          uint64_t key, ::common::SortBy * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_common_SortBy_iter(
      const google::protobuf::Map<uint64_t, ::common::SortBy>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_common_SortBy_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::common::SortBy** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::common::SortBy>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::common::SortBy>* __rust_proto_thunk__Map_bool_common_SortBy_new() {
    return new google::protobuf::Map<bool, ::common::SortBy>();
  }
  void __rust_proto_thunk__Map_bool_common_SortBy_free(const google::protobuf::Map<bool, ::common::SortBy>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_common_SortBy_clear(google::protobuf::Map<bool, ::common::SortBy> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_common_SortBy_size(const google::protobuf::Map<bool, ::common::SortBy>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_common_SortBy_insert(google::protobuf::Map<bool, ::common::SortBy> * m,
                          bool key, ::common::SortBy value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_SortBy_get(const google::protobuf::Map<bool, ::common::SortBy>* m,
                       bool key, const ::common::SortBy** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::SortBy* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_SortBy_remove(google::protobuf::Map<bool, ::common::SortBy> * m,
                          bool key, ::common::SortBy * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_common_SortBy_iter(
      const google::protobuf::Map<bool, ::common::SortBy>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_common_SortBy_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::common::SortBy** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::common::SortBy>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::common::SortBy>* __rust_proto_thunk__Map_ProtoStr_common_SortBy_new() {
    return new google::protobuf::Map<std::string, ::common::SortBy>();
  }
  void __rust_proto_thunk__Map_ProtoStr_common_SortBy_free(const google::protobuf::Map<std::string, ::common::SortBy>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_common_SortBy_clear(google::protobuf::Map<std::string, ::common::SortBy> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_common_SortBy_size(const google::protobuf::Map<std::string, ::common::SortBy>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_SortBy_insert(google::protobuf::Map<std::string, ::common::SortBy> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::SortBy value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_SortBy_get(const google::protobuf::Map<std::string, ::common::SortBy>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::common::SortBy** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::common::SortBy* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_SortBy_remove(google::protobuf::Map<std::string, ::common::SortBy> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::SortBy * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_common_SortBy_iter(
      const google::protobuf::Map<std::string, ::common::SortBy>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_common_SortBy_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::common::SortBy** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::common::SortBy>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// common.AssetAuthority
        // clang-format off
extern "C" {
void* __rust_proto_thunk__common_AssetAuthority_new() { return new ::common::AssetAuthority(); }
void __rust_proto_thunk__common_AssetAuthority_delete(void* ptr) { delete static_cast<::common::AssetAuthority*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__common_AssetAuthority_serialize(::common::AssetAuthority* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__common_AssetAuthority_parse(::common::AssetAuthority* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__common_AssetAuthority_copy_from(::common::AssetAuthority* dst, const ::common::AssetAuthority* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__common_AssetAuthority_repeated_len(google::protobuf::RepeatedPtrField<::common::AssetAuthority>* field) {
  return field->size();
}
const ::common::AssetAuthority& __rust_proto_thunk__common_AssetAuthority_repeated_get(
  google::protobuf::RepeatedPtrField<::common::AssetAuthority>* field,
  size_t index) {
  return field->Get(index);
}
::common::AssetAuthority* __rust_proto_thunk__common_AssetAuthority_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::common::AssetAuthority>* field,
  size_t index) {
  return field->Mutable(index);
}
::common::AssetAuthority* __rust_proto_thunk__common_AssetAuthority_repeated_add(google::protobuf::RepeatedPtrField<::common::AssetAuthority>* field) {
  return field->Add();
}
void __rust_proto_thunk__common_AssetAuthority_repeated_clear(google::protobuf::RepeatedPtrField<::common::AssetAuthority>* field) {
  field->Clear();
}
void __rust_proto_thunk__common_AssetAuthority_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::common::AssetAuthority>& dst,
  const google::protobuf::RepeatedPtrField<::common::AssetAuthority>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__common_AssetAuthority_get_verifier(::common::AssetAuthority* msg) {
  absl::string_view val = msg->verifier();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__common_AssetAuthority_set_verifier(::common::AssetAuthority* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_verifier(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__common_AssetAuthority_get_url(::common::AssetAuthority* msg) {
  absl::string_view val = msg->url();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__common_AssetAuthority_set_url(::common::AssetAuthority* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_url(absl::string_view(s.ptr, s.len));
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::common::AssetAuthority>* __rust_proto_thunk__Map_i32_common_AssetAuthority_new() {
    return new google::protobuf::Map<int32_t, ::common::AssetAuthority>();
  }
  void __rust_proto_thunk__Map_i32_common_AssetAuthority_free(const google::protobuf::Map<int32_t, ::common::AssetAuthority>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_common_AssetAuthority_clear(google::protobuf::Map<int32_t, ::common::AssetAuthority> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_common_AssetAuthority_size(const google::protobuf::Map<int32_t, ::common::AssetAuthority>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_common_AssetAuthority_insert(google::protobuf::Map<int32_t, ::common::AssetAuthority> * m,
                          int32_t key, ::common::AssetAuthority value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_AssetAuthority_get(const google::protobuf::Map<int32_t, ::common::AssetAuthority>* m,
                       int32_t key, const ::common::AssetAuthority** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::AssetAuthority* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_AssetAuthority_remove(google::protobuf::Map<int32_t, ::common::AssetAuthority> * m,
                          int32_t key, ::common::AssetAuthority * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_common_AssetAuthority_iter(
      const google::protobuf::Map<int32_t, ::common::AssetAuthority>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_common_AssetAuthority_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::common::AssetAuthority** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::common::AssetAuthority>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::common::AssetAuthority>* __rust_proto_thunk__Map_u32_common_AssetAuthority_new() {
    return new google::protobuf::Map<uint32_t, ::common::AssetAuthority>();
  }
  void __rust_proto_thunk__Map_u32_common_AssetAuthority_free(const google::protobuf::Map<uint32_t, ::common::AssetAuthority>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_common_AssetAuthority_clear(google::protobuf::Map<uint32_t, ::common::AssetAuthority> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_common_AssetAuthority_size(const google::protobuf::Map<uint32_t, ::common::AssetAuthority>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_common_AssetAuthority_insert(google::protobuf::Map<uint32_t, ::common::AssetAuthority> * m,
                          uint32_t key, ::common::AssetAuthority value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_AssetAuthority_get(const google::protobuf::Map<uint32_t, ::common::AssetAuthority>* m,
                       uint32_t key, const ::common::AssetAuthority** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::AssetAuthority* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_AssetAuthority_remove(google::protobuf::Map<uint32_t, ::common::AssetAuthority> * m,
                          uint32_t key, ::common::AssetAuthority * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_common_AssetAuthority_iter(
      const google::protobuf::Map<uint32_t, ::common::AssetAuthority>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_common_AssetAuthority_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::common::AssetAuthority** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::common::AssetAuthority>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::common::AssetAuthority>* __rust_proto_thunk__Map_i64_common_AssetAuthority_new() {
    return new google::protobuf::Map<int64_t, ::common::AssetAuthority>();
  }
  void __rust_proto_thunk__Map_i64_common_AssetAuthority_free(const google::protobuf::Map<int64_t, ::common::AssetAuthority>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_common_AssetAuthority_clear(google::protobuf::Map<int64_t, ::common::AssetAuthority> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_common_AssetAuthority_size(const google::protobuf::Map<int64_t, ::common::AssetAuthority>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_common_AssetAuthority_insert(google::protobuf::Map<int64_t, ::common::AssetAuthority> * m,
                          int64_t key, ::common::AssetAuthority value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_AssetAuthority_get(const google::protobuf::Map<int64_t, ::common::AssetAuthority>* m,
                       int64_t key, const ::common::AssetAuthority** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::AssetAuthority* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_AssetAuthority_remove(google::protobuf::Map<int64_t, ::common::AssetAuthority> * m,
                          int64_t key, ::common::AssetAuthority * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_common_AssetAuthority_iter(
      const google::protobuf::Map<int64_t, ::common::AssetAuthority>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_common_AssetAuthority_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::common::AssetAuthority** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::common::AssetAuthority>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::common::AssetAuthority>* __rust_proto_thunk__Map_u64_common_AssetAuthority_new() {
    return new google::protobuf::Map<uint64_t, ::common::AssetAuthority>();
  }
  void __rust_proto_thunk__Map_u64_common_AssetAuthority_free(const google::protobuf::Map<uint64_t, ::common::AssetAuthority>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_common_AssetAuthority_clear(google::protobuf::Map<uint64_t, ::common::AssetAuthority> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_common_AssetAuthority_size(const google::protobuf::Map<uint64_t, ::common::AssetAuthority>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_common_AssetAuthority_insert(google::protobuf::Map<uint64_t, ::common::AssetAuthority> * m,
                          uint64_t key, ::common::AssetAuthority value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_AssetAuthority_get(const google::protobuf::Map<uint64_t, ::common::AssetAuthority>* m,
                       uint64_t key, const ::common::AssetAuthority** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::AssetAuthority* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_AssetAuthority_remove(google::protobuf::Map<uint64_t, ::common::AssetAuthority> * m,
                          uint64_t key, ::common::AssetAuthority * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_common_AssetAuthority_iter(
      const google::protobuf::Map<uint64_t, ::common::AssetAuthority>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_common_AssetAuthority_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::common::AssetAuthority** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::common::AssetAuthority>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::common::AssetAuthority>* __rust_proto_thunk__Map_bool_common_AssetAuthority_new() {
    return new google::protobuf::Map<bool, ::common::AssetAuthority>();
  }
  void __rust_proto_thunk__Map_bool_common_AssetAuthority_free(const google::protobuf::Map<bool, ::common::AssetAuthority>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_common_AssetAuthority_clear(google::protobuf::Map<bool, ::common::AssetAuthority> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_common_AssetAuthority_size(const google::protobuf::Map<bool, ::common::AssetAuthority>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_common_AssetAuthority_insert(google::protobuf::Map<bool, ::common::AssetAuthority> * m,
                          bool key, ::common::AssetAuthority value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_AssetAuthority_get(const google::protobuf::Map<bool, ::common::AssetAuthority>* m,
                       bool key, const ::common::AssetAuthority** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::AssetAuthority* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_AssetAuthority_remove(google::protobuf::Map<bool, ::common::AssetAuthority> * m,
                          bool key, ::common::AssetAuthority * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_common_AssetAuthority_iter(
      const google::protobuf::Map<bool, ::common::AssetAuthority>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_common_AssetAuthority_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::common::AssetAuthority** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::common::AssetAuthority>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::common::AssetAuthority>* __rust_proto_thunk__Map_ProtoStr_common_AssetAuthority_new() {
    return new google::protobuf::Map<std::string, ::common::AssetAuthority>();
  }
  void __rust_proto_thunk__Map_ProtoStr_common_AssetAuthority_free(const google::protobuf::Map<std::string, ::common::AssetAuthority>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_common_AssetAuthority_clear(google::protobuf::Map<std::string, ::common::AssetAuthority> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_common_AssetAuthority_size(const google::protobuf::Map<std::string, ::common::AssetAuthority>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_AssetAuthority_insert(google::protobuf::Map<std::string, ::common::AssetAuthority> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::AssetAuthority value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_AssetAuthority_get(const google::protobuf::Map<std::string, ::common::AssetAuthority>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::common::AssetAuthority** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::common::AssetAuthority* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_AssetAuthority_remove(google::protobuf::Map<std::string, ::common::AssetAuthority> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::AssetAuthority * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_common_AssetAuthority_iter(
      const google::protobuf::Map<std::string, ::common::AssetAuthority>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_common_AssetAuthority_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::common::AssetAuthority** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::common::AssetAuthority>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// common.AssetDenomination
        // clang-format off
extern "C" {
void* __rust_proto_thunk__common_AssetDenomination_new() { return new ::common::AssetDenomination(); }
void __rust_proto_thunk__common_AssetDenomination_delete(void* ptr) { delete static_cast<::common::AssetDenomination*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__common_AssetDenomination_serialize(::common::AssetDenomination* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__common_AssetDenomination_parse(::common::AssetDenomination* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__common_AssetDenomination_copy_from(::common::AssetDenomination* dst, const ::common::AssetDenomination* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__common_AssetDenomination_repeated_len(google::protobuf::RepeatedPtrField<::common::AssetDenomination>* field) {
  return field->size();
}
const ::common::AssetDenomination& __rust_proto_thunk__common_AssetDenomination_repeated_get(
  google::protobuf::RepeatedPtrField<::common::AssetDenomination>* field,
  size_t index) {
  return field->Get(index);
}
::common::AssetDenomination* __rust_proto_thunk__common_AssetDenomination_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::common::AssetDenomination>* field,
  size_t index) {
  return field->Mutable(index);
}
::common::AssetDenomination* __rust_proto_thunk__common_AssetDenomination_repeated_add(google::protobuf::RepeatedPtrField<::common::AssetDenomination>* field) {
  return field->Add();
}
void __rust_proto_thunk__common_AssetDenomination_repeated_clear(google::protobuf::RepeatedPtrField<::common::AssetDenomination>* field) {
  field->Clear();
}
void __rust_proto_thunk__common_AssetDenomination_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::common::AssetDenomination>& dst,
  const google::protobuf::RepeatedPtrField<::common::AssetDenomination>& src) {
  dst = src;
}

void __rust_proto_thunk__common_AssetDenomination_clear_denominations(::common::AssetDenomination* msg) {
  msg->clear_denominations();
}
google::protobuf::RepeatedField<::int64_t>* __rust_proto_thunk__common_AssetDenomination_get_mut_denominations(::common::AssetDenomination* msg) {
  return msg->mutable_denominations();
}
const google::protobuf::RepeatedField<::int64_t>* __rust_proto_thunk__common_AssetDenomination_get_denominations(
    const ::common::AssetDenomination* msg) {
  return &msg->denominations();
}

::uint32_t __rust_proto_thunk__common_AssetDenomination_get_precision(::common::AssetDenomination* msg) { return msg->precision(); }
void __rust_proto_thunk__common_AssetDenomination_set_precision(::common::AssetDenomination* msg, ::uint32_t val) {
  msg->set_precision(val);
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::common::AssetDenomination>* __rust_proto_thunk__Map_i32_common_AssetDenomination_new() {
    return new google::protobuf::Map<int32_t, ::common::AssetDenomination>();
  }
  void __rust_proto_thunk__Map_i32_common_AssetDenomination_free(const google::protobuf::Map<int32_t, ::common::AssetDenomination>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_common_AssetDenomination_clear(google::protobuf::Map<int32_t, ::common::AssetDenomination> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_common_AssetDenomination_size(const google::protobuf::Map<int32_t, ::common::AssetDenomination>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_common_AssetDenomination_insert(google::protobuf::Map<int32_t, ::common::AssetDenomination> * m,
                          int32_t key, ::common::AssetDenomination value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_AssetDenomination_get(const google::protobuf::Map<int32_t, ::common::AssetDenomination>* m,
                       int32_t key, const ::common::AssetDenomination** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::AssetDenomination* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_AssetDenomination_remove(google::protobuf::Map<int32_t, ::common::AssetDenomination> * m,
                          int32_t key, ::common::AssetDenomination * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_common_AssetDenomination_iter(
      const google::protobuf::Map<int32_t, ::common::AssetDenomination>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_common_AssetDenomination_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::common::AssetDenomination** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::common::AssetDenomination>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::common::AssetDenomination>* __rust_proto_thunk__Map_u32_common_AssetDenomination_new() {
    return new google::protobuf::Map<uint32_t, ::common::AssetDenomination>();
  }
  void __rust_proto_thunk__Map_u32_common_AssetDenomination_free(const google::protobuf::Map<uint32_t, ::common::AssetDenomination>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_common_AssetDenomination_clear(google::protobuf::Map<uint32_t, ::common::AssetDenomination> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_common_AssetDenomination_size(const google::protobuf::Map<uint32_t, ::common::AssetDenomination>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_common_AssetDenomination_insert(google::protobuf::Map<uint32_t, ::common::AssetDenomination> * m,
                          uint32_t key, ::common::AssetDenomination value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_AssetDenomination_get(const google::protobuf::Map<uint32_t, ::common::AssetDenomination>* m,
                       uint32_t key, const ::common::AssetDenomination** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::AssetDenomination* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_AssetDenomination_remove(google::protobuf::Map<uint32_t, ::common::AssetDenomination> * m,
                          uint32_t key, ::common::AssetDenomination * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_common_AssetDenomination_iter(
      const google::protobuf::Map<uint32_t, ::common::AssetDenomination>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_common_AssetDenomination_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::common::AssetDenomination** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::common::AssetDenomination>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::common::AssetDenomination>* __rust_proto_thunk__Map_i64_common_AssetDenomination_new() {
    return new google::protobuf::Map<int64_t, ::common::AssetDenomination>();
  }
  void __rust_proto_thunk__Map_i64_common_AssetDenomination_free(const google::protobuf::Map<int64_t, ::common::AssetDenomination>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_common_AssetDenomination_clear(google::protobuf::Map<int64_t, ::common::AssetDenomination> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_common_AssetDenomination_size(const google::protobuf::Map<int64_t, ::common::AssetDenomination>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_common_AssetDenomination_insert(google::protobuf::Map<int64_t, ::common::AssetDenomination> * m,
                          int64_t key, ::common::AssetDenomination value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_AssetDenomination_get(const google::protobuf::Map<int64_t, ::common::AssetDenomination>* m,
                       int64_t key, const ::common::AssetDenomination** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::AssetDenomination* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_AssetDenomination_remove(google::protobuf::Map<int64_t, ::common::AssetDenomination> * m,
                          int64_t key, ::common::AssetDenomination * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_common_AssetDenomination_iter(
      const google::protobuf::Map<int64_t, ::common::AssetDenomination>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_common_AssetDenomination_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::common::AssetDenomination** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::common::AssetDenomination>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::common::AssetDenomination>* __rust_proto_thunk__Map_u64_common_AssetDenomination_new() {
    return new google::protobuf::Map<uint64_t, ::common::AssetDenomination>();
  }
  void __rust_proto_thunk__Map_u64_common_AssetDenomination_free(const google::protobuf::Map<uint64_t, ::common::AssetDenomination>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_common_AssetDenomination_clear(google::protobuf::Map<uint64_t, ::common::AssetDenomination> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_common_AssetDenomination_size(const google::protobuf::Map<uint64_t, ::common::AssetDenomination>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_common_AssetDenomination_insert(google::protobuf::Map<uint64_t, ::common::AssetDenomination> * m,
                          uint64_t key, ::common::AssetDenomination value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_AssetDenomination_get(const google::protobuf::Map<uint64_t, ::common::AssetDenomination>* m,
                       uint64_t key, const ::common::AssetDenomination** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::AssetDenomination* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_AssetDenomination_remove(google::protobuf::Map<uint64_t, ::common::AssetDenomination> * m,
                          uint64_t key, ::common::AssetDenomination * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_common_AssetDenomination_iter(
      const google::protobuf::Map<uint64_t, ::common::AssetDenomination>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_common_AssetDenomination_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::common::AssetDenomination** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::common::AssetDenomination>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::common::AssetDenomination>* __rust_proto_thunk__Map_bool_common_AssetDenomination_new() {
    return new google::protobuf::Map<bool, ::common::AssetDenomination>();
  }
  void __rust_proto_thunk__Map_bool_common_AssetDenomination_free(const google::protobuf::Map<bool, ::common::AssetDenomination>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_common_AssetDenomination_clear(google::protobuf::Map<bool, ::common::AssetDenomination> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_common_AssetDenomination_size(const google::protobuf::Map<bool, ::common::AssetDenomination>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_common_AssetDenomination_insert(google::protobuf::Map<bool, ::common::AssetDenomination> * m,
                          bool key, ::common::AssetDenomination value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_AssetDenomination_get(const google::protobuf::Map<bool, ::common::AssetDenomination>* m,
                       bool key, const ::common::AssetDenomination** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::AssetDenomination* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_AssetDenomination_remove(google::protobuf::Map<bool, ::common::AssetDenomination> * m,
                          bool key, ::common::AssetDenomination * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_common_AssetDenomination_iter(
      const google::protobuf::Map<bool, ::common::AssetDenomination>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_common_AssetDenomination_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::common::AssetDenomination** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::common::AssetDenomination>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::common::AssetDenomination>* __rust_proto_thunk__Map_ProtoStr_common_AssetDenomination_new() {
    return new google::protobuf::Map<std::string, ::common::AssetDenomination>();
  }
  void __rust_proto_thunk__Map_ProtoStr_common_AssetDenomination_free(const google::protobuf::Map<std::string, ::common::AssetDenomination>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_common_AssetDenomination_clear(google::protobuf::Map<std::string, ::common::AssetDenomination> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_common_AssetDenomination_size(const google::protobuf::Map<std::string, ::common::AssetDenomination>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_AssetDenomination_insert(google::protobuf::Map<std::string, ::common::AssetDenomination> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::AssetDenomination value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_AssetDenomination_get(const google::protobuf::Map<std::string, ::common::AssetDenomination>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::common::AssetDenomination** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::common::AssetDenomination* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_AssetDenomination_remove(google::protobuf::Map<std::string, ::common::AssetDenomination> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::AssetDenomination * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_common_AssetDenomination_iter(
      const google::protobuf::Map<std::string, ::common::AssetDenomination>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_common_AssetDenomination_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::common::AssetDenomination** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::common::AssetDenomination>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// common.ServiceType
extern "C" {
  __PB_RUST_EXPOSE_SCALAR_MAP_METHODS_FOR_VALUE_TYPE(
      ::common::ServiceType, common_ServiceType, ::common::ServiceType, value, cpp_value)
}

// common.SignatureSystem
extern "C" {
  __PB_RUST_EXPOSE_SCALAR_MAP_METHODS_FOR_VALUE_TYPE(
      ::common::SignatureSystem, common_SignatureSystem, ::common::SignatureSystem, value, cpp_value)
}

// common.LogicalOperator
extern "C" {
  __PB_RUST_EXPOSE_SCALAR_MAP_METHODS_FOR_VALUE_TYPE(
      ::common::LogicalOperator, common_LogicalOperator, ::common::LogicalOperator, value, cpp_value)
}

// common.SortDirection
extern "C" {
  __PB_RUST_EXPOSE_SCALAR_MAP_METHODS_FOR_VALUE_TYPE(
      ::common::SortDirection, common_SortDirection, ::common::SortDirection, value, cpp_value)
}

