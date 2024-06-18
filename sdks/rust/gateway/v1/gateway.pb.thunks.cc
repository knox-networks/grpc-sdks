#include "gateway/v1/gateway.proto.h"
#include "google/protobuf/rust/cpp_kernel/cpp_api.h"
// gateway.LogEntry
        // clang-format off
extern "C" {
void* __rust_proto_thunk__gateway_LogEntry_new() { return new ::gateway::LogEntry(); }
void __rust_proto_thunk__gateway_LogEntry_delete(void* ptr) { delete static_cast<::gateway::LogEntry*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__gateway_LogEntry_serialize(::gateway::LogEntry* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__gateway_LogEntry_parse(::gateway::LogEntry* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__gateway_LogEntry_copy_from(::gateway::LogEntry* dst, const ::gateway::LogEntry* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__gateway_LogEntry_repeated_len(google::protobuf::RepeatedPtrField<::gateway::LogEntry>* field) {
  return field->size();
}
const ::gateway::LogEntry& __rust_proto_thunk__gateway_LogEntry_repeated_get(
  google::protobuf::RepeatedPtrField<::gateway::LogEntry>* field,
  size_t index) {
  return field->Get(index);
}
::gateway::LogEntry* __rust_proto_thunk__gateway_LogEntry_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::gateway::LogEntry>* field,
  size_t index) {
  return field->Mutable(index);
}
::gateway::LogEntry* __rust_proto_thunk__gateway_LogEntry_repeated_add(google::protobuf::RepeatedPtrField<::gateway::LogEntry>* field) {
  return field->Add();
}
void __rust_proto_thunk__gateway_LogEntry_repeated_clear(google::protobuf::RepeatedPtrField<::gateway::LogEntry>* field) {
  field->Clear();
}
void __rust_proto_thunk__gateway_LogEntry_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::gateway::LogEntry>& dst,
  const google::protobuf::RepeatedPtrField<::gateway::LogEntry>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__gateway_LogEntry_get_verifier(::gateway::LogEntry* msg) {
  absl::string_view val = msg->verifier();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__gateway_LogEntry_set_verifier(::gateway::LogEntry* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_verifier(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__gateway_LogEntry_get_note(::gateway::LogEntry* msg) {
  absl::string_view val = msg->note();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__gateway_LogEntry_set_note(::gateway::LogEntry* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_note(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__gateway_LogEntry_get_timestamp(::gateway::LogEntry* msg) {
  absl::string_view val = msg->timestamp();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__gateway_LogEntry_set_timestamp(::gateway::LogEntry* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_timestamp(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__gateway_LogEntry_get_origin(::gateway::LogEntry* msg) {
  absl::string_view val = msg->origin();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__gateway_LogEntry_set_origin(::gateway::LogEntry* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_origin(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__gateway_LogEntry_get_connection_id(::gateway::LogEntry* msg) {
  absl::string_view val = msg->connection_id();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__gateway_LogEntry_set_connection_id(::gateway::LogEntry* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_connection_id(absl::string_view(s.ptr, s.len));
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::gateway::LogEntry>* __rust_proto_thunk__Map_i32_gateway_LogEntry_new() {
    return new google::protobuf::Map<int32_t, ::gateway::LogEntry>();
  }
  void __rust_proto_thunk__Map_i32_gateway_LogEntry_free(const google::protobuf::Map<int32_t, ::gateway::LogEntry>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_gateway_LogEntry_clear(google::protobuf::Map<int32_t, ::gateway::LogEntry> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_gateway_LogEntry_size(const google::protobuf::Map<int32_t, ::gateway::LogEntry>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_gateway_LogEntry_insert(google::protobuf::Map<int32_t, ::gateway::LogEntry> * m,
                          int32_t key, ::gateway::LogEntry value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_gateway_LogEntry_get(const google::protobuf::Map<int32_t, ::gateway::LogEntry>* m,
                       int32_t key, const ::gateway::LogEntry** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::gateway::LogEntry* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_gateway_LogEntry_remove(google::protobuf::Map<int32_t, ::gateway::LogEntry> * m,
                          int32_t key, ::gateway::LogEntry * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_gateway_LogEntry_iter(
      const google::protobuf::Map<int32_t, ::gateway::LogEntry>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_gateway_LogEntry_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::gateway::LogEntry** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::gateway::LogEntry>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::gateway::LogEntry>* __rust_proto_thunk__Map_u32_gateway_LogEntry_new() {
    return new google::protobuf::Map<uint32_t, ::gateway::LogEntry>();
  }
  void __rust_proto_thunk__Map_u32_gateway_LogEntry_free(const google::protobuf::Map<uint32_t, ::gateway::LogEntry>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_gateway_LogEntry_clear(google::protobuf::Map<uint32_t, ::gateway::LogEntry> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_gateway_LogEntry_size(const google::protobuf::Map<uint32_t, ::gateway::LogEntry>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_gateway_LogEntry_insert(google::protobuf::Map<uint32_t, ::gateway::LogEntry> * m,
                          uint32_t key, ::gateway::LogEntry value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_gateway_LogEntry_get(const google::protobuf::Map<uint32_t, ::gateway::LogEntry>* m,
                       uint32_t key, const ::gateway::LogEntry** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::gateway::LogEntry* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_gateway_LogEntry_remove(google::protobuf::Map<uint32_t, ::gateway::LogEntry> * m,
                          uint32_t key, ::gateway::LogEntry * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_gateway_LogEntry_iter(
      const google::protobuf::Map<uint32_t, ::gateway::LogEntry>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_gateway_LogEntry_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::gateway::LogEntry** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::gateway::LogEntry>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::gateway::LogEntry>* __rust_proto_thunk__Map_i64_gateway_LogEntry_new() {
    return new google::protobuf::Map<int64_t, ::gateway::LogEntry>();
  }
  void __rust_proto_thunk__Map_i64_gateway_LogEntry_free(const google::protobuf::Map<int64_t, ::gateway::LogEntry>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_gateway_LogEntry_clear(google::protobuf::Map<int64_t, ::gateway::LogEntry> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_gateway_LogEntry_size(const google::protobuf::Map<int64_t, ::gateway::LogEntry>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_gateway_LogEntry_insert(google::protobuf::Map<int64_t, ::gateway::LogEntry> * m,
                          int64_t key, ::gateway::LogEntry value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_gateway_LogEntry_get(const google::protobuf::Map<int64_t, ::gateway::LogEntry>* m,
                       int64_t key, const ::gateway::LogEntry** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::gateway::LogEntry* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_gateway_LogEntry_remove(google::protobuf::Map<int64_t, ::gateway::LogEntry> * m,
                          int64_t key, ::gateway::LogEntry * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_gateway_LogEntry_iter(
      const google::protobuf::Map<int64_t, ::gateway::LogEntry>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_gateway_LogEntry_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::gateway::LogEntry** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::gateway::LogEntry>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::gateway::LogEntry>* __rust_proto_thunk__Map_u64_gateway_LogEntry_new() {
    return new google::protobuf::Map<uint64_t, ::gateway::LogEntry>();
  }
  void __rust_proto_thunk__Map_u64_gateway_LogEntry_free(const google::protobuf::Map<uint64_t, ::gateway::LogEntry>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_gateway_LogEntry_clear(google::protobuf::Map<uint64_t, ::gateway::LogEntry> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_gateway_LogEntry_size(const google::protobuf::Map<uint64_t, ::gateway::LogEntry>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_gateway_LogEntry_insert(google::protobuf::Map<uint64_t, ::gateway::LogEntry> * m,
                          uint64_t key, ::gateway::LogEntry value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_gateway_LogEntry_get(const google::protobuf::Map<uint64_t, ::gateway::LogEntry>* m,
                       uint64_t key, const ::gateway::LogEntry** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::gateway::LogEntry* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_gateway_LogEntry_remove(google::protobuf::Map<uint64_t, ::gateway::LogEntry> * m,
                          uint64_t key, ::gateway::LogEntry * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_gateway_LogEntry_iter(
      const google::protobuf::Map<uint64_t, ::gateway::LogEntry>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_gateway_LogEntry_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::gateway::LogEntry** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::gateway::LogEntry>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::gateway::LogEntry>* __rust_proto_thunk__Map_bool_gateway_LogEntry_new() {
    return new google::protobuf::Map<bool, ::gateway::LogEntry>();
  }
  void __rust_proto_thunk__Map_bool_gateway_LogEntry_free(const google::protobuf::Map<bool, ::gateway::LogEntry>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_gateway_LogEntry_clear(google::protobuf::Map<bool, ::gateway::LogEntry> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_gateway_LogEntry_size(const google::protobuf::Map<bool, ::gateway::LogEntry>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_gateway_LogEntry_insert(google::protobuf::Map<bool, ::gateway::LogEntry> * m,
                          bool key, ::gateway::LogEntry value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_gateway_LogEntry_get(const google::protobuf::Map<bool, ::gateway::LogEntry>* m,
                       bool key, const ::gateway::LogEntry** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::gateway::LogEntry* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_gateway_LogEntry_remove(google::protobuf::Map<bool, ::gateway::LogEntry> * m,
                          bool key, ::gateway::LogEntry * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_gateway_LogEntry_iter(
      const google::protobuf::Map<bool, ::gateway::LogEntry>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_gateway_LogEntry_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::gateway::LogEntry** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::gateway::LogEntry>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::gateway::LogEntry>* __rust_proto_thunk__Map_ProtoStr_gateway_LogEntry_new() {
    return new google::protobuf::Map<std::string, ::gateway::LogEntry>();
  }
  void __rust_proto_thunk__Map_ProtoStr_gateway_LogEntry_free(const google::protobuf::Map<std::string, ::gateway::LogEntry>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_gateway_LogEntry_clear(google::protobuf::Map<std::string, ::gateway::LogEntry> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_gateway_LogEntry_size(const google::protobuf::Map<std::string, ::gateway::LogEntry>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_gateway_LogEntry_insert(google::protobuf::Map<std::string, ::gateway::LogEntry> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::gateway::LogEntry value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_gateway_LogEntry_get(const google::protobuf::Map<std::string, ::gateway::LogEntry>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::gateway::LogEntry** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::gateway::LogEntry* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_gateway_LogEntry_remove(google::protobuf::Map<std::string, ::gateway::LogEntry> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::gateway::LogEntry * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_gateway_LogEntry_iter(
      const google::protobuf::Map<std::string, ::gateway::LogEntry>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_gateway_LogEntry_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::gateway::LogEntry** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::gateway::LogEntry>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// gateway.ConnectionsRequest
        // clang-format off
extern "C" {
void* __rust_proto_thunk__gateway_ConnectionsRequest_new() { return new ::gateway::ConnectionsRequest(); }
void __rust_proto_thunk__gateway_ConnectionsRequest_delete(void* ptr) { delete static_cast<::gateway::ConnectionsRequest*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__gateway_ConnectionsRequest_serialize(::gateway::ConnectionsRequest* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__gateway_ConnectionsRequest_parse(::gateway::ConnectionsRequest* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__gateway_ConnectionsRequest_copy_from(::gateway::ConnectionsRequest* dst, const ::gateway::ConnectionsRequest* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__gateway_ConnectionsRequest_repeated_len(google::protobuf::RepeatedPtrField<::gateway::ConnectionsRequest>* field) {
  return field->size();
}
const ::gateway::ConnectionsRequest& __rust_proto_thunk__gateway_ConnectionsRequest_repeated_get(
  google::protobuf::RepeatedPtrField<::gateway::ConnectionsRequest>* field,
  size_t index) {
  return field->Get(index);
}
::gateway::ConnectionsRequest* __rust_proto_thunk__gateway_ConnectionsRequest_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::gateway::ConnectionsRequest>* field,
  size_t index) {
  return field->Mutable(index);
}
::gateway::ConnectionsRequest* __rust_proto_thunk__gateway_ConnectionsRequest_repeated_add(google::protobuf::RepeatedPtrField<::gateway::ConnectionsRequest>* field) {
  return field->Add();
}
void __rust_proto_thunk__gateway_ConnectionsRequest_repeated_clear(google::protobuf::RepeatedPtrField<::gateway::ConnectionsRequest>* field) {
  field->Clear();
}
void __rust_proto_thunk__gateway_ConnectionsRequest_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::gateway::ConnectionsRequest>& dst,
  const google::protobuf::RepeatedPtrField<::gateway::ConnectionsRequest>& src) {
  dst = src;
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::gateway::ConnectionsRequest>* __rust_proto_thunk__Map_i32_gateway_ConnectionsRequest_new() {
    return new google::protobuf::Map<int32_t, ::gateway::ConnectionsRequest>();
  }
  void __rust_proto_thunk__Map_i32_gateway_ConnectionsRequest_free(const google::protobuf::Map<int32_t, ::gateway::ConnectionsRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_gateway_ConnectionsRequest_clear(google::protobuf::Map<int32_t, ::gateway::ConnectionsRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_gateway_ConnectionsRequest_size(const google::protobuf::Map<int32_t, ::gateway::ConnectionsRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_gateway_ConnectionsRequest_insert(google::protobuf::Map<int32_t, ::gateway::ConnectionsRequest> * m,
                          int32_t key, ::gateway::ConnectionsRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_gateway_ConnectionsRequest_get(const google::protobuf::Map<int32_t, ::gateway::ConnectionsRequest>* m,
                       int32_t key, const ::gateway::ConnectionsRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::gateway::ConnectionsRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_gateway_ConnectionsRequest_remove(google::protobuf::Map<int32_t, ::gateway::ConnectionsRequest> * m,
                          int32_t key, ::gateway::ConnectionsRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_gateway_ConnectionsRequest_iter(
      const google::protobuf::Map<int32_t, ::gateway::ConnectionsRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_gateway_ConnectionsRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::gateway::ConnectionsRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::gateway::ConnectionsRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::gateway::ConnectionsRequest>* __rust_proto_thunk__Map_u32_gateway_ConnectionsRequest_new() {
    return new google::protobuf::Map<uint32_t, ::gateway::ConnectionsRequest>();
  }
  void __rust_proto_thunk__Map_u32_gateway_ConnectionsRequest_free(const google::protobuf::Map<uint32_t, ::gateway::ConnectionsRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_gateway_ConnectionsRequest_clear(google::protobuf::Map<uint32_t, ::gateway::ConnectionsRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_gateway_ConnectionsRequest_size(const google::protobuf::Map<uint32_t, ::gateway::ConnectionsRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_gateway_ConnectionsRequest_insert(google::protobuf::Map<uint32_t, ::gateway::ConnectionsRequest> * m,
                          uint32_t key, ::gateway::ConnectionsRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_gateway_ConnectionsRequest_get(const google::protobuf::Map<uint32_t, ::gateway::ConnectionsRequest>* m,
                       uint32_t key, const ::gateway::ConnectionsRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::gateway::ConnectionsRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_gateway_ConnectionsRequest_remove(google::protobuf::Map<uint32_t, ::gateway::ConnectionsRequest> * m,
                          uint32_t key, ::gateway::ConnectionsRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_gateway_ConnectionsRequest_iter(
      const google::protobuf::Map<uint32_t, ::gateway::ConnectionsRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_gateway_ConnectionsRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::gateway::ConnectionsRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::gateway::ConnectionsRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::gateway::ConnectionsRequest>* __rust_proto_thunk__Map_i64_gateway_ConnectionsRequest_new() {
    return new google::protobuf::Map<int64_t, ::gateway::ConnectionsRequest>();
  }
  void __rust_proto_thunk__Map_i64_gateway_ConnectionsRequest_free(const google::protobuf::Map<int64_t, ::gateway::ConnectionsRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_gateway_ConnectionsRequest_clear(google::protobuf::Map<int64_t, ::gateway::ConnectionsRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_gateway_ConnectionsRequest_size(const google::protobuf::Map<int64_t, ::gateway::ConnectionsRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_gateway_ConnectionsRequest_insert(google::protobuf::Map<int64_t, ::gateway::ConnectionsRequest> * m,
                          int64_t key, ::gateway::ConnectionsRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_gateway_ConnectionsRequest_get(const google::protobuf::Map<int64_t, ::gateway::ConnectionsRequest>* m,
                       int64_t key, const ::gateway::ConnectionsRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::gateway::ConnectionsRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_gateway_ConnectionsRequest_remove(google::protobuf::Map<int64_t, ::gateway::ConnectionsRequest> * m,
                          int64_t key, ::gateway::ConnectionsRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_gateway_ConnectionsRequest_iter(
      const google::protobuf::Map<int64_t, ::gateway::ConnectionsRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_gateway_ConnectionsRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::gateway::ConnectionsRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::gateway::ConnectionsRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::gateway::ConnectionsRequest>* __rust_proto_thunk__Map_u64_gateway_ConnectionsRequest_new() {
    return new google::protobuf::Map<uint64_t, ::gateway::ConnectionsRequest>();
  }
  void __rust_proto_thunk__Map_u64_gateway_ConnectionsRequest_free(const google::protobuf::Map<uint64_t, ::gateway::ConnectionsRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_gateway_ConnectionsRequest_clear(google::protobuf::Map<uint64_t, ::gateway::ConnectionsRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_gateway_ConnectionsRequest_size(const google::protobuf::Map<uint64_t, ::gateway::ConnectionsRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_gateway_ConnectionsRequest_insert(google::protobuf::Map<uint64_t, ::gateway::ConnectionsRequest> * m,
                          uint64_t key, ::gateway::ConnectionsRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_gateway_ConnectionsRequest_get(const google::protobuf::Map<uint64_t, ::gateway::ConnectionsRequest>* m,
                       uint64_t key, const ::gateway::ConnectionsRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::gateway::ConnectionsRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_gateway_ConnectionsRequest_remove(google::protobuf::Map<uint64_t, ::gateway::ConnectionsRequest> * m,
                          uint64_t key, ::gateway::ConnectionsRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_gateway_ConnectionsRequest_iter(
      const google::protobuf::Map<uint64_t, ::gateway::ConnectionsRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_gateway_ConnectionsRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::gateway::ConnectionsRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::gateway::ConnectionsRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::gateway::ConnectionsRequest>* __rust_proto_thunk__Map_bool_gateway_ConnectionsRequest_new() {
    return new google::protobuf::Map<bool, ::gateway::ConnectionsRequest>();
  }
  void __rust_proto_thunk__Map_bool_gateway_ConnectionsRequest_free(const google::protobuf::Map<bool, ::gateway::ConnectionsRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_gateway_ConnectionsRequest_clear(google::protobuf::Map<bool, ::gateway::ConnectionsRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_gateway_ConnectionsRequest_size(const google::protobuf::Map<bool, ::gateway::ConnectionsRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_gateway_ConnectionsRequest_insert(google::protobuf::Map<bool, ::gateway::ConnectionsRequest> * m,
                          bool key, ::gateway::ConnectionsRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_gateway_ConnectionsRequest_get(const google::protobuf::Map<bool, ::gateway::ConnectionsRequest>* m,
                       bool key, const ::gateway::ConnectionsRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::gateway::ConnectionsRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_gateway_ConnectionsRequest_remove(google::protobuf::Map<bool, ::gateway::ConnectionsRequest> * m,
                          bool key, ::gateway::ConnectionsRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_gateway_ConnectionsRequest_iter(
      const google::protobuf::Map<bool, ::gateway::ConnectionsRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_gateway_ConnectionsRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::gateway::ConnectionsRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::gateway::ConnectionsRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::gateway::ConnectionsRequest>* __rust_proto_thunk__Map_ProtoStr_gateway_ConnectionsRequest_new() {
    return new google::protobuf::Map<std::string, ::gateway::ConnectionsRequest>();
  }
  void __rust_proto_thunk__Map_ProtoStr_gateway_ConnectionsRequest_free(const google::protobuf::Map<std::string, ::gateway::ConnectionsRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_gateway_ConnectionsRequest_clear(google::protobuf::Map<std::string, ::gateway::ConnectionsRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_gateway_ConnectionsRequest_size(const google::protobuf::Map<std::string, ::gateway::ConnectionsRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_gateway_ConnectionsRequest_insert(google::protobuf::Map<std::string, ::gateway::ConnectionsRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::gateway::ConnectionsRequest value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_gateway_ConnectionsRequest_get(const google::protobuf::Map<std::string, ::gateway::ConnectionsRequest>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::gateway::ConnectionsRequest** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::gateway::ConnectionsRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_gateway_ConnectionsRequest_remove(google::protobuf::Map<std::string, ::gateway::ConnectionsRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::gateway::ConnectionsRequest * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_gateway_ConnectionsRequest_iter(
      const google::protobuf::Map<std::string, ::gateway::ConnectionsRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_gateway_ConnectionsRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::gateway::ConnectionsRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::gateway::ConnectionsRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// gateway.ConnectionsResponse
        // clang-format off
extern "C" {
void* __rust_proto_thunk__gateway_ConnectionsResponse_new() { return new ::gateway::ConnectionsResponse(); }
void __rust_proto_thunk__gateway_ConnectionsResponse_delete(void* ptr) { delete static_cast<::gateway::ConnectionsResponse*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__gateway_ConnectionsResponse_serialize(::gateway::ConnectionsResponse* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__gateway_ConnectionsResponse_parse(::gateway::ConnectionsResponse* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__gateway_ConnectionsResponse_copy_from(::gateway::ConnectionsResponse* dst, const ::gateway::ConnectionsResponse* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__gateway_ConnectionsResponse_repeated_len(google::protobuf::RepeatedPtrField<::gateway::ConnectionsResponse>* field) {
  return field->size();
}
const ::gateway::ConnectionsResponse& __rust_proto_thunk__gateway_ConnectionsResponse_repeated_get(
  google::protobuf::RepeatedPtrField<::gateway::ConnectionsResponse>* field,
  size_t index) {
  return field->Get(index);
}
::gateway::ConnectionsResponse* __rust_proto_thunk__gateway_ConnectionsResponse_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::gateway::ConnectionsResponse>* field,
  size_t index) {
  return field->Mutable(index);
}
::gateway::ConnectionsResponse* __rust_proto_thunk__gateway_ConnectionsResponse_repeated_add(google::protobuf::RepeatedPtrField<::gateway::ConnectionsResponse>* field) {
  return field->Add();
}
void __rust_proto_thunk__gateway_ConnectionsResponse_repeated_clear(google::protobuf::RepeatedPtrField<::gateway::ConnectionsResponse>* field) {
  field->Clear();
}
void __rust_proto_thunk__gateway_ConnectionsResponse_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::gateway::ConnectionsResponse>& dst,
  const google::protobuf::RepeatedPtrField<::gateway::ConnectionsResponse>& src) {
  dst = src;
}

void __rust_proto_thunk__gateway_ConnectionsResponse_clear_active(::gateway::ConnectionsResponse* msg) {
  msg->clear_active();
}
google::protobuf::RepeatedPtrField<std::string>* __rust_proto_thunk__gateway_ConnectionsResponse_get_mut_active(::gateway::ConnectionsResponse* msg) {
  return msg->mutable_active();
}
const google::protobuf::RepeatedPtrField<std::string>* __rust_proto_thunk__gateway_ConnectionsResponse_get_active(
    const ::gateway::ConnectionsResponse* msg) {
  return &msg->active();
}

void __rust_proto_thunk__gateway_ConnectionsResponse_clear_created(::gateway::ConnectionsResponse* msg) {
  msg->clear_created();
}
google::protobuf::RepeatedPtrField<::gateway::LogEntry>* __rust_proto_thunk__gateway_ConnectionsResponse_get_mut_created(::gateway::ConnectionsResponse* msg) {
  return msg->mutable_created();
}
const google::protobuf::RepeatedPtrField<::gateway::LogEntry>* __rust_proto_thunk__gateway_ConnectionsResponse_get_created(
    const ::gateway::ConnectionsResponse* msg) {
  return &msg->created();
}

void __rust_proto_thunk__gateway_ConnectionsResponse_clear_removed(::gateway::ConnectionsResponse* msg) {
  msg->clear_removed();
}
google::protobuf::RepeatedPtrField<::gateway::LogEntry>* __rust_proto_thunk__gateway_ConnectionsResponse_get_mut_removed(::gateway::ConnectionsResponse* msg) {
  return msg->mutable_removed();
}
const google::protobuf::RepeatedPtrField<::gateway::LogEntry>* __rust_proto_thunk__gateway_ConnectionsResponse_get_removed(
    const ::gateway::ConnectionsResponse* msg) {
  return &msg->removed();
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::gateway::ConnectionsResponse>* __rust_proto_thunk__Map_i32_gateway_ConnectionsResponse_new() {
    return new google::protobuf::Map<int32_t, ::gateway::ConnectionsResponse>();
  }
  void __rust_proto_thunk__Map_i32_gateway_ConnectionsResponse_free(const google::protobuf::Map<int32_t, ::gateway::ConnectionsResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_gateway_ConnectionsResponse_clear(google::protobuf::Map<int32_t, ::gateway::ConnectionsResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_gateway_ConnectionsResponse_size(const google::protobuf::Map<int32_t, ::gateway::ConnectionsResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_gateway_ConnectionsResponse_insert(google::protobuf::Map<int32_t, ::gateway::ConnectionsResponse> * m,
                          int32_t key, ::gateway::ConnectionsResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_gateway_ConnectionsResponse_get(const google::protobuf::Map<int32_t, ::gateway::ConnectionsResponse>* m,
                       int32_t key, const ::gateway::ConnectionsResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::gateway::ConnectionsResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_gateway_ConnectionsResponse_remove(google::protobuf::Map<int32_t, ::gateway::ConnectionsResponse> * m,
                          int32_t key, ::gateway::ConnectionsResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_gateway_ConnectionsResponse_iter(
      const google::protobuf::Map<int32_t, ::gateway::ConnectionsResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_gateway_ConnectionsResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::gateway::ConnectionsResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::gateway::ConnectionsResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::gateway::ConnectionsResponse>* __rust_proto_thunk__Map_u32_gateway_ConnectionsResponse_new() {
    return new google::protobuf::Map<uint32_t, ::gateway::ConnectionsResponse>();
  }
  void __rust_proto_thunk__Map_u32_gateway_ConnectionsResponse_free(const google::protobuf::Map<uint32_t, ::gateway::ConnectionsResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_gateway_ConnectionsResponse_clear(google::protobuf::Map<uint32_t, ::gateway::ConnectionsResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_gateway_ConnectionsResponse_size(const google::protobuf::Map<uint32_t, ::gateway::ConnectionsResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_gateway_ConnectionsResponse_insert(google::protobuf::Map<uint32_t, ::gateway::ConnectionsResponse> * m,
                          uint32_t key, ::gateway::ConnectionsResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_gateway_ConnectionsResponse_get(const google::protobuf::Map<uint32_t, ::gateway::ConnectionsResponse>* m,
                       uint32_t key, const ::gateway::ConnectionsResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::gateway::ConnectionsResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_gateway_ConnectionsResponse_remove(google::protobuf::Map<uint32_t, ::gateway::ConnectionsResponse> * m,
                          uint32_t key, ::gateway::ConnectionsResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_gateway_ConnectionsResponse_iter(
      const google::protobuf::Map<uint32_t, ::gateway::ConnectionsResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_gateway_ConnectionsResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::gateway::ConnectionsResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::gateway::ConnectionsResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::gateway::ConnectionsResponse>* __rust_proto_thunk__Map_i64_gateway_ConnectionsResponse_new() {
    return new google::protobuf::Map<int64_t, ::gateway::ConnectionsResponse>();
  }
  void __rust_proto_thunk__Map_i64_gateway_ConnectionsResponse_free(const google::protobuf::Map<int64_t, ::gateway::ConnectionsResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_gateway_ConnectionsResponse_clear(google::protobuf::Map<int64_t, ::gateway::ConnectionsResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_gateway_ConnectionsResponse_size(const google::protobuf::Map<int64_t, ::gateway::ConnectionsResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_gateway_ConnectionsResponse_insert(google::protobuf::Map<int64_t, ::gateway::ConnectionsResponse> * m,
                          int64_t key, ::gateway::ConnectionsResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_gateway_ConnectionsResponse_get(const google::protobuf::Map<int64_t, ::gateway::ConnectionsResponse>* m,
                       int64_t key, const ::gateway::ConnectionsResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::gateway::ConnectionsResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_gateway_ConnectionsResponse_remove(google::protobuf::Map<int64_t, ::gateway::ConnectionsResponse> * m,
                          int64_t key, ::gateway::ConnectionsResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_gateway_ConnectionsResponse_iter(
      const google::protobuf::Map<int64_t, ::gateway::ConnectionsResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_gateway_ConnectionsResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::gateway::ConnectionsResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::gateway::ConnectionsResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::gateway::ConnectionsResponse>* __rust_proto_thunk__Map_u64_gateway_ConnectionsResponse_new() {
    return new google::protobuf::Map<uint64_t, ::gateway::ConnectionsResponse>();
  }
  void __rust_proto_thunk__Map_u64_gateway_ConnectionsResponse_free(const google::protobuf::Map<uint64_t, ::gateway::ConnectionsResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_gateway_ConnectionsResponse_clear(google::protobuf::Map<uint64_t, ::gateway::ConnectionsResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_gateway_ConnectionsResponse_size(const google::protobuf::Map<uint64_t, ::gateway::ConnectionsResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_gateway_ConnectionsResponse_insert(google::protobuf::Map<uint64_t, ::gateway::ConnectionsResponse> * m,
                          uint64_t key, ::gateway::ConnectionsResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_gateway_ConnectionsResponse_get(const google::protobuf::Map<uint64_t, ::gateway::ConnectionsResponse>* m,
                       uint64_t key, const ::gateway::ConnectionsResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::gateway::ConnectionsResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_gateway_ConnectionsResponse_remove(google::protobuf::Map<uint64_t, ::gateway::ConnectionsResponse> * m,
                          uint64_t key, ::gateway::ConnectionsResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_gateway_ConnectionsResponse_iter(
      const google::protobuf::Map<uint64_t, ::gateway::ConnectionsResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_gateway_ConnectionsResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::gateway::ConnectionsResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::gateway::ConnectionsResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::gateway::ConnectionsResponse>* __rust_proto_thunk__Map_bool_gateway_ConnectionsResponse_new() {
    return new google::protobuf::Map<bool, ::gateway::ConnectionsResponse>();
  }
  void __rust_proto_thunk__Map_bool_gateway_ConnectionsResponse_free(const google::protobuf::Map<bool, ::gateway::ConnectionsResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_gateway_ConnectionsResponse_clear(google::protobuf::Map<bool, ::gateway::ConnectionsResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_gateway_ConnectionsResponse_size(const google::protobuf::Map<bool, ::gateway::ConnectionsResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_gateway_ConnectionsResponse_insert(google::protobuf::Map<bool, ::gateway::ConnectionsResponse> * m,
                          bool key, ::gateway::ConnectionsResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_gateway_ConnectionsResponse_get(const google::protobuf::Map<bool, ::gateway::ConnectionsResponse>* m,
                       bool key, const ::gateway::ConnectionsResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::gateway::ConnectionsResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_gateway_ConnectionsResponse_remove(google::protobuf::Map<bool, ::gateway::ConnectionsResponse> * m,
                          bool key, ::gateway::ConnectionsResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_gateway_ConnectionsResponse_iter(
      const google::protobuf::Map<bool, ::gateway::ConnectionsResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_gateway_ConnectionsResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::gateway::ConnectionsResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::gateway::ConnectionsResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::gateway::ConnectionsResponse>* __rust_proto_thunk__Map_ProtoStr_gateway_ConnectionsResponse_new() {
    return new google::protobuf::Map<std::string, ::gateway::ConnectionsResponse>();
  }
  void __rust_proto_thunk__Map_ProtoStr_gateway_ConnectionsResponse_free(const google::protobuf::Map<std::string, ::gateway::ConnectionsResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_gateway_ConnectionsResponse_clear(google::protobuf::Map<std::string, ::gateway::ConnectionsResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_gateway_ConnectionsResponse_size(const google::protobuf::Map<std::string, ::gateway::ConnectionsResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_gateway_ConnectionsResponse_insert(google::protobuf::Map<std::string, ::gateway::ConnectionsResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::gateway::ConnectionsResponse value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_gateway_ConnectionsResponse_get(const google::protobuf::Map<std::string, ::gateway::ConnectionsResponse>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::gateway::ConnectionsResponse** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::gateway::ConnectionsResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_gateway_ConnectionsResponse_remove(google::protobuf::Map<std::string, ::gateway::ConnectionsResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::gateway::ConnectionsResponse * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_gateway_ConnectionsResponse_iter(
      const google::protobuf::Map<std::string, ::gateway::ConnectionsResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_gateway_ConnectionsResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::gateway::ConnectionsResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::gateway::ConnectionsResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

