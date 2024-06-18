#include "treasury/v1/treasury.proto.h"
#include "google/protobuf/rust/cpp_kernel/cpp_api.h"
// treasury.GetSupplyRequest
        // clang-format off
extern "C" {
void* __rust_proto_thunk__treasury_GetSupplyRequest_new() { return new ::treasury::GetSupplyRequest(); }
void __rust_proto_thunk__treasury_GetSupplyRequest_delete(void* ptr) { delete static_cast<::treasury::GetSupplyRequest*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__treasury_GetSupplyRequest_serialize(::treasury::GetSupplyRequest* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__treasury_GetSupplyRequest_parse(::treasury::GetSupplyRequest* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__treasury_GetSupplyRequest_copy_from(::treasury::GetSupplyRequest* dst, const ::treasury::GetSupplyRequest* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__treasury_GetSupplyRequest_repeated_len(google::protobuf::RepeatedPtrField<::treasury::GetSupplyRequest>* field) {
  return field->size();
}
const ::treasury::GetSupplyRequest& __rust_proto_thunk__treasury_GetSupplyRequest_repeated_get(
  google::protobuf::RepeatedPtrField<::treasury::GetSupplyRequest>* field,
  size_t index) {
  return field->Get(index);
}
::treasury::GetSupplyRequest* __rust_proto_thunk__treasury_GetSupplyRequest_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::treasury::GetSupplyRequest>* field,
  size_t index) {
  return field->Mutable(index);
}
::treasury::GetSupplyRequest* __rust_proto_thunk__treasury_GetSupplyRequest_repeated_add(google::protobuf::RepeatedPtrField<::treasury::GetSupplyRequest>* field) {
  return field->Add();
}
void __rust_proto_thunk__treasury_GetSupplyRequest_repeated_clear(google::protobuf::RepeatedPtrField<::treasury::GetSupplyRequest>* field) {
  field->Clear();
}
void __rust_proto_thunk__treasury_GetSupplyRequest_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::treasury::GetSupplyRequest>& dst,
  const google::protobuf::RepeatedPtrField<::treasury::GetSupplyRequest>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__treasury_GetSupplyRequest_get_currency_code(::treasury::GetSupplyRequest* msg) {
  absl::string_view val = msg->currency_code();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__treasury_GetSupplyRequest_set_currency_code(::treasury::GetSupplyRequest* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_currency_code(absl::string_view(s.ptr, s.len));
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::treasury::GetSupplyRequest>* __rust_proto_thunk__Map_i32_treasury_GetSupplyRequest_new() {
    return new google::protobuf::Map<int32_t, ::treasury::GetSupplyRequest>();
  }
  void __rust_proto_thunk__Map_i32_treasury_GetSupplyRequest_free(const google::protobuf::Map<int32_t, ::treasury::GetSupplyRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_treasury_GetSupplyRequest_clear(google::protobuf::Map<int32_t, ::treasury::GetSupplyRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_treasury_GetSupplyRequest_size(const google::protobuf::Map<int32_t, ::treasury::GetSupplyRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_treasury_GetSupplyRequest_insert(google::protobuf::Map<int32_t, ::treasury::GetSupplyRequest> * m,
                          int32_t key, ::treasury::GetSupplyRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_treasury_GetSupplyRequest_get(const google::protobuf::Map<int32_t, ::treasury::GetSupplyRequest>* m,
                       int32_t key, const ::treasury::GetSupplyRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::treasury::GetSupplyRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_treasury_GetSupplyRequest_remove(google::protobuf::Map<int32_t, ::treasury::GetSupplyRequest> * m,
                          int32_t key, ::treasury::GetSupplyRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_treasury_GetSupplyRequest_iter(
      const google::protobuf::Map<int32_t, ::treasury::GetSupplyRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_treasury_GetSupplyRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::treasury::GetSupplyRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::treasury::GetSupplyRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::treasury::GetSupplyRequest>* __rust_proto_thunk__Map_u32_treasury_GetSupplyRequest_new() {
    return new google::protobuf::Map<uint32_t, ::treasury::GetSupplyRequest>();
  }
  void __rust_proto_thunk__Map_u32_treasury_GetSupplyRequest_free(const google::protobuf::Map<uint32_t, ::treasury::GetSupplyRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_treasury_GetSupplyRequest_clear(google::protobuf::Map<uint32_t, ::treasury::GetSupplyRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_treasury_GetSupplyRequest_size(const google::protobuf::Map<uint32_t, ::treasury::GetSupplyRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_treasury_GetSupplyRequest_insert(google::protobuf::Map<uint32_t, ::treasury::GetSupplyRequest> * m,
                          uint32_t key, ::treasury::GetSupplyRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_treasury_GetSupplyRequest_get(const google::protobuf::Map<uint32_t, ::treasury::GetSupplyRequest>* m,
                       uint32_t key, const ::treasury::GetSupplyRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::treasury::GetSupplyRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_treasury_GetSupplyRequest_remove(google::protobuf::Map<uint32_t, ::treasury::GetSupplyRequest> * m,
                          uint32_t key, ::treasury::GetSupplyRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_treasury_GetSupplyRequest_iter(
      const google::protobuf::Map<uint32_t, ::treasury::GetSupplyRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_treasury_GetSupplyRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::treasury::GetSupplyRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::treasury::GetSupplyRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::treasury::GetSupplyRequest>* __rust_proto_thunk__Map_i64_treasury_GetSupplyRequest_new() {
    return new google::protobuf::Map<int64_t, ::treasury::GetSupplyRequest>();
  }
  void __rust_proto_thunk__Map_i64_treasury_GetSupplyRequest_free(const google::protobuf::Map<int64_t, ::treasury::GetSupplyRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_treasury_GetSupplyRequest_clear(google::protobuf::Map<int64_t, ::treasury::GetSupplyRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_treasury_GetSupplyRequest_size(const google::protobuf::Map<int64_t, ::treasury::GetSupplyRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_treasury_GetSupplyRequest_insert(google::protobuf::Map<int64_t, ::treasury::GetSupplyRequest> * m,
                          int64_t key, ::treasury::GetSupplyRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_treasury_GetSupplyRequest_get(const google::protobuf::Map<int64_t, ::treasury::GetSupplyRequest>* m,
                       int64_t key, const ::treasury::GetSupplyRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::treasury::GetSupplyRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_treasury_GetSupplyRequest_remove(google::protobuf::Map<int64_t, ::treasury::GetSupplyRequest> * m,
                          int64_t key, ::treasury::GetSupplyRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_treasury_GetSupplyRequest_iter(
      const google::protobuf::Map<int64_t, ::treasury::GetSupplyRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_treasury_GetSupplyRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::treasury::GetSupplyRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::treasury::GetSupplyRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::treasury::GetSupplyRequest>* __rust_proto_thunk__Map_u64_treasury_GetSupplyRequest_new() {
    return new google::protobuf::Map<uint64_t, ::treasury::GetSupplyRequest>();
  }
  void __rust_proto_thunk__Map_u64_treasury_GetSupplyRequest_free(const google::protobuf::Map<uint64_t, ::treasury::GetSupplyRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_treasury_GetSupplyRequest_clear(google::protobuf::Map<uint64_t, ::treasury::GetSupplyRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_treasury_GetSupplyRequest_size(const google::protobuf::Map<uint64_t, ::treasury::GetSupplyRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_treasury_GetSupplyRequest_insert(google::protobuf::Map<uint64_t, ::treasury::GetSupplyRequest> * m,
                          uint64_t key, ::treasury::GetSupplyRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_treasury_GetSupplyRequest_get(const google::protobuf::Map<uint64_t, ::treasury::GetSupplyRequest>* m,
                       uint64_t key, const ::treasury::GetSupplyRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::treasury::GetSupplyRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_treasury_GetSupplyRequest_remove(google::protobuf::Map<uint64_t, ::treasury::GetSupplyRequest> * m,
                          uint64_t key, ::treasury::GetSupplyRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_treasury_GetSupplyRequest_iter(
      const google::protobuf::Map<uint64_t, ::treasury::GetSupplyRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_treasury_GetSupplyRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::treasury::GetSupplyRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::treasury::GetSupplyRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::treasury::GetSupplyRequest>* __rust_proto_thunk__Map_bool_treasury_GetSupplyRequest_new() {
    return new google::protobuf::Map<bool, ::treasury::GetSupplyRequest>();
  }
  void __rust_proto_thunk__Map_bool_treasury_GetSupplyRequest_free(const google::protobuf::Map<bool, ::treasury::GetSupplyRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_treasury_GetSupplyRequest_clear(google::protobuf::Map<bool, ::treasury::GetSupplyRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_treasury_GetSupplyRequest_size(const google::protobuf::Map<bool, ::treasury::GetSupplyRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_treasury_GetSupplyRequest_insert(google::protobuf::Map<bool, ::treasury::GetSupplyRequest> * m,
                          bool key, ::treasury::GetSupplyRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_treasury_GetSupplyRequest_get(const google::protobuf::Map<bool, ::treasury::GetSupplyRequest>* m,
                       bool key, const ::treasury::GetSupplyRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::treasury::GetSupplyRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_treasury_GetSupplyRequest_remove(google::protobuf::Map<bool, ::treasury::GetSupplyRequest> * m,
                          bool key, ::treasury::GetSupplyRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_treasury_GetSupplyRequest_iter(
      const google::protobuf::Map<bool, ::treasury::GetSupplyRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_treasury_GetSupplyRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::treasury::GetSupplyRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::treasury::GetSupplyRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::treasury::GetSupplyRequest>* __rust_proto_thunk__Map_ProtoStr_treasury_GetSupplyRequest_new() {
    return new google::protobuf::Map<std::string, ::treasury::GetSupplyRequest>();
  }
  void __rust_proto_thunk__Map_ProtoStr_treasury_GetSupplyRequest_free(const google::protobuf::Map<std::string, ::treasury::GetSupplyRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_treasury_GetSupplyRequest_clear(google::protobuf::Map<std::string, ::treasury::GetSupplyRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_treasury_GetSupplyRequest_size(const google::protobuf::Map<std::string, ::treasury::GetSupplyRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_treasury_GetSupplyRequest_insert(google::protobuf::Map<std::string, ::treasury::GetSupplyRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::treasury::GetSupplyRequest value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_treasury_GetSupplyRequest_get(const google::protobuf::Map<std::string, ::treasury::GetSupplyRequest>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::treasury::GetSupplyRequest** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::treasury::GetSupplyRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_treasury_GetSupplyRequest_remove(google::protobuf::Map<std::string, ::treasury::GetSupplyRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::treasury::GetSupplyRequest * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_treasury_GetSupplyRequest_iter(
      const google::protobuf::Map<std::string, ::treasury::GetSupplyRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_treasury_GetSupplyRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::treasury::GetSupplyRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::treasury::GetSupplyRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// treasury.DenominationCount
        // clang-format off
extern "C" {
void* __rust_proto_thunk__treasury_DenominationCount_new() { return new ::treasury::DenominationCount(); }
void __rust_proto_thunk__treasury_DenominationCount_delete(void* ptr) { delete static_cast<::treasury::DenominationCount*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__treasury_DenominationCount_serialize(::treasury::DenominationCount* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__treasury_DenominationCount_parse(::treasury::DenominationCount* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__treasury_DenominationCount_copy_from(::treasury::DenominationCount* dst, const ::treasury::DenominationCount* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__treasury_DenominationCount_repeated_len(google::protobuf::RepeatedPtrField<::treasury::DenominationCount>* field) {
  return field->size();
}
const ::treasury::DenominationCount& __rust_proto_thunk__treasury_DenominationCount_repeated_get(
  google::protobuf::RepeatedPtrField<::treasury::DenominationCount>* field,
  size_t index) {
  return field->Get(index);
}
::treasury::DenominationCount* __rust_proto_thunk__treasury_DenominationCount_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::treasury::DenominationCount>* field,
  size_t index) {
  return field->Mutable(index);
}
::treasury::DenominationCount* __rust_proto_thunk__treasury_DenominationCount_repeated_add(google::protobuf::RepeatedPtrField<::treasury::DenominationCount>* field) {
  return field->Add();
}
void __rust_proto_thunk__treasury_DenominationCount_repeated_clear(google::protobuf::RepeatedPtrField<::treasury::DenominationCount>* field) {
  field->Clear();
}
void __rust_proto_thunk__treasury_DenominationCount_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::treasury::DenominationCount>& dst,
  const google::protobuf::RepeatedPtrField<::treasury::DenominationCount>& src) {
  dst = src;
}

const void* __rust_proto_thunk__treasury_DenominationCount_get_denomination(::treasury::DenominationCount* msg) {
  return static_cast<const void*>(&msg->denomination());
}
void* __rust_proto_thunk__treasury_DenominationCount_get_mut_denomination(::treasury::DenominationCount* msg) {
  return static_cast<void*>(msg->mutable_denomination());
}
void __rust_proto_thunk__treasury_DenominationCount_clear_denomination(::treasury::DenominationCount* msg) { msg->clear_denomination(); }
bool __rust_proto_thunk__treasury_DenominationCount_has_denomination(::treasury::DenominationCount* msg) { return msg->has_denomination(); }

::uint32_t __rust_proto_thunk__treasury_DenominationCount_get_count(::treasury::DenominationCount* msg) { return msg->count(); }
void __rust_proto_thunk__treasury_DenominationCount_set_count(::treasury::DenominationCount* msg, ::uint32_t val) {
  msg->set_count(val);
}

const void* __rust_proto_thunk__treasury_DenominationCount_get_total(::treasury::DenominationCount* msg) {
  return static_cast<const void*>(&msg->total());
}
void* __rust_proto_thunk__treasury_DenominationCount_get_mut_total(::treasury::DenominationCount* msg) {
  return static_cast<void*>(msg->mutable_total());
}
void __rust_proto_thunk__treasury_DenominationCount_clear_total(::treasury::DenominationCount* msg) { msg->clear_total(); }
bool __rust_proto_thunk__treasury_DenominationCount_has_total(::treasury::DenominationCount* msg) { return msg->has_total(); }


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::treasury::DenominationCount>* __rust_proto_thunk__Map_i32_treasury_DenominationCount_new() {
    return new google::protobuf::Map<int32_t, ::treasury::DenominationCount>();
  }
  void __rust_proto_thunk__Map_i32_treasury_DenominationCount_free(const google::protobuf::Map<int32_t, ::treasury::DenominationCount>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_treasury_DenominationCount_clear(google::protobuf::Map<int32_t, ::treasury::DenominationCount> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_treasury_DenominationCount_size(const google::protobuf::Map<int32_t, ::treasury::DenominationCount>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_treasury_DenominationCount_insert(google::protobuf::Map<int32_t, ::treasury::DenominationCount> * m,
                          int32_t key, ::treasury::DenominationCount value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_treasury_DenominationCount_get(const google::protobuf::Map<int32_t, ::treasury::DenominationCount>* m,
                       int32_t key, const ::treasury::DenominationCount** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::treasury::DenominationCount* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_treasury_DenominationCount_remove(google::protobuf::Map<int32_t, ::treasury::DenominationCount> * m,
                          int32_t key, ::treasury::DenominationCount * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_treasury_DenominationCount_iter(
      const google::protobuf::Map<int32_t, ::treasury::DenominationCount>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_treasury_DenominationCount_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::treasury::DenominationCount** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::treasury::DenominationCount>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::treasury::DenominationCount>* __rust_proto_thunk__Map_u32_treasury_DenominationCount_new() {
    return new google::protobuf::Map<uint32_t, ::treasury::DenominationCount>();
  }
  void __rust_proto_thunk__Map_u32_treasury_DenominationCount_free(const google::protobuf::Map<uint32_t, ::treasury::DenominationCount>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_treasury_DenominationCount_clear(google::protobuf::Map<uint32_t, ::treasury::DenominationCount> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_treasury_DenominationCount_size(const google::protobuf::Map<uint32_t, ::treasury::DenominationCount>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_treasury_DenominationCount_insert(google::protobuf::Map<uint32_t, ::treasury::DenominationCount> * m,
                          uint32_t key, ::treasury::DenominationCount value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_treasury_DenominationCount_get(const google::protobuf::Map<uint32_t, ::treasury::DenominationCount>* m,
                       uint32_t key, const ::treasury::DenominationCount** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::treasury::DenominationCount* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_treasury_DenominationCount_remove(google::protobuf::Map<uint32_t, ::treasury::DenominationCount> * m,
                          uint32_t key, ::treasury::DenominationCount * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_treasury_DenominationCount_iter(
      const google::protobuf::Map<uint32_t, ::treasury::DenominationCount>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_treasury_DenominationCount_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::treasury::DenominationCount** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::treasury::DenominationCount>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::treasury::DenominationCount>* __rust_proto_thunk__Map_i64_treasury_DenominationCount_new() {
    return new google::protobuf::Map<int64_t, ::treasury::DenominationCount>();
  }
  void __rust_proto_thunk__Map_i64_treasury_DenominationCount_free(const google::protobuf::Map<int64_t, ::treasury::DenominationCount>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_treasury_DenominationCount_clear(google::protobuf::Map<int64_t, ::treasury::DenominationCount> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_treasury_DenominationCount_size(const google::protobuf::Map<int64_t, ::treasury::DenominationCount>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_treasury_DenominationCount_insert(google::protobuf::Map<int64_t, ::treasury::DenominationCount> * m,
                          int64_t key, ::treasury::DenominationCount value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_treasury_DenominationCount_get(const google::protobuf::Map<int64_t, ::treasury::DenominationCount>* m,
                       int64_t key, const ::treasury::DenominationCount** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::treasury::DenominationCount* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_treasury_DenominationCount_remove(google::protobuf::Map<int64_t, ::treasury::DenominationCount> * m,
                          int64_t key, ::treasury::DenominationCount * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_treasury_DenominationCount_iter(
      const google::protobuf::Map<int64_t, ::treasury::DenominationCount>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_treasury_DenominationCount_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::treasury::DenominationCount** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::treasury::DenominationCount>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::treasury::DenominationCount>* __rust_proto_thunk__Map_u64_treasury_DenominationCount_new() {
    return new google::protobuf::Map<uint64_t, ::treasury::DenominationCount>();
  }
  void __rust_proto_thunk__Map_u64_treasury_DenominationCount_free(const google::protobuf::Map<uint64_t, ::treasury::DenominationCount>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_treasury_DenominationCount_clear(google::protobuf::Map<uint64_t, ::treasury::DenominationCount> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_treasury_DenominationCount_size(const google::protobuf::Map<uint64_t, ::treasury::DenominationCount>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_treasury_DenominationCount_insert(google::protobuf::Map<uint64_t, ::treasury::DenominationCount> * m,
                          uint64_t key, ::treasury::DenominationCount value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_treasury_DenominationCount_get(const google::protobuf::Map<uint64_t, ::treasury::DenominationCount>* m,
                       uint64_t key, const ::treasury::DenominationCount** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::treasury::DenominationCount* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_treasury_DenominationCount_remove(google::protobuf::Map<uint64_t, ::treasury::DenominationCount> * m,
                          uint64_t key, ::treasury::DenominationCount * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_treasury_DenominationCount_iter(
      const google::protobuf::Map<uint64_t, ::treasury::DenominationCount>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_treasury_DenominationCount_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::treasury::DenominationCount** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::treasury::DenominationCount>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::treasury::DenominationCount>* __rust_proto_thunk__Map_bool_treasury_DenominationCount_new() {
    return new google::protobuf::Map<bool, ::treasury::DenominationCount>();
  }
  void __rust_proto_thunk__Map_bool_treasury_DenominationCount_free(const google::protobuf::Map<bool, ::treasury::DenominationCount>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_treasury_DenominationCount_clear(google::protobuf::Map<bool, ::treasury::DenominationCount> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_treasury_DenominationCount_size(const google::protobuf::Map<bool, ::treasury::DenominationCount>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_treasury_DenominationCount_insert(google::protobuf::Map<bool, ::treasury::DenominationCount> * m,
                          bool key, ::treasury::DenominationCount value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_treasury_DenominationCount_get(const google::protobuf::Map<bool, ::treasury::DenominationCount>* m,
                       bool key, const ::treasury::DenominationCount** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::treasury::DenominationCount* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_treasury_DenominationCount_remove(google::protobuf::Map<bool, ::treasury::DenominationCount> * m,
                          bool key, ::treasury::DenominationCount * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_treasury_DenominationCount_iter(
      const google::protobuf::Map<bool, ::treasury::DenominationCount>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_treasury_DenominationCount_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::treasury::DenominationCount** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::treasury::DenominationCount>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::treasury::DenominationCount>* __rust_proto_thunk__Map_ProtoStr_treasury_DenominationCount_new() {
    return new google::protobuf::Map<std::string, ::treasury::DenominationCount>();
  }
  void __rust_proto_thunk__Map_ProtoStr_treasury_DenominationCount_free(const google::protobuf::Map<std::string, ::treasury::DenominationCount>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_treasury_DenominationCount_clear(google::protobuf::Map<std::string, ::treasury::DenominationCount> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_treasury_DenominationCount_size(const google::protobuf::Map<std::string, ::treasury::DenominationCount>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_treasury_DenominationCount_insert(google::protobuf::Map<std::string, ::treasury::DenominationCount> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::treasury::DenominationCount value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_treasury_DenominationCount_get(const google::protobuf::Map<std::string, ::treasury::DenominationCount>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::treasury::DenominationCount** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::treasury::DenominationCount* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_treasury_DenominationCount_remove(google::protobuf::Map<std::string, ::treasury::DenominationCount> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::treasury::DenominationCount * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_treasury_DenominationCount_iter(
      const google::protobuf::Map<std::string, ::treasury::DenominationCount>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_treasury_DenominationCount_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::treasury::DenominationCount** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::treasury::DenominationCount>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// treasury.GetSupplyResponse
        // clang-format off
extern "C" {
void* __rust_proto_thunk__treasury_GetSupplyResponse_new() { return new ::treasury::GetSupplyResponse(); }
void __rust_proto_thunk__treasury_GetSupplyResponse_delete(void* ptr) { delete static_cast<::treasury::GetSupplyResponse*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__treasury_GetSupplyResponse_serialize(::treasury::GetSupplyResponse* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__treasury_GetSupplyResponse_parse(::treasury::GetSupplyResponse* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__treasury_GetSupplyResponse_copy_from(::treasury::GetSupplyResponse* dst, const ::treasury::GetSupplyResponse* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__treasury_GetSupplyResponse_repeated_len(google::protobuf::RepeatedPtrField<::treasury::GetSupplyResponse>* field) {
  return field->size();
}
const ::treasury::GetSupplyResponse& __rust_proto_thunk__treasury_GetSupplyResponse_repeated_get(
  google::protobuf::RepeatedPtrField<::treasury::GetSupplyResponse>* field,
  size_t index) {
  return field->Get(index);
}
::treasury::GetSupplyResponse* __rust_proto_thunk__treasury_GetSupplyResponse_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::treasury::GetSupplyResponse>* field,
  size_t index) {
  return field->Mutable(index);
}
::treasury::GetSupplyResponse* __rust_proto_thunk__treasury_GetSupplyResponse_repeated_add(google::protobuf::RepeatedPtrField<::treasury::GetSupplyResponse>* field) {
  return field->Add();
}
void __rust_proto_thunk__treasury_GetSupplyResponse_repeated_clear(google::protobuf::RepeatedPtrField<::treasury::GetSupplyResponse>* field) {
  field->Clear();
}
void __rust_proto_thunk__treasury_GetSupplyResponse_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::treasury::GetSupplyResponse>& dst,
  const google::protobuf::RepeatedPtrField<::treasury::GetSupplyResponse>& src) {
  dst = src;
}

void __rust_proto_thunk__treasury_GetSupplyResponse_clear_supply(::treasury::GetSupplyResponse* msg) {
  msg->clear_supply();
}
google::protobuf::RepeatedPtrField<::treasury::DenominationCount>* __rust_proto_thunk__treasury_GetSupplyResponse_get_mut_supply(::treasury::GetSupplyResponse* msg) {
  return msg->mutable_supply();
}
const google::protobuf::RepeatedPtrField<::treasury::DenominationCount>* __rust_proto_thunk__treasury_GetSupplyResponse_get_supply(
    const ::treasury::GetSupplyResponse* msg) {
  return &msg->supply();
}

const void* __rust_proto_thunk__treasury_GetSupplyResponse_get_total(::treasury::GetSupplyResponse* msg) {
  return static_cast<const void*>(&msg->total());
}
void* __rust_proto_thunk__treasury_GetSupplyResponse_get_mut_total(::treasury::GetSupplyResponse* msg) {
  return static_cast<void*>(msg->mutable_total());
}
void __rust_proto_thunk__treasury_GetSupplyResponse_clear_total(::treasury::GetSupplyResponse* msg) { msg->clear_total(); }
bool __rust_proto_thunk__treasury_GetSupplyResponse_has_total(::treasury::GetSupplyResponse* msg) { return msg->has_total(); }


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::treasury::GetSupplyResponse>* __rust_proto_thunk__Map_i32_treasury_GetSupplyResponse_new() {
    return new google::protobuf::Map<int32_t, ::treasury::GetSupplyResponse>();
  }
  void __rust_proto_thunk__Map_i32_treasury_GetSupplyResponse_free(const google::protobuf::Map<int32_t, ::treasury::GetSupplyResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_treasury_GetSupplyResponse_clear(google::protobuf::Map<int32_t, ::treasury::GetSupplyResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_treasury_GetSupplyResponse_size(const google::protobuf::Map<int32_t, ::treasury::GetSupplyResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_treasury_GetSupplyResponse_insert(google::protobuf::Map<int32_t, ::treasury::GetSupplyResponse> * m,
                          int32_t key, ::treasury::GetSupplyResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_treasury_GetSupplyResponse_get(const google::protobuf::Map<int32_t, ::treasury::GetSupplyResponse>* m,
                       int32_t key, const ::treasury::GetSupplyResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::treasury::GetSupplyResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_treasury_GetSupplyResponse_remove(google::protobuf::Map<int32_t, ::treasury::GetSupplyResponse> * m,
                          int32_t key, ::treasury::GetSupplyResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_treasury_GetSupplyResponse_iter(
      const google::protobuf::Map<int32_t, ::treasury::GetSupplyResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_treasury_GetSupplyResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::treasury::GetSupplyResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::treasury::GetSupplyResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::treasury::GetSupplyResponse>* __rust_proto_thunk__Map_u32_treasury_GetSupplyResponse_new() {
    return new google::protobuf::Map<uint32_t, ::treasury::GetSupplyResponse>();
  }
  void __rust_proto_thunk__Map_u32_treasury_GetSupplyResponse_free(const google::protobuf::Map<uint32_t, ::treasury::GetSupplyResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_treasury_GetSupplyResponse_clear(google::protobuf::Map<uint32_t, ::treasury::GetSupplyResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_treasury_GetSupplyResponse_size(const google::protobuf::Map<uint32_t, ::treasury::GetSupplyResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_treasury_GetSupplyResponse_insert(google::protobuf::Map<uint32_t, ::treasury::GetSupplyResponse> * m,
                          uint32_t key, ::treasury::GetSupplyResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_treasury_GetSupplyResponse_get(const google::protobuf::Map<uint32_t, ::treasury::GetSupplyResponse>* m,
                       uint32_t key, const ::treasury::GetSupplyResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::treasury::GetSupplyResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_treasury_GetSupplyResponse_remove(google::protobuf::Map<uint32_t, ::treasury::GetSupplyResponse> * m,
                          uint32_t key, ::treasury::GetSupplyResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_treasury_GetSupplyResponse_iter(
      const google::protobuf::Map<uint32_t, ::treasury::GetSupplyResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_treasury_GetSupplyResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::treasury::GetSupplyResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::treasury::GetSupplyResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::treasury::GetSupplyResponse>* __rust_proto_thunk__Map_i64_treasury_GetSupplyResponse_new() {
    return new google::protobuf::Map<int64_t, ::treasury::GetSupplyResponse>();
  }
  void __rust_proto_thunk__Map_i64_treasury_GetSupplyResponse_free(const google::protobuf::Map<int64_t, ::treasury::GetSupplyResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_treasury_GetSupplyResponse_clear(google::protobuf::Map<int64_t, ::treasury::GetSupplyResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_treasury_GetSupplyResponse_size(const google::protobuf::Map<int64_t, ::treasury::GetSupplyResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_treasury_GetSupplyResponse_insert(google::protobuf::Map<int64_t, ::treasury::GetSupplyResponse> * m,
                          int64_t key, ::treasury::GetSupplyResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_treasury_GetSupplyResponse_get(const google::protobuf::Map<int64_t, ::treasury::GetSupplyResponse>* m,
                       int64_t key, const ::treasury::GetSupplyResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::treasury::GetSupplyResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_treasury_GetSupplyResponse_remove(google::protobuf::Map<int64_t, ::treasury::GetSupplyResponse> * m,
                          int64_t key, ::treasury::GetSupplyResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_treasury_GetSupplyResponse_iter(
      const google::protobuf::Map<int64_t, ::treasury::GetSupplyResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_treasury_GetSupplyResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::treasury::GetSupplyResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::treasury::GetSupplyResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::treasury::GetSupplyResponse>* __rust_proto_thunk__Map_u64_treasury_GetSupplyResponse_new() {
    return new google::protobuf::Map<uint64_t, ::treasury::GetSupplyResponse>();
  }
  void __rust_proto_thunk__Map_u64_treasury_GetSupplyResponse_free(const google::protobuf::Map<uint64_t, ::treasury::GetSupplyResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_treasury_GetSupplyResponse_clear(google::protobuf::Map<uint64_t, ::treasury::GetSupplyResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_treasury_GetSupplyResponse_size(const google::protobuf::Map<uint64_t, ::treasury::GetSupplyResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_treasury_GetSupplyResponse_insert(google::protobuf::Map<uint64_t, ::treasury::GetSupplyResponse> * m,
                          uint64_t key, ::treasury::GetSupplyResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_treasury_GetSupplyResponse_get(const google::protobuf::Map<uint64_t, ::treasury::GetSupplyResponse>* m,
                       uint64_t key, const ::treasury::GetSupplyResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::treasury::GetSupplyResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_treasury_GetSupplyResponse_remove(google::protobuf::Map<uint64_t, ::treasury::GetSupplyResponse> * m,
                          uint64_t key, ::treasury::GetSupplyResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_treasury_GetSupplyResponse_iter(
      const google::protobuf::Map<uint64_t, ::treasury::GetSupplyResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_treasury_GetSupplyResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::treasury::GetSupplyResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::treasury::GetSupplyResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::treasury::GetSupplyResponse>* __rust_proto_thunk__Map_bool_treasury_GetSupplyResponse_new() {
    return new google::protobuf::Map<bool, ::treasury::GetSupplyResponse>();
  }
  void __rust_proto_thunk__Map_bool_treasury_GetSupplyResponse_free(const google::protobuf::Map<bool, ::treasury::GetSupplyResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_treasury_GetSupplyResponse_clear(google::protobuf::Map<bool, ::treasury::GetSupplyResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_treasury_GetSupplyResponse_size(const google::protobuf::Map<bool, ::treasury::GetSupplyResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_treasury_GetSupplyResponse_insert(google::protobuf::Map<bool, ::treasury::GetSupplyResponse> * m,
                          bool key, ::treasury::GetSupplyResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_treasury_GetSupplyResponse_get(const google::protobuf::Map<bool, ::treasury::GetSupplyResponse>* m,
                       bool key, const ::treasury::GetSupplyResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::treasury::GetSupplyResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_treasury_GetSupplyResponse_remove(google::protobuf::Map<bool, ::treasury::GetSupplyResponse> * m,
                          bool key, ::treasury::GetSupplyResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_treasury_GetSupplyResponse_iter(
      const google::protobuf::Map<bool, ::treasury::GetSupplyResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_treasury_GetSupplyResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::treasury::GetSupplyResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::treasury::GetSupplyResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::treasury::GetSupplyResponse>* __rust_proto_thunk__Map_ProtoStr_treasury_GetSupplyResponse_new() {
    return new google::protobuf::Map<std::string, ::treasury::GetSupplyResponse>();
  }
  void __rust_proto_thunk__Map_ProtoStr_treasury_GetSupplyResponse_free(const google::protobuf::Map<std::string, ::treasury::GetSupplyResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_treasury_GetSupplyResponse_clear(google::protobuf::Map<std::string, ::treasury::GetSupplyResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_treasury_GetSupplyResponse_size(const google::protobuf::Map<std::string, ::treasury::GetSupplyResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_treasury_GetSupplyResponse_insert(google::protobuf::Map<std::string, ::treasury::GetSupplyResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::treasury::GetSupplyResponse value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_treasury_GetSupplyResponse_get(const google::protobuf::Map<std::string, ::treasury::GetSupplyResponse>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::treasury::GetSupplyResponse** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::treasury::GetSupplyResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_treasury_GetSupplyResponse_remove(google::protobuf::Map<std::string, ::treasury::GetSupplyResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::treasury::GetSupplyResponse * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_treasury_GetSupplyResponse_iter(
      const google::protobuf::Map<std::string, ::treasury::GetSupplyResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_treasury_GetSupplyResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::treasury::GetSupplyResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::treasury::GetSupplyResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// treasury.ListTransactionsRequest
        // clang-format off
extern "C" {
void* __rust_proto_thunk__treasury_ListTransactionsRequest_new() { return new ::treasury::ListTransactionsRequest(); }
void __rust_proto_thunk__treasury_ListTransactionsRequest_delete(void* ptr) { delete static_cast<::treasury::ListTransactionsRequest*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__treasury_ListTransactionsRequest_serialize(::treasury::ListTransactionsRequest* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__treasury_ListTransactionsRequest_parse(::treasury::ListTransactionsRequest* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__treasury_ListTransactionsRequest_copy_from(::treasury::ListTransactionsRequest* dst, const ::treasury::ListTransactionsRequest* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__treasury_ListTransactionsRequest_repeated_len(google::protobuf::RepeatedPtrField<::treasury::ListTransactionsRequest>* field) {
  return field->size();
}
const ::treasury::ListTransactionsRequest& __rust_proto_thunk__treasury_ListTransactionsRequest_repeated_get(
  google::protobuf::RepeatedPtrField<::treasury::ListTransactionsRequest>* field,
  size_t index) {
  return field->Get(index);
}
::treasury::ListTransactionsRequest* __rust_proto_thunk__treasury_ListTransactionsRequest_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::treasury::ListTransactionsRequest>* field,
  size_t index) {
  return field->Mutable(index);
}
::treasury::ListTransactionsRequest* __rust_proto_thunk__treasury_ListTransactionsRequest_repeated_add(google::protobuf::RepeatedPtrField<::treasury::ListTransactionsRequest>* field) {
  return field->Add();
}
void __rust_proto_thunk__treasury_ListTransactionsRequest_repeated_clear(google::protobuf::RepeatedPtrField<::treasury::ListTransactionsRequest>* field) {
  field->Clear();
}
void __rust_proto_thunk__treasury_ListTransactionsRequest_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::treasury::ListTransactionsRequest>& dst,
  const google::protobuf::RepeatedPtrField<::treasury::ListTransactionsRequest>& src) {
  dst = src;
}

::int32_t __rust_proto_thunk__treasury_ListTransactionsRequest_get_page(::treasury::ListTransactionsRequest* msg) { return msg->page(); }
void __rust_proto_thunk__treasury_ListTransactionsRequest_set_page(::treasury::ListTransactionsRequest* msg, ::int32_t val) {
  msg->set_page(val);
}

::int32_t __rust_proto_thunk__treasury_ListTransactionsRequest_get_page_size(::treasury::ListTransactionsRequest* msg) { return msg->page_size(); }
void __rust_proto_thunk__treasury_ListTransactionsRequest_set_page_size(::treasury::ListTransactionsRequest* msg, ::int32_t val) {
  msg->set_page_size(val);
}

const void* __rust_proto_thunk__treasury_ListTransactionsRequest_get_sort_by(::treasury::ListTransactionsRequest* msg) {
  return static_cast<const void*>(&msg->sort_by());
}
void* __rust_proto_thunk__treasury_ListTransactionsRequest_get_mut_sort_by(::treasury::ListTransactionsRequest* msg) {
  return static_cast<void*>(msg->mutable_sort_by());
}
void __rust_proto_thunk__treasury_ListTransactionsRequest_clear_sort_by(::treasury::ListTransactionsRequest* msg) { msg->clear_sort_by(); }
bool __rust_proto_thunk__treasury_ListTransactionsRequest_has_sort_by(::treasury::ListTransactionsRequest* msg) { return msg->has_sort_by(); }

const void* __rust_proto_thunk__treasury_ListTransactionsRequest_get_filters(::treasury::ListTransactionsRequest* msg) {
  return static_cast<const void*>(&msg->filters());
}
void* __rust_proto_thunk__treasury_ListTransactionsRequest_get_mut_filters(::treasury::ListTransactionsRequest* msg) {
  return static_cast<void*>(msg->mutable_filters());
}
void __rust_proto_thunk__treasury_ListTransactionsRequest_clear_filters(::treasury::ListTransactionsRequest* msg) { msg->clear_filters(); }
bool __rust_proto_thunk__treasury_ListTransactionsRequest_has_filters(::treasury::ListTransactionsRequest* msg) { return msg->has_filters(); }


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::treasury::ListTransactionsRequest>* __rust_proto_thunk__Map_i32_treasury_ListTransactionsRequest_new() {
    return new google::protobuf::Map<int32_t, ::treasury::ListTransactionsRequest>();
  }
  void __rust_proto_thunk__Map_i32_treasury_ListTransactionsRequest_free(const google::protobuf::Map<int32_t, ::treasury::ListTransactionsRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_treasury_ListTransactionsRequest_clear(google::protobuf::Map<int32_t, ::treasury::ListTransactionsRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_treasury_ListTransactionsRequest_size(const google::protobuf::Map<int32_t, ::treasury::ListTransactionsRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_treasury_ListTransactionsRequest_insert(google::protobuf::Map<int32_t, ::treasury::ListTransactionsRequest> * m,
                          int32_t key, ::treasury::ListTransactionsRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_treasury_ListTransactionsRequest_get(const google::protobuf::Map<int32_t, ::treasury::ListTransactionsRequest>* m,
                       int32_t key, const ::treasury::ListTransactionsRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::treasury::ListTransactionsRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_treasury_ListTransactionsRequest_remove(google::protobuf::Map<int32_t, ::treasury::ListTransactionsRequest> * m,
                          int32_t key, ::treasury::ListTransactionsRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_treasury_ListTransactionsRequest_iter(
      const google::protobuf::Map<int32_t, ::treasury::ListTransactionsRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_treasury_ListTransactionsRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::treasury::ListTransactionsRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::treasury::ListTransactionsRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::treasury::ListTransactionsRequest>* __rust_proto_thunk__Map_u32_treasury_ListTransactionsRequest_new() {
    return new google::protobuf::Map<uint32_t, ::treasury::ListTransactionsRequest>();
  }
  void __rust_proto_thunk__Map_u32_treasury_ListTransactionsRequest_free(const google::protobuf::Map<uint32_t, ::treasury::ListTransactionsRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_treasury_ListTransactionsRequest_clear(google::protobuf::Map<uint32_t, ::treasury::ListTransactionsRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_treasury_ListTransactionsRequest_size(const google::protobuf::Map<uint32_t, ::treasury::ListTransactionsRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_treasury_ListTransactionsRequest_insert(google::protobuf::Map<uint32_t, ::treasury::ListTransactionsRequest> * m,
                          uint32_t key, ::treasury::ListTransactionsRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_treasury_ListTransactionsRequest_get(const google::protobuf::Map<uint32_t, ::treasury::ListTransactionsRequest>* m,
                       uint32_t key, const ::treasury::ListTransactionsRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::treasury::ListTransactionsRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_treasury_ListTransactionsRequest_remove(google::protobuf::Map<uint32_t, ::treasury::ListTransactionsRequest> * m,
                          uint32_t key, ::treasury::ListTransactionsRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_treasury_ListTransactionsRequest_iter(
      const google::protobuf::Map<uint32_t, ::treasury::ListTransactionsRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_treasury_ListTransactionsRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::treasury::ListTransactionsRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::treasury::ListTransactionsRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::treasury::ListTransactionsRequest>* __rust_proto_thunk__Map_i64_treasury_ListTransactionsRequest_new() {
    return new google::protobuf::Map<int64_t, ::treasury::ListTransactionsRequest>();
  }
  void __rust_proto_thunk__Map_i64_treasury_ListTransactionsRequest_free(const google::protobuf::Map<int64_t, ::treasury::ListTransactionsRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_treasury_ListTransactionsRequest_clear(google::protobuf::Map<int64_t, ::treasury::ListTransactionsRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_treasury_ListTransactionsRequest_size(const google::protobuf::Map<int64_t, ::treasury::ListTransactionsRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_treasury_ListTransactionsRequest_insert(google::protobuf::Map<int64_t, ::treasury::ListTransactionsRequest> * m,
                          int64_t key, ::treasury::ListTransactionsRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_treasury_ListTransactionsRequest_get(const google::protobuf::Map<int64_t, ::treasury::ListTransactionsRequest>* m,
                       int64_t key, const ::treasury::ListTransactionsRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::treasury::ListTransactionsRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_treasury_ListTransactionsRequest_remove(google::protobuf::Map<int64_t, ::treasury::ListTransactionsRequest> * m,
                          int64_t key, ::treasury::ListTransactionsRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_treasury_ListTransactionsRequest_iter(
      const google::protobuf::Map<int64_t, ::treasury::ListTransactionsRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_treasury_ListTransactionsRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::treasury::ListTransactionsRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::treasury::ListTransactionsRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::treasury::ListTransactionsRequest>* __rust_proto_thunk__Map_u64_treasury_ListTransactionsRequest_new() {
    return new google::protobuf::Map<uint64_t, ::treasury::ListTransactionsRequest>();
  }
  void __rust_proto_thunk__Map_u64_treasury_ListTransactionsRequest_free(const google::protobuf::Map<uint64_t, ::treasury::ListTransactionsRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_treasury_ListTransactionsRequest_clear(google::protobuf::Map<uint64_t, ::treasury::ListTransactionsRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_treasury_ListTransactionsRequest_size(const google::protobuf::Map<uint64_t, ::treasury::ListTransactionsRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_treasury_ListTransactionsRequest_insert(google::protobuf::Map<uint64_t, ::treasury::ListTransactionsRequest> * m,
                          uint64_t key, ::treasury::ListTransactionsRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_treasury_ListTransactionsRequest_get(const google::protobuf::Map<uint64_t, ::treasury::ListTransactionsRequest>* m,
                       uint64_t key, const ::treasury::ListTransactionsRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::treasury::ListTransactionsRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_treasury_ListTransactionsRequest_remove(google::protobuf::Map<uint64_t, ::treasury::ListTransactionsRequest> * m,
                          uint64_t key, ::treasury::ListTransactionsRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_treasury_ListTransactionsRequest_iter(
      const google::protobuf::Map<uint64_t, ::treasury::ListTransactionsRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_treasury_ListTransactionsRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::treasury::ListTransactionsRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::treasury::ListTransactionsRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::treasury::ListTransactionsRequest>* __rust_proto_thunk__Map_bool_treasury_ListTransactionsRequest_new() {
    return new google::protobuf::Map<bool, ::treasury::ListTransactionsRequest>();
  }
  void __rust_proto_thunk__Map_bool_treasury_ListTransactionsRequest_free(const google::protobuf::Map<bool, ::treasury::ListTransactionsRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_treasury_ListTransactionsRequest_clear(google::protobuf::Map<bool, ::treasury::ListTransactionsRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_treasury_ListTransactionsRequest_size(const google::protobuf::Map<bool, ::treasury::ListTransactionsRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_treasury_ListTransactionsRequest_insert(google::protobuf::Map<bool, ::treasury::ListTransactionsRequest> * m,
                          bool key, ::treasury::ListTransactionsRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_treasury_ListTransactionsRequest_get(const google::protobuf::Map<bool, ::treasury::ListTransactionsRequest>* m,
                       bool key, const ::treasury::ListTransactionsRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::treasury::ListTransactionsRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_treasury_ListTransactionsRequest_remove(google::protobuf::Map<bool, ::treasury::ListTransactionsRequest> * m,
                          bool key, ::treasury::ListTransactionsRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_treasury_ListTransactionsRequest_iter(
      const google::protobuf::Map<bool, ::treasury::ListTransactionsRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_treasury_ListTransactionsRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::treasury::ListTransactionsRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::treasury::ListTransactionsRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::treasury::ListTransactionsRequest>* __rust_proto_thunk__Map_ProtoStr_treasury_ListTransactionsRequest_new() {
    return new google::protobuf::Map<std::string, ::treasury::ListTransactionsRequest>();
  }
  void __rust_proto_thunk__Map_ProtoStr_treasury_ListTransactionsRequest_free(const google::protobuf::Map<std::string, ::treasury::ListTransactionsRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_treasury_ListTransactionsRequest_clear(google::protobuf::Map<std::string, ::treasury::ListTransactionsRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_treasury_ListTransactionsRequest_size(const google::protobuf::Map<std::string, ::treasury::ListTransactionsRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_treasury_ListTransactionsRequest_insert(google::protobuf::Map<std::string, ::treasury::ListTransactionsRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::treasury::ListTransactionsRequest value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_treasury_ListTransactionsRequest_get(const google::protobuf::Map<std::string, ::treasury::ListTransactionsRequest>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::treasury::ListTransactionsRequest** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::treasury::ListTransactionsRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_treasury_ListTransactionsRequest_remove(google::protobuf::Map<std::string, ::treasury::ListTransactionsRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::treasury::ListTransactionsRequest * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_treasury_ListTransactionsRequest_iter(
      const google::protobuf::Map<std::string, ::treasury::ListTransactionsRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_treasury_ListTransactionsRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::treasury::ListTransactionsRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::treasury::ListTransactionsRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// treasury.ContractSummary
        // clang-format off
extern "C" {
void* __rust_proto_thunk__treasury_ContractSummary_new() { return new ::treasury::ContractSummary(); }
void __rust_proto_thunk__treasury_ContractSummary_delete(void* ptr) { delete static_cast<::treasury::ContractSummary*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__treasury_ContractSummary_serialize(::treasury::ContractSummary* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__treasury_ContractSummary_parse(::treasury::ContractSummary* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__treasury_ContractSummary_copy_from(::treasury::ContractSummary* dst, const ::treasury::ContractSummary* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__treasury_ContractSummary_repeated_len(google::protobuf::RepeatedPtrField<::treasury::ContractSummary>* field) {
  return field->size();
}
const ::treasury::ContractSummary& __rust_proto_thunk__treasury_ContractSummary_repeated_get(
  google::protobuf::RepeatedPtrField<::treasury::ContractSummary>* field,
  size_t index) {
  return field->Get(index);
}
::treasury::ContractSummary* __rust_proto_thunk__treasury_ContractSummary_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::treasury::ContractSummary>* field,
  size_t index) {
  return field->Mutable(index);
}
::treasury::ContractSummary* __rust_proto_thunk__treasury_ContractSummary_repeated_add(google::protobuf::RepeatedPtrField<::treasury::ContractSummary>* field) {
  return field->Add();
}
void __rust_proto_thunk__treasury_ContractSummary_repeated_clear(google::protobuf::RepeatedPtrField<::treasury::ContractSummary>* field) {
  field->Clear();
}
void __rust_proto_thunk__treasury_ContractSummary_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::treasury::ContractSummary>& dst,
  const google::protobuf::RepeatedPtrField<::treasury::ContractSummary>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__treasury_ContractSummary_get_contract_id(::treasury::ContractSummary* msg) {
  absl::string_view val = msg->contract_id();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__treasury_ContractSummary_set_contract_id(::treasury::ContractSummary* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_contract_id(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__treasury_ContractSummary_get_contract_type(::treasury::ContractSummary* msg) {
  absl::string_view val = msg->contract_type();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__treasury_ContractSummary_set_contract_type(::treasury::ContractSummary* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_contract_type(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__treasury_ContractSummary_get_created(::treasury::ContractSummary* msg) {
  absl::string_view val = msg->created();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__treasury_ContractSummary_set_created(::treasury::ContractSummary* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_created(absl::string_view(s.ptr, s.len));
}

void __rust_proto_thunk__treasury_ContractSummary_clear_counterparties(::treasury::ContractSummary* msg) {
  msg->clear_counterparties();
}
google::protobuf::RepeatedPtrField<std::string>* __rust_proto_thunk__treasury_ContractSummary_get_mut_counterparties(::treasury::ContractSummary* msg) {
  return msg->mutable_counterparties();
}
const google::protobuf::RepeatedPtrField<std::string>* __rust_proto_thunk__treasury_ContractSummary_get_counterparties(
    const ::treasury::ContractSummary* msg) {
  return &msg->counterparties();
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__treasury_ContractSummary_get_memo(::treasury::ContractSummary* msg) {
  absl::string_view val = msg->memo();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__treasury_ContractSummary_set_memo(::treasury::ContractSummary* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_memo(absl::string_view(s.ptr, s.len));
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::treasury::ContractSummary>* __rust_proto_thunk__Map_i32_treasury_ContractSummary_new() {
    return new google::protobuf::Map<int32_t, ::treasury::ContractSummary>();
  }
  void __rust_proto_thunk__Map_i32_treasury_ContractSummary_free(const google::protobuf::Map<int32_t, ::treasury::ContractSummary>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_treasury_ContractSummary_clear(google::protobuf::Map<int32_t, ::treasury::ContractSummary> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_treasury_ContractSummary_size(const google::protobuf::Map<int32_t, ::treasury::ContractSummary>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_treasury_ContractSummary_insert(google::protobuf::Map<int32_t, ::treasury::ContractSummary> * m,
                          int32_t key, ::treasury::ContractSummary value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_treasury_ContractSummary_get(const google::protobuf::Map<int32_t, ::treasury::ContractSummary>* m,
                       int32_t key, const ::treasury::ContractSummary** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::treasury::ContractSummary* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_treasury_ContractSummary_remove(google::protobuf::Map<int32_t, ::treasury::ContractSummary> * m,
                          int32_t key, ::treasury::ContractSummary * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_treasury_ContractSummary_iter(
      const google::protobuf::Map<int32_t, ::treasury::ContractSummary>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_treasury_ContractSummary_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::treasury::ContractSummary** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::treasury::ContractSummary>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::treasury::ContractSummary>* __rust_proto_thunk__Map_u32_treasury_ContractSummary_new() {
    return new google::protobuf::Map<uint32_t, ::treasury::ContractSummary>();
  }
  void __rust_proto_thunk__Map_u32_treasury_ContractSummary_free(const google::protobuf::Map<uint32_t, ::treasury::ContractSummary>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_treasury_ContractSummary_clear(google::protobuf::Map<uint32_t, ::treasury::ContractSummary> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_treasury_ContractSummary_size(const google::protobuf::Map<uint32_t, ::treasury::ContractSummary>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_treasury_ContractSummary_insert(google::protobuf::Map<uint32_t, ::treasury::ContractSummary> * m,
                          uint32_t key, ::treasury::ContractSummary value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_treasury_ContractSummary_get(const google::protobuf::Map<uint32_t, ::treasury::ContractSummary>* m,
                       uint32_t key, const ::treasury::ContractSummary** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::treasury::ContractSummary* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_treasury_ContractSummary_remove(google::protobuf::Map<uint32_t, ::treasury::ContractSummary> * m,
                          uint32_t key, ::treasury::ContractSummary * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_treasury_ContractSummary_iter(
      const google::protobuf::Map<uint32_t, ::treasury::ContractSummary>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_treasury_ContractSummary_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::treasury::ContractSummary** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::treasury::ContractSummary>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::treasury::ContractSummary>* __rust_proto_thunk__Map_i64_treasury_ContractSummary_new() {
    return new google::protobuf::Map<int64_t, ::treasury::ContractSummary>();
  }
  void __rust_proto_thunk__Map_i64_treasury_ContractSummary_free(const google::protobuf::Map<int64_t, ::treasury::ContractSummary>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_treasury_ContractSummary_clear(google::protobuf::Map<int64_t, ::treasury::ContractSummary> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_treasury_ContractSummary_size(const google::protobuf::Map<int64_t, ::treasury::ContractSummary>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_treasury_ContractSummary_insert(google::protobuf::Map<int64_t, ::treasury::ContractSummary> * m,
                          int64_t key, ::treasury::ContractSummary value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_treasury_ContractSummary_get(const google::protobuf::Map<int64_t, ::treasury::ContractSummary>* m,
                       int64_t key, const ::treasury::ContractSummary** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::treasury::ContractSummary* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_treasury_ContractSummary_remove(google::protobuf::Map<int64_t, ::treasury::ContractSummary> * m,
                          int64_t key, ::treasury::ContractSummary * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_treasury_ContractSummary_iter(
      const google::protobuf::Map<int64_t, ::treasury::ContractSummary>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_treasury_ContractSummary_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::treasury::ContractSummary** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::treasury::ContractSummary>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::treasury::ContractSummary>* __rust_proto_thunk__Map_u64_treasury_ContractSummary_new() {
    return new google::protobuf::Map<uint64_t, ::treasury::ContractSummary>();
  }
  void __rust_proto_thunk__Map_u64_treasury_ContractSummary_free(const google::protobuf::Map<uint64_t, ::treasury::ContractSummary>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_treasury_ContractSummary_clear(google::protobuf::Map<uint64_t, ::treasury::ContractSummary> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_treasury_ContractSummary_size(const google::protobuf::Map<uint64_t, ::treasury::ContractSummary>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_treasury_ContractSummary_insert(google::protobuf::Map<uint64_t, ::treasury::ContractSummary> * m,
                          uint64_t key, ::treasury::ContractSummary value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_treasury_ContractSummary_get(const google::protobuf::Map<uint64_t, ::treasury::ContractSummary>* m,
                       uint64_t key, const ::treasury::ContractSummary** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::treasury::ContractSummary* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_treasury_ContractSummary_remove(google::protobuf::Map<uint64_t, ::treasury::ContractSummary> * m,
                          uint64_t key, ::treasury::ContractSummary * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_treasury_ContractSummary_iter(
      const google::protobuf::Map<uint64_t, ::treasury::ContractSummary>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_treasury_ContractSummary_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::treasury::ContractSummary** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::treasury::ContractSummary>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::treasury::ContractSummary>* __rust_proto_thunk__Map_bool_treasury_ContractSummary_new() {
    return new google::protobuf::Map<bool, ::treasury::ContractSummary>();
  }
  void __rust_proto_thunk__Map_bool_treasury_ContractSummary_free(const google::protobuf::Map<bool, ::treasury::ContractSummary>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_treasury_ContractSummary_clear(google::protobuf::Map<bool, ::treasury::ContractSummary> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_treasury_ContractSummary_size(const google::protobuf::Map<bool, ::treasury::ContractSummary>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_treasury_ContractSummary_insert(google::protobuf::Map<bool, ::treasury::ContractSummary> * m,
                          bool key, ::treasury::ContractSummary value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_treasury_ContractSummary_get(const google::protobuf::Map<bool, ::treasury::ContractSummary>* m,
                       bool key, const ::treasury::ContractSummary** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::treasury::ContractSummary* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_treasury_ContractSummary_remove(google::protobuf::Map<bool, ::treasury::ContractSummary> * m,
                          bool key, ::treasury::ContractSummary * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_treasury_ContractSummary_iter(
      const google::protobuf::Map<bool, ::treasury::ContractSummary>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_treasury_ContractSummary_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::treasury::ContractSummary** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::treasury::ContractSummary>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::treasury::ContractSummary>* __rust_proto_thunk__Map_ProtoStr_treasury_ContractSummary_new() {
    return new google::protobuf::Map<std::string, ::treasury::ContractSummary>();
  }
  void __rust_proto_thunk__Map_ProtoStr_treasury_ContractSummary_free(const google::protobuf::Map<std::string, ::treasury::ContractSummary>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_treasury_ContractSummary_clear(google::protobuf::Map<std::string, ::treasury::ContractSummary> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_treasury_ContractSummary_size(const google::protobuf::Map<std::string, ::treasury::ContractSummary>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_treasury_ContractSummary_insert(google::protobuf::Map<std::string, ::treasury::ContractSummary> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::treasury::ContractSummary value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_treasury_ContractSummary_get(const google::protobuf::Map<std::string, ::treasury::ContractSummary>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::treasury::ContractSummary** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::treasury::ContractSummary* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_treasury_ContractSummary_remove(google::protobuf::Map<std::string, ::treasury::ContractSummary> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::treasury::ContractSummary * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_treasury_ContractSummary_iter(
      const google::protobuf::Map<std::string, ::treasury::ContractSummary>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_treasury_ContractSummary_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::treasury::ContractSummary** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::treasury::ContractSummary>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// treasury.ListTransactionsResponse
        // clang-format off
extern "C" {
void* __rust_proto_thunk__treasury_ListTransactionsResponse_new() { return new ::treasury::ListTransactionsResponse(); }
void __rust_proto_thunk__treasury_ListTransactionsResponse_delete(void* ptr) { delete static_cast<::treasury::ListTransactionsResponse*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__treasury_ListTransactionsResponse_serialize(::treasury::ListTransactionsResponse* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__treasury_ListTransactionsResponse_parse(::treasury::ListTransactionsResponse* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__treasury_ListTransactionsResponse_copy_from(::treasury::ListTransactionsResponse* dst, const ::treasury::ListTransactionsResponse* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__treasury_ListTransactionsResponse_repeated_len(google::protobuf::RepeatedPtrField<::treasury::ListTransactionsResponse>* field) {
  return field->size();
}
const ::treasury::ListTransactionsResponse& __rust_proto_thunk__treasury_ListTransactionsResponse_repeated_get(
  google::protobuf::RepeatedPtrField<::treasury::ListTransactionsResponse>* field,
  size_t index) {
  return field->Get(index);
}
::treasury::ListTransactionsResponse* __rust_proto_thunk__treasury_ListTransactionsResponse_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::treasury::ListTransactionsResponse>* field,
  size_t index) {
  return field->Mutable(index);
}
::treasury::ListTransactionsResponse* __rust_proto_thunk__treasury_ListTransactionsResponse_repeated_add(google::protobuf::RepeatedPtrField<::treasury::ListTransactionsResponse>* field) {
  return field->Add();
}
void __rust_proto_thunk__treasury_ListTransactionsResponse_repeated_clear(google::protobuf::RepeatedPtrField<::treasury::ListTransactionsResponse>* field) {
  field->Clear();
}
void __rust_proto_thunk__treasury_ListTransactionsResponse_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::treasury::ListTransactionsResponse>& dst,
  const google::protobuf::RepeatedPtrField<::treasury::ListTransactionsResponse>& src) {
  dst = src;
}

void __rust_proto_thunk__treasury_ListTransactionsResponse_clear_contracts(::treasury::ListTransactionsResponse* msg) {
  msg->clear_contracts();
}
google::protobuf::RepeatedPtrField<::treasury::ContractSummary>* __rust_proto_thunk__treasury_ListTransactionsResponse_get_mut_contracts(::treasury::ListTransactionsResponse* msg) {
  return msg->mutable_contracts();
}
const google::protobuf::RepeatedPtrField<::treasury::ContractSummary>* __rust_proto_thunk__treasury_ListTransactionsResponse_get_contracts(
    const ::treasury::ListTransactionsResponse* msg) {
  return &msg->contracts();
}

::uint32_t __rust_proto_thunk__treasury_ListTransactionsResponse_get_totalContracts(::treasury::ListTransactionsResponse* msg) { return msg->totalcontracts(); }
void __rust_proto_thunk__treasury_ListTransactionsResponse_set_totalContracts(::treasury::ListTransactionsResponse* msg, ::uint32_t val) {
  msg->set_totalcontracts(val);
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::treasury::ListTransactionsResponse>* __rust_proto_thunk__Map_i32_treasury_ListTransactionsResponse_new() {
    return new google::protobuf::Map<int32_t, ::treasury::ListTransactionsResponse>();
  }
  void __rust_proto_thunk__Map_i32_treasury_ListTransactionsResponse_free(const google::protobuf::Map<int32_t, ::treasury::ListTransactionsResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_treasury_ListTransactionsResponse_clear(google::protobuf::Map<int32_t, ::treasury::ListTransactionsResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_treasury_ListTransactionsResponse_size(const google::protobuf::Map<int32_t, ::treasury::ListTransactionsResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_treasury_ListTransactionsResponse_insert(google::protobuf::Map<int32_t, ::treasury::ListTransactionsResponse> * m,
                          int32_t key, ::treasury::ListTransactionsResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_treasury_ListTransactionsResponse_get(const google::protobuf::Map<int32_t, ::treasury::ListTransactionsResponse>* m,
                       int32_t key, const ::treasury::ListTransactionsResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::treasury::ListTransactionsResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_treasury_ListTransactionsResponse_remove(google::protobuf::Map<int32_t, ::treasury::ListTransactionsResponse> * m,
                          int32_t key, ::treasury::ListTransactionsResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_treasury_ListTransactionsResponse_iter(
      const google::protobuf::Map<int32_t, ::treasury::ListTransactionsResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_treasury_ListTransactionsResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::treasury::ListTransactionsResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::treasury::ListTransactionsResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::treasury::ListTransactionsResponse>* __rust_proto_thunk__Map_u32_treasury_ListTransactionsResponse_new() {
    return new google::protobuf::Map<uint32_t, ::treasury::ListTransactionsResponse>();
  }
  void __rust_proto_thunk__Map_u32_treasury_ListTransactionsResponse_free(const google::protobuf::Map<uint32_t, ::treasury::ListTransactionsResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_treasury_ListTransactionsResponse_clear(google::protobuf::Map<uint32_t, ::treasury::ListTransactionsResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_treasury_ListTransactionsResponse_size(const google::protobuf::Map<uint32_t, ::treasury::ListTransactionsResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_treasury_ListTransactionsResponse_insert(google::protobuf::Map<uint32_t, ::treasury::ListTransactionsResponse> * m,
                          uint32_t key, ::treasury::ListTransactionsResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_treasury_ListTransactionsResponse_get(const google::protobuf::Map<uint32_t, ::treasury::ListTransactionsResponse>* m,
                       uint32_t key, const ::treasury::ListTransactionsResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::treasury::ListTransactionsResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_treasury_ListTransactionsResponse_remove(google::protobuf::Map<uint32_t, ::treasury::ListTransactionsResponse> * m,
                          uint32_t key, ::treasury::ListTransactionsResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_treasury_ListTransactionsResponse_iter(
      const google::protobuf::Map<uint32_t, ::treasury::ListTransactionsResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_treasury_ListTransactionsResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::treasury::ListTransactionsResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::treasury::ListTransactionsResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::treasury::ListTransactionsResponse>* __rust_proto_thunk__Map_i64_treasury_ListTransactionsResponse_new() {
    return new google::protobuf::Map<int64_t, ::treasury::ListTransactionsResponse>();
  }
  void __rust_proto_thunk__Map_i64_treasury_ListTransactionsResponse_free(const google::protobuf::Map<int64_t, ::treasury::ListTransactionsResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_treasury_ListTransactionsResponse_clear(google::protobuf::Map<int64_t, ::treasury::ListTransactionsResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_treasury_ListTransactionsResponse_size(const google::protobuf::Map<int64_t, ::treasury::ListTransactionsResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_treasury_ListTransactionsResponse_insert(google::protobuf::Map<int64_t, ::treasury::ListTransactionsResponse> * m,
                          int64_t key, ::treasury::ListTransactionsResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_treasury_ListTransactionsResponse_get(const google::protobuf::Map<int64_t, ::treasury::ListTransactionsResponse>* m,
                       int64_t key, const ::treasury::ListTransactionsResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::treasury::ListTransactionsResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_treasury_ListTransactionsResponse_remove(google::protobuf::Map<int64_t, ::treasury::ListTransactionsResponse> * m,
                          int64_t key, ::treasury::ListTransactionsResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_treasury_ListTransactionsResponse_iter(
      const google::protobuf::Map<int64_t, ::treasury::ListTransactionsResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_treasury_ListTransactionsResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::treasury::ListTransactionsResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::treasury::ListTransactionsResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::treasury::ListTransactionsResponse>* __rust_proto_thunk__Map_u64_treasury_ListTransactionsResponse_new() {
    return new google::protobuf::Map<uint64_t, ::treasury::ListTransactionsResponse>();
  }
  void __rust_proto_thunk__Map_u64_treasury_ListTransactionsResponse_free(const google::protobuf::Map<uint64_t, ::treasury::ListTransactionsResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_treasury_ListTransactionsResponse_clear(google::protobuf::Map<uint64_t, ::treasury::ListTransactionsResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_treasury_ListTransactionsResponse_size(const google::protobuf::Map<uint64_t, ::treasury::ListTransactionsResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_treasury_ListTransactionsResponse_insert(google::protobuf::Map<uint64_t, ::treasury::ListTransactionsResponse> * m,
                          uint64_t key, ::treasury::ListTransactionsResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_treasury_ListTransactionsResponse_get(const google::protobuf::Map<uint64_t, ::treasury::ListTransactionsResponse>* m,
                       uint64_t key, const ::treasury::ListTransactionsResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::treasury::ListTransactionsResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_treasury_ListTransactionsResponse_remove(google::protobuf::Map<uint64_t, ::treasury::ListTransactionsResponse> * m,
                          uint64_t key, ::treasury::ListTransactionsResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_treasury_ListTransactionsResponse_iter(
      const google::protobuf::Map<uint64_t, ::treasury::ListTransactionsResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_treasury_ListTransactionsResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::treasury::ListTransactionsResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::treasury::ListTransactionsResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::treasury::ListTransactionsResponse>* __rust_proto_thunk__Map_bool_treasury_ListTransactionsResponse_new() {
    return new google::protobuf::Map<bool, ::treasury::ListTransactionsResponse>();
  }
  void __rust_proto_thunk__Map_bool_treasury_ListTransactionsResponse_free(const google::protobuf::Map<bool, ::treasury::ListTransactionsResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_treasury_ListTransactionsResponse_clear(google::protobuf::Map<bool, ::treasury::ListTransactionsResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_treasury_ListTransactionsResponse_size(const google::protobuf::Map<bool, ::treasury::ListTransactionsResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_treasury_ListTransactionsResponse_insert(google::protobuf::Map<bool, ::treasury::ListTransactionsResponse> * m,
                          bool key, ::treasury::ListTransactionsResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_treasury_ListTransactionsResponse_get(const google::protobuf::Map<bool, ::treasury::ListTransactionsResponse>* m,
                       bool key, const ::treasury::ListTransactionsResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::treasury::ListTransactionsResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_treasury_ListTransactionsResponse_remove(google::protobuf::Map<bool, ::treasury::ListTransactionsResponse> * m,
                          bool key, ::treasury::ListTransactionsResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_treasury_ListTransactionsResponse_iter(
      const google::protobuf::Map<bool, ::treasury::ListTransactionsResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_treasury_ListTransactionsResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::treasury::ListTransactionsResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::treasury::ListTransactionsResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::treasury::ListTransactionsResponse>* __rust_proto_thunk__Map_ProtoStr_treasury_ListTransactionsResponse_new() {
    return new google::protobuf::Map<std::string, ::treasury::ListTransactionsResponse>();
  }
  void __rust_proto_thunk__Map_ProtoStr_treasury_ListTransactionsResponse_free(const google::protobuf::Map<std::string, ::treasury::ListTransactionsResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_treasury_ListTransactionsResponse_clear(google::protobuf::Map<std::string, ::treasury::ListTransactionsResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_treasury_ListTransactionsResponse_size(const google::protobuf::Map<std::string, ::treasury::ListTransactionsResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_treasury_ListTransactionsResponse_insert(google::protobuf::Map<std::string, ::treasury::ListTransactionsResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::treasury::ListTransactionsResponse value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_treasury_ListTransactionsResponse_get(const google::protobuf::Map<std::string, ::treasury::ListTransactionsResponse>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::treasury::ListTransactionsResponse** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::treasury::ListTransactionsResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_treasury_ListTransactionsResponse_remove(google::protobuf::Map<std::string, ::treasury::ListTransactionsResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::treasury::ListTransactionsResponse * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_treasury_ListTransactionsResponse_iter(
      const google::protobuf::Map<std::string, ::treasury::ListTransactionsResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_treasury_ListTransactionsResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::treasury::ListTransactionsResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::treasury::ListTransactionsResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

