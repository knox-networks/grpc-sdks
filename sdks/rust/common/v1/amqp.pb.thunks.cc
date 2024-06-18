#include "common/v1/amqp.proto.h"
#include "google/protobuf/rust/cpp_kernel/cpp_api.h"
// common.Payload
        // clang-format off
extern "C" {
void* __rust_proto_thunk__common_Payload_new() { return new ::common::Payload(); }
void __rust_proto_thunk__common_Payload_delete(void* ptr) { delete static_cast<::common::Payload*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__common_Payload_serialize(::common::Payload* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__common_Payload_parse(::common::Payload* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__common_Payload_copy_from(::common::Payload* dst, const ::common::Payload* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__common_Payload_repeated_len(google::protobuf::RepeatedPtrField<::common::Payload>* field) {
  return field->size();
}
const ::common::Payload& __rust_proto_thunk__common_Payload_repeated_get(
  google::protobuf::RepeatedPtrField<::common::Payload>* field,
  size_t index) {
  return field->Get(index);
}
::common::Payload* __rust_proto_thunk__common_Payload_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::common::Payload>* field,
  size_t index) {
  return field->Mutable(index);
}
::common::Payload* __rust_proto_thunk__common_Payload_repeated_add(google::protobuf::RepeatedPtrField<::common::Payload>* field) {
  return field->Add();
}
void __rust_proto_thunk__common_Payload_repeated_clear(google::protobuf::RepeatedPtrField<::common::Payload>* field) {
  field->Clear();
}
void __rust_proto_thunk__common_Payload_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::common::Payload>& dst,
  const google::protobuf::RepeatedPtrField<::common::Payload>& src) {
  dst = src;
}

::common::PayloadType __rust_proto_thunk__common_Payload_get_payload_type(::common::Payload* msg) { return msg->payload_type(); }
void __rust_proto_thunk__common_Payload_set_payload_type(::common::Payload* msg, ::common::PayloadType val) {
  msg->set_payload_type(val);
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__common_Payload_get_data(::common::Payload* msg) {
  absl::string_view val = msg->data();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__common_Payload_set_data(::common::Payload* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_data(absl::string_view(s.ptr, s.len));
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::common::Payload>* __rust_proto_thunk__Map_i32_common_Payload_new() {
    return new google::protobuf::Map<int32_t, ::common::Payload>();
  }
  void __rust_proto_thunk__Map_i32_common_Payload_free(const google::protobuf::Map<int32_t, ::common::Payload>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_common_Payload_clear(google::protobuf::Map<int32_t, ::common::Payload> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_common_Payload_size(const google::protobuf::Map<int32_t, ::common::Payload>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_common_Payload_insert(google::protobuf::Map<int32_t, ::common::Payload> * m,
                          int32_t key, ::common::Payload value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_Payload_get(const google::protobuf::Map<int32_t, ::common::Payload>* m,
                       int32_t key, const ::common::Payload** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::Payload* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_Payload_remove(google::protobuf::Map<int32_t, ::common::Payload> * m,
                          int32_t key, ::common::Payload * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_common_Payload_iter(
      const google::protobuf::Map<int32_t, ::common::Payload>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_common_Payload_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::common::Payload** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::common::Payload>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::common::Payload>* __rust_proto_thunk__Map_u32_common_Payload_new() {
    return new google::protobuf::Map<uint32_t, ::common::Payload>();
  }
  void __rust_proto_thunk__Map_u32_common_Payload_free(const google::protobuf::Map<uint32_t, ::common::Payload>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_common_Payload_clear(google::protobuf::Map<uint32_t, ::common::Payload> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_common_Payload_size(const google::protobuf::Map<uint32_t, ::common::Payload>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_common_Payload_insert(google::protobuf::Map<uint32_t, ::common::Payload> * m,
                          uint32_t key, ::common::Payload value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_Payload_get(const google::protobuf::Map<uint32_t, ::common::Payload>* m,
                       uint32_t key, const ::common::Payload** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::Payload* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_Payload_remove(google::protobuf::Map<uint32_t, ::common::Payload> * m,
                          uint32_t key, ::common::Payload * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_common_Payload_iter(
      const google::protobuf::Map<uint32_t, ::common::Payload>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_common_Payload_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::common::Payload** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::common::Payload>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::common::Payload>* __rust_proto_thunk__Map_i64_common_Payload_new() {
    return new google::protobuf::Map<int64_t, ::common::Payload>();
  }
  void __rust_proto_thunk__Map_i64_common_Payload_free(const google::protobuf::Map<int64_t, ::common::Payload>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_common_Payload_clear(google::protobuf::Map<int64_t, ::common::Payload> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_common_Payload_size(const google::protobuf::Map<int64_t, ::common::Payload>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_common_Payload_insert(google::protobuf::Map<int64_t, ::common::Payload> * m,
                          int64_t key, ::common::Payload value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_Payload_get(const google::protobuf::Map<int64_t, ::common::Payload>* m,
                       int64_t key, const ::common::Payload** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::Payload* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_Payload_remove(google::protobuf::Map<int64_t, ::common::Payload> * m,
                          int64_t key, ::common::Payload * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_common_Payload_iter(
      const google::protobuf::Map<int64_t, ::common::Payload>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_common_Payload_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::common::Payload** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::common::Payload>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::common::Payload>* __rust_proto_thunk__Map_u64_common_Payload_new() {
    return new google::protobuf::Map<uint64_t, ::common::Payload>();
  }
  void __rust_proto_thunk__Map_u64_common_Payload_free(const google::protobuf::Map<uint64_t, ::common::Payload>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_common_Payload_clear(google::protobuf::Map<uint64_t, ::common::Payload> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_common_Payload_size(const google::protobuf::Map<uint64_t, ::common::Payload>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_common_Payload_insert(google::protobuf::Map<uint64_t, ::common::Payload> * m,
                          uint64_t key, ::common::Payload value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_Payload_get(const google::protobuf::Map<uint64_t, ::common::Payload>* m,
                       uint64_t key, const ::common::Payload** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::Payload* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_Payload_remove(google::protobuf::Map<uint64_t, ::common::Payload> * m,
                          uint64_t key, ::common::Payload * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_common_Payload_iter(
      const google::protobuf::Map<uint64_t, ::common::Payload>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_common_Payload_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::common::Payload** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::common::Payload>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::common::Payload>* __rust_proto_thunk__Map_bool_common_Payload_new() {
    return new google::protobuf::Map<bool, ::common::Payload>();
  }
  void __rust_proto_thunk__Map_bool_common_Payload_free(const google::protobuf::Map<bool, ::common::Payload>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_common_Payload_clear(google::protobuf::Map<bool, ::common::Payload> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_common_Payload_size(const google::protobuf::Map<bool, ::common::Payload>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_common_Payload_insert(google::protobuf::Map<bool, ::common::Payload> * m,
                          bool key, ::common::Payload value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_Payload_get(const google::protobuf::Map<bool, ::common::Payload>* m,
                       bool key, const ::common::Payload** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::Payload* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_Payload_remove(google::protobuf::Map<bool, ::common::Payload> * m,
                          bool key, ::common::Payload * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_common_Payload_iter(
      const google::protobuf::Map<bool, ::common::Payload>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_common_Payload_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::common::Payload** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::common::Payload>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::common::Payload>* __rust_proto_thunk__Map_ProtoStr_common_Payload_new() {
    return new google::protobuf::Map<std::string, ::common::Payload>();
  }
  void __rust_proto_thunk__Map_ProtoStr_common_Payload_free(const google::protobuf::Map<std::string, ::common::Payload>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_common_Payload_clear(google::protobuf::Map<std::string, ::common::Payload> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_common_Payload_size(const google::protobuf::Map<std::string, ::common::Payload>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_Payload_insert(google::protobuf::Map<std::string, ::common::Payload> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::Payload value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_Payload_get(const google::protobuf::Map<std::string, ::common::Payload>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::common::Payload** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::common::Payload* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_Payload_remove(google::protobuf::Map<std::string, ::common::Payload> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::Payload * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_common_Payload_iter(
      const google::protobuf::Map<std::string, ::common::Payload>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_common_Payload_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::common::Payload** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::common::Payload>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// common.NewTransactionData
        // clang-format off
extern "C" {
void* __rust_proto_thunk__common_NewTransactionData_new() { return new ::common::NewTransactionData(); }
void __rust_proto_thunk__common_NewTransactionData_delete(void* ptr) { delete static_cast<::common::NewTransactionData*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__common_NewTransactionData_serialize(::common::NewTransactionData* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__common_NewTransactionData_parse(::common::NewTransactionData* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__common_NewTransactionData_copy_from(::common::NewTransactionData* dst, const ::common::NewTransactionData* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__common_NewTransactionData_repeated_len(google::protobuf::RepeatedPtrField<::common::NewTransactionData>* field) {
  return field->size();
}
const ::common::NewTransactionData& __rust_proto_thunk__common_NewTransactionData_repeated_get(
  google::protobuf::RepeatedPtrField<::common::NewTransactionData>* field,
  size_t index) {
  return field->Get(index);
}
::common::NewTransactionData* __rust_proto_thunk__common_NewTransactionData_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::common::NewTransactionData>* field,
  size_t index) {
  return field->Mutable(index);
}
::common::NewTransactionData* __rust_proto_thunk__common_NewTransactionData_repeated_add(google::protobuf::RepeatedPtrField<::common::NewTransactionData>* field) {
  return field->Add();
}
void __rust_proto_thunk__common_NewTransactionData_repeated_clear(google::protobuf::RepeatedPtrField<::common::NewTransactionData>* field) {
  field->Clear();
}
void __rust_proto_thunk__common_NewTransactionData_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::common::NewTransactionData>& dst,
  const google::protobuf::RepeatedPtrField<::common::NewTransactionData>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__common_NewTransactionData_get_participant(::common::NewTransactionData* msg) {
  absl::string_view val = msg->participant();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__common_NewTransactionData_set_participant(::common::NewTransactionData* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_participant(absl::string_view(s.ptr, s.len));
}

const void* __rust_proto_thunk__common_NewTransactionData_get_contract(::common::NewTransactionData* msg) {
  return static_cast<const void*>(&msg->contract());
}
void* __rust_proto_thunk__common_NewTransactionData_get_mut_contract(::common::NewTransactionData* msg) {
  return static_cast<void*>(msg->mutable_contract());
}
void __rust_proto_thunk__common_NewTransactionData_clear_contract(::common::NewTransactionData* msg) { msg->clear_contract(); }
bool __rust_proto_thunk__common_NewTransactionData_has_contract(::common::NewTransactionData* msg) { return msg->has_contract(); }


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::common::NewTransactionData>* __rust_proto_thunk__Map_i32_common_NewTransactionData_new() {
    return new google::protobuf::Map<int32_t, ::common::NewTransactionData>();
  }
  void __rust_proto_thunk__Map_i32_common_NewTransactionData_free(const google::protobuf::Map<int32_t, ::common::NewTransactionData>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_common_NewTransactionData_clear(google::protobuf::Map<int32_t, ::common::NewTransactionData> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_common_NewTransactionData_size(const google::protobuf::Map<int32_t, ::common::NewTransactionData>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_common_NewTransactionData_insert(google::protobuf::Map<int32_t, ::common::NewTransactionData> * m,
                          int32_t key, ::common::NewTransactionData value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_NewTransactionData_get(const google::protobuf::Map<int32_t, ::common::NewTransactionData>* m,
                       int32_t key, const ::common::NewTransactionData** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::NewTransactionData* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_NewTransactionData_remove(google::protobuf::Map<int32_t, ::common::NewTransactionData> * m,
                          int32_t key, ::common::NewTransactionData * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_common_NewTransactionData_iter(
      const google::protobuf::Map<int32_t, ::common::NewTransactionData>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_common_NewTransactionData_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::common::NewTransactionData** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::common::NewTransactionData>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::common::NewTransactionData>* __rust_proto_thunk__Map_u32_common_NewTransactionData_new() {
    return new google::protobuf::Map<uint32_t, ::common::NewTransactionData>();
  }
  void __rust_proto_thunk__Map_u32_common_NewTransactionData_free(const google::protobuf::Map<uint32_t, ::common::NewTransactionData>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_common_NewTransactionData_clear(google::protobuf::Map<uint32_t, ::common::NewTransactionData> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_common_NewTransactionData_size(const google::protobuf::Map<uint32_t, ::common::NewTransactionData>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_common_NewTransactionData_insert(google::protobuf::Map<uint32_t, ::common::NewTransactionData> * m,
                          uint32_t key, ::common::NewTransactionData value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_NewTransactionData_get(const google::protobuf::Map<uint32_t, ::common::NewTransactionData>* m,
                       uint32_t key, const ::common::NewTransactionData** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::NewTransactionData* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_NewTransactionData_remove(google::protobuf::Map<uint32_t, ::common::NewTransactionData> * m,
                          uint32_t key, ::common::NewTransactionData * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_common_NewTransactionData_iter(
      const google::protobuf::Map<uint32_t, ::common::NewTransactionData>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_common_NewTransactionData_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::common::NewTransactionData** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::common::NewTransactionData>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::common::NewTransactionData>* __rust_proto_thunk__Map_i64_common_NewTransactionData_new() {
    return new google::protobuf::Map<int64_t, ::common::NewTransactionData>();
  }
  void __rust_proto_thunk__Map_i64_common_NewTransactionData_free(const google::protobuf::Map<int64_t, ::common::NewTransactionData>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_common_NewTransactionData_clear(google::protobuf::Map<int64_t, ::common::NewTransactionData> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_common_NewTransactionData_size(const google::protobuf::Map<int64_t, ::common::NewTransactionData>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_common_NewTransactionData_insert(google::protobuf::Map<int64_t, ::common::NewTransactionData> * m,
                          int64_t key, ::common::NewTransactionData value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_NewTransactionData_get(const google::protobuf::Map<int64_t, ::common::NewTransactionData>* m,
                       int64_t key, const ::common::NewTransactionData** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::NewTransactionData* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_NewTransactionData_remove(google::protobuf::Map<int64_t, ::common::NewTransactionData> * m,
                          int64_t key, ::common::NewTransactionData * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_common_NewTransactionData_iter(
      const google::protobuf::Map<int64_t, ::common::NewTransactionData>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_common_NewTransactionData_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::common::NewTransactionData** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::common::NewTransactionData>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::common::NewTransactionData>* __rust_proto_thunk__Map_u64_common_NewTransactionData_new() {
    return new google::protobuf::Map<uint64_t, ::common::NewTransactionData>();
  }
  void __rust_proto_thunk__Map_u64_common_NewTransactionData_free(const google::protobuf::Map<uint64_t, ::common::NewTransactionData>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_common_NewTransactionData_clear(google::protobuf::Map<uint64_t, ::common::NewTransactionData> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_common_NewTransactionData_size(const google::protobuf::Map<uint64_t, ::common::NewTransactionData>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_common_NewTransactionData_insert(google::protobuf::Map<uint64_t, ::common::NewTransactionData> * m,
                          uint64_t key, ::common::NewTransactionData value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_NewTransactionData_get(const google::protobuf::Map<uint64_t, ::common::NewTransactionData>* m,
                       uint64_t key, const ::common::NewTransactionData** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::NewTransactionData* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_NewTransactionData_remove(google::protobuf::Map<uint64_t, ::common::NewTransactionData> * m,
                          uint64_t key, ::common::NewTransactionData * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_common_NewTransactionData_iter(
      const google::protobuf::Map<uint64_t, ::common::NewTransactionData>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_common_NewTransactionData_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::common::NewTransactionData** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::common::NewTransactionData>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::common::NewTransactionData>* __rust_proto_thunk__Map_bool_common_NewTransactionData_new() {
    return new google::protobuf::Map<bool, ::common::NewTransactionData>();
  }
  void __rust_proto_thunk__Map_bool_common_NewTransactionData_free(const google::protobuf::Map<bool, ::common::NewTransactionData>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_common_NewTransactionData_clear(google::protobuf::Map<bool, ::common::NewTransactionData> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_common_NewTransactionData_size(const google::protobuf::Map<bool, ::common::NewTransactionData>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_common_NewTransactionData_insert(google::protobuf::Map<bool, ::common::NewTransactionData> * m,
                          bool key, ::common::NewTransactionData value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_NewTransactionData_get(const google::protobuf::Map<bool, ::common::NewTransactionData>* m,
                       bool key, const ::common::NewTransactionData** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::NewTransactionData* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_NewTransactionData_remove(google::protobuf::Map<bool, ::common::NewTransactionData> * m,
                          bool key, ::common::NewTransactionData * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_common_NewTransactionData_iter(
      const google::protobuf::Map<bool, ::common::NewTransactionData>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_common_NewTransactionData_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::common::NewTransactionData** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::common::NewTransactionData>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::common::NewTransactionData>* __rust_proto_thunk__Map_ProtoStr_common_NewTransactionData_new() {
    return new google::protobuf::Map<std::string, ::common::NewTransactionData>();
  }
  void __rust_proto_thunk__Map_ProtoStr_common_NewTransactionData_free(const google::protobuf::Map<std::string, ::common::NewTransactionData>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_common_NewTransactionData_clear(google::protobuf::Map<std::string, ::common::NewTransactionData> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_common_NewTransactionData_size(const google::protobuf::Map<std::string, ::common::NewTransactionData>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_NewTransactionData_insert(google::protobuf::Map<std::string, ::common::NewTransactionData> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::NewTransactionData value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_NewTransactionData_get(const google::protobuf::Map<std::string, ::common::NewTransactionData>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::common::NewTransactionData** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::common::NewTransactionData* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_NewTransactionData_remove(google::protobuf::Map<std::string, ::common::NewTransactionData> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::NewTransactionData * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_common_NewTransactionData_iter(
      const google::protobuf::Map<std::string, ::common::NewTransactionData>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_common_NewTransactionData_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::common::NewTransactionData** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::common::NewTransactionData>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// common.AckProposalData
        // clang-format off
extern "C" {
void* __rust_proto_thunk__common_AckProposalData_new() { return new ::common::AckProposalData(); }
void __rust_proto_thunk__common_AckProposalData_delete(void* ptr) { delete static_cast<::common::AckProposalData*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__common_AckProposalData_serialize(::common::AckProposalData* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__common_AckProposalData_parse(::common::AckProposalData* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__common_AckProposalData_copy_from(::common::AckProposalData* dst, const ::common::AckProposalData* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__common_AckProposalData_repeated_len(google::protobuf::RepeatedPtrField<::common::AckProposalData>* field) {
  return field->size();
}
const ::common::AckProposalData& __rust_proto_thunk__common_AckProposalData_repeated_get(
  google::protobuf::RepeatedPtrField<::common::AckProposalData>* field,
  size_t index) {
  return field->Get(index);
}
::common::AckProposalData* __rust_proto_thunk__common_AckProposalData_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::common::AckProposalData>* field,
  size_t index) {
  return field->Mutable(index);
}
::common::AckProposalData* __rust_proto_thunk__common_AckProposalData_repeated_add(google::protobuf::RepeatedPtrField<::common::AckProposalData>* field) {
  return field->Add();
}
void __rust_proto_thunk__common_AckProposalData_repeated_clear(google::protobuf::RepeatedPtrField<::common::AckProposalData>* field) {
  field->Clear();
}
void __rust_proto_thunk__common_AckProposalData_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::common::AckProposalData>& dst,
  const google::protobuf::RepeatedPtrField<::common::AckProposalData>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__common_AckProposalData_get_participant(::common::AckProposalData* msg) {
  absl::string_view val = msg->participant();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__common_AckProposalData_set_participant(::common::AckProposalData* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_participant(absl::string_view(s.ptr, s.len));
}

const void* __rust_proto_thunk__common_AckProposalData_get_contract(::common::AckProposalData* msg) {
  return static_cast<const void*>(&msg->contract());
}
void* __rust_proto_thunk__common_AckProposalData_get_mut_contract(::common::AckProposalData* msg) {
  return static_cast<void*>(msg->mutable_contract());
}
void __rust_proto_thunk__common_AckProposalData_clear_contract(::common::AckProposalData* msg) { msg->clear_contract(); }
bool __rust_proto_thunk__common_AckProposalData_has_contract(::common::AckProposalData* msg) { return msg->has_contract(); }

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__common_AckProposalData_get_originator(::common::AckProposalData* msg) {
  absl::string_view val = msg->originator();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__common_AckProposalData_set_originator(::common::AckProposalData* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_originator(absl::string_view(s.ptr, s.len));
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::common::AckProposalData>* __rust_proto_thunk__Map_i32_common_AckProposalData_new() {
    return new google::protobuf::Map<int32_t, ::common::AckProposalData>();
  }
  void __rust_proto_thunk__Map_i32_common_AckProposalData_free(const google::protobuf::Map<int32_t, ::common::AckProposalData>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_common_AckProposalData_clear(google::protobuf::Map<int32_t, ::common::AckProposalData> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_common_AckProposalData_size(const google::protobuf::Map<int32_t, ::common::AckProposalData>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_common_AckProposalData_insert(google::protobuf::Map<int32_t, ::common::AckProposalData> * m,
                          int32_t key, ::common::AckProposalData value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_AckProposalData_get(const google::protobuf::Map<int32_t, ::common::AckProposalData>* m,
                       int32_t key, const ::common::AckProposalData** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::AckProposalData* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_AckProposalData_remove(google::protobuf::Map<int32_t, ::common::AckProposalData> * m,
                          int32_t key, ::common::AckProposalData * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_common_AckProposalData_iter(
      const google::protobuf::Map<int32_t, ::common::AckProposalData>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_common_AckProposalData_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::common::AckProposalData** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::common::AckProposalData>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::common::AckProposalData>* __rust_proto_thunk__Map_u32_common_AckProposalData_new() {
    return new google::protobuf::Map<uint32_t, ::common::AckProposalData>();
  }
  void __rust_proto_thunk__Map_u32_common_AckProposalData_free(const google::protobuf::Map<uint32_t, ::common::AckProposalData>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_common_AckProposalData_clear(google::protobuf::Map<uint32_t, ::common::AckProposalData> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_common_AckProposalData_size(const google::protobuf::Map<uint32_t, ::common::AckProposalData>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_common_AckProposalData_insert(google::protobuf::Map<uint32_t, ::common::AckProposalData> * m,
                          uint32_t key, ::common::AckProposalData value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_AckProposalData_get(const google::protobuf::Map<uint32_t, ::common::AckProposalData>* m,
                       uint32_t key, const ::common::AckProposalData** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::AckProposalData* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_AckProposalData_remove(google::protobuf::Map<uint32_t, ::common::AckProposalData> * m,
                          uint32_t key, ::common::AckProposalData * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_common_AckProposalData_iter(
      const google::protobuf::Map<uint32_t, ::common::AckProposalData>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_common_AckProposalData_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::common::AckProposalData** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::common::AckProposalData>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::common::AckProposalData>* __rust_proto_thunk__Map_i64_common_AckProposalData_new() {
    return new google::protobuf::Map<int64_t, ::common::AckProposalData>();
  }
  void __rust_proto_thunk__Map_i64_common_AckProposalData_free(const google::protobuf::Map<int64_t, ::common::AckProposalData>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_common_AckProposalData_clear(google::protobuf::Map<int64_t, ::common::AckProposalData> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_common_AckProposalData_size(const google::protobuf::Map<int64_t, ::common::AckProposalData>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_common_AckProposalData_insert(google::protobuf::Map<int64_t, ::common::AckProposalData> * m,
                          int64_t key, ::common::AckProposalData value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_AckProposalData_get(const google::protobuf::Map<int64_t, ::common::AckProposalData>* m,
                       int64_t key, const ::common::AckProposalData** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::AckProposalData* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_AckProposalData_remove(google::protobuf::Map<int64_t, ::common::AckProposalData> * m,
                          int64_t key, ::common::AckProposalData * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_common_AckProposalData_iter(
      const google::protobuf::Map<int64_t, ::common::AckProposalData>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_common_AckProposalData_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::common::AckProposalData** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::common::AckProposalData>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::common::AckProposalData>* __rust_proto_thunk__Map_u64_common_AckProposalData_new() {
    return new google::protobuf::Map<uint64_t, ::common::AckProposalData>();
  }
  void __rust_proto_thunk__Map_u64_common_AckProposalData_free(const google::protobuf::Map<uint64_t, ::common::AckProposalData>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_common_AckProposalData_clear(google::protobuf::Map<uint64_t, ::common::AckProposalData> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_common_AckProposalData_size(const google::protobuf::Map<uint64_t, ::common::AckProposalData>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_common_AckProposalData_insert(google::protobuf::Map<uint64_t, ::common::AckProposalData> * m,
                          uint64_t key, ::common::AckProposalData value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_AckProposalData_get(const google::protobuf::Map<uint64_t, ::common::AckProposalData>* m,
                       uint64_t key, const ::common::AckProposalData** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::AckProposalData* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_AckProposalData_remove(google::protobuf::Map<uint64_t, ::common::AckProposalData> * m,
                          uint64_t key, ::common::AckProposalData * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_common_AckProposalData_iter(
      const google::protobuf::Map<uint64_t, ::common::AckProposalData>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_common_AckProposalData_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::common::AckProposalData** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::common::AckProposalData>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::common::AckProposalData>* __rust_proto_thunk__Map_bool_common_AckProposalData_new() {
    return new google::protobuf::Map<bool, ::common::AckProposalData>();
  }
  void __rust_proto_thunk__Map_bool_common_AckProposalData_free(const google::protobuf::Map<bool, ::common::AckProposalData>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_common_AckProposalData_clear(google::protobuf::Map<bool, ::common::AckProposalData> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_common_AckProposalData_size(const google::protobuf::Map<bool, ::common::AckProposalData>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_common_AckProposalData_insert(google::protobuf::Map<bool, ::common::AckProposalData> * m,
                          bool key, ::common::AckProposalData value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_AckProposalData_get(const google::protobuf::Map<bool, ::common::AckProposalData>* m,
                       bool key, const ::common::AckProposalData** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::AckProposalData* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_AckProposalData_remove(google::protobuf::Map<bool, ::common::AckProposalData> * m,
                          bool key, ::common::AckProposalData * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_common_AckProposalData_iter(
      const google::protobuf::Map<bool, ::common::AckProposalData>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_common_AckProposalData_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::common::AckProposalData** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::common::AckProposalData>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::common::AckProposalData>* __rust_proto_thunk__Map_ProtoStr_common_AckProposalData_new() {
    return new google::protobuf::Map<std::string, ::common::AckProposalData>();
  }
  void __rust_proto_thunk__Map_ProtoStr_common_AckProposalData_free(const google::protobuf::Map<std::string, ::common::AckProposalData>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_common_AckProposalData_clear(google::protobuf::Map<std::string, ::common::AckProposalData> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_common_AckProposalData_size(const google::protobuf::Map<std::string, ::common::AckProposalData>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_AckProposalData_insert(google::protobuf::Map<std::string, ::common::AckProposalData> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::AckProposalData value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_AckProposalData_get(const google::protobuf::Map<std::string, ::common::AckProposalData>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::common::AckProposalData** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::common::AckProposalData* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_AckProposalData_remove(google::protobuf::Map<std::string, ::common::AckProposalData> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::AckProposalData * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_common_AckProposalData_iter(
      const google::protobuf::Map<std::string, ::common::AckProposalData>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_common_AckProposalData_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::common::AckProposalData** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::common::AckProposalData>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// common.ProposalAckNackData
        // clang-format off
extern "C" {
void* __rust_proto_thunk__common_ProposalAckNackData_new() { return new ::common::ProposalAckNackData(); }
void __rust_proto_thunk__common_ProposalAckNackData_delete(void* ptr) { delete static_cast<::common::ProposalAckNackData*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__common_ProposalAckNackData_serialize(::common::ProposalAckNackData* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__common_ProposalAckNackData_parse(::common::ProposalAckNackData* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__common_ProposalAckNackData_copy_from(::common::ProposalAckNackData* dst, const ::common::ProposalAckNackData* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__common_ProposalAckNackData_repeated_len(google::protobuf::RepeatedPtrField<::common::ProposalAckNackData>* field) {
  return field->size();
}
const ::common::ProposalAckNackData& __rust_proto_thunk__common_ProposalAckNackData_repeated_get(
  google::protobuf::RepeatedPtrField<::common::ProposalAckNackData>* field,
  size_t index) {
  return field->Get(index);
}
::common::ProposalAckNackData* __rust_proto_thunk__common_ProposalAckNackData_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::common::ProposalAckNackData>* field,
  size_t index) {
  return field->Mutable(index);
}
::common::ProposalAckNackData* __rust_proto_thunk__common_ProposalAckNackData_repeated_add(google::protobuf::RepeatedPtrField<::common::ProposalAckNackData>* field) {
  return field->Add();
}
void __rust_proto_thunk__common_ProposalAckNackData_repeated_clear(google::protobuf::RepeatedPtrField<::common::ProposalAckNackData>* field) {
  field->Clear();
}
void __rust_proto_thunk__common_ProposalAckNackData_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::common::ProposalAckNackData>& dst,
  const google::protobuf::RepeatedPtrField<::common::ProposalAckNackData>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__common_ProposalAckNackData_get_participant(::common::ProposalAckNackData* msg) {
  absl::string_view val = msg->participant();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__common_ProposalAckNackData_set_participant(::common::ProposalAckNackData* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_participant(absl::string_view(s.ptr, s.len));
}

const void* __rust_proto_thunk__common_ProposalAckNackData_get_contract(::common::ProposalAckNackData* msg) {
  return static_cast<const void*>(&msg->contract());
}
void* __rust_proto_thunk__common_ProposalAckNackData_get_mut_contract(::common::ProposalAckNackData* msg) {
  return static_cast<void*>(msg->mutable_contract());
}
void __rust_proto_thunk__common_ProposalAckNackData_clear_contract(::common::ProposalAckNackData* msg) { msg->clear_contract(); }
bool __rust_proto_thunk__common_ProposalAckNackData_has_contract(::common::ProposalAckNackData* msg) { return msg->has_contract(); }

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__common_ProposalAckNackData_get_originator(::common::ProposalAckNackData* msg) {
  absl::string_view val = msg->originator();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__common_ProposalAckNackData_set_originator(::common::ProposalAckNackData* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_originator(absl::string_view(s.ptr, s.len));
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::common::ProposalAckNackData>* __rust_proto_thunk__Map_i32_common_ProposalAckNackData_new() {
    return new google::protobuf::Map<int32_t, ::common::ProposalAckNackData>();
  }
  void __rust_proto_thunk__Map_i32_common_ProposalAckNackData_free(const google::protobuf::Map<int32_t, ::common::ProposalAckNackData>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_common_ProposalAckNackData_clear(google::protobuf::Map<int32_t, ::common::ProposalAckNackData> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_common_ProposalAckNackData_size(const google::protobuf::Map<int32_t, ::common::ProposalAckNackData>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_common_ProposalAckNackData_insert(google::protobuf::Map<int32_t, ::common::ProposalAckNackData> * m,
                          int32_t key, ::common::ProposalAckNackData value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_ProposalAckNackData_get(const google::protobuf::Map<int32_t, ::common::ProposalAckNackData>* m,
                       int32_t key, const ::common::ProposalAckNackData** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::ProposalAckNackData* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_ProposalAckNackData_remove(google::protobuf::Map<int32_t, ::common::ProposalAckNackData> * m,
                          int32_t key, ::common::ProposalAckNackData * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_common_ProposalAckNackData_iter(
      const google::protobuf::Map<int32_t, ::common::ProposalAckNackData>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_common_ProposalAckNackData_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::common::ProposalAckNackData** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::common::ProposalAckNackData>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::common::ProposalAckNackData>* __rust_proto_thunk__Map_u32_common_ProposalAckNackData_new() {
    return new google::protobuf::Map<uint32_t, ::common::ProposalAckNackData>();
  }
  void __rust_proto_thunk__Map_u32_common_ProposalAckNackData_free(const google::protobuf::Map<uint32_t, ::common::ProposalAckNackData>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_common_ProposalAckNackData_clear(google::protobuf::Map<uint32_t, ::common::ProposalAckNackData> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_common_ProposalAckNackData_size(const google::protobuf::Map<uint32_t, ::common::ProposalAckNackData>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_common_ProposalAckNackData_insert(google::protobuf::Map<uint32_t, ::common::ProposalAckNackData> * m,
                          uint32_t key, ::common::ProposalAckNackData value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_ProposalAckNackData_get(const google::protobuf::Map<uint32_t, ::common::ProposalAckNackData>* m,
                       uint32_t key, const ::common::ProposalAckNackData** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::ProposalAckNackData* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_ProposalAckNackData_remove(google::protobuf::Map<uint32_t, ::common::ProposalAckNackData> * m,
                          uint32_t key, ::common::ProposalAckNackData * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_common_ProposalAckNackData_iter(
      const google::protobuf::Map<uint32_t, ::common::ProposalAckNackData>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_common_ProposalAckNackData_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::common::ProposalAckNackData** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::common::ProposalAckNackData>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::common::ProposalAckNackData>* __rust_proto_thunk__Map_i64_common_ProposalAckNackData_new() {
    return new google::protobuf::Map<int64_t, ::common::ProposalAckNackData>();
  }
  void __rust_proto_thunk__Map_i64_common_ProposalAckNackData_free(const google::protobuf::Map<int64_t, ::common::ProposalAckNackData>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_common_ProposalAckNackData_clear(google::protobuf::Map<int64_t, ::common::ProposalAckNackData> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_common_ProposalAckNackData_size(const google::protobuf::Map<int64_t, ::common::ProposalAckNackData>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_common_ProposalAckNackData_insert(google::protobuf::Map<int64_t, ::common::ProposalAckNackData> * m,
                          int64_t key, ::common::ProposalAckNackData value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_ProposalAckNackData_get(const google::protobuf::Map<int64_t, ::common::ProposalAckNackData>* m,
                       int64_t key, const ::common::ProposalAckNackData** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::ProposalAckNackData* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_ProposalAckNackData_remove(google::protobuf::Map<int64_t, ::common::ProposalAckNackData> * m,
                          int64_t key, ::common::ProposalAckNackData * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_common_ProposalAckNackData_iter(
      const google::protobuf::Map<int64_t, ::common::ProposalAckNackData>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_common_ProposalAckNackData_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::common::ProposalAckNackData** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::common::ProposalAckNackData>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::common::ProposalAckNackData>* __rust_proto_thunk__Map_u64_common_ProposalAckNackData_new() {
    return new google::protobuf::Map<uint64_t, ::common::ProposalAckNackData>();
  }
  void __rust_proto_thunk__Map_u64_common_ProposalAckNackData_free(const google::protobuf::Map<uint64_t, ::common::ProposalAckNackData>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_common_ProposalAckNackData_clear(google::protobuf::Map<uint64_t, ::common::ProposalAckNackData> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_common_ProposalAckNackData_size(const google::protobuf::Map<uint64_t, ::common::ProposalAckNackData>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_common_ProposalAckNackData_insert(google::protobuf::Map<uint64_t, ::common::ProposalAckNackData> * m,
                          uint64_t key, ::common::ProposalAckNackData value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_ProposalAckNackData_get(const google::protobuf::Map<uint64_t, ::common::ProposalAckNackData>* m,
                       uint64_t key, const ::common::ProposalAckNackData** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::ProposalAckNackData* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_ProposalAckNackData_remove(google::protobuf::Map<uint64_t, ::common::ProposalAckNackData> * m,
                          uint64_t key, ::common::ProposalAckNackData * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_common_ProposalAckNackData_iter(
      const google::protobuf::Map<uint64_t, ::common::ProposalAckNackData>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_common_ProposalAckNackData_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::common::ProposalAckNackData** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::common::ProposalAckNackData>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::common::ProposalAckNackData>* __rust_proto_thunk__Map_bool_common_ProposalAckNackData_new() {
    return new google::protobuf::Map<bool, ::common::ProposalAckNackData>();
  }
  void __rust_proto_thunk__Map_bool_common_ProposalAckNackData_free(const google::protobuf::Map<bool, ::common::ProposalAckNackData>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_common_ProposalAckNackData_clear(google::protobuf::Map<bool, ::common::ProposalAckNackData> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_common_ProposalAckNackData_size(const google::protobuf::Map<bool, ::common::ProposalAckNackData>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_common_ProposalAckNackData_insert(google::protobuf::Map<bool, ::common::ProposalAckNackData> * m,
                          bool key, ::common::ProposalAckNackData value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_ProposalAckNackData_get(const google::protobuf::Map<bool, ::common::ProposalAckNackData>* m,
                       bool key, const ::common::ProposalAckNackData** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::ProposalAckNackData* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_ProposalAckNackData_remove(google::protobuf::Map<bool, ::common::ProposalAckNackData> * m,
                          bool key, ::common::ProposalAckNackData * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_common_ProposalAckNackData_iter(
      const google::protobuf::Map<bool, ::common::ProposalAckNackData>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_common_ProposalAckNackData_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::common::ProposalAckNackData** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::common::ProposalAckNackData>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::common::ProposalAckNackData>* __rust_proto_thunk__Map_ProtoStr_common_ProposalAckNackData_new() {
    return new google::protobuf::Map<std::string, ::common::ProposalAckNackData>();
  }
  void __rust_proto_thunk__Map_ProtoStr_common_ProposalAckNackData_free(const google::protobuf::Map<std::string, ::common::ProposalAckNackData>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_common_ProposalAckNackData_clear(google::protobuf::Map<std::string, ::common::ProposalAckNackData> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_common_ProposalAckNackData_size(const google::protobuf::Map<std::string, ::common::ProposalAckNackData>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_ProposalAckNackData_insert(google::protobuf::Map<std::string, ::common::ProposalAckNackData> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::ProposalAckNackData value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_ProposalAckNackData_get(const google::protobuf::Map<std::string, ::common::ProposalAckNackData>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::common::ProposalAckNackData** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::common::ProposalAckNackData* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_ProposalAckNackData_remove(google::protobuf::Map<std::string, ::common::ProposalAckNackData> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::ProposalAckNackData * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_common_ProposalAckNackData_iter(
      const google::protobuf::Map<std::string, ::common::ProposalAckNackData>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_common_ProposalAckNackData_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::common::ProposalAckNackData** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::common::ProposalAckNackData>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// common.RevealSecretData
        // clang-format off
extern "C" {
void* __rust_proto_thunk__common_RevealSecretData_new() { return new ::common::RevealSecretData(); }
void __rust_proto_thunk__common_RevealSecretData_delete(void* ptr) { delete static_cast<::common::RevealSecretData*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__common_RevealSecretData_serialize(::common::RevealSecretData* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__common_RevealSecretData_parse(::common::RevealSecretData* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__common_RevealSecretData_copy_from(::common::RevealSecretData* dst, const ::common::RevealSecretData* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__common_RevealSecretData_repeated_len(google::protobuf::RepeatedPtrField<::common::RevealSecretData>* field) {
  return field->size();
}
const ::common::RevealSecretData& __rust_proto_thunk__common_RevealSecretData_repeated_get(
  google::protobuf::RepeatedPtrField<::common::RevealSecretData>* field,
  size_t index) {
  return field->Get(index);
}
::common::RevealSecretData* __rust_proto_thunk__common_RevealSecretData_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::common::RevealSecretData>* field,
  size_t index) {
  return field->Mutable(index);
}
::common::RevealSecretData* __rust_proto_thunk__common_RevealSecretData_repeated_add(google::protobuf::RepeatedPtrField<::common::RevealSecretData>* field) {
  return field->Add();
}
void __rust_proto_thunk__common_RevealSecretData_repeated_clear(google::protobuf::RepeatedPtrField<::common::RevealSecretData>* field) {
  field->Clear();
}
void __rust_proto_thunk__common_RevealSecretData_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::common::RevealSecretData>& dst,
  const google::protobuf::RepeatedPtrField<::common::RevealSecretData>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__common_RevealSecretData_get_participant(::common::RevealSecretData* msg) {
  absl::string_view val = msg->participant();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__common_RevealSecretData_set_participant(::common::RevealSecretData* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_participant(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__common_RevealSecretData_get_contract_id(::common::RevealSecretData* msg) {
  absl::string_view val = msg->contract_id();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__common_RevealSecretData_set_contract_id(::common::RevealSecretData* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_contract_id(absl::string_view(s.ptr, s.len));
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::common::RevealSecretData>* __rust_proto_thunk__Map_i32_common_RevealSecretData_new() {
    return new google::protobuf::Map<int32_t, ::common::RevealSecretData>();
  }
  void __rust_proto_thunk__Map_i32_common_RevealSecretData_free(const google::protobuf::Map<int32_t, ::common::RevealSecretData>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_common_RevealSecretData_clear(google::protobuf::Map<int32_t, ::common::RevealSecretData> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_common_RevealSecretData_size(const google::protobuf::Map<int32_t, ::common::RevealSecretData>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_common_RevealSecretData_insert(google::protobuf::Map<int32_t, ::common::RevealSecretData> * m,
                          int32_t key, ::common::RevealSecretData value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_RevealSecretData_get(const google::protobuf::Map<int32_t, ::common::RevealSecretData>* m,
                       int32_t key, const ::common::RevealSecretData** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::RevealSecretData* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_RevealSecretData_remove(google::protobuf::Map<int32_t, ::common::RevealSecretData> * m,
                          int32_t key, ::common::RevealSecretData * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_common_RevealSecretData_iter(
      const google::protobuf::Map<int32_t, ::common::RevealSecretData>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_common_RevealSecretData_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::common::RevealSecretData** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::common::RevealSecretData>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::common::RevealSecretData>* __rust_proto_thunk__Map_u32_common_RevealSecretData_new() {
    return new google::protobuf::Map<uint32_t, ::common::RevealSecretData>();
  }
  void __rust_proto_thunk__Map_u32_common_RevealSecretData_free(const google::protobuf::Map<uint32_t, ::common::RevealSecretData>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_common_RevealSecretData_clear(google::protobuf::Map<uint32_t, ::common::RevealSecretData> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_common_RevealSecretData_size(const google::protobuf::Map<uint32_t, ::common::RevealSecretData>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_common_RevealSecretData_insert(google::protobuf::Map<uint32_t, ::common::RevealSecretData> * m,
                          uint32_t key, ::common::RevealSecretData value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_RevealSecretData_get(const google::protobuf::Map<uint32_t, ::common::RevealSecretData>* m,
                       uint32_t key, const ::common::RevealSecretData** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::RevealSecretData* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_RevealSecretData_remove(google::protobuf::Map<uint32_t, ::common::RevealSecretData> * m,
                          uint32_t key, ::common::RevealSecretData * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_common_RevealSecretData_iter(
      const google::protobuf::Map<uint32_t, ::common::RevealSecretData>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_common_RevealSecretData_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::common::RevealSecretData** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::common::RevealSecretData>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::common::RevealSecretData>* __rust_proto_thunk__Map_i64_common_RevealSecretData_new() {
    return new google::protobuf::Map<int64_t, ::common::RevealSecretData>();
  }
  void __rust_proto_thunk__Map_i64_common_RevealSecretData_free(const google::protobuf::Map<int64_t, ::common::RevealSecretData>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_common_RevealSecretData_clear(google::protobuf::Map<int64_t, ::common::RevealSecretData> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_common_RevealSecretData_size(const google::protobuf::Map<int64_t, ::common::RevealSecretData>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_common_RevealSecretData_insert(google::protobuf::Map<int64_t, ::common::RevealSecretData> * m,
                          int64_t key, ::common::RevealSecretData value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_RevealSecretData_get(const google::protobuf::Map<int64_t, ::common::RevealSecretData>* m,
                       int64_t key, const ::common::RevealSecretData** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::RevealSecretData* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_RevealSecretData_remove(google::protobuf::Map<int64_t, ::common::RevealSecretData> * m,
                          int64_t key, ::common::RevealSecretData * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_common_RevealSecretData_iter(
      const google::protobuf::Map<int64_t, ::common::RevealSecretData>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_common_RevealSecretData_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::common::RevealSecretData** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::common::RevealSecretData>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::common::RevealSecretData>* __rust_proto_thunk__Map_u64_common_RevealSecretData_new() {
    return new google::protobuf::Map<uint64_t, ::common::RevealSecretData>();
  }
  void __rust_proto_thunk__Map_u64_common_RevealSecretData_free(const google::protobuf::Map<uint64_t, ::common::RevealSecretData>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_common_RevealSecretData_clear(google::protobuf::Map<uint64_t, ::common::RevealSecretData> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_common_RevealSecretData_size(const google::protobuf::Map<uint64_t, ::common::RevealSecretData>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_common_RevealSecretData_insert(google::protobuf::Map<uint64_t, ::common::RevealSecretData> * m,
                          uint64_t key, ::common::RevealSecretData value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_RevealSecretData_get(const google::protobuf::Map<uint64_t, ::common::RevealSecretData>* m,
                       uint64_t key, const ::common::RevealSecretData** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::RevealSecretData* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_RevealSecretData_remove(google::protobuf::Map<uint64_t, ::common::RevealSecretData> * m,
                          uint64_t key, ::common::RevealSecretData * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_common_RevealSecretData_iter(
      const google::protobuf::Map<uint64_t, ::common::RevealSecretData>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_common_RevealSecretData_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::common::RevealSecretData** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::common::RevealSecretData>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::common::RevealSecretData>* __rust_proto_thunk__Map_bool_common_RevealSecretData_new() {
    return new google::protobuf::Map<bool, ::common::RevealSecretData>();
  }
  void __rust_proto_thunk__Map_bool_common_RevealSecretData_free(const google::protobuf::Map<bool, ::common::RevealSecretData>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_common_RevealSecretData_clear(google::protobuf::Map<bool, ::common::RevealSecretData> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_common_RevealSecretData_size(const google::protobuf::Map<bool, ::common::RevealSecretData>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_common_RevealSecretData_insert(google::protobuf::Map<bool, ::common::RevealSecretData> * m,
                          bool key, ::common::RevealSecretData value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_RevealSecretData_get(const google::protobuf::Map<bool, ::common::RevealSecretData>* m,
                       bool key, const ::common::RevealSecretData** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::RevealSecretData* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_RevealSecretData_remove(google::protobuf::Map<bool, ::common::RevealSecretData> * m,
                          bool key, ::common::RevealSecretData * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_common_RevealSecretData_iter(
      const google::protobuf::Map<bool, ::common::RevealSecretData>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_common_RevealSecretData_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::common::RevealSecretData** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::common::RevealSecretData>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::common::RevealSecretData>* __rust_proto_thunk__Map_ProtoStr_common_RevealSecretData_new() {
    return new google::protobuf::Map<std::string, ::common::RevealSecretData>();
  }
  void __rust_proto_thunk__Map_ProtoStr_common_RevealSecretData_free(const google::protobuf::Map<std::string, ::common::RevealSecretData>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_common_RevealSecretData_clear(google::protobuf::Map<std::string, ::common::RevealSecretData> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_common_RevealSecretData_size(const google::protobuf::Map<std::string, ::common::RevealSecretData>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_RevealSecretData_insert(google::protobuf::Map<std::string, ::common::RevealSecretData> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::RevealSecretData value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_RevealSecretData_get(const google::protobuf::Map<std::string, ::common::RevealSecretData>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::common::RevealSecretData** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::common::RevealSecretData* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_RevealSecretData_remove(google::protobuf::Map<std::string, ::common::RevealSecretData> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::RevealSecretData * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_common_RevealSecretData_iter(
      const google::protobuf::Map<std::string, ::common::RevealSecretData>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_common_RevealSecretData_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::common::RevealSecretData** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::common::RevealSecretData>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// common.SecretRevealedData
        // clang-format off
extern "C" {
void* __rust_proto_thunk__common_SecretRevealedData_new() { return new ::common::SecretRevealedData(); }
void __rust_proto_thunk__common_SecretRevealedData_delete(void* ptr) { delete static_cast<::common::SecretRevealedData*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__common_SecretRevealedData_serialize(::common::SecretRevealedData* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__common_SecretRevealedData_parse(::common::SecretRevealedData* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__common_SecretRevealedData_copy_from(::common::SecretRevealedData* dst, const ::common::SecretRevealedData* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__common_SecretRevealedData_repeated_len(google::protobuf::RepeatedPtrField<::common::SecretRevealedData>* field) {
  return field->size();
}
const ::common::SecretRevealedData& __rust_proto_thunk__common_SecretRevealedData_repeated_get(
  google::protobuf::RepeatedPtrField<::common::SecretRevealedData>* field,
  size_t index) {
  return field->Get(index);
}
::common::SecretRevealedData* __rust_proto_thunk__common_SecretRevealedData_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::common::SecretRevealedData>* field,
  size_t index) {
  return field->Mutable(index);
}
::common::SecretRevealedData* __rust_proto_thunk__common_SecretRevealedData_repeated_add(google::protobuf::RepeatedPtrField<::common::SecretRevealedData>* field) {
  return field->Add();
}
void __rust_proto_thunk__common_SecretRevealedData_repeated_clear(google::protobuf::RepeatedPtrField<::common::SecretRevealedData>* field) {
  field->Clear();
}
void __rust_proto_thunk__common_SecretRevealedData_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::common::SecretRevealedData>& dst,
  const google::protobuf::RepeatedPtrField<::common::SecretRevealedData>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__common_SecretRevealedData_get_participant(::common::SecretRevealedData* msg) {
  absl::string_view val = msg->participant();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__common_SecretRevealedData_set_participant(::common::SecretRevealedData* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_participant(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__common_SecretRevealedData_get_contract_id(::common::SecretRevealedData* msg) {
  absl::string_view val = msg->contract_id();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__common_SecretRevealedData_set_contract_id(::common::SecretRevealedData* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_contract_id(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__common_SecretRevealedData_get_secret(::common::SecretRevealedData* msg) {
  absl::string_view val = msg->secret();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__common_SecretRevealedData_set_secret(::common::SecretRevealedData* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_secret(absl::string_view(s.ptr, s.len));
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::common::SecretRevealedData>* __rust_proto_thunk__Map_i32_common_SecretRevealedData_new() {
    return new google::protobuf::Map<int32_t, ::common::SecretRevealedData>();
  }
  void __rust_proto_thunk__Map_i32_common_SecretRevealedData_free(const google::protobuf::Map<int32_t, ::common::SecretRevealedData>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_common_SecretRevealedData_clear(google::protobuf::Map<int32_t, ::common::SecretRevealedData> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_common_SecretRevealedData_size(const google::protobuf::Map<int32_t, ::common::SecretRevealedData>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_common_SecretRevealedData_insert(google::protobuf::Map<int32_t, ::common::SecretRevealedData> * m,
                          int32_t key, ::common::SecretRevealedData value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_SecretRevealedData_get(const google::protobuf::Map<int32_t, ::common::SecretRevealedData>* m,
                       int32_t key, const ::common::SecretRevealedData** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::SecretRevealedData* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_SecretRevealedData_remove(google::protobuf::Map<int32_t, ::common::SecretRevealedData> * m,
                          int32_t key, ::common::SecretRevealedData * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_common_SecretRevealedData_iter(
      const google::protobuf::Map<int32_t, ::common::SecretRevealedData>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_common_SecretRevealedData_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::common::SecretRevealedData** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::common::SecretRevealedData>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::common::SecretRevealedData>* __rust_proto_thunk__Map_u32_common_SecretRevealedData_new() {
    return new google::protobuf::Map<uint32_t, ::common::SecretRevealedData>();
  }
  void __rust_proto_thunk__Map_u32_common_SecretRevealedData_free(const google::protobuf::Map<uint32_t, ::common::SecretRevealedData>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_common_SecretRevealedData_clear(google::protobuf::Map<uint32_t, ::common::SecretRevealedData> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_common_SecretRevealedData_size(const google::protobuf::Map<uint32_t, ::common::SecretRevealedData>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_common_SecretRevealedData_insert(google::protobuf::Map<uint32_t, ::common::SecretRevealedData> * m,
                          uint32_t key, ::common::SecretRevealedData value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_SecretRevealedData_get(const google::protobuf::Map<uint32_t, ::common::SecretRevealedData>* m,
                       uint32_t key, const ::common::SecretRevealedData** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::SecretRevealedData* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_SecretRevealedData_remove(google::protobuf::Map<uint32_t, ::common::SecretRevealedData> * m,
                          uint32_t key, ::common::SecretRevealedData * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_common_SecretRevealedData_iter(
      const google::protobuf::Map<uint32_t, ::common::SecretRevealedData>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_common_SecretRevealedData_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::common::SecretRevealedData** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::common::SecretRevealedData>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::common::SecretRevealedData>* __rust_proto_thunk__Map_i64_common_SecretRevealedData_new() {
    return new google::protobuf::Map<int64_t, ::common::SecretRevealedData>();
  }
  void __rust_proto_thunk__Map_i64_common_SecretRevealedData_free(const google::protobuf::Map<int64_t, ::common::SecretRevealedData>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_common_SecretRevealedData_clear(google::protobuf::Map<int64_t, ::common::SecretRevealedData> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_common_SecretRevealedData_size(const google::protobuf::Map<int64_t, ::common::SecretRevealedData>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_common_SecretRevealedData_insert(google::protobuf::Map<int64_t, ::common::SecretRevealedData> * m,
                          int64_t key, ::common::SecretRevealedData value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_SecretRevealedData_get(const google::protobuf::Map<int64_t, ::common::SecretRevealedData>* m,
                       int64_t key, const ::common::SecretRevealedData** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::SecretRevealedData* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_SecretRevealedData_remove(google::protobuf::Map<int64_t, ::common::SecretRevealedData> * m,
                          int64_t key, ::common::SecretRevealedData * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_common_SecretRevealedData_iter(
      const google::protobuf::Map<int64_t, ::common::SecretRevealedData>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_common_SecretRevealedData_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::common::SecretRevealedData** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::common::SecretRevealedData>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::common::SecretRevealedData>* __rust_proto_thunk__Map_u64_common_SecretRevealedData_new() {
    return new google::protobuf::Map<uint64_t, ::common::SecretRevealedData>();
  }
  void __rust_proto_thunk__Map_u64_common_SecretRevealedData_free(const google::protobuf::Map<uint64_t, ::common::SecretRevealedData>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_common_SecretRevealedData_clear(google::protobuf::Map<uint64_t, ::common::SecretRevealedData> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_common_SecretRevealedData_size(const google::protobuf::Map<uint64_t, ::common::SecretRevealedData>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_common_SecretRevealedData_insert(google::protobuf::Map<uint64_t, ::common::SecretRevealedData> * m,
                          uint64_t key, ::common::SecretRevealedData value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_SecretRevealedData_get(const google::protobuf::Map<uint64_t, ::common::SecretRevealedData>* m,
                       uint64_t key, const ::common::SecretRevealedData** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::SecretRevealedData* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_SecretRevealedData_remove(google::protobuf::Map<uint64_t, ::common::SecretRevealedData> * m,
                          uint64_t key, ::common::SecretRevealedData * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_common_SecretRevealedData_iter(
      const google::protobuf::Map<uint64_t, ::common::SecretRevealedData>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_common_SecretRevealedData_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::common::SecretRevealedData** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::common::SecretRevealedData>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::common::SecretRevealedData>* __rust_proto_thunk__Map_bool_common_SecretRevealedData_new() {
    return new google::protobuf::Map<bool, ::common::SecretRevealedData>();
  }
  void __rust_proto_thunk__Map_bool_common_SecretRevealedData_free(const google::protobuf::Map<bool, ::common::SecretRevealedData>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_common_SecretRevealedData_clear(google::protobuf::Map<bool, ::common::SecretRevealedData> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_common_SecretRevealedData_size(const google::protobuf::Map<bool, ::common::SecretRevealedData>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_common_SecretRevealedData_insert(google::protobuf::Map<bool, ::common::SecretRevealedData> * m,
                          bool key, ::common::SecretRevealedData value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_SecretRevealedData_get(const google::protobuf::Map<bool, ::common::SecretRevealedData>* m,
                       bool key, const ::common::SecretRevealedData** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::SecretRevealedData* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_SecretRevealedData_remove(google::protobuf::Map<bool, ::common::SecretRevealedData> * m,
                          bool key, ::common::SecretRevealedData * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_common_SecretRevealedData_iter(
      const google::protobuf::Map<bool, ::common::SecretRevealedData>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_common_SecretRevealedData_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::common::SecretRevealedData** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::common::SecretRevealedData>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::common::SecretRevealedData>* __rust_proto_thunk__Map_ProtoStr_common_SecretRevealedData_new() {
    return new google::protobuf::Map<std::string, ::common::SecretRevealedData>();
  }
  void __rust_proto_thunk__Map_ProtoStr_common_SecretRevealedData_free(const google::protobuf::Map<std::string, ::common::SecretRevealedData>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_common_SecretRevealedData_clear(google::protobuf::Map<std::string, ::common::SecretRevealedData> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_common_SecretRevealedData_size(const google::protobuf::Map<std::string, ::common::SecretRevealedData>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_SecretRevealedData_insert(google::protobuf::Map<std::string, ::common::SecretRevealedData> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::SecretRevealedData value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_SecretRevealedData_get(const google::protobuf::Map<std::string, ::common::SecretRevealedData>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::common::SecretRevealedData** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::common::SecretRevealedData* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_SecretRevealedData_remove(google::protobuf::Map<std::string, ::common::SecretRevealedData> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::SecretRevealedData * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_common_SecretRevealedData_iter(
      const google::protobuf::Map<std::string, ::common::SecretRevealedData>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_common_SecretRevealedData_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::common::SecretRevealedData** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::common::SecretRevealedData>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// common.ReceiveSecretData
        // clang-format off
extern "C" {
void* __rust_proto_thunk__common_ReceiveSecretData_new() { return new ::common::ReceiveSecretData(); }
void __rust_proto_thunk__common_ReceiveSecretData_delete(void* ptr) { delete static_cast<::common::ReceiveSecretData*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__common_ReceiveSecretData_serialize(::common::ReceiveSecretData* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__common_ReceiveSecretData_parse(::common::ReceiveSecretData* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__common_ReceiveSecretData_copy_from(::common::ReceiveSecretData* dst, const ::common::ReceiveSecretData* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__common_ReceiveSecretData_repeated_len(google::protobuf::RepeatedPtrField<::common::ReceiveSecretData>* field) {
  return field->size();
}
const ::common::ReceiveSecretData& __rust_proto_thunk__common_ReceiveSecretData_repeated_get(
  google::protobuf::RepeatedPtrField<::common::ReceiveSecretData>* field,
  size_t index) {
  return field->Get(index);
}
::common::ReceiveSecretData* __rust_proto_thunk__common_ReceiveSecretData_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::common::ReceiveSecretData>* field,
  size_t index) {
  return field->Mutable(index);
}
::common::ReceiveSecretData* __rust_proto_thunk__common_ReceiveSecretData_repeated_add(google::protobuf::RepeatedPtrField<::common::ReceiveSecretData>* field) {
  return field->Add();
}
void __rust_proto_thunk__common_ReceiveSecretData_repeated_clear(google::protobuf::RepeatedPtrField<::common::ReceiveSecretData>* field) {
  field->Clear();
}
void __rust_proto_thunk__common_ReceiveSecretData_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::common::ReceiveSecretData>& dst,
  const google::protobuf::RepeatedPtrField<::common::ReceiveSecretData>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__common_ReceiveSecretData_get_participant(::common::ReceiveSecretData* msg) {
  absl::string_view val = msg->participant();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__common_ReceiveSecretData_set_participant(::common::ReceiveSecretData* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_participant(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__common_ReceiveSecretData_get_contract_id(::common::ReceiveSecretData* msg) {
  absl::string_view val = msg->contract_id();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__common_ReceiveSecretData_set_contract_id(::common::ReceiveSecretData* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_contract_id(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__common_ReceiveSecretData_get_secret(::common::ReceiveSecretData* msg) {
  absl::string_view val = msg->secret();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__common_ReceiveSecretData_set_secret(::common::ReceiveSecretData* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_secret(absl::string_view(s.ptr, s.len));
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::common::ReceiveSecretData>* __rust_proto_thunk__Map_i32_common_ReceiveSecretData_new() {
    return new google::protobuf::Map<int32_t, ::common::ReceiveSecretData>();
  }
  void __rust_proto_thunk__Map_i32_common_ReceiveSecretData_free(const google::protobuf::Map<int32_t, ::common::ReceiveSecretData>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_common_ReceiveSecretData_clear(google::protobuf::Map<int32_t, ::common::ReceiveSecretData> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_common_ReceiveSecretData_size(const google::protobuf::Map<int32_t, ::common::ReceiveSecretData>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_common_ReceiveSecretData_insert(google::protobuf::Map<int32_t, ::common::ReceiveSecretData> * m,
                          int32_t key, ::common::ReceiveSecretData value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_ReceiveSecretData_get(const google::protobuf::Map<int32_t, ::common::ReceiveSecretData>* m,
                       int32_t key, const ::common::ReceiveSecretData** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::ReceiveSecretData* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_ReceiveSecretData_remove(google::protobuf::Map<int32_t, ::common::ReceiveSecretData> * m,
                          int32_t key, ::common::ReceiveSecretData * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_common_ReceiveSecretData_iter(
      const google::protobuf::Map<int32_t, ::common::ReceiveSecretData>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_common_ReceiveSecretData_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::common::ReceiveSecretData** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::common::ReceiveSecretData>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::common::ReceiveSecretData>* __rust_proto_thunk__Map_u32_common_ReceiveSecretData_new() {
    return new google::protobuf::Map<uint32_t, ::common::ReceiveSecretData>();
  }
  void __rust_proto_thunk__Map_u32_common_ReceiveSecretData_free(const google::protobuf::Map<uint32_t, ::common::ReceiveSecretData>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_common_ReceiveSecretData_clear(google::protobuf::Map<uint32_t, ::common::ReceiveSecretData> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_common_ReceiveSecretData_size(const google::protobuf::Map<uint32_t, ::common::ReceiveSecretData>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_common_ReceiveSecretData_insert(google::protobuf::Map<uint32_t, ::common::ReceiveSecretData> * m,
                          uint32_t key, ::common::ReceiveSecretData value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_ReceiveSecretData_get(const google::protobuf::Map<uint32_t, ::common::ReceiveSecretData>* m,
                       uint32_t key, const ::common::ReceiveSecretData** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::ReceiveSecretData* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_ReceiveSecretData_remove(google::protobuf::Map<uint32_t, ::common::ReceiveSecretData> * m,
                          uint32_t key, ::common::ReceiveSecretData * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_common_ReceiveSecretData_iter(
      const google::protobuf::Map<uint32_t, ::common::ReceiveSecretData>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_common_ReceiveSecretData_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::common::ReceiveSecretData** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::common::ReceiveSecretData>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::common::ReceiveSecretData>* __rust_proto_thunk__Map_i64_common_ReceiveSecretData_new() {
    return new google::protobuf::Map<int64_t, ::common::ReceiveSecretData>();
  }
  void __rust_proto_thunk__Map_i64_common_ReceiveSecretData_free(const google::protobuf::Map<int64_t, ::common::ReceiveSecretData>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_common_ReceiveSecretData_clear(google::protobuf::Map<int64_t, ::common::ReceiveSecretData> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_common_ReceiveSecretData_size(const google::protobuf::Map<int64_t, ::common::ReceiveSecretData>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_common_ReceiveSecretData_insert(google::protobuf::Map<int64_t, ::common::ReceiveSecretData> * m,
                          int64_t key, ::common::ReceiveSecretData value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_ReceiveSecretData_get(const google::protobuf::Map<int64_t, ::common::ReceiveSecretData>* m,
                       int64_t key, const ::common::ReceiveSecretData** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::ReceiveSecretData* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_ReceiveSecretData_remove(google::protobuf::Map<int64_t, ::common::ReceiveSecretData> * m,
                          int64_t key, ::common::ReceiveSecretData * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_common_ReceiveSecretData_iter(
      const google::protobuf::Map<int64_t, ::common::ReceiveSecretData>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_common_ReceiveSecretData_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::common::ReceiveSecretData** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::common::ReceiveSecretData>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::common::ReceiveSecretData>* __rust_proto_thunk__Map_u64_common_ReceiveSecretData_new() {
    return new google::protobuf::Map<uint64_t, ::common::ReceiveSecretData>();
  }
  void __rust_proto_thunk__Map_u64_common_ReceiveSecretData_free(const google::protobuf::Map<uint64_t, ::common::ReceiveSecretData>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_common_ReceiveSecretData_clear(google::protobuf::Map<uint64_t, ::common::ReceiveSecretData> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_common_ReceiveSecretData_size(const google::protobuf::Map<uint64_t, ::common::ReceiveSecretData>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_common_ReceiveSecretData_insert(google::protobuf::Map<uint64_t, ::common::ReceiveSecretData> * m,
                          uint64_t key, ::common::ReceiveSecretData value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_ReceiveSecretData_get(const google::protobuf::Map<uint64_t, ::common::ReceiveSecretData>* m,
                       uint64_t key, const ::common::ReceiveSecretData** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::ReceiveSecretData* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_ReceiveSecretData_remove(google::protobuf::Map<uint64_t, ::common::ReceiveSecretData> * m,
                          uint64_t key, ::common::ReceiveSecretData * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_common_ReceiveSecretData_iter(
      const google::protobuf::Map<uint64_t, ::common::ReceiveSecretData>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_common_ReceiveSecretData_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::common::ReceiveSecretData** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::common::ReceiveSecretData>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::common::ReceiveSecretData>* __rust_proto_thunk__Map_bool_common_ReceiveSecretData_new() {
    return new google::protobuf::Map<bool, ::common::ReceiveSecretData>();
  }
  void __rust_proto_thunk__Map_bool_common_ReceiveSecretData_free(const google::protobuf::Map<bool, ::common::ReceiveSecretData>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_common_ReceiveSecretData_clear(google::protobuf::Map<bool, ::common::ReceiveSecretData> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_common_ReceiveSecretData_size(const google::protobuf::Map<bool, ::common::ReceiveSecretData>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_common_ReceiveSecretData_insert(google::protobuf::Map<bool, ::common::ReceiveSecretData> * m,
                          bool key, ::common::ReceiveSecretData value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_ReceiveSecretData_get(const google::protobuf::Map<bool, ::common::ReceiveSecretData>* m,
                       bool key, const ::common::ReceiveSecretData** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::ReceiveSecretData* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_ReceiveSecretData_remove(google::protobuf::Map<bool, ::common::ReceiveSecretData> * m,
                          bool key, ::common::ReceiveSecretData * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_common_ReceiveSecretData_iter(
      const google::protobuf::Map<bool, ::common::ReceiveSecretData>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_common_ReceiveSecretData_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::common::ReceiveSecretData** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::common::ReceiveSecretData>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::common::ReceiveSecretData>* __rust_proto_thunk__Map_ProtoStr_common_ReceiveSecretData_new() {
    return new google::protobuf::Map<std::string, ::common::ReceiveSecretData>();
  }
  void __rust_proto_thunk__Map_ProtoStr_common_ReceiveSecretData_free(const google::protobuf::Map<std::string, ::common::ReceiveSecretData>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_common_ReceiveSecretData_clear(google::protobuf::Map<std::string, ::common::ReceiveSecretData> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_common_ReceiveSecretData_size(const google::protobuf::Map<std::string, ::common::ReceiveSecretData>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_ReceiveSecretData_insert(google::protobuf::Map<std::string, ::common::ReceiveSecretData> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::ReceiveSecretData value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_ReceiveSecretData_get(const google::protobuf::Map<std::string, ::common::ReceiveSecretData>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::common::ReceiveSecretData** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::common::ReceiveSecretData* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_ReceiveSecretData_remove(google::protobuf::Map<std::string, ::common::ReceiveSecretData> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::ReceiveSecretData * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_common_ReceiveSecretData_iter(
      const google::protobuf::Map<std::string, ::common::ReceiveSecretData>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_common_ReceiveSecretData_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::common::ReceiveSecretData** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::common::ReceiveSecretData>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// common.ReceiveTerminationData
        // clang-format off
extern "C" {
void* __rust_proto_thunk__common_ReceiveTerminationData_new() { return new ::common::ReceiveTerminationData(); }
void __rust_proto_thunk__common_ReceiveTerminationData_delete(void* ptr) { delete static_cast<::common::ReceiveTerminationData*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__common_ReceiveTerminationData_serialize(::common::ReceiveTerminationData* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__common_ReceiveTerminationData_parse(::common::ReceiveTerminationData* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__common_ReceiveTerminationData_copy_from(::common::ReceiveTerminationData* dst, const ::common::ReceiveTerminationData* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__common_ReceiveTerminationData_repeated_len(google::protobuf::RepeatedPtrField<::common::ReceiveTerminationData>* field) {
  return field->size();
}
const ::common::ReceiveTerminationData& __rust_proto_thunk__common_ReceiveTerminationData_repeated_get(
  google::protobuf::RepeatedPtrField<::common::ReceiveTerminationData>* field,
  size_t index) {
  return field->Get(index);
}
::common::ReceiveTerminationData* __rust_proto_thunk__common_ReceiveTerminationData_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::common::ReceiveTerminationData>* field,
  size_t index) {
  return field->Mutable(index);
}
::common::ReceiveTerminationData* __rust_proto_thunk__common_ReceiveTerminationData_repeated_add(google::protobuf::RepeatedPtrField<::common::ReceiveTerminationData>* field) {
  return field->Add();
}
void __rust_proto_thunk__common_ReceiveTerminationData_repeated_clear(google::protobuf::RepeatedPtrField<::common::ReceiveTerminationData>* field) {
  field->Clear();
}
void __rust_proto_thunk__common_ReceiveTerminationData_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::common::ReceiveTerminationData>& dst,
  const google::protobuf::RepeatedPtrField<::common::ReceiveTerminationData>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__common_ReceiveTerminationData_get_participant(::common::ReceiveTerminationData* msg) {
  absl::string_view val = msg->participant();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__common_ReceiveTerminationData_set_participant(::common::ReceiveTerminationData* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_participant(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__common_ReceiveTerminationData_get_contract_id(::common::ReceiveTerminationData* msg) {
  absl::string_view val = msg->contract_id();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__common_ReceiveTerminationData_set_contract_id(::common::ReceiveTerminationData* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_contract_id(absl::string_view(s.ptr, s.len));
}

::common::TerminationResult __rust_proto_thunk__common_ReceiveTerminationData_get_result(::common::ReceiveTerminationData* msg) { return msg->result(); }
void __rust_proto_thunk__common_ReceiveTerminationData_set_result(::common::ReceiveTerminationData* msg, ::common::TerminationResult val) {
  msg->set_result(val);
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::common::ReceiveTerminationData>* __rust_proto_thunk__Map_i32_common_ReceiveTerminationData_new() {
    return new google::protobuf::Map<int32_t, ::common::ReceiveTerminationData>();
  }
  void __rust_proto_thunk__Map_i32_common_ReceiveTerminationData_free(const google::protobuf::Map<int32_t, ::common::ReceiveTerminationData>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_common_ReceiveTerminationData_clear(google::protobuf::Map<int32_t, ::common::ReceiveTerminationData> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_common_ReceiveTerminationData_size(const google::protobuf::Map<int32_t, ::common::ReceiveTerminationData>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_common_ReceiveTerminationData_insert(google::protobuf::Map<int32_t, ::common::ReceiveTerminationData> * m,
                          int32_t key, ::common::ReceiveTerminationData value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_ReceiveTerminationData_get(const google::protobuf::Map<int32_t, ::common::ReceiveTerminationData>* m,
                       int32_t key, const ::common::ReceiveTerminationData** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::ReceiveTerminationData* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_ReceiveTerminationData_remove(google::protobuf::Map<int32_t, ::common::ReceiveTerminationData> * m,
                          int32_t key, ::common::ReceiveTerminationData * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_common_ReceiveTerminationData_iter(
      const google::protobuf::Map<int32_t, ::common::ReceiveTerminationData>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_common_ReceiveTerminationData_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::common::ReceiveTerminationData** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::common::ReceiveTerminationData>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::common::ReceiveTerminationData>* __rust_proto_thunk__Map_u32_common_ReceiveTerminationData_new() {
    return new google::protobuf::Map<uint32_t, ::common::ReceiveTerminationData>();
  }
  void __rust_proto_thunk__Map_u32_common_ReceiveTerminationData_free(const google::protobuf::Map<uint32_t, ::common::ReceiveTerminationData>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_common_ReceiveTerminationData_clear(google::protobuf::Map<uint32_t, ::common::ReceiveTerminationData> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_common_ReceiveTerminationData_size(const google::protobuf::Map<uint32_t, ::common::ReceiveTerminationData>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_common_ReceiveTerminationData_insert(google::protobuf::Map<uint32_t, ::common::ReceiveTerminationData> * m,
                          uint32_t key, ::common::ReceiveTerminationData value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_ReceiveTerminationData_get(const google::protobuf::Map<uint32_t, ::common::ReceiveTerminationData>* m,
                       uint32_t key, const ::common::ReceiveTerminationData** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::ReceiveTerminationData* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_ReceiveTerminationData_remove(google::protobuf::Map<uint32_t, ::common::ReceiveTerminationData> * m,
                          uint32_t key, ::common::ReceiveTerminationData * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_common_ReceiveTerminationData_iter(
      const google::protobuf::Map<uint32_t, ::common::ReceiveTerminationData>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_common_ReceiveTerminationData_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::common::ReceiveTerminationData** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::common::ReceiveTerminationData>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::common::ReceiveTerminationData>* __rust_proto_thunk__Map_i64_common_ReceiveTerminationData_new() {
    return new google::protobuf::Map<int64_t, ::common::ReceiveTerminationData>();
  }
  void __rust_proto_thunk__Map_i64_common_ReceiveTerminationData_free(const google::protobuf::Map<int64_t, ::common::ReceiveTerminationData>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_common_ReceiveTerminationData_clear(google::protobuf::Map<int64_t, ::common::ReceiveTerminationData> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_common_ReceiveTerminationData_size(const google::protobuf::Map<int64_t, ::common::ReceiveTerminationData>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_common_ReceiveTerminationData_insert(google::protobuf::Map<int64_t, ::common::ReceiveTerminationData> * m,
                          int64_t key, ::common::ReceiveTerminationData value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_ReceiveTerminationData_get(const google::protobuf::Map<int64_t, ::common::ReceiveTerminationData>* m,
                       int64_t key, const ::common::ReceiveTerminationData** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::ReceiveTerminationData* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_ReceiveTerminationData_remove(google::protobuf::Map<int64_t, ::common::ReceiveTerminationData> * m,
                          int64_t key, ::common::ReceiveTerminationData * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_common_ReceiveTerminationData_iter(
      const google::protobuf::Map<int64_t, ::common::ReceiveTerminationData>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_common_ReceiveTerminationData_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::common::ReceiveTerminationData** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::common::ReceiveTerminationData>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::common::ReceiveTerminationData>* __rust_proto_thunk__Map_u64_common_ReceiveTerminationData_new() {
    return new google::protobuf::Map<uint64_t, ::common::ReceiveTerminationData>();
  }
  void __rust_proto_thunk__Map_u64_common_ReceiveTerminationData_free(const google::protobuf::Map<uint64_t, ::common::ReceiveTerminationData>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_common_ReceiveTerminationData_clear(google::protobuf::Map<uint64_t, ::common::ReceiveTerminationData> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_common_ReceiveTerminationData_size(const google::protobuf::Map<uint64_t, ::common::ReceiveTerminationData>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_common_ReceiveTerminationData_insert(google::protobuf::Map<uint64_t, ::common::ReceiveTerminationData> * m,
                          uint64_t key, ::common::ReceiveTerminationData value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_ReceiveTerminationData_get(const google::protobuf::Map<uint64_t, ::common::ReceiveTerminationData>* m,
                       uint64_t key, const ::common::ReceiveTerminationData** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::ReceiveTerminationData* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_ReceiveTerminationData_remove(google::protobuf::Map<uint64_t, ::common::ReceiveTerminationData> * m,
                          uint64_t key, ::common::ReceiveTerminationData * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_common_ReceiveTerminationData_iter(
      const google::protobuf::Map<uint64_t, ::common::ReceiveTerminationData>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_common_ReceiveTerminationData_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::common::ReceiveTerminationData** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::common::ReceiveTerminationData>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::common::ReceiveTerminationData>* __rust_proto_thunk__Map_bool_common_ReceiveTerminationData_new() {
    return new google::protobuf::Map<bool, ::common::ReceiveTerminationData>();
  }
  void __rust_proto_thunk__Map_bool_common_ReceiveTerminationData_free(const google::protobuf::Map<bool, ::common::ReceiveTerminationData>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_common_ReceiveTerminationData_clear(google::protobuf::Map<bool, ::common::ReceiveTerminationData> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_common_ReceiveTerminationData_size(const google::protobuf::Map<bool, ::common::ReceiveTerminationData>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_common_ReceiveTerminationData_insert(google::protobuf::Map<bool, ::common::ReceiveTerminationData> * m,
                          bool key, ::common::ReceiveTerminationData value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_ReceiveTerminationData_get(const google::protobuf::Map<bool, ::common::ReceiveTerminationData>* m,
                       bool key, const ::common::ReceiveTerminationData** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::ReceiveTerminationData* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_ReceiveTerminationData_remove(google::protobuf::Map<bool, ::common::ReceiveTerminationData> * m,
                          bool key, ::common::ReceiveTerminationData * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_common_ReceiveTerminationData_iter(
      const google::protobuf::Map<bool, ::common::ReceiveTerminationData>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_common_ReceiveTerminationData_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::common::ReceiveTerminationData** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::common::ReceiveTerminationData>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::common::ReceiveTerminationData>* __rust_proto_thunk__Map_ProtoStr_common_ReceiveTerminationData_new() {
    return new google::protobuf::Map<std::string, ::common::ReceiveTerminationData>();
  }
  void __rust_proto_thunk__Map_ProtoStr_common_ReceiveTerminationData_free(const google::protobuf::Map<std::string, ::common::ReceiveTerminationData>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_common_ReceiveTerminationData_clear(google::protobuf::Map<std::string, ::common::ReceiveTerminationData> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_common_ReceiveTerminationData_size(const google::protobuf::Map<std::string, ::common::ReceiveTerminationData>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_ReceiveTerminationData_insert(google::protobuf::Map<std::string, ::common::ReceiveTerminationData> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::ReceiveTerminationData value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_ReceiveTerminationData_get(const google::protobuf::Map<std::string, ::common::ReceiveTerminationData>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::common::ReceiveTerminationData** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::common::ReceiveTerminationData* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_ReceiveTerminationData_remove(google::protobuf::Map<std::string, ::common::ReceiveTerminationData> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::ReceiveTerminationData * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_common_ReceiveTerminationData_iter(
      const google::protobuf::Map<std::string, ::common::ReceiveTerminationData>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_common_ReceiveTerminationData_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::common::ReceiveTerminationData** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::common::ReceiveTerminationData>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// common.PayloadType
extern "C" {
  __PB_RUST_EXPOSE_SCALAR_MAP_METHODS_FOR_VALUE_TYPE(
      ::common::PayloadType, common_PayloadType, ::common::PayloadType, value, cpp_value)
}

// common.TerminationResult
extern "C" {
  __PB_RUST_EXPOSE_SCALAR_MAP_METHODS_FOR_VALUE_TYPE(
      ::common::TerminationResult, common_TerminationResult, ::common::TerminationResult, value, cpp_value)
}

