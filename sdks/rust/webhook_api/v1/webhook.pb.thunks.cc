#include "webhook_api/v1/webhook.proto.h"
#include "google/protobuf/rust/cpp_kernel/cpp_api.h"
// webhook_api.v1.Webhook
        // clang-format off
extern "C" {
void* __rust_proto_thunk__webhook_api_v1_Webhook_new() { return new ::webhook_api::v1::Webhook(); }
void __rust_proto_thunk__webhook_api_v1_Webhook_delete(void* ptr) { delete static_cast<::webhook_api::v1::Webhook*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__webhook_api_v1_Webhook_serialize(::webhook_api::v1::Webhook* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__webhook_api_v1_Webhook_parse(::webhook_api::v1::Webhook* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__webhook_api_v1_Webhook_copy_from(::webhook_api::v1::Webhook* dst, const ::webhook_api::v1::Webhook* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__webhook_api_v1_Webhook_repeated_len(google::protobuf::RepeatedPtrField<::webhook_api::v1::Webhook>* field) {
  return field->size();
}
const ::webhook_api::v1::Webhook& __rust_proto_thunk__webhook_api_v1_Webhook_repeated_get(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::Webhook>* field,
  size_t index) {
  return field->Get(index);
}
::webhook_api::v1::Webhook* __rust_proto_thunk__webhook_api_v1_Webhook_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::Webhook>* field,
  size_t index) {
  return field->Mutable(index);
}
::webhook_api::v1::Webhook* __rust_proto_thunk__webhook_api_v1_Webhook_repeated_add(google::protobuf::RepeatedPtrField<::webhook_api::v1::Webhook>* field) {
  return field->Add();
}
void __rust_proto_thunk__webhook_api_v1_Webhook_repeated_clear(google::protobuf::RepeatedPtrField<::webhook_api::v1::Webhook>* field) {
  field->Clear();
}
void __rust_proto_thunk__webhook_api_v1_Webhook_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::Webhook>& dst,
  const google::protobuf::RepeatedPtrField<::webhook_api::v1::Webhook>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__webhook_api_v1_Webhook_get_id(::webhook_api::v1::Webhook* msg) {
  absl::string_view val = msg->id();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__webhook_api_v1_Webhook_set_id(::webhook_api::v1::Webhook* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_id(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__webhook_api_v1_Webhook_get_owner_id(::webhook_api::v1::Webhook* msg) {
  absl::string_view val = msg->owner_id();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__webhook_api_v1_Webhook_set_owner_id(::webhook_api::v1::Webhook* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_owner_id(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__webhook_api_v1_Webhook_get_name(::webhook_api::v1::Webhook* msg) {
  absl::string_view val = msg->name();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__webhook_api_v1_Webhook_set_name(::webhook_api::v1::Webhook* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_name(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__webhook_api_v1_Webhook_get_webhook_url(::webhook_api::v1::Webhook* msg) {
  absl::string_view val = msg->webhook_url();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__webhook_api_v1_Webhook_set_webhook_url(::webhook_api::v1::Webhook* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_webhook_url(absl::string_view(s.ptr, s.len));
}

void __rust_proto_thunk__webhook_api_v1_Webhook_clear_supported_events(::webhook_api::v1::Webhook* msg) {
  msg->clear_supported_events();
}
google::protobuf::RepeatedField<int>* __rust_proto_thunk__webhook_api_v1_Webhook_get_mut_supported_events(::webhook_api::v1::Webhook* msg) {
  return msg->mutable_supported_events();
}
const google::protobuf::RepeatedField<int>* __rust_proto_thunk__webhook_api_v1_Webhook_get_supported_events(
    const ::webhook_api::v1::Webhook* msg) {
  return &msg->supported_events();
}

::webhook_api::v1::DeliveryMethodType __rust_proto_thunk__webhook_api_v1_Webhook_get_delivery_method(::webhook_api::v1::Webhook* msg) { return msg->delivery_method(); }
void __rust_proto_thunk__webhook_api_v1_Webhook_set_delivery_method(::webhook_api::v1::Webhook* msg, ::webhook_api::v1::DeliveryMethodType val) {
  msg->set_delivery_method(val);
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::webhook_api::v1::Webhook>* __rust_proto_thunk__Map_i32_webhook_api_v1_Webhook_new() {
    return new google::protobuf::Map<int32_t, ::webhook_api::v1::Webhook>();
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_Webhook_free(const google::protobuf::Map<int32_t, ::webhook_api::v1::Webhook>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_Webhook_clear(google::protobuf::Map<int32_t, ::webhook_api::v1::Webhook> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_webhook_api_v1_Webhook_size(const google::protobuf::Map<int32_t, ::webhook_api::v1::Webhook>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_Webhook_insert(google::protobuf::Map<int32_t, ::webhook_api::v1::Webhook> * m,
                          int32_t key, ::webhook_api::v1::Webhook value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_Webhook_get(const google::protobuf::Map<int32_t, ::webhook_api::v1::Webhook>* m,
                       int32_t key, const ::webhook_api::v1::Webhook** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::Webhook* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_Webhook_remove(google::protobuf::Map<int32_t, ::webhook_api::v1::Webhook> * m,
                          int32_t key, ::webhook_api::v1::Webhook * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_webhook_api_v1_Webhook_iter(
      const google::protobuf::Map<int32_t, ::webhook_api::v1::Webhook>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_Webhook_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::webhook_api::v1::Webhook** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::webhook_api::v1::Webhook>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::webhook_api::v1::Webhook>* __rust_proto_thunk__Map_u32_webhook_api_v1_Webhook_new() {
    return new google::protobuf::Map<uint32_t, ::webhook_api::v1::Webhook>();
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_Webhook_free(const google::protobuf::Map<uint32_t, ::webhook_api::v1::Webhook>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_Webhook_clear(google::protobuf::Map<uint32_t, ::webhook_api::v1::Webhook> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_webhook_api_v1_Webhook_size(const google::protobuf::Map<uint32_t, ::webhook_api::v1::Webhook>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_Webhook_insert(google::protobuf::Map<uint32_t, ::webhook_api::v1::Webhook> * m,
                          uint32_t key, ::webhook_api::v1::Webhook value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_Webhook_get(const google::protobuf::Map<uint32_t, ::webhook_api::v1::Webhook>* m,
                       uint32_t key, const ::webhook_api::v1::Webhook** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::Webhook* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_Webhook_remove(google::protobuf::Map<uint32_t, ::webhook_api::v1::Webhook> * m,
                          uint32_t key, ::webhook_api::v1::Webhook * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_webhook_api_v1_Webhook_iter(
      const google::protobuf::Map<uint32_t, ::webhook_api::v1::Webhook>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_Webhook_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::webhook_api::v1::Webhook** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::webhook_api::v1::Webhook>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::webhook_api::v1::Webhook>* __rust_proto_thunk__Map_i64_webhook_api_v1_Webhook_new() {
    return new google::protobuf::Map<int64_t, ::webhook_api::v1::Webhook>();
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_Webhook_free(const google::protobuf::Map<int64_t, ::webhook_api::v1::Webhook>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_Webhook_clear(google::protobuf::Map<int64_t, ::webhook_api::v1::Webhook> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_webhook_api_v1_Webhook_size(const google::protobuf::Map<int64_t, ::webhook_api::v1::Webhook>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_Webhook_insert(google::protobuf::Map<int64_t, ::webhook_api::v1::Webhook> * m,
                          int64_t key, ::webhook_api::v1::Webhook value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_Webhook_get(const google::protobuf::Map<int64_t, ::webhook_api::v1::Webhook>* m,
                       int64_t key, const ::webhook_api::v1::Webhook** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::Webhook* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_Webhook_remove(google::protobuf::Map<int64_t, ::webhook_api::v1::Webhook> * m,
                          int64_t key, ::webhook_api::v1::Webhook * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_webhook_api_v1_Webhook_iter(
      const google::protobuf::Map<int64_t, ::webhook_api::v1::Webhook>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_Webhook_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::webhook_api::v1::Webhook** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::webhook_api::v1::Webhook>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::webhook_api::v1::Webhook>* __rust_proto_thunk__Map_u64_webhook_api_v1_Webhook_new() {
    return new google::protobuf::Map<uint64_t, ::webhook_api::v1::Webhook>();
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_Webhook_free(const google::protobuf::Map<uint64_t, ::webhook_api::v1::Webhook>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_Webhook_clear(google::protobuf::Map<uint64_t, ::webhook_api::v1::Webhook> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_webhook_api_v1_Webhook_size(const google::protobuf::Map<uint64_t, ::webhook_api::v1::Webhook>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_Webhook_insert(google::protobuf::Map<uint64_t, ::webhook_api::v1::Webhook> * m,
                          uint64_t key, ::webhook_api::v1::Webhook value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_Webhook_get(const google::protobuf::Map<uint64_t, ::webhook_api::v1::Webhook>* m,
                       uint64_t key, const ::webhook_api::v1::Webhook** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::Webhook* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_Webhook_remove(google::protobuf::Map<uint64_t, ::webhook_api::v1::Webhook> * m,
                          uint64_t key, ::webhook_api::v1::Webhook * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_webhook_api_v1_Webhook_iter(
      const google::protobuf::Map<uint64_t, ::webhook_api::v1::Webhook>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_Webhook_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::webhook_api::v1::Webhook** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::webhook_api::v1::Webhook>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::webhook_api::v1::Webhook>* __rust_proto_thunk__Map_bool_webhook_api_v1_Webhook_new() {
    return new google::protobuf::Map<bool, ::webhook_api::v1::Webhook>();
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_Webhook_free(const google::protobuf::Map<bool, ::webhook_api::v1::Webhook>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_Webhook_clear(google::protobuf::Map<bool, ::webhook_api::v1::Webhook> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_webhook_api_v1_Webhook_size(const google::protobuf::Map<bool, ::webhook_api::v1::Webhook>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_Webhook_insert(google::protobuf::Map<bool, ::webhook_api::v1::Webhook> * m,
                          bool key, ::webhook_api::v1::Webhook value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_Webhook_get(const google::protobuf::Map<bool, ::webhook_api::v1::Webhook>* m,
                       bool key, const ::webhook_api::v1::Webhook** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::Webhook* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_Webhook_remove(google::protobuf::Map<bool, ::webhook_api::v1::Webhook> * m,
                          bool key, ::webhook_api::v1::Webhook * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_webhook_api_v1_Webhook_iter(
      const google::protobuf::Map<bool, ::webhook_api::v1::Webhook>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_Webhook_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::webhook_api::v1::Webhook** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::webhook_api::v1::Webhook>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::webhook_api::v1::Webhook>* __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_Webhook_new() {
    return new google::protobuf::Map<std::string, ::webhook_api::v1::Webhook>();
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_Webhook_free(const google::protobuf::Map<std::string, ::webhook_api::v1::Webhook>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_Webhook_clear(google::protobuf::Map<std::string, ::webhook_api::v1::Webhook> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_Webhook_size(const google::protobuf::Map<std::string, ::webhook_api::v1::Webhook>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_Webhook_insert(google::protobuf::Map<std::string, ::webhook_api::v1::Webhook> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::Webhook value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_Webhook_get(const google::protobuf::Map<std::string, ::webhook_api::v1::Webhook>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::webhook_api::v1::Webhook** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::Webhook* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_Webhook_remove(google::protobuf::Map<std::string, ::webhook_api::v1::Webhook> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::Webhook * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_Webhook_iter(
      const google::protobuf::Map<std::string, ::webhook_api::v1::Webhook>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_Webhook_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::webhook_api::v1::Webhook** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::webhook_api::v1::Webhook>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// webhook_api.v1.CreateWalletPayload
        // clang-format off
extern "C" {
void* __rust_proto_thunk__webhook_api_v1_CreateWalletPayload_new() { return new ::webhook_api::v1::CreateWalletPayload(); }
void __rust_proto_thunk__webhook_api_v1_CreateWalletPayload_delete(void* ptr) { delete static_cast<::webhook_api::v1::CreateWalletPayload*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__webhook_api_v1_CreateWalletPayload_serialize(::webhook_api::v1::CreateWalletPayload* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__webhook_api_v1_CreateWalletPayload_parse(::webhook_api::v1::CreateWalletPayload* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__webhook_api_v1_CreateWalletPayload_copy_from(::webhook_api::v1::CreateWalletPayload* dst, const ::webhook_api::v1::CreateWalletPayload* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__webhook_api_v1_CreateWalletPayload_repeated_len(google::protobuf::RepeatedPtrField<::webhook_api::v1::CreateWalletPayload>* field) {
  return field->size();
}
const ::webhook_api::v1::CreateWalletPayload& __rust_proto_thunk__webhook_api_v1_CreateWalletPayload_repeated_get(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::CreateWalletPayload>* field,
  size_t index) {
  return field->Get(index);
}
::webhook_api::v1::CreateWalletPayload* __rust_proto_thunk__webhook_api_v1_CreateWalletPayload_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::CreateWalletPayload>* field,
  size_t index) {
  return field->Mutable(index);
}
::webhook_api::v1::CreateWalletPayload* __rust_proto_thunk__webhook_api_v1_CreateWalletPayload_repeated_add(google::protobuf::RepeatedPtrField<::webhook_api::v1::CreateWalletPayload>* field) {
  return field->Add();
}
void __rust_proto_thunk__webhook_api_v1_CreateWalletPayload_repeated_clear(google::protobuf::RepeatedPtrField<::webhook_api::v1::CreateWalletPayload>* field) {
  field->Clear();
}
void __rust_proto_thunk__webhook_api_v1_CreateWalletPayload_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::CreateWalletPayload>& dst,
  const google::protobuf::RepeatedPtrField<::webhook_api::v1::CreateWalletPayload>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__webhook_api_v1_CreateWalletPayload_get_name(::webhook_api::v1::CreateWalletPayload* msg) {
  absl::string_view val = msg->name();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__webhook_api_v1_CreateWalletPayload_set_name(::webhook_api::v1::CreateWalletPayload* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_name(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__webhook_api_v1_CreateWalletPayload_get_did(::webhook_api::v1::CreateWalletPayload* msg) {
  absl::string_view val = msg->did();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__webhook_api_v1_CreateWalletPayload_set_did(::webhook_api::v1::CreateWalletPayload* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_did(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__webhook_api_v1_CreateWalletPayload_get_created(::webhook_api::v1::CreateWalletPayload* msg) {
  absl::string_view val = msg->created();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__webhook_api_v1_CreateWalletPayload_set_created(::webhook_api::v1::CreateWalletPayload* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_created(absl::string_view(s.ptr, s.len));
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::webhook_api::v1::CreateWalletPayload>* __rust_proto_thunk__Map_i32_webhook_api_v1_CreateWalletPayload_new() {
    return new google::protobuf::Map<int32_t, ::webhook_api::v1::CreateWalletPayload>();
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_CreateWalletPayload_free(const google::protobuf::Map<int32_t, ::webhook_api::v1::CreateWalletPayload>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_CreateWalletPayload_clear(google::protobuf::Map<int32_t, ::webhook_api::v1::CreateWalletPayload> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_webhook_api_v1_CreateWalletPayload_size(const google::protobuf::Map<int32_t, ::webhook_api::v1::CreateWalletPayload>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_CreateWalletPayload_insert(google::protobuf::Map<int32_t, ::webhook_api::v1::CreateWalletPayload> * m,
                          int32_t key, ::webhook_api::v1::CreateWalletPayload value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_CreateWalletPayload_get(const google::protobuf::Map<int32_t, ::webhook_api::v1::CreateWalletPayload>* m,
                       int32_t key, const ::webhook_api::v1::CreateWalletPayload** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::CreateWalletPayload* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_CreateWalletPayload_remove(google::protobuf::Map<int32_t, ::webhook_api::v1::CreateWalletPayload> * m,
                          int32_t key, ::webhook_api::v1::CreateWalletPayload * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_webhook_api_v1_CreateWalletPayload_iter(
      const google::protobuf::Map<int32_t, ::webhook_api::v1::CreateWalletPayload>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_CreateWalletPayload_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::webhook_api::v1::CreateWalletPayload** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::webhook_api::v1::CreateWalletPayload>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::webhook_api::v1::CreateWalletPayload>* __rust_proto_thunk__Map_u32_webhook_api_v1_CreateWalletPayload_new() {
    return new google::protobuf::Map<uint32_t, ::webhook_api::v1::CreateWalletPayload>();
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_CreateWalletPayload_free(const google::protobuf::Map<uint32_t, ::webhook_api::v1::CreateWalletPayload>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_CreateWalletPayload_clear(google::protobuf::Map<uint32_t, ::webhook_api::v1::CreateWalletPayload> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_webhook_api_v1_CreateWalletPayload_size(const google::protobuf::Map<uint32_t, ::webhook_api::v1::CreateWalletPayload>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_CreateWalletPayload_insert(google::protobuf::Map<uint32_t, ::webhook_api::v1::CreateWalletPayload> * m,
                          uint32_t key, ::webhook_api::v1::CreateWalletPayload value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_CreateWalletPayload_get(const google::protobuf::Map<uint32_t, ::webhook_api::v1::CreateWalletPayload>* m,
                       uint32_t key, const ::webhook_api::v1::CreateWalletPayload** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::CreateWalletPayload* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_CreateWalletPayload_remove(google::protobuf::Map<uint32_t, ::webhook_api::v1::CreateWalletPayload> * m,
                          uint32_t key, ::webhook_api::v1::CreateWalletPayload * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_webhook_api_v1_CreateWalletPayload_iter(
      const google::protobuf::Map<uint32_t, ::webhook_api::v1::CreateWalletPayload>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_CreateWalletPayload_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::webhook_api::v1::CreateWalletPayload** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::webhook_api::v1::CreateWalletPayload>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::webhook_api::v1::CreateWalletPayload>* __rust_proto_thunk__Map_i64_webhook_api_v1_CreateWalletPayload_new() {
    return new google::protobuf::Map<int64_t, ::webhook_api::v1::CreateWalletPayload>();
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_CreateWalletPayload_free(const google::protobuf::Map<int64_t, ::webhook_api::v1::CreateWalletPayload>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_CreateWalletPayload_clear(google::protobuf::Map<int64_t, ::webhook_api::v1::CreateWalletPayload> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_webhook_api_v1_CreateWalletPayload_size(const google::protobuf::Map<int64_t, ::webhook_api::v1::CreateWalletPayload>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_CreateWalletPayload_insert(google::protobuf::Map<int64_t, ::webhook_api::v1::CreateWalletPayload> * m,
                          int64_t key, ::webhook_api::v1::CreateWalletPayload value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_CreateWalletPayload_get(const google::protobuf::Map<int64_t, ::webhook_api::v1::CreateWalletPayload>* m,
                       int64_t key, const ::webhook_api::v1::CreateWalletPayload** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::CreateWalletPayload* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_CreateWalletPayload_remove(google::protobuf::Map<int64_t, ::webhook_api::v1::CreateWalletPayload> * m,
                          int64_t key, ::webhook_api::v1::CreateWalletPayload * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_webhook_api_v1_CreateWalletPayload_iter(
      const google::protobuf::Map<int64_t, ::webhook_api::v1::CreateWalletPayload>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_CreateWalletPayload_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::webhook_api::v1::CreateWalletPayload** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::webhook_api::v1::CreateWalletPayload>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::webhook_api::v1::CreateWalletPayload>* __rust_proto_thunk__Map_u64_webhook_api_v1_CreateWalletPayload_new() {
    return new google::protobuf::Map<uint64_t, ::webhook_api::v1::CreateWalletPayload>();
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_CreateWalletPayload_free(const google::protobuf::Map<uint64_t, ::webhook_api::v1::CreateWalletPayload>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_CreateWalletPayload_clear(google::protobuf::Map<uint64_t, ::webhook_api::v1::CreateWalletPayload> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_webhook_api_v1_CreateWalletPayload_size(const google::protobuf::Map<uint64_t, ::webhook_api::v1::CreateWalletPayload>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_CreateWalletPayload_insert(google::protobuf::Map<uint64_t, ::webhook_api::v1::CreateWalletPayload> * m,
                          uint64_t key, ::webhook_api::v1::CreateWalletPayload value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_CreateWalletPayload_get(const google::protobuf::Map<uint64_t, ::webhook_api::v1::CreateWalletPayload>* m,
                       uint64_t key, const ::webhook_api::v1::CreateWalletPayload** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::CreateWalletPayload* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_CreateWalletPayload_remove(google::protobuf::Map<uint64_t, ::webhook_api::v1::CreateWalletPayload> * m,
                          uint64_t key, ::webhook_api::v1::CreateWalletPayload * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_webhook_api_v1_CreateWalletPayload_iter(
      const google::protobuf::Map<uint64_t, ::webhook_api::v1::CreateWalletPayload>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_CreateWalletPayload_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::webhook_api::v1::CreateWalletPayload** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::webhook_api::v1::CreateWalletPayload>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::webhook_api::v1::CreateWalletPayload>* __rust_proto_thunk__Map_bool_webhook_api_v1_CreateWalletPayload_new() {
    return new google::protobuf::Map<bool, ::webhook_api::v1::CreateWalletPayload>();
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_CreateWalletPayload_free(const google::protobuf::Map<bool, ::webhook_api::v1::CreateWalletPayload>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_CreateWalletPayload_clear(google::protobuf::Map<bool, ::webhook_api::v1::CreateWalletPayload> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_webhook_api_v1_CreateWalletPayload_size(const google::protobuf::Map<bool, ::webhook_api::v1::CreateWalletPayload>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_CreateWalletPayload_insert(google::protobuf::Map<bool, ::webhook_api::v1::CreateWalletPayload> * m,
                          bool key, ::webhook_api::v1::CreateWalletPayload value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_CreateWalletPayload_get(const google::protobuf::Map<bool, ::webhook_api::v1::CreateWalletPayload>* m,
                       bool key, const ::webhook_api::v1::CreateWalletPayload** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::CreateWalletPayload* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_CreateWalletPayload_remove(google::protobuf::Map<bool, ::webhook_api::v1::CreateWalletPayload> * m,
                          bool key, ::webhook_api::v1::CreateWalletPayload * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_webhook_api_v1_CreateWalletPayload_iter(
      const google::protobuf::Map<bool, ::webhook_api::v1::CreateWalletPayload>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_CreateWalletPayload_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::webhook_api::v1::CreateWalletPayload** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::webhook_api::v1::CreateWalletPayload>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::webhook_api::v1::CreateWalletPayload>* __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_CreateWalletPayload_new() {
    return new google::protobuf::Map<std::string, ::webhook_api::v1::CreateWalletPayload>();
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_CreateWalletPayload_free(const google::protobuf::Map<std::string, ::webhook_api::v1::CreateWalletPayload>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_CreateWalletPayload_clear(google::protobuf::Map<std::string, ::webhook_api::v1::CreateWalletPayload> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_CreateWalletPayload_size(const google::protobuf::Map<std::string, ::webhook_api::v1::CreateWalletPayload>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_CreateWalletPayload_insert(google::protobuf::Map<std::string, ::webhook_api::v1::CreateWalletPayload> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::CreateWalletPayload value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_CreateWalletPayload_get(const google::protobuf::Map<std::string, ::webhook_api::v1::CreateWalletPayload>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::webhook_api::v1::CreateWalletPayload** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::CreateWalletPayload* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_CreateWalletPayload_remove(google::protobuf::Map<std::string, ::webhook_api::v1::CreateWalletPayload> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::CreateWalletPayload * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_CreateWalletPayload_iter(
      const google::protobuf::Map<std::string, ::webhook_api::v1::CreateWalletPayload>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_CreateWalletPayload_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::webhook_api::v1::CreateWalletPayload** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::webhook_api::v1::CreateWalletPayload>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// webhook_api.v1.PromissoryReceivedPayload
        // clang-format off
extern "C" {
void* __rust_proto_thunk__webhook_api_v1_PromissoryReceivedPayload_new() { return new ::webhook_api::v1::PromissoryReceivedPayload(); }
void __rust_proto_thunk__webhook_api_v1_PromissoryReceivedPayload_delete(void* ptr) { delete static_cast<::webhook_api::v1::PromissoryReceivedPayload*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__webhook_api_v1_PromissoryReceivedPayload_serialize(::webhook_api::v1::PromissoryReceivedPayload* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__webhook_api_v1_PromissoryReceivedPayload_parse(::webhook_api::v1::PromissoryReceivedPayload* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__webhook_api_v1_PromissoryReceivedPayload_copy_from(::webhook_api::v1::PromissoryReceivedPayload* dst, const ::webhook_api::v1::PromissoryReceivedPayload* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__webhook_api_v1_PromissoryReceivedPayload_repeated_len(google::protobuf::RepeatedPtrField<::webhook_api::v1::PromissoryReceivedPayload>* field) {
  return field->size();
}
const ::webhook_api::v1::PromissoryReceivedPayload& __rust_proto_thunk__webhook_api_v1_PromissoryReceivedPayload_repeated_get(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::PromissoryReceivedPayload>* field,
  size_t index) {
  return field->Get(index);
}
::webhook_api::v1::PromissoryReceivedPayload* __rust_proto_thunk__webhook_api_v1_PromissoryReceivedPayload_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::PromissoryReceivedPayload>* field,
  size_t index) {
  return field->Mutable(index);
}
::webhook_api::v1::PromissoryReceivedPayload* __rust_proto_thunk__webhook_api_v1_PromissoryReceivedPayload_repeated_add(google::protobuf::RepeatedPtrField<::webhook_api::v1::PromissoryReceivedPayload>* field) {
  return field->Add();
}
void __rust_proto_thunk__webhook_api_v1_PromissoryReceivedPayload_repeated_clear(google::protobuf::RepeatedPtrField<::webhook_api::v1::PromissoryReceivedPayload>* field) {
  field->Clear();
}
void __rust_proto_thunk__webhook_api_v1_PromissoryReceivedPayload_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::PromissoryReceivedPayload>& dst,
  const google::protobuf::RepeatedPtrField<::webhook_api::v1::PromissoryReceivedPayload>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__webhook_api_v1_PromissoryReceivedPayload_get_wallet_id(::webhook_api::v1::PromissoryReceivedPayload* msg) {
  absl::string_view val = msg->wallet_id();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__webhook_api_v1_PromissoryReceivedPayload_set_wallet_id(::webhook_api::v1::PromissoryReceivedPayload* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_wallet_id(absl::string_view(s.ptr, s.len));
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::webhook_api::v1::PromissoryReceivedPayload>* __rust_proto_thunk__Map_i32_webhook_api_v1_PromissoryReceivedPayload_new() {
    return new google::protobuf::Map<int32_t, ::webhook_api::v1::PromissoryReceivedPayload>();
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_PromissoryReceivedPayload_free(const google::protobuf::Map<int32_t, ::webhook_api::v1::PromissoryReceivedPayload>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_PromissoryReceivedPayload_clear(google::protobuf::Map<int32_t, ::webhook_api::v1::PromissoryReceivedPayload> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_webhook_api_v1_PromissoryReceivedPayload_size(const google::protobuf::Map<int32_t, ::webhook_api::v1::PromissoryReceivedPayload>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_PromissoryReceivedPayload_insert(google::protobuf::Map<int32_t, ::webhook_api::v1::PromissoryReceivedPayload> * m,
                          int32_t key, ::webhook_api::v1::PromissoryReceivedPayload value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_PromissoryReceivedPayload_get(const google::protobuf::Map<int32_t, ::webhook_api::v1::PromissoryReceivedPayload>* m,
                       int32_t key, const ::webhook_api::v1::PromissoryReceivedPayload** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::PromissoryReceivedPayload* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_PromissoryReceivedPayload_remove(google::protobuf::Map<int32_t, ::webhook_api::v1::PromissoryReceivedPayload> * m,
                          int32_t key, ::webhook_api::v1::PromissoryReceivedPayload * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_webhook_api_v1_PromissoryReceivedPayload_iter(
      const google::protobuf::Map<int32_t, ::webhook_api::v1::PromissoryReceivedPayload>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_PromissoryReceivedPayload_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::webhook_api::v1::PromissoryReceivedPayload** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::webhook_api::v1::PromissoryReceivedPayload>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::webhook_api::v1::PromissoryReceivedPayload>* __rust_proto_thunk__Map_u32_webhook_api_v1_PromissoryReceivedPayload_new() {
    return new google::protobuf::Map<uint32_t, ::webhook_api::v1::PromissoryReceivedPayload>();
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_PromissoryReceivedPayload_free(const google::protobuf::Map<uint32_t, ::webhook_api::v1::PromissoryReceivedPayload>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_PromissoryReceivedPayload_clear(google::protobuf::Map<uint32_t, ::webhook_api::v1::PromissoryReceivedPayload> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_webhook_api_v1_PromissoryReceivedPayload_size(const google::protobuf::Map<uint32_t, ::webhook_api::v1::PromissoryReceivedPayload>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_PromissoryReceivedPayload_insert(google::protobuf::Map<uint32_t, ::webhook_api::v1::PromissoryReceivedPayload> * m,
                          uint32_t key, ::webhook_api::v1::PromissoryReceivedPayload value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_PromissoryReceivedPayload_get(const google::protobuf::Map<uint32_t, ::webhook_api::v1::PromissoryReceivedPayload>* m,
                       uint32_t key, const ::webhook_api::v1::PromissoryReceivedPayload** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::PromissoryReceivedPayload* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_PromissoryReceivedPayload_remove(google::protobuf::Map<uint32_t, ::webhook_api::v1::PromissoryReceivedPayload> * m,
                          uint32_t key, ::webhook_api::v1::PromissoryReceivedPayload * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_webhook_api_v1_PromissoryReceivedPayload_iter(
      const google::protobuf::Map<uint32_t, ::webhook_api::v1::PromissoryReceivedPayload>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_PromissoryReceivedPayload_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::webhook_api::v1::PromissoryReceivedPayload** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::webhook_api::v1::PromissoryReceivedPayload>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::webhook_api::v1::PromissoryReceivedPayload>* __rust_proto_thunk__Map_i64_webhook_api_v1_PromissoryReceivedPayload_new() {
    return new google::protobuf::Map<int64_t, ::webhook_api::v1::PromissoryReceivedPayload>();
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_PromissoryReceivedPayload_free(const google::protobuf::Map<int64_t, ::webhook_api::v1::PromissoryReceivedPayload>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_PromissoryReceivedPayload_clear(google::protobuf::Map<int64_t, ::webhook_api::v1::PromissoryReceivedPayload> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_webhook_api_v1_PromissoryReceivedPayload_size(const google::protobuf::Map<int64_t, ::webhook_api::v1::PromissoryReceivedPayload>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_PromissoryReceivedPayload_insert(google::protobuf::Map<int64_t, ::webhook_api::v1::PromissoryReceivedPayload> * m,
                          int64_t key, ::webhook_api::v1::PromissoryReceivedPayload value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_PromissoryReceivedPayload_get(const google::protobuf::Map<int64_t, ::webhook_api::v1::PromissoryReceivedPayload>* m,
                       int64_t key, const ::webhook_api::v1::PromissoryReceivedPayload** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::PromissoryReceivedPayload* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_PromissoryReceivedPayload_remove(google::protobuf::Map<int64_t, ::webhook_api::v1::PromissoryReceivedPayload> * m,
                          int64_t key, ::webhook_api::v1::PromissoryReceivedPayload * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_webhook_api_v1_PromissoryReceivedPayload_iter(
      const google::protobuf::Map<int64_t, ::webhook_api::v1::PromissoryReceivedPayload>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_PromissoryReceivedPayload_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::webhook_api::v1::PromissoryReceivedPayload** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::webhook_api::v1::PromissoryReceivedPayload>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::webhook_api::v1::PromissoryReceivedPayload>* __rust_proto_thunk__Map_u64_webhook_api_v1_PromissoryReceivedPayload_new() {
    return new google::protobuf::Map<uint64_t, ::webhook_api::v1::PromissoryReceivedPayload>();
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_PromissoryReceivedPayload_free(const google::protobuf::Map<uint64_t, ::webhook_api::v1::PromissoryReceivedPayload>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_PromissoryReceivedPayload_clear(google::protobuf::Map<uint64_t, ::webhook_api::v1::PromissoryReceivedPayload> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_webhook_api_v1_PromissoryReceivedPayload_size(const google::protobuf::Map<uint64_t, ::webhook_api::v1::PromissoryReceivedPayload>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_PromissoryReceivedPayload_insert(google::protobuf::Map<uint64_t, ::webhook_api::v1::PromissoryReceivedPayload> * m,
                          uint64_t key, ::webhook_api::v1::PromissoryReceivedPayload value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_PromissoryReceivedPayload_get(const google::protobuf::Map<uint64_t, ::webhook_api::v1::PromissoryReceivedPayload>* m,
                       uint64_t key, const ::webhook_api::v1::PromissoryReceivedPayload** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::PromissoryReceivedPayload* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_PromissoryReceivedPayload_remove(google::protobuf::Map<uint64_t, ::webhook_api::v1::PromissoryReceivedPayload> * m,
                          uint64_t key, ::webhook_api::v1::PromissoryReceivedPayload * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_webhook_api_v1_PromissoryReceivedPayload_iter(
      const google::protobuf::Map<uint64_t, ::webhook_api::v1::PromissoryReceivedPayload>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_PromissoryReceivedPayload_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::webhook_api::v1::PromissoryReceivedPayload** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::webhook_api::v1::PromissoryReceivedPayload>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::webhook_api::v1::PromissoryReceivedPayload>* __rust_proto_thunk__Map_bool_webhook_api_v1_PromissoryReceivedPayload_new() {
    return new google::protobuf::Map<bool, ::webhook_api::v1::PromissoryReceivedPayload>();
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_PromissoryReceivedPayload_free(const google::protobuf::Map<bool, ::webhook_api::v1::PromissoryReceivedPayload>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_PromissoryReceivedPayload_clear(google::protobuf::Map<bool, ::webhook_api::v1::PromissoryReceivedPayload> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_webhook_api_v1_PromissoryReceivedPayload_size(const google::protobuf::Map<bool, ::webhook_api::v1::PromissoryReceivedPayload>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_PromissoryReceivedPayload_insert(google::protobuf::Map<bool, ::webhook_api::v1::PromissoryReceivedPayload> * m,
                          bool key, ::webhook_api::v1::PromissoryReceivedPayload value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_PromissoryReceivedPayload_get(const google::protobuf::Map<bool, ::webhook_api::v1::PromissoryReceivedPayload>* m,
                       bool key, const ::webhook_api::v1::PromissoryReceivedPayload** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::PromissoryReceivedPayload* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_PromissoryReceivedPayload_remove(google::protobuf::Map<bool, ::webhook_api::v1::PromissoryReceivedPayload> * m,
                          bool key, ::webhook_api::v1::PromissoryReceivedPayload * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_webhook_api_v1_PromissoryReceivedPayload_iter(
      const google::protobuf::Map<bool, ::webhook_api::v1::PromissoryReceivedPayload>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_PromissoryReceivedPayload_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::webhook_api::v1::PromissoryReceivedPayload** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::webhook_api::v1::PromissoryReceivedPayload>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::webhook_api::v1::PromissoryReceivedPayload>* __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_PromissoryReceivedPayload_new() {
    return new google::protobuf::Map<std::string, ::webhook_api::v1::PromissoryReceivedPayload>();
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_PromissoryReceivedPayload_free(const google::protobuf::Map<std::string, ::webhook_api::v1::PromissoryReceivedPayload>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_PromissoryReceivedPayload_clear(google::protobuf::Map<std::string, ::webhook_api::v1::PromissoryReceivedPayload> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_PromissoryReceivedPayload_size(const google::protobuf::Map<std::string, ::webhook_api::v1::PromissoryReceivedPayload>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_PromissoryReceivedPayload_insert(google::protobuf::Map<std::string, ::webhook_api::v1::PromissoryReceivedPayload> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::PromissoryReceivedPayload value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_PromissoryReceivedPayload_get(const google::protobuf::Map<std::string, ::webhook_api::v1::PromissoryReceivedPayload>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::webhook_api::v1::PromissoryReceivedPayload** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::PromissoryReceivedPayload* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_PromissoryReceivedPayload_remove(google::protobuf::Map<std::string, ::webhook_api::v1::PromissoryReceivedPayload> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::PromissoryReceivedPayload * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_PromissoryReceivedPayload_iter(
      const google::protobuf::Map<std::string, ::webhook_api::v1::PromissoryReceivedPayload>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_PromissoryReceivedPayload_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::webhook_api::v1::PromissoryReceivedPayload** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::webhook_api::v1::PromissoryReceivedPayload>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// webhook_api.v1.PingPayload
        // clang-format off
extern "C" {
void* __rust_proto_thunk__webhook_api_v1_PingPayload_new() { return new ::webhook_api::v1::PingPayload(); }
void __rust_proto_thunk__webhook_api_v1_PingPayload_delete(void* ptr) { delete static_cast<::webhook_api::v1::PingPayload*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__webhook_api_v1_PingPayload_serialize(::webhook_api::v1::PingPayload* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__webhook_api_v1_PingPayload_parse(::webhook_api::v1::PingPayload* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__webhook_api_v1_PingPayload_copy_from(::webhook_api::v1::PingPayload* dst, const ::webhook_api::v1::PingPayload* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__webhook_api_v1_PingPayload_repeated_len(google::protobuf::RepeatedPtrField<::webhook_api::v1::PingPayload>* field) {
  return field->size();
}
const ::webhook_api::v1::PingPayload& __rust_proto_thunk__webhook_api_v1_PingPayload_repeated_get(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::PingPayload>* field,
  size_t index) {
  return field->Get(index);
}
::webhook_api::v1::PingPayload* __rust_proto_thunk__webhook_api_v1_PingPayload_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::PingPayload>* field,
  size_t index) {
  return field->Mutable(index);
}
::webhook_api::v1::PingPayload* __rust_proto_thunk__webhook_api_v1_PingPayload_repeated_add(google::protobuf::RepeatedPtrField<::webhook_api::v1::PingPayload>* field) {
  return field->Add();
}
void __rust_proto_thunk__webhook_api_v1_PingPayload_repeated_clear(google::protobuf::RepeatedPtrField<::webhook_api::v1::PingPayload>* field) {
  field->Clear();
}
void __rust_proto_thunk__webhook_api_v1_PingPayload_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::PingPayload>& dst,
  const google::protobuf::RepeatedPtrField<::webhook_api::v1::PingPayload>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__webhook_api_v1_PingPayload_get_message(::webhook_api::v1::PingPayload* msg) {
  absl::string_view val = msg->message();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__webhook_api_v1_PingPayload_set_message(::webhook_api::v1::PingPayload* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_message(absl::string_view(s.ptr, s.len));
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::webhook_api::v1::PingPayload>* __rust_proto_thunk__Map_i32_webhook_api_v1_PingPayload_new() {
    return new google::protobuf::Map<int32_t, ::webhook_api::v1::PingPayload>();
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_PingPayload_free(const google::protobuf::Map<int32_t, ::webhook_api::v1::PingPayload>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_PingPayload_clear(google::protobuf::Map<int32_t, ::webhook_api::v1::PingPayload> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_webhook_api_v1_PingPayload_size(const google::protobuf::Map<int32_t, ::webhook_api::v1::PingPayload>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_PingPayload_insert(google::protobuf::Map<int32_t, ::webhook_api::v1::PingPayload> * m,
                          int32_t key, ::webhook_api::v1::PingPayload value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_PingPayload_get(const google::protobuf::Map<int32_t, ::webhook_api::v1::PingPayload>* m,
                       int32_t key, const ::webhook_api::v1::PingPayload** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::PingPayload* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_PingPayload_remove(google::protobuf::Map<int32_t, ::webhook_api::v1::PingPayload> * m,
                          int32_t key, ::webhook_api::v1::PingPayload * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_webhook_api_v1_PingPayload_iter(
      const google::protobuf::Map<int32_t, ::webhook_api::v1::PingPayload>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_PingPayload_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::webhook_api::v1::PingPayload** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::webhook_api::v1::PingPayload>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::webhook_api::v1::PingPayload>* __rust_proto_thunk__Map_u32_webhook_api_v1_PingPayload_new() {
    return new google::protobuf::Map<uint32_t, ::webhook_api::v1::PingPayload>();
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_PingPayload_free(const google::protobuf::Map<uint32_t, ::webhook_api::v1::PingPayload>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_PingPayload_clear(google::protobuf::Map<uint32_t, ::webhook_api::v1::PingPayload> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_webhook_api_v1_PingPayload_size(const google::protobuf::Map<uint32_t, ::webhook_api::v1::PingPayload>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_PingPayload_insert(google::protobuf::Map<uint32_t, ::webhook_api::v1::PingPayload> * m,
                          uint32_t key, ::webhook_api::v1::PingPayload value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_PingPayload_get(const google::protobuf::Map<uint32_t, ::webhook_api::v1::PingPayload>* m,
                       uint32_t key, const ::webhook_api::v1::PingPayload** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::PingPayload* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_PingPayload_remove(google::protobuf::Map<uint32_t, ::webhook_api::v1::PingPayload> * m,
                          uint32_t key, ::webhook_api::v1::PingPayload * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_webhook_api_v1_PingPayload_iter(
      const google::protobuf::Map<uint32_t, ::webhook_api::v1::PingPayload>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_PingPayload_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::webhook_api::v1::PingPayload** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::webhook_api::v1::PingPayload>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::webhook_api::v1::PingPayload>* __rust_proto_thunk__Map_i64_webhook_api_v1_PingPayload_new() {
    return new google::protobuf::Map<int64_t, ::webhook_api::v1::PingPayload>();
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_PingPayload_free(const google::protobuf::Map<int64_t, ::webhook_api::v1::PingPayload>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_PingPayload_clear(google::protobuf::Map<int64_t, ::webhook_api::v1::PingPayload> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_webhook_api_v1_PingPayload_size(const google::protobuf::Map<int64_t, ::webhook_api::v1::PingPayload>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_PingPayload_insert(google::protobuf::Map<int64_t, ::webhook_api::v1::PingPayload> * m,
                          int64_t key, ::webhook_api::v1::PingPayload value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_PingPayload_get(const google::protobuf::Map<int64_t, ::webhook_api::v1::PingPayload>* m,
                       int64_t key, const ::webhook_api::v1::PingPayload** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::PingPayload* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_PingPayload_remove(google::protobuf::Map<int64_t, ::webhook_api::v1::PingPayload> * m,
                          int64_t key, ::webhook_api::v1::PingPayload * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_webhook_api_v1_PingPayload_iter(
      const google::protobuf::Map<int64_t, ::webhook_api::v1::PingPayload>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_PingPayload_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::webhook_api::v1::PingPayload** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::webhook_api::v1::PingPayload>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::webhook_api::v1::PingPayload>* __rust_proto_thunk__Map_u64_webhook_api_v1_PingPayload_new() {
    return new google::protobuf::Map<uint64_t, ::webhook_api::v1::PingPayload>();
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_PingPayload_free(const google::protobuf::Map<uint64_t, ::webhook_api::v1::PingPayload>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_PingPayload_clear(google::protobuf::Map<uint64_t, ::webhook_api::v1::PingPayload> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_webhook_api_v1_PingPayload_size(const google::protobuf::Map<uint64_t, ::webhook_api::v1::PingPayload>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_PingPayload_insert(google::protobuf::Map<uint64_t, ::webhook_api::v1::PingPayload> * m,
                          uint64_t key, ::webhook_api::v1::PingPayload value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_PingPayload_get(const google::protobuf::Map<uint64_t, ::webhook_api::v1::PingPayload>* m,
                       uint64_t key, const ::webhook_api::v1::PingPayload** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::PingPayload* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_PingPayload_remove(google::protobuf::Map<uint64_t, ::webhook_api::v1::PingPayload> * m,
                          uint64_t key, ::webhook_api::v1::PingPayload * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_webhook_api_v1_PingPayload_iter(
      const google::protobuf::Map<uint64_t, ::webhook_api::v1::PingPayload>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_PingPayload_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::webhook_api::v1::PingPayload** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::webhook_api::v1::PingPayload>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::webhook_api::v1::PingPayload>* __rust_proto_thunk__Map_bool_webhook_api_v1_PingPayload_new() {
    return new google::protobuf::Map<bool, ::webhook_api::v1::PingPayload>();
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_PingPayload_free(const google::protobuf::Map<bool, ::webhook_api::v1::PingPayload>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_PingPayload_clear(google::protobuf::Map<bool, ::webhook_api::v1::PingPayload> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_webhook_api_v1_PingPayload_size(const google::protobuf::Map<bool, ::webhook_api::v1::PingPayload>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_PingPayload_insert(google::protobuf::Map<bool, ::webhook_api::v1::PingPayload> * m,
                          bool key, ::webhook_api::v1::PingPayload value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_PingPayload_get(const google::protobuf::Map<bool, ::webhook_api::v1::PingPayload>* m,
                       bool key, const ::webhook_api::v1::PingPayload** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::PingPayload* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_PingPayload_remove(google::protobuf::Map<bool, ::webhook_api::v1::PingPayload> * m,
                          bool key, ::webhook_api::v1::PingPayload * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_webhook_api_v1_PingPayload_iter(
      const google::protobuf::Map<bool, ::webhook_api::v1::PingPayload>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_PingPayload_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::webhook_api::v1::PingPayload** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::webhook_api::v1::PingPayload>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::webhook_api::v1::PingPayload>* __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_PingPayload_new() {
    return new google::protobuf::Map<std::string, ::webhook_api::v1::PingPayload>();
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_PingPayload_free(const google::protobuf::Map<std::string, ::webhook_api::v1::PingPayload>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_PingPayload_clear(google::protobuf::Map<std::string, ::webhook_api::v1::PingPayload> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_PingPayload_size(const google::protobuf::Map<std::string, ::webhook_api::v1::PingPayload>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_PingPayload_insert(google::protobuf::Map<std::string, ::webhook_api::v1::PingPayload> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::PingPayload value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_PingPayload_get(const google::protobuf::Map<std::string, ::webhook_api::v1::PingPayload>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::webhook_api::v1::PingPayload** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::PingPayload* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_PingPayload_remove(google::protobuf::Map<std::string, ::webhook_api::v1::PingPayload> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::PingPayload * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_PingPayload_iter(
      const google::protobuf::Map<std::string, ::webhook_api::v1::PingPayload>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_PingPayload_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::webhook_api::v1::PingPayload** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::webhook_api::v1::PingPayload>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// webhook_api.v1.ContractProposalCompletedPayload
        // clang-format off
extern "C" {
void* __rust_proto_thunk__webhook_api_v1_ContractProposalCompletedPayload_new() { return new ::webhook_api::v1::ContractProposalCompletedPayload(); }
void __rust_proto_thunk__webhook_api_v1_ContractProposalCompletedPayload_delete(void* ptr) { delete static_cast<::webhook_api::v1::ContractProposalCompletedPayload*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__webhook_api_v1_ContractProposalCompletedPayload_serialize(::webhook_api::v1::ContractProposalCompletedPayload* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__webhook_api_v1_ContractProposalCompletedPayload_parse(::webhook_api::v1::ContractProposalCompletedPayload* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__webhook_api_v1_ContractProposalCompletedPayload_copy_from(::webhook_api::v1::ContractProposalCompletedPayload* dst, const ::webhook_api::v1::ContractProposalCompletedPayload* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__webhook_api_v1_ContractProposalCompletedPayload_repeated_len(google::protobuf::RepeatedPtrField<::webhook_api::v1::ContractProposalCompletedPayload>* field) {
  return field->size();
}
const ::webhook_api::v1::ContractProposalCompletedPayload& __rust_proto_thunk__webhook_api_v1_ContractProposalCompletedPayload_repeated_get(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::ContractProposalCompletedPayload>* field,
  size_t index) {
  return field->Get(index);
}
::webhook_api::v1::ContractProposalCompletedPayload* __rust_proto_thunk__webhook_api_v1_ContractProposalCompletedPayload_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::ContractProposalCompletedPayload>* field,
  size_t index) {
  return field->Mutable(index);
}
::webhook_api::v1::ContractProposalCompletedPayload* __rust_proto_thunk__webhook_api_v1_ContractProposalCompletedPayload_repeated_add(google::protobuf::RepeatedPtrField<::webhook_api::v1::ContractProposalCompletedPayload>* field) {
  return field->Add();
}
void __rust_proto_thunk__webhook_api_v1_ContractProposalCompletedPayload_repeated_clear(google::protobuf::RepeatedPtrField<::webhook_api::v1::ContractProposalCompletedPayload>* field) {
  field->Clear();
}
void __rust_proto_thunk__webhook_api_v1_ContractProposalCompletedPayload_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::ContractProposalCompletedPayload>& dst,
  const google::protobuf::RepeatedPtrField<::webhook_api::v1::ContractProposalCompletedPayload>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__webhook_api_v1_ContractProposalCompletedPayload_get_contract_id(::webhook_api::v1::ContractProposalCompletedPayload* msg) {
  absl::string_view val = msg->contract_id();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__webhook_api_v1_ContractProposalCompletedPayload_set_contract_id(::webhook_api::v1::ContractProposalCompletedPayload* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_contract_id(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__webhook_api_v1_ContractProposalCompletedPayload_get_owner_id(::webhook_api::v1::ContractProposalCompletedPayload* msg) {
  absl::string_view val = msg->owner_id();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__webhook_api_v1_ContractProposalCompletedPayload_set_owner_id(::webhook_api::v1::ContractProposalCompletedPayload* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_owner_id(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__webhook_api_v1_ContractProposalCompletedPayload_get_wallet_id(::webhook_api::v1::ContractProposalCompletedPayload* msg) {
  absl::string_view val = msg->wallet_id();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__webhook_api_v1_ContractProposalCompletedPayload_set_wallet_id(::webhook_api::v1::ContractProposalCompletedPayload* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_wallet_id(absl::string_view(s.ptr, s.len));
}

void __rust_proto_thunk__webhook_api_v1_ContractProposalCompletedPayload_clear_commitments(::webhook_api::v1::ContractProposalCompletedPayload* msg) {
  msg->clear_commitments();
}
google::protobuf::RepeatedPtrField<::common::Commitment>* __rust_proto_thunk__webhook_api_v1_ContractProposalCompletedPayload_get_mut_commitments(::webhook_api::v1::ContractProposalCompletedPayload* msg) {
  return msg->mutable_commitments();
}
const google::protobuf::RepeatedPtrField<::common::Commitment>* __rust_proto_thunk__webhook_api_v1_ContractProposalCompletedPayload_get_commitments(
    const ::webhook_api::v1::ContractProposalCompletedPayload* msg) {
  return &msg->commitments();
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::webhook_api::v1::ContractProposalCompletedPayload>* __rust_proto_thunk__Map_i32_webhook_api_v1_ContractProposalCompletedPayload_new() {
    return new google::protobuf::Map<int32_t, ::webhook_api::v1::ContractProposalCompletedPayload>();
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_ContractProposalCompletedPayload_free(const google::protobuf::Map<int32_t, ::webhook_api::v1::ContractProposalCompletedPayload>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_ContractProposalCompletedPayload_clear(google::protobuf::Map<int32_t, ::webhook_api::v1::ContractProposalCompletedPayload> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_webhook_api_v1_ContractProposalCompletedPayload_size(const google::protobuf::Map<int32_t, ::webhook_api::v1::ContractProposalCompletedPayload>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_ContractProposalCompletedPayload_insert(google::protobuf::Map<int32_t, ::webhook_api::v1::ContractProposalCompletedPayload> * m,
                          int32_t key, ::webhook_api::v1::ContractProposalCompletedPayload value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_ContractProposalCompletedPayload_get(const google::protobuf::Map<int32_t, ::webhook_api::v1::ContractProposalCompletedPayload>* m,
                       int32_t key, const ::webhook_api::v1::ContractProposalCompletedPayload** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ContractProposalCompletedPayload* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_ContractProposalCompletedPayload_remove(google::protobuf::Map<int32_t, ::webhook_api::v1::ContractProposalCompletedPayload> * m,
                          int32_t key, ::webhook_api::v1::ContractProposalCompletedPayload * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_webhook_api_v1_ContractProposalCompletedPayload_iter(
      const google::protobuf::Map<int32_t, ::webhook_api::v1::ContractProposalCompletedPayload>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_ContractProposalCompletedPayload_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::webhook_api::v1::ContractProposalCompletedPayload** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::webhook_api::v1::ContractProposalCompletedPayload>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::webhook_api::v1::ContractProposalCompletedPayload>* __rust_proto_thunk__Map_u32_webhook_api_v1_ContractProposalCompletedPayload_new() {
    return new google::protobuf::Map<uint32_t, ::webhook_api::v1::ContractProposalCompletedPayload>();
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_ContractProposalCompletedPayload_free(const google::protobuf::Map<uint32_t, ::webhook_api::v1::ContractProposalCompletedPayload>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_ContractProposalCompletedPayload_clear(google::protobuf::Map<uint32_t, ::webhook_api::v1::ContractProposalCompletedPayload> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_webhook_api_v1_ContractProposalCompletedPayload_size(const google::protobuf::Map<uint32_t, ::webhook_api::v1::ContractProposalCompletedPayload>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_ContractProposalCompletedPayload_insert(google::protobuf::Map<uint32_t, ::webhook_api::v1::ContractProposalCompletedPayload> * m,
                          uint32_t key, ::webhook_api::v1::ContractProposalCompletedPayload value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_ContractProposalCompletedPayload_get(const google::protobuf::Map<uint32_t, ::webhook_api::v1::ContractProposalCompletedPayload>* m,
                       uint32_t key, const ::webhook_api::v1::ContractProposalCompletedPayload** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ContractProposalCompletedPayload* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_ContractProposalCompletedPayload_remove(google::protobuf::Map<uint32_t, ::webhook_api::v1::ContractProposalCompletedPayload> * m,
                          uint32_t key, ::webhook_api::v1::ContractProposalCompletedPayload * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_webhook_api_v1_ContractProposalCompletedPayload_iter(
      const google::protobuf::Map<uint32_t, ::webhook_api::v1::ContractProposalCompletedPayload>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_ContractProposalCompletedPayload_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::webhook_api::v1::ContractProposalCompletedPayload** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::webhook_api::v1::ContractProposalCompletedPayload>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::webhook_api::v1::ContractProposalCompletedPayload>* __rust_proto_thunk__Map_i64_webhook_api_v1_ContractProposalCompletedPayload_new() {
    return new google::protobuf::Map<int64_t, ::webhook_api::v1::ContractProposalCompletedPayload>();
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_ContractProposalCompletedPayload_free(const google::protobuf::Map<int64_t, ::webhook_api::v1::ContractProposalCompletedPayload>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_ContractProposalCompletedPayload_clear(google::protobuf::Map<int64_t, ::webhook_api::v1::ContractProposalCompletedPayload> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_webhook_api_v1_ContractProposalCompletedPayload_size(const google::protobuf::Map<int64_t, ::webhook_api::v1::ContractProposalCompletedPayload>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_ContractProposalCompletedPayload_insert(google::protobuf::Map<int64_t, ::webhook_api::v1::ContractProposalCompletedPayload> * m,
                          int64_t key, ::webhook_api::v1::ContractProposalCompletedPayload value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_ContractProposalCompletedPayload_get(const google::protobuf::Map<int64_t, ::webhook_api::v1::ContractProposalCompletedPayload>* m,
                       int64_t key, const ::webhook_api::v1::ContractProposalCompletedPayload** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ContractProposalCompletedPayload* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_ContractProposalCompletedPayload_remove(google::protobuf::Map<int64_t, ::webhook_api::v1::ContractProposalCompletedPayload> * m,
                          int64_t key, ::webhook_api::v1::ContractProposalCompletedPayload * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_webhook_api_v1_ContractProposalCompletedPayload_iter(
      const google::protobuf::Map<int64_t, ::webhook_api::v1::ContractProposalCompletedPayload>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_ContractProposalCompletedPayload_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::webhook_api::v1::ContractProposalCompletedPayload** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::webhook_api::v1::ContractProposalCompletedPayload>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::webhook_api::v1::ContractProposalCompletedPayload>* __rust_proto_thunk__Map_u64_webhook_api_v1_ContractProposalCompletedPayload_new() {
    return new google::protobuf::Map<uint64_t, ::webhook_api::v1::ContractProposalCompletedPayload>();
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_ContractProposalCompletedPayload_free(const google::protobuf::Map<uint64_t, ::webhook_api::v1::ContractProposalCompletedPayload>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_ContractProposalCompletedPayload_clear(google::protobuf::Map<uint64_t, ::webhook_api::v1::ContractProposalCompletedPayload> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_webhook_api_v1_ContractProposalCompletedPayload_size(const google::protobuf::Map<uint64_t, ::webhook_api::v1::ContractProposalCompletedPayload>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_ContractProposalCompletedPayload_insert(google::protobuf::Map<uint64_t, ::webhook_api::v1::ContractProposalCompletedPayload> * m,
                          uint64_t key, ::webhook_api::v1::ContractProposalCompletedPayload value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_ContractProposalCompletedPayload_get(const google::protobuf::Map<uint64_t, ::webhook_api::v1::ContractProposalCompletedPayload>* m,
                       uint64_t key, const ::webhook_api::v1::ContractProposalCompletedPayload** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ContractProposalCompletedPayload* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_ContractProposalCompletedPayload_remove(google::protobuf::Map<uint64_t, ::webhook_api::v1::ContractProposalCompletedPayload> * m,
                          uint64_t key, ::webhook_api::v1::ContractProposalCompletedPayload * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_webhook_api_v1_ContractProposalCompletedPayload_iter(
      const google::protobuf::Map<uint64_t, ::webhook_api::v1::ContractProposalCompletedPayload>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_ContractProposalCompletedPayload_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::webhook_api::v1::ContractProposalCompletedPayload** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::webhook_api::v1::ContractProposalCompletedPayload>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::webhook_api::v1::ContractProposalCompletedPayload>* __rust_proto_thunk__Map_bool_webhook_api_v1_ContractProposalCompletedPayload_new() {
    return new google::protobuf::Map<bool, ::webhook_api::v1::ContractProposalCompletedPayload>();
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_ContractProposalCompletedPayload_free(const google::protobuf::Map<bool, ::webhook_api::v1::ContractProposalCompletedPayload>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_ContractProposalCompletedPayload_clear(google::protobuf::Map<bool, ::webhook_api::v1::ContractProposalCompletedPayload> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_webhook_api_v1_ContractProposalCompletedPayload_size(const google::protobuf::Map<bool, ::webhook_api::v1::ContractProposalCompletedPayload>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_ContractProposalCompletedPayload_insert(google::protobuf::Map<bool, ::webhook_api::v1::ContractProposalCompletedPayload> * m,
                          bool key, ::webhook_api::v1::ContractProposalCompletedPayload value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_ContractProposalCompletedPayload_get(const google::protobuf::Map<bool, ::webhook_api::v1::ContractProposalCompletedPayload>* m,
                       bool key, const ::webhook_api::v1::ContractProposalCompletedPayload** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ContractProposalCompletedPayload* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_ContractProposalCompletedPayload_remove(google::protobuf::Map<bool, ::webhook_api::v1::ContractProposalCompletedPayload> * m,
                          bool key, ::webhook_api::v1::ContractProposalCompletedPayload * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_webhook_api_v1_ContractProposalCompletedPayload_iter(
      const google::protobuf::Map<bool, ::webhook_api::v1::ContractProposalCompletedPayload>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_ContractProposalCompletedPayload_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::webhook_api::v1::ContractProposalCompletedPayload** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::webhook_api::v1::ContractProposalCompletedPayload>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::webhook_api::v1::ContractProposalCompletedPayload>* __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ContractProposalCompletedPayload_new() {
    return new google::protobuf::Map<std::string, ::webhook_api::v1::ContractProposalCompletedPayload>();
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ContractProposalCompletedPayload_free(const google::protobuf::Map<std::string, ::webhook_api::v1::ContractProposalCompletedPayload>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ContractProposalCompletedPayload_clear(google::protobuf::Map<std::string, ::webhook_api::v1::ContractProposalCompletedPayload> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ContractProposalCompletedPayload_size(const google::protobuf::Map<std::string, ::webhook_api::v1::ContractProposalCompletedPayload>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ContractProposalCompletedPayload_insert(google::protobuf::Map<std::string, ::webhook_api::v1::ContractProposalCompletedPayload> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::ContractProposalCompletedPayload value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ContractProposalCompletedPayload_get(const google::protobuf::Map<std::string, ::webhook_api::v1::ContractProposalCompletedPayload>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::webhook_api::v1::ContractProposalCompletedPayload** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ContractProposalCompletedPayload* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ContractProposalCompletedPayload_remove(google::protobuf::Map<std::string, ::webhook_api::v1::ContractProposalCompletedPayload> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::ContractProposalCompletedPayload * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ContractProposalCompletedPayload_iter(
      const google::protobuf::Map<std::string, ::webhook_api::v1::ContractProposalCompletedPayload>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ContractProposalCompletedPayload_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::webhook_api::v1::ContractProposalCompletedPayload** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::webhook_api::v1::ContractProposalCompletedPayload>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// webhook_api.v1.ContractPaymentCompletedPayload
        // clang-format off
extern "C" {
void* __rust_proto_thunk__webhook_api_v1_ContractPaymentCompletedPayload_new() { return new ::webhook_api::v1::ContractPaymentCompletedPayload(); }
void __rust_proto_thunk__webhook_api_v1_ContractPaymentCompletedPayload_delete(void* ptr) { delete static_cast<::webhook_api::v1::ContractPaymentCompletedPayload*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__webhook_api_v1_ContractPaymentCompletedPayload_serialize(::webhook_api::v1::ContractPaymentCompletedPayload* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__webhook_api_v1_ContractPaymentCompletedPayload_parse(::webhook_api::v1::ContractPaymentCompletedPayload* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__webhook_api_v1_ContractPaymentCompletedPayload_copy_from(::webhook_api::v1::ContractPaymentCompletedPayload* dst, const ::webhook_api::v1::ContractPaymentCompletedPayload* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__webhook_api_v1_ContractPaymentCompletedPayload_repeated_len(google::protobuf::RepeatedPtrField<::webhook_api::v1::ContractPaymentCompletedPayload>* field) {
  return field->size();
}
const ::webhook_api::v1::ContractPaymentCompletedPayload& __rust_proto_thunk__webhook_api_v1_ContractPaymentCompletedPayload_repeated_get(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::ContractPaymentCompletedPayload>* field,
  size_t index) {
  return field->Get(index);
}
::webhook_api::v1::ContractPaymentCompletedPayload* __rust_proto_thunk__webhook_api_v1_ContractPaymentCompletedPayload_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::ContractPaymentCompletedPayload>* field,
  size_t index) {
  return field->Mutable(index);
}
::webhook_api::v1::ContractPaymentCompletedPayload* __rust_proto_thunk__webhook_api_v1_ContractPaymentCompletedPayload_repeated_add(google::protobuf::RepeatedPtrField<::webhook_api::v1::ContractPaymentCompletedPayload>* field) {
  return field->Add();
}
void __rust_proto_thunk__webhook_api_v1_ContractPaymentCompletedPayload_repeated_clear(google::protobuf::RepeatedPtrField<::webhook_api::v1::ContractPaymentCompletedPayload>* field) {
  field->Clear();
}
void __rust_proto_thunk__webhook_api_v1_ContractPaymentCompletedPayload_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::ContractPaymentCompletedPayload>& dst,
  const google::protobuf::RepeatedPtrField<::webhook_api::v1::ContractPaymentCompletedPayload>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__webhook_api_v1_ContractPaymentCompletedPayload_get_contract_id(::webhook_api::v1::ContractPaymentCompletedPayload* msg) {
  absl::string_view val = msg->contract_id();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__webhook_api_v1_ContractPaymentCompletedPayload_set_contract_id(::webhook_api::v1::ContractPaymentCompletedPayload* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_contract_id(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__webhook_api_v1_ContractPaymentCompletedPayload_get_owner_id(::webhook_api::v1::ContractPaymentCompletedPayload* msg) {
  absl::string_view val = msg->owner_id();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__webhook_api_v1_ContractPaymentCompletedPayload_set_owner_id(::webhook_api::v1::ContractPaymentCompletedPayload* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_owner_id(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__webhook_api_v1_ContractPaymentCompletedPayload_get_wallet_id(::webhook_api::v1::ContractPaymentCompletedPayload* msg) {
  absl::string_view val = msg->wallet_id();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__webhook_api_v1_ContractPaymentCompletedPayload_set_wallet_id(::webhook_api::v1::ContractPaymentCompletedPayload* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_wallet_id(absl::string_view(s.ptr, s.len));
}

void __rust_proto_thunk__webhook_api_v1_ContractPaymentCompletedPayload_clear_commitments(::webhook_api::v1::ContractPaymentCompletedPayload* msg) {
  msg->clear_commitments();
}
google::protobuf::RepeatedPtrField<::common::Commitment>* __rust_proto_thunk__webhook_api_v1_ContractPaymentCompletedPayload_get_mut_commitments(::webhook_api::v1::ContractPaymentCompletedPayload* msg) {
  return msg->mutable_commitments();
}
const google::protobuf::RepeatedPtrField<::common::Commitment>* __rust_proto_thunk__webhook_api_v1_ContractPaymentCompletedPayload_get_commitments(
    const ::webhook_api::v1::ContractPaymentCompletedPayload* msg) {
  return &msg->commitments();
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::webhook_api::v1::ContractPaymentCompletedPayload>* __rust_proto_thunk__Map_i32_webhook_api_v1_ContractPaymentCompletedPayload_new() {
    return new google::protobuf::Map<int32_t, ::webhook_api::v1::ContractPaymentCompletedPayload>();
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_ContractPaymentCompletedPayload_free(const google::protobuf::Map<int32_t, ::webhook_api::v1::ContractPaymentCompletedPayload>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_ContractPaymentCompletedPayload_clear(google::protobuf::Map<int32_t, ::webhook_api::v1::ContractPaymentCompletedPayload> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_webhook_api_v1_ContractPaymentCompletedPayload_size(const google::protobuf::Map<int32_t, ::webhook_api::v1::ContractPaymentCompletedPayload>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_ContractPaymentCompletedPayload_insert(google::protobuf::Map<int32_t, ::webhook_api::v1::ContractPaymentCompletedPayload> * m,
                          int32_t key, ::webhook_api::v1::ContractPaymentCompletedPayload value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_ContractPaymentCompletedPayload_get(const google::protobuf::Map<int32_t, ::webhook_api::v1::ContractPaymentCompletedPayload>* m,
                       int32_t key, const ::webhook_api::v1::ContractPaymentCompletedPayload** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ContractPaymentCompletedPayload* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_ContractPaymentCompletedPayload_remove(google::protobuf::Map<int32_t, ::webhook_api::v1::ContractPaymentCompletedPayload> * m,
                          int32_t key, ::webhook_api::v1::ContractPaymentCompletedPayload * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_webhook_api_v1_ContractPaymentCompletedPayload_iter(
      const google::protobuf::Map<int32_t, ::webhook_api::v1::ContractPaymentCompletedPayload>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_ContractPaymentCompletedPayload_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::webhook_api::v1::ContractPaymentCompletedPayload** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::webhook_api::v1::ContractPaymentCompletedPayload>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::webhook_api::v1::ContractPaymentCompletedPayload>* __rust_proto_thunk__Map_u32_webhook_api_v1_ContractPaymentCompletedPayload_new() {
    return new google::protobuf::Map<uint32_t, ::webhook_api::v1::ContractPaymentCompletedPayload>();
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_ContractPaymentCompletedPayload_free(const google::protobuf::Map<uint32_t, ::webhook_api::v1::ContractPaymentCompletedPayload>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_ContractPaymentCompletedPayload_clear(google::protobuf::Map<uint32_t, ::webhook_api::v1::ContractPaymentCompletedPayload> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_webhook_api_v1_ContractPaymentCompletedPayload_size(const google::protobuf::Map<uint32_t, ::webhook_api::v1::ContractPaymentCompletedPayload>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_ContractPaymentCompletedPayload_insert(google::protobuf::Map<uint32_t, ::webhook_api::v1::ContractPaymentCompletedPayload> * m,
                          uint32_t key, ::webhook_api::v1::ContractPaymentCompletedPayload value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_ContractPaymentCompletedPayload_get(const google::protobuf::Map<uint32_t, ::webhook_api::v1::ContractPaymentCompletedPayload>* m,
                       uint32_t key, const ::webhook_api::v1::ContractPaymentCompletedPayload** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ContractPaymentCompletedPayload* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_ContractPaymentCompletedPayload_remove(google::protobuf::Map<uint32_t, ::webhook_api::v1::ContractPaymentCompletedPayload> * m,
                          uint32_t key, ::webhook_api::v1::ContractPaymentCompletedPayload * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_webhook_api_v1_ContractPaymentCompletedPayload_iter(
      const google::protobuf::Map<uint32_t, ::webhook_api::v1::ContractPaymentCompletedPayload>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_ContractPaymentCompletedPayload_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::webhook_api::v1::ContractPaymentCompletedPayload** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::webhook_api::v1::ContractPaymentCompletedPayload>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::webhook_api::v1::ContractPaymentCompletedPayload>* __rust_proto_thunk__Map_i64_webhook_api_v1_ContractPaymentCompletedPayload_new() {
    return new google::protobuf::Map<int64_t, ::webhook_api::v1::ContractPaymentCompletedPayload>();
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_ContractPaymentCompletedPayload_free(const google::protobuf::Map<int64_t, ::webhook_api::v1::ContractPaymentCompletedPayload>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_ContractPaymentCompletedPayload_clear(google::protobuf::Map<int64_t, ::webhook_api::v1::ContractPaymentCompletedPayload> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_webhook_api_v1_ContractPaymentCompletedPayload_size(const google::protobuf::Map<int64_t, ::webhook_api::v1::ContractPaymentCompletedPayload>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_ContractPaymentCompletedPayload_insert(google::protobuf::Map<int64_t, ::webhook_api::v1::ContractPaymentCompletedPayload> * m,
                          int64_t key, ::webhook_api::v1::ContractPaymentCompletedPayload value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_ContractPaymentCompletedPayload_get(const google::protobuf::Map<int64_t, ::webhook_api::v1::ContractPaymentCompletedPayload>* m,
                       int64_t key, const ::webhook_api::v1::ContractPaymentCompletedPayload** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ContractPaymentCompletedPayload* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_ContractPaymentCompletedPayload_remove(google::protobuf::Map<int64_t, ::webhook_api::v1::ContractPaymentCompletedPayload> * m,
                          int64_t key, ::webhook_api::v1::ContractPaymentCompletedPayload * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_webhook_api_v1_ContractPaymentCompletedPayload_iter(
      const google::protobuf::Map<int64_t, ::webhook_api::v1::ContractPaymentCompletedPayload>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_ContractPaymentCompletedPayload_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::webhook_api::v1::ContractPaymentCompletedPayload** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::webhook_api::v1::ContractPaymentCompletedPayload>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::webhook_api::v1::ContractPaymentCompletedPayload>* __rust_proto_thunk__Map_u64_webhook_api_v1_ContractPaymentCompletedPayload_new() {
    return new google::protobuf::Map<uint64_t, ::webhook_api::v1::ContractPaymentCompletedPayload>();
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_ContractPaymentCompletedPayload_free(const google::protobuf::Map<uint64_t, ::webhook_api::v1::ContractPaymentCompletedPayload>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_ContractPaymentCompletedPayload_clear(google::protobuf::Map<uint64_t, ::webhook_api::v1::ContractPaymentCompletedPayload> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_webhook_api_v1_ContractPaymentCompletedPayload_size(const google::protobuf::Map<uint64_t, ::webhook_api::v1::ContractPaymentCompletedPayload>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_ContractPaymentCompletedPayload_insert(google::protobuf::Map<uint64_t, ::webhook_api::v1::ContractPaymentCompletedPayload> * m,
                          uint64_t key, ::webhook_api::v1::ContractPaymentCompletedPayload value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_ContractPaymentCompletedPayload_get(const google::protobuf::Map<uint64_t, ::webhook_api::v1::ContractPaymentCompletedPayload>* m,
                       uint64_t key, const ::webhook_api::v1::ContractPaymentCompletedPayload** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ContractPaymentCompletedPayload* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_ContractPaymentCompletedPayload_remove(google::protobuf::Map<uint64_t, ::webhook_api::v1::ContractPaymentCompletedPayload> * m,
                          uint64_t key, ::webhook_api::v1::ContractPaymentCompletedPayload * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_webhook_api_v1_ContractPaymentCompletedPayload_iter(
      const google::protobuf::Map<uint64_t, ::webhook_api::v1::ContractPaymentCompletedPayload>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_ContractPaymentCompletedPayload_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::webhook_api::v1::ContractPaymentCompletedPayload** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::webhook_api::v1::ContractPaymentCompletedPayload>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::webhook_api::v1::ContractPaymentCompletedPayload>* __rust_proto_thunk__Map_bool_webhook_api_v1_ContractPaymentCompletedPayload_new() {
    return new google::protobuf::Map<bool, ::webhook_api::v1::ContractPaymentCompletedPayload>();
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_ContractPaymentCompletedPayload_free(const google::protobuf::Map<bool, ::webhook_api::v1::ContractPaymentCompletedPayload>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_ContractPaymentCompletedPayload_clear(google::protobuf::Map<bool, ::webhook_api::v1::ContractPaymentCompletedPayload> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_webhook_api_v1_ContractPaymentCompletedPayload_size(const google::protobuf::Map<bool, ::webhook_api::v1::ContractPaymentCompletedPayload>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_ContractPaymentCompletedPayload_insert(google::protobuf::Map<bool, ::webhook_api::v1::ContractPaymentCompletedPayload> * m,
                          bool key, ::webhook_api::v1::ContractPaymentCompletedPayload value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_ContractPaymentCompletedPayload_get(const google::protobuf::Map<bool, ::webhook_api::v1::ContractPaymentCompletedPayload>* m,
                       bool key, const ::webhook_api::v1::ContractPaymentCompletedPayload** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ContractPaymentCompletedPayload* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_ContractPaymentCompletedPayload_remove(google::protobuf::Map<bool, ::webhook_api::v1::ContractPaymentCompletedPayload> * m,
                          bool key, ::webhook_api::v1::ContractPaymentCompletedPayload * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_webhook_api_v1_ContractPaymentCompletedPayload_iter(
      const google::protobuf::Map<bool, ::webhook_api::v1::ContractPaymentCompletedPayload>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_ContractPaymentCompletedPayload_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::webhook_api::v1::ContractPaymentCompletedPayload** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::webhook_api::v1::ContractPaymentCompletedPayload>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::webhook_api::v1::ContractPaymentCompletedPayload>* __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ContractPaymentCompletedPayload_new() {
    return new google::protobuf::Map<std::string, ::webhook_api::v1::ContractPaymentCompletedPayload>();
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ContractPaymentCompletedPayload_free(const google::protobuf::Map<std::string, ::webhook_api::v1::ContractPaymentCompletedPayload>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ContractPaymentCompletedPayload_clear(google::protobuf::Map<std::string, ::webhook_api::v1::ContractPaymentCompletedPayload> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ContractPaymentCompletedPayload_size(const google::protobuf::Map<std::string, ::webhook_api::v1::ContractPaymentCompletedPayload>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ContractPaymentCompletedPayload_insert(google::protobuf::Map<std::string, ::webhook_api::v1::ContractPaymentCompletedPayload> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::ContractPaymentCompletedPayload value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ContractPaymentCompletedPayload_get(const google::protobuf::Map<std::string, ::webhook_api::v1::ContractPaymentCompletedPayload>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::webhook_api::v1::ContractPaymentCompletedPayload** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ContractPaymentCompletedPayload* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ContractPaymentCompletedPayload_remove(google::protobuf::Map<std::string, ::webhook_api::v1::ContractPaymentCompletedPayload> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::ContractPaymentCompletedPayload * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ContractPaymentCompletedPayload_iter(
      const google::protobuf::Map<std::string, ::webhook_api::v1::ContractPaymentCompletedPayload>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ContractPaymentCompletedPayload_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::webhook_api::v1::ContractPaymentCompletedPayload** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::webhook_api::v1::ContractPaymentCompletedPayload>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// webhook_api.v1.Event
        // clang-format off
extern "C" {
void* __rust_proto_thunk__webhook_api_v1_Event_new() { return new ::webhook_api::v1::Event(); }
void __rust_proto_thunk__webhook_api_v1_Event_delete(void* ptr) { delete static_cast<::webhook_api::v1::Event*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__webhook_api_v1_Event_serialize(::webhook_api::v1::Event* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__webhook_api_v1_Event_parse(::webhook_api::v1::Event* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__webhook_api_v1_Event_copy_from(::webhook_api::v1::Event* dst, const ::webhook_api::v1::Event* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__webhook_api_v1_Event_repeated_len(google::protobuf::RepeatedPtrField<::webhook_api::v1::Event>* field) {
  return field->size();
}
const ::webhook_api::v1::Event& __rust_proto_thunk__webhook_api_v1_Event_repeated_get(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::Event>* field,
  size_t index) {
  return field->Get(index);
}
::webhook_api::v1::Event* __rust_proto_thunk__webhook_api_v1_Event_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::Event>* field,
  size_t index) {
  return field->Mutable(index);
}
::webhook_api::v1::Event* __rust_proto_thunk__webhook_api_v1_Event_repeated_add(google::protobuf::RepeatedPtrField<::webhook_api::v1::Event>* field) {
  return field->Add();
}
void __rust_proto_thunk__webhook_api_v1_Event_repeated_clear(google::protobuf::RepeatedPtrField<::webhook_api::v1::Event>* field) {
  field->Clear();
}
void __rust_proto_thunk__webhook_api_v1_Event_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::Event>& dst,
  const google::protobuf::RepeatedPtrField<::webhook_api::v1::Event>& src) {
  dst = src;
}

::webhook_api::v1::EventType __rust_proto_thunk__webhook_api_v1_Event_get_event_type(::webhook_api::v1::Event* msg) { return msg->event_type(); }
void __rust_proto_thunk__webhook_api_v1_Event_set_event_type(::webhook_api::v1::Event* msg, ::webhook_api::v1::EventType val) {
  msg->set_event_type(val);
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__webhook_api_v1_Event_get_signature(::webhook_api::v1::Event* msg) {
  absl::string_view val = msg->signature();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__webhook_api_v1_Event_set_signature(::webhook_api::v1::Event* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_signature(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__webhook_api_v1_Event_get_event_id(::webhook_api::v1::Event* msg) {
  absl::string_view val = msg->event_id();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__webhook_api_v1_Event_set_event_id(::webhook_api::v1::Event* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_event_id(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__webhook_api_v1_Event_get_delivery_date(::webhook_api::v1::Event* msg) {
  absl::string_view val = msg->delivery_date();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__webhook_api_v1_Event_set_delivery_date(::webhook_api::v1::Event* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_delivery_date(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__webhook_api_v1_Event_get_owner_id(::webhook_api::v1::Event* msg) {
  absl::string_view val = msg->owner_id();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__webhook_api_v1_Event_set_owner_id(::webhook_api::v1::Event* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_owner_id(absl::string_view(s.ptr, s.len));
}

const void* __rust_proto_thunk__webhook_api_v1_Event_get_wallet_created_payload(::webhook_api::v1::Event* msg) {
  return static_cast<const void*>(&msg->wallet_created_payload());
}
void* __rust_proto_thunk__webhook_api_v1_Event_get_mut_wallet_created_payload(::webhook_api::v1::Event* msg) {
  return static_cast<void*>(msg->mutable_wallet_created_payload());
}
void __rust_proto_thunk__webhook_api_v1_Event_clear_wallet_created_payload(::webhook_api::v1::Event* msg) { msg->clear_wallet_created_payload(); }
bool __rust_proto_thunk__webhook_api_v1_Event_has_wallet_created_payload(::webhook_api::v1::Event* msg) { return msg->has_wallet_created_payload(); }

const void* __rust_proto_thunk__webhook_api_v1_Event_get_promissory_received_payload(::webhook_api::v1::Event* msg) {
  return static_cast<const void*>(&msg->promissory_received_payload());
}
void* __rust_proto_thunk__webhook_api_v1_Event_get_mut_promissory_received_payload(::webhook_api::v1::Event* msg) {
  return static_cast<void*>(msg->mutable_promissory_received_payload());
}
void __rust_proto_thunk__webhook_api_v1_Event_clear_promissory_received_payload(::webhook_api::v1::Event* msg) { msg->clear_promissory_received_payload(); }
bool __rust_proto_thunk__webhook_api_v1_Event_has_promissory_received_payload(::webhook_api::v1::Event* msg) { return msg->has_promissory_received_payload(); }

const void* __rust_proto_thunk__webhook_api_v1_Event_get_ping_payload(::webhook_api::v1::Event* msg) {
  return static_cast<const void*>(&msg->ping_payload());
}
void* __rust_proto_thunk__webhook_api_v1_Event_get_mut_ping_payload(::webhook_api::v1::Event* msg) {
  return static_cast<void*>(msg->mutable_ping_payload());
}
void __rust_proto_thunk__webhook_api_v1_Event_clear_ping_payload(::webhook_api::v1::Event* msg) { msg->clear_ping_payload(); }
bool __rust_proto_thunk__webhook_api_v1_Event_has_ping_payload(::webhook_api::v1::Event* msg) { return msg->has_ping_payload(); }

const void* __rust_proto_thunk__webhook_api_v1_Event_get_contract_proposal_completed_payload(::webhook_api::v1::Event* msg) {
  return static_cast<const void*>(&msg->contract_proposal_completed_payload());
}
void* __rust_proto_thunk__webhook_api_v1_Event_get_mut_contract_proposal_completed_payload(::webhook_api::v1::Event* msg) {
  return static_cast<void*>(msg->mutable_contract_proposal_completed_payload());
}
void __rust_proto_thunk__webhook_api_v1_Event_clear_contract_proposal_completed_payload(::webhook_api::v1::Event* msg) { msg->clear_contract_proposal_completed_payload(); }
bool __rust_proto_thunk__webhook_api_v1_Event_has_contract_proposal_completed_payload(::webhook_api::v1::Event* msg) { return msg->has_contract_proposal_completed_payload(); }

const void* __rust_proto_thunk__webhook_api_v1_Event_get_contract_payment_completed_payload(::webhook_api::v1::Event* msg) {
  return static_cast<const void*>(&msg->contract_payment_completed_payload());
}
void* __rust_proto_thunk__webhook_api_v1_Event_get_mut_contract_payment_completed_payload(::webhook_api::v1::Event* msg) {
  return static_cast<void*>(msg->mutable_contract_payment_completed_payload());
}
void __rust_proto_thunk__webhook_api_v1_Event_clear_contract_payment_completed_payload(::webhook_api::v1::Event* msg) { msg->clear_contract_payment_completed_payload(); }
bool __rust_proto_thunk__webhook_api_v1_Event_has_contract_payment_completed_payload(::webhook_api::v1::Event* msg) { return msg->has_contract_payment_completed_payload(); }

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__webhook_api_v1_Event_get_created_date(::webhook_api::v1::Event* msg) {
  absl::string_view val = msg->created_date();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__webhook_api_v1_Event_set_created_date(::webhook_api::v1::Event* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_created_date(absl::string_view(s.ptr, s.len));
}


::webhook_api::v1::Event::PayloadCase __rust_proto_thunk__webhook_api_v1_Event_case_payload(::webhook_api::v1::Event* msg) {
  return msg->payload_case();
}
}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::webhook_api::v1::Event>* __rust_proto_thunk__Map_i32_webhook_api_v1_Event_new() {
    return new google::protobuf::Map<int32_t, ::webhook_api::v1::Event>();
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_Event_free(const google::protobuf::Map<int32_t, ::webhook_api::v1::Event>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_Event_clear(google::protobuf::Map<int32_t, ::webhook_api::v1::Event> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_webhook_api_v1_Event_size(const google::protobuf::Map<int32_t, ::webhook_api::v1::Event>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_Event_insert(google::protobuf::Map<int32_t, ::webhook_api::v1::Event> * m,
                          int32_t key, ::webhook_api::v1::Event value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_Event_get(const google::protobuf::Map<int32_t, ::webhook_api::v1::Event>* m,
                       int32_t key, const ::webhook_api::v1::Event** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::Event* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_Event_remove(google::protobuf::Map<int32_t, ::webhook_api::v1::Event> * m,
                          int32_t key, ::webhook_api::v1::Event * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_webhook_api_v1_Event_iter(
      const google::protobuf::Map<int32_t, ::webhook_api::v1::Event>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_Event_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::webhook_api::v1::Event** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::webhook_api::v1::Event>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::webhook_api::v1::Event>* __rust_proto_thunk__Map_u32_webhook_api_v1_Event_new() {
    return new google::protobuf::Map<uint32_t, ::webhook_api::v1::Event>();
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_Event_free(const google::protobuf::Map<uint32_t, ::webhook_api::v1::Event>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_Event_clear(google::protobuf::Map<uint32_t, ::webhook_api::v1::Event> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_webhook_api_v1_Event_size(const google::protobuf::Map<uint32_t, ::webhook_api::v1::Event>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_Event_insert(google::protobuf::Map<uint32_t, ::webhook_api::v1::Event> * m,
                          uint32_t key, ::webhook_api::v1::Event value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_Event_get(const google::protobuf::Map<uint32_t, ::webhook_api::v1::Event>* m,
                       uint32_t key, const ::webhook_api::v1::Event** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::Event* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_Event_remove(google::protobuf::Map<uint32_t, ::webhook_api::v1::Event> * m,
                          uint32_t key, ::webhook_api::v1::Event * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_webhook_api_v1_Event_iter(
      const google::protobuf::Map<uint32_t, ::webhook_api::v1::Event>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_Event_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::webhook_api::v1::Event** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::webhook_api::v1::Event>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::webhook_api::v1::Event>* __rust_proto_thunk__Map_i64_webhook_api_v1_Event_new() {
    return new google::protobuf::Map<int64_t, ::webhook_api::v1::Event>();
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_Event_free(const google::protobuf::Map<int64_t, ::webhook_api::v1::Event>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_Event_clear(google::protobuf::Map<int64_t, ::webhook_api::v1::Event> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_webhook_api_v1_Event_size(const google::protobuf::Map<int64_t, ::webhook_api::v1::Event>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_Event_insert(google::protobuf::Map<int64_t, ::webhook_api::v1::Event> * m,
                          int64_t key, ::webhook_api::v1::Event value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_Event_get(const google::protobuf::Map<int64_t, ::webhook_api::v1::Event>* m,
                       int64_t key, const ::webhook_api::v1::Event** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::Event* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_Event_remove(google::protobuf::Map<int64_t, ::webhook_api::v1::Event> * m,
                          int64_t key, ::webhook_api::v1::Event * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_webhook_api_v1_Event_iter(
      const google::protobuf::Map<int64_t, ::webhook_api::v1::Event>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_Event_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::webhook_api::v1::Event** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::webhook_api::v1::Event>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::webhook_api::v1::Event>* __rust_proto_thunk__Map_u64_webhook_api_v1_Event_new() {
    return new google::protobuf::Map<uint64_t, ::webhook_api::v1::Event>();
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_Event_free(const google::protobuf::Map<uint64_t, ::webhook_api::v1::Event>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_Event_clear(google::protobuf::Map<uint64_t, ::webhook_api::v1::Event> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_webhook_api_v1_Event_size(const google::protobuf::Map<uint64_t, ::webhook_api::v1::Event>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_Event_insert(google::protobuf::Map<uint64_t, ::webhook_api::v1::Event> * m,
                          uint64_t key, ::webhook_api::v1::Event value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_Event_get(const google::protobuf::Map<uint64_t, ::webhook_api::v1::Event>* m,
                       uint64_t key, const ::webhook_api::v1::Event** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::Event* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_Event_remove(google::protobuf::Map<uint64_t, ::webhook_api::v1::Event> * m,
                          uint64_t key, ::webhook_api::v1::Event * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_webhook_api_v1_Event_iter(
      const google::protobuf::Map<uint64_t, ::webhook_api::v1::Event>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_Event_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::webhook_api::v1::Event** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::webhook_api::v1::Event>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::webhook_api::v1::Event>* __rust_proto_thunk__Map_bool_webhook_api_v1_Event_new() {
    return new google::protobuf::Map<bool, ::webhook_api::v1::Event>();
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_Event_free(const google::protobuf::Map<bool, ::webhook_api::v1::Event>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_Event_clear(google::protobuf::Map<bool, ::webhook_api::v1::Event> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_webhook_api_v1_Event_size(const google::protobuf::Map<bool, ::webhook_api::v1::Event>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_Event_insert(google::protobuf::Map<bool, ::webhook_api::v1::Event> * m,
                          bool key, ::webhook_api::v1::Event value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_Event_get(const google::protobuf::Map<bool, ::webhook_api::v1::Event>* m,
                       bool key, const ::webhook_api::v1::Event** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::Event* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_Event_remove(google::protobuf::Map<bool, ::webhook_api::v1::Event> * m,
                          bool key, ::webhook_api::v1::Event * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_webhook_api_v1_Event_iter(
      const google::protobuf::Map<bool, ::webhook_api::v1::Event>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_Event_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::webhook_api::v1::Event** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::webhook_api::v1::Event>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::webhook_api::v1::Event>* __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_Event_new() {
    return new google::protobuf::Map<std::string, ::webhook_api::v1::Event>();
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_Event_free(const google::protobuf::Map<std::string, ::webhook_api::v1::Event>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_Event_clear(google::protobuf::Map<std::string, ::webhook_api::v1::Event> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_Event_size(const google::protobuf::Map<std::string, ::webhook_api::v1::Event>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_Event_insert(google::protobuf::Map<std::string, ::webhook_api::v1::Event> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::Event value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_Event_get(const google::protobuf::Map<std::string, ::webhook_api::v1::Event>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::webhook_api::v1::Event** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::Event* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_Event_remove(google::protobuf::Map<std::string, ::webhook_api::v1::Event> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::Event * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_Event_iter(
      const google::protobuf::Map<std::string, ::webhook_api::v1::Event>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_Event_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::webhook_api::v1::Event** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::webhook_api::v1::Event>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// webhook_api.v1.CreateWebhookRequest
        // clang-format off
extern "C" {
void* __rust_proto_thunk__webhook_api_v1_CreateWebhookRequest_new() { return new ::webhook_api::v1::CreateWebhookRequest(); }
void __rust_proto_thunk__webhook_api_v1_CreateWebhookRequest_delete(void* ptr) { delete static_cast<::webhook_api::v1::CreateWebhookRequest*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__webhook_api_v1_CreateWebhookRequest_serialize(::webhook_api::v1::CreateWebhookRequest* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__webhook_api_v1_CreateWebhookRequest_parse(::webhook_api::v1::CreateWebhookRequest* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__webhook_api_v1_CreateWebhookRequest_copy_from(::webhook_api::v1::CreateWebhookRequest* dst, const ::webhook_api::v1::CreateWebhookRequest* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__webhook_api_v1_CreateWebhookRequest_repeated_len(google::protobuf::RepeatedPtrField<::webhook_api::v1::CreateWebhookRequest>* field) {
  return field->size();
}
const ::webhook_api::v1::CreateWebhookRequest& __rust_proto_thunk__webhook_api_v1_CreateWebhookRequest_repeated_get(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::CreateWebhookRequest>* field,
  size_t index) {
  return field->Get(index);
}
::webhook_api::v1::CreateWebhookRequest* __rust_proto_thunk__webhook_api_v1_CreateWebhookRequest_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::CreateWebhookRequest>* field,
  size_t index) {
  return field->Mutable(index);
}
::webhook_api::v1::CreateWebhookRequest* __rust_proto_thunk__webhook_api_v1_CreateWebhookRequest_repeated_add(google::protobuf::RepeatedPtrField<::webhook_api::v1::CreateWebhookRequest>* field) {
  return field->Add();
}
void __rust_proto_thunk__webhook_api_v1_CreateWebhookRequest_repeated_clear(google::protobuf::RepeatedPtrField<::webhook_api::v1::CreateWebhookRequest>* field) {
  field->Clear();
}
void __rust_proto_thunk__webhook_api_v1_CreateWebhookRequest_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::CreateWebhookRequest>& dst,
  const google::protobuf::RepeatedPtrField<::webhook_api::v1::CreateWebhookRequest>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__webhook_api_v1_CreateWebhookRequest_get_api_key(::webhook_api::v1::CreateWebhookRequest* msg) {
  absl::string_view val = msg->api_key();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__webhook_api_v1_CreateWebhookRequest_set_api_key(::webhook_api::v1::CreateWebhookRequest* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_api_key(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__webhook_api_v1_CreateWebhookRequest_get_name(::webhook_api::v1::CreateWebhookRequest* msg) {
  absl::string_view val = msg->name();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__webhook_api_v1_CreateWebhookRequest_set_name(::webhook_api::v1::CreateWebhookRequest* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_name(absl::string_view(s.ptr, s.len));
}

void __rust_proto_thunk__webhook_api_v1_CreateWebhookRequest_clear_supported_events(::webhook_api::v1::CreateWebhookRequest* msg) {
  msg->clear_supported_events();
}
google::protobuf::RepeatedField<int>* __rust_proto_thunk__webhook_api_v1_CreateWebhookRequest_get_mut_supported_events(::webhook_api::v1::CreateWebhookRequest* msg) {
  return msg->mutable_supported_events();
}
const google::protobuf::RepeatedField<int>* __rust_proto_thunk__webhook_api_v1_CreateWebhookRequest_get_supported_events(
    const ::webhook_api::v1::CreateWebhookRequest* msg) {
  return &msg->supported_events();
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__webhook_api_v1_CreateWebhookRequest_get_webhook_url(::webhook_api::v1::CreateWebhookRequest* msg) {
  absl::string_view val = msg->webhook_url();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__webhook_api_v1_CreateWebhookRequest_set_webhook_url(::webhook_api::v1::CreateWebhookRequest* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_webhook_url(absl::string_view(s.ptr, s.len));
}

::webhook_api::v1::DeliveryMethodType __rust_proto_thunk__webhook_api_v1_CreateWebhookRequest_get_delivery_method(::webhook_api::v1::CreateWebhookRequest* msg) { return msg->delivery_method(); }
void __rust_proto_thunk__webhook_api_v1_CreateWebhookRequest_set_delivery_method(::webhook_api::v1::CreateWebhookRequest* msg, ::webhook_api::v1::DeliveryMethodType val) {
  msg->set_delivery_method(val);
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::webhook_api::v1::CreateWebhookRequest>* __rust_proto_thunk__Map_i32_webhook_api_v1_CreateWebhookRequest_new() {
    return new google::protobuf::Map<int32_t, ::webhook_api::v1::CreateWebhookRequest>();
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_CreateWebhookRequest_free(const google::protobuf::Map<int32_t, ::webhook_api::v1::CreateWebhookRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_CreateWebhookRequest_clear(google::protobuf::Map<int32_t, ::webhook_api::v1::CreateWebhookRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_webhook_api_v1_CreateWebhookRequest_size(const google::protobuf::Map<int32_t, ::webhook_api::v1::CreateWebhookRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_CreateWebhookRequest_insert(google::protobuf::Map<int32_t, ::webhook_api::v1::CreateWebhookRequest> * m,
                          int32_t key, ::webhook_api::v1::CreateWebhookRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_CreateWebhookRequest_get(const google::protobuf::Map<int32_t, ::webhook_api::v1::CreateWebhookRequest>* m,
                       int32_t key, const ::webhook_api::v1::CreateWebhookRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::CreateWebhookRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_CreateWebhookRequest_remove(google::protobuf::Map<int32_t, ::webhook_api::v1::CreateWebhookRequest> * m,
                          int32_t key, ::webhook_api::v1::CreateWebhookRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_webhook_api_v1_CreateWebhookRequest_iter(
      const google::protobuf::Map<int32_t, ::webhook_api::v1::CreateWebhookRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_CreateWebhookRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::webhook_api::v1::CreateWebhookRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::webhook_api::v1::CreateWebhookRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::webhook_api::v1::CreateWebhookRequest>* __rust_proto_thunk__Map_u32_webhook_api_v1_CreateWebhookRequest_new() {
    return new google::protobuf::Map<uint32_t, ::webhook_api::v1::CreateWebhookRequest>();
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_CreateWebhookRequest_free(const google::protobuf::Map<uint32_t, ::webhook_api::v1::CreateWebhookRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_CreateWebhookRequest_clear(google::protobuf::Map<uint32_t, ::webhook_api::v1::CreateWebhookRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_webhook_api_v1_CreateWebhookRequest_size(const google::protobuf::Map<uint32_t, ::webhook_api::v1::CreateWebhookRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_CreateWebhookRequest_insert(google::protobuf::Map<uint32_t, ::webhook_api::v1::CreateWebhookRequest> * m,
                          uint32_t key, ::webhook_api::v1::CreateWebhookRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_CreateWebhookRequest_get(const google::protobuf::Map<uint32_t, ::webhook_api::v1::CreateWebhookRequest>* m,
                       uint32_t key, const ::webhook_api::v1::CreateWebhookRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::CreateWebhookRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_CreateWebhookRequest_remove(google::protobuf::Map<uint32_t, ::webhook_api::v1::CreateWebhookRequest> * m,
                          uint32_t key, ::webhook_api::v1::CreateWebhookRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_webhook_api_v1_CreateWebhookRequest_iter(
      const google::protobuf::Map<uint32_t, ::webhook_api::v1::CreateWebhookRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_CreateWebhookRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::webhook_api::v1::CreateWebhookRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::webhook_api::v1::CreateWebhookRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::webhook_api::v1::CreateWebhookRequest>* __rust_proto_thunk__Map_i64_webhook_api_v1_CreateWebhookRequest_new() {
    return new google::protobuf::Map<int64_t, ::webhook_api::v1::CreateWebhookRequest>();
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_CreateWebhookRequest_free(const google::protobuf::Map<int64_t, ::webhook_api::v1::CreateWebhookRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_CreateWebhookRequest_clear(google::protobuf::Map<int64_t, ::webhook_api::v1::CreateWebhookRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_webhook_api_v1_CreateWebhookRequest_size(const google::protobuf::Map<int64_t, ::webhook_api::v1::CreateWebhookRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_CreateWebhookRequest_insert(google::protobuf::Map<int64_t, ::webhook_api::v1::CreateWebhookRequest> * m,
                          int64_t key, ::webhook_api::v1::CreateWebhookRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_CreateWebhookRequest_get(const google::protobuf::Map<int64_t, ::webhook_api::v1::CreateWebhookRequest>* m,
                       int64_t key, const ::webhook_api::v1::CreateWebhookRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::CreateWebhookRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_CreateWebhookRequest_remove(google::protobuf::Map<int64_t, ::webhook_api::v1::CreateWebhookRequest> * m,
                          int64_t key, ::webhook_api::v1::CreateWebhookRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_webhook_api_v1_CreateWebhookRequest_iter(
      const google::protobuf::Map<int64_t, ::webhook_api::v1::CreateWebhookRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_CreateWebhookRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::webhook_api::v1::CreateWebhookRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::webhook_api::v1::CreateWebhookRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::webhook_api::v1::CreateWebhookRequest>* __rust_proto_thunk__Map_u64_webhook_api_v1_CreateWebhookRequest_new() {
    return new google::protobuf::Map<uint64_t, ::webhook_api::v1::CreateWebhookRequest>();
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_CreateWebhookRequest_free(const google::protobuf::Map<uint64_t, ::webhook_api::v1::CreateWebhookRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_CreateWebhookRequest_clear(google::protobuf::Map<uint64_t, ::webhook_api::v1::CreateWebhookRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_webhook_api_v1_CreateWebhookRequest_size(const google::protobuf::Map<uint64_t, ::webhook_api::v1::CreateWebhookRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_CreateWebhookRequest_insert(google::protobuf::Map<uint64_t, ::webhook_api::v1::CreateWebhookRequest> * m,
                          uint64_t key, ::webhook_api::v1::CreateWebhookRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_CreateWebhookRequest_get(const google::protobuf::Map<uint64_t, ::webhook_api::v1::CreateWebhookRequest>* m,
                       uint64_t key, const ::webhook_api::v1::CreateWebhookRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::CreateWebhookRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_CreateWebhookRequest_remove(google::protobuf::Map<uint64_t, ::webhook_api::v1::CreateWebhookRequest> * m,
                          uint64_t key, ::webhook_api::v1::CreateWebhookRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_webhook_api_v1_CreateWebhookRequest_iter(
      const google::protobuf::Map<uint64_t, ::webhook_api::v1::CreateWebhookRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_CreateWebhookRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::webhook_api::v1::CreateWebhookRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::webhook_api::v1::CreateWebhookRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::webhook_api::v1::CreateWebhookRequest>* __rust_proto_thunk__Map_bool_webhook_api_v1_CreateWebhookRequest_new() {
    return new google::protobuf::Map<bool, ::webhook_api::v1::CreateWebhookRequest>();
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_CreateWebhookRequest_free(const google::protobuf::Map<bool, ::webhook_api::v1::CreateWebhookRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_CreateWebhookRequest_clear(google::protobuf::Map<bool, ::webhook_api::v1::CreateWebhookRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_webhook_api_v1_CreateWebhookRequest_size(const google::protobuf::Map<bool, ::webhook_api::v1::CreateWebhookRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_CreateWebhookRequest_insert(google::protobuf::Map<bool, ::webhook_api::v1::CreateWebhookRequest> * m,
                          bool key, ::webhook_api::v1::CreateWebhookRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_CreateWebhookRequest_get(const google::protobuf::Map<bool, ::webhook_api::v1::CreateWebhookRequest>* m,
                       bool key, const ::webhook_api::v1::CreateWebhookRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::CreateWebhookRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_CreateWebhookRequest_remove(google::protobuf::Map<bool, ::webhook_api::v1::CreateWebhookRequest> * m,
                          bool key, ::webhook_api::v1::CreateWebhookRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_webhook_api_v1_CreateWebhookRequest_iter(
      const google::protobuf::Map<bool, ::webhook_api::v1::CreateWebhookRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_CreateWebhookRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::webhook_api::v1::CreateWebhookRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::webhook_api::v1::CreateWebhookRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::webhook_api::v1::CreateWebhookRequest>* __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_CreateWebhookRequest_new() {
    return new google::protobuf::Map<std::string, ::webhook_api::v1::CreateWebhookRequest>();
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_CreateWebhookRequest_free(const google::protobuf::Map<std::string, ::webhook_api::v1::CreateWebhookRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_CreateWebhookRequest_clear(google::protobuf::Map<std::string, ::webhook_api::v1::CreateWebhookRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_CreateWebhookRequest_size(const google::protobuf::Map<std::string, ::webhook_api::v1::CreateWebhookRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_CreateWebhookRequest_insert(google::protobuf::Map<std::string, ::webhook_api::v1::CreateWebhookRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::CreateWebhookRequest value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_CreateWebhookRequest_get(const google::protobuf::Map<std::string, ::webhook_api::v1::CreateWebhookRequest>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::webhook_api::v1::CreateWebhookRequest** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::CreateWebhookRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_CreateWebhookRequest_remove(google::protobuf::Map<std::string, ::webhook_api::v1::CreateWebhookRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::CreateWebhookRequest * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_CreateWebhookRequest_iter(
      const google::protobuf::Map<std::string, ::webhook_api::v1::CreateWebhookRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_CreateWebhookRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::webhook_api::v1::CreateWebhookRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::webhook_api::v1::CreateWebhookRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// webhook_api.v1.CreateWebhookResponse
        // clang-format off
extern "C" {
void* __rust_proto_thunk__webhook_api_v1_CreateWebhookResponse_new() { return new ::webhook_api::v1::CreateWebhookResponse(); }
void __rust_proto_thunk__webhook_api_v1_CreateWebhookResponse_delete(void* ptr) { delete static_cast<::webhook_api::v1::CreateWebhookResponse*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__webhook_api_v1_CreateWebhookResponse_serialize(::webhook_api::v1::CreateWebhookResponse* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__webhook_api_v1_CreateWebhookResponse_parse(::webhook_api::v1::CreateWebhookResponse* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__webhook_api_v1_CreateWebhookResponse_copy_from(::webhook_api::v1::CreateWebhookResponse* dst, const ::webhook_api::v1::CreateWebhookResponse* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__webhook_api_v1_CreateWebhookResponse_repeated_len(google::protobuf::RepeatedPtrField<::webhook_api::v1::CreateWebhookResponse>* field) {
  return field->size();
}
const ::webhook_api::v1::CreateWebhookResponse& __rust_proto_thunk__webhook_api_v1_CreateWebhookResponse_repeated_get(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::CreateWebhookResponse>* field,
  size_t index) {
  return field->Get(index);
}
::webhook_api::v1::CreateWebhookResponse* __rust_proto_thunk__webhook_api_v1_CreateWebhookResponse_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::CreateWebhookResponse>* field,
  size_t index) {
  return field->Mutable(index);
}
::webhook_api::v1::CreateWebhookResponse* __rust_proto_thunk__webhook_api_v1_CreateWebhookResponse_repeated_add(google::protobuf::RepeatedPtrField<::webhook_api::v1::CreateWebhookResponse>* field) {
  return field->Add();
}
void __rust_proto_thunk__webhook_api_v1_CreateWebhookResponse_repeated_clear(google::protobuf::RepeatedPtrField<::webhook_api::v1::CreateWebhookResponse>* field) {
  field->Clear();
}
void __rust_proto_thunk__webhook_api_v1_CreateWebhookResponse_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::CreateWebhookResponse>& dst,
  const google::protobuf::RepeatedPtrField<::webhook_api::v1::CreateWebhookResponse>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__webhook_api_v1_CreateWebhookResponse_get_webhook_public_key(::webhook_api::v1::CreateWebhookResponse* msg) {
  absl::string_view val = msg->webhook_public_key();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__webhook_api_v1_CreateWebhookResponse_set_webhook_public_key(::webhook_api::v1::CreateWebhookResponse* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_webhook_public_key(absl::string_view(s.ptr, s.len));
}

const void* __rust_proto_thunk__webhook_api_v1_CreateWebhookResponse_get_webhook(::webhook_api::v1::CreateWebhookResponse* msg) {
  return static_cast<const void*>(&msg->webhook());
}
void* __rust_proto_thunk__webhook_api_v1_CreateWebhookResponse_get_mut_webhook(::webhook_api::v1::CreateWebhookResponse* msg) {
  return static_cast<void*>(msg->mutable_webhook());
}
void __rust_proto_thunk__webhook_api_v1_CreateWebhookResponse_clear_webhook(::webhook_api::v1::CreateWebhookResponse* msg) { msg->clear_webhook(); }
bool __rust_proto_thunk__webhook_api_v1_CreateWebhookResponse_has_webhook(::webhook_api::v1::CreateWebhookResponse* msg) { return msg->has_webhook(); }


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::webhook_api::v1::CreateWebhookResponse>* __rust_proto_thunk__Map_i32_webhook_api_v1_CreateWebhookResponse_new() {
    return new google::protobuf::Map<int32_t, ::webhook_api::v1::CreateWebhookResponse>();
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_CreateWebhookResponse_free(const google::protobuf::Map<int32_t, ::webhook_api::v1::CreateWebhookResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_CreateWebhookResponse_clear(google::protobuf::Map<int32_t, ::webhook_api::v1::CreateWebhookResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_webhook_api_v1_CreateWebhookResponse_size(const google::protobuf::Map<int32_t, ::webhook_api::v1::CreateWebhookResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_CreateWebhookResponse_insert(google::protobuf::Map<int32_t, ::webhook_api::v1::CreateWebhookResponse> * m,
                          int32_t key, ::webhook_api::v1::CreateWebhookResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_CreateWebhookResponse_get(const google::protobuf::Map<int32_t, ::webhook_api::v1::CreateWebhookResponse>* m,
                       int32_t key, const ::webhook_api::v1::CreateWebhookResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::CreateWebhookResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_CreateWebhookResponse_remove(google::protobuf::Map<int32_t, ::webhook_api::v1::CreateWebhookResponse> * m,
                          int32_t key, ::webhook_api::v1::CreateWebhookResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_webhook_api_v1_CreateWebhookResponse_iter(
      const google::protobuf::Map<int32_t, ::webhook_api::v1::CreateWebhookResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_CreateWebhookResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::webhook_api::v1::CreateWebhookResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::webhook_api::v1::CreateWebhookResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::webhook_api::v1::CreateWebhookResponse>* __rust_proto_thunk__Map_u32_webhook_api_v1_CreateWebhookResponse_new() {
    return new google::protobuf::Map<uint32_t, ::webhook_api::v1::CreateWebhookResponse>();
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_CreateWebhookResponse_free(const google::protobuf::Map<uint32_t, ::webhook_api::v1::CreateWebhookResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_CreateWebhookResponse_clear(google::protobuf::Map<uint32_t, ::webhook_api::v1::CreateWebhookResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_webhook_api_v1_CreateWebhookResponse_size(const google::protobuf::Map<uint32_t, ::webhook_api::v1::CreateWebhookResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_CreateWebhookResponse_insert(google::protobuf::Map<uint32_t, ::webhook_api::v1::CreateWebhookResponse> * m,
                          uint32_t key, ::webhook_api::v1::CreateWebhookResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_CreateWebhookResponse_get(const google::protobuf::Map<uint32_t, ::webhook_api::v1::CreateWebhookResponse>* m,
                       uint32_t key, const ::webhook_api::v1::CreateWebhookResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::CreateWebhookResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_CreateWebhookResponse_remove(google::protobuf::Map<uint32_t, ::webhook_api::v1::CreateWebhookResponse> * m,
                          uint32_t key, ::webhook_api::v1::CreateWebhookResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_webhook_api_v1_CreateWebhookResponse_iter(
      const google::protobuf::Map<uint32_t, ::webhook_api::v1::CreateWebhookResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_CreateWebhookResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::webhook_api::v1::CreateWebhookResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::webhook_api::v1::CreateWebhookResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::webhook_api::v1::CreateWebhookResponse>* __rust_proto_thunk__Map_i64_webhook_api_v1_CreateWebhookResponse_new() {
    return new google::protobuf::Map<int64_t, ::webhook_api::v1::CreateWebhookResponse>();
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_CreateWebhookResponse_free(const google::protobuf::Map<int64_t, ::webhook_api::v1::CreateWebhookResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_CreateWebhookResponse_clear(google::protobuf::Map<int64_t, ::webhook_api::v1::CreateWebhookResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_webhook_api_v1_CreateWebhookResponse_size(const google::protobuf::Map<int64_t, ::webhook_api::v1::CreateWebhookResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_CreateWebhookResponse_insert(google::protobuf::Map<int64_t, ::webhook_api::v1::CreateWebhookResponse> * m,
                          int64_t key, ::webhook_api::v1::CreateWebhookResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_CreateWebhookResponse_get(const google::protobuf::Map<int64_t, ::webhook_api::v1::CreateWebhookResponse>* m,
                       int64_t key, const ::webhook_api::v1::CreateWebhookResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::CreateWebhookResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_CreateWebhookResponse_remove(google::protobuf::Map<int64_t, ::webhook_api::v1::CreateWebhookResponse> * m,
                          int64_t key, ::webhook_api::v1::CreateWebhookResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_webhook_api_v1_CreateWebhookResponse_iter(
      const google::protobuf::Map<int64_t, ::webhook_api::v1::CreateWebhookResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_CreateWebhookResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::webhook_api::v1::CreateWebhookResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::webhook_api::v1::CreateWebhookResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::webhook_api::v1::CreateWebhookResponse>* __rust_proto_thunk__Map_u64_webhook_api_v1_CreateWebhookResponse_new() {
    return new google::protobuf::Map<uint64_t, ::webhook_api::v1::CreateWebhookResponse>();
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_CreateWebhookResponse_free(const google::protobuf::Map<uint64_t, ::webhook_api::v1::CreateWebhookResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_CreateWebhookResponse_clear(google::protobuf::Map<uint64_t, ::webhook_api::v1::CreateWebhookResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_webhook_api_v1_CreateWebhookResponse_size(const google::protobuf::Map<uint64_t, ::webhook_api::v1::CreateWebhookResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_CreateWebhookResponse_insert(google::protobuf::Map<uint64_t, ::webhook_api::v1::CreateWebhookResponse> * m,
                          uint64_t key, ::webhook_api::v1::CreateWebhookResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_CreateWebhookResponse_get(const google::protobuf::Map<uint64_t, ::webhook_api::v1::CreateWebhookResponse>* m,
                       uint64_t key, const ::webhook_api::v1::CreateWebhookResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::CreateWebhookResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_CreateWebhookResponse_remove(google::protobuf::Map<uint64_t, ::webhook_api::v1::CreateWebhookResponse> * m,
                          uint64_t key, ::webhook_api::v1::CreateWebhookResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_webhook_api_v1_CreateWebhookResponse_iter(
      const google::protobuf::Map<uint64_t, ::webhook_api::v1::CreateWebhookResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_CreateWebhookResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::webhook_api::v1::CreateWebhookResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::webhook_api::v1::CreateWebhookResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::webhook_api::v1::CreateWebhookResponse>* __rust_proto_thunk__Map_bool_webhook_api_v1_CreateWebhookResponse_new() {
    return new google::protobuf::Map<bool, ::webhook_api::v1::CreateWebhookResponse>();
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_CreateWebhookResponse_free(const google::protobuf::Map<bool, ::webhook_api::v1::CreateWebhookResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_CreateWebhookResponse_clear(google::protobuf::Map<bool, ::webhook_api::v1::CreateWebhookResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_webhook_api_v1_CreateWebhookResponse_size(const google::protobuf::Map<bool, ::webhook_api::v1::CreateWebhookResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_CreateWebhookResponse_insert(google::protobuf::Map<bool, ::webhook_api::v1::CreateWebhookResponse> * m,
                          bool key, ::webhook_api::v1::CreateWebhookResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_CreateWebhookResponse_get(const google::protobuf::Map<bool, ::webhook_api::v1::CreateWebhookResponse>* m,
                       bool key, const ::webhook_api::v1::CreateWebhookResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::CreateWebhookResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_CreateWebhookResponse_remove(google::protobuf::Map<bool, ::webhook_api::v1::CreateWebhookResponse> * m,
                          bool key, ::webhook_api::v1::CreateWebhookResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_webhook_api_v1_CreateWebhookResponse_iter(
      const google::protobuf::Map<bool, ::webhook_api::v1::CreateWebhookResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_CreateWebhookResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::webhook_api::v1::CreateWebhookResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::webhook_api::v1::CreateWebhookResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::webhook_api::v1::CreateWebhookResponse>* __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_CreateWebhookResponse_new() {
    return new google::protobuf::Map<std::string, ::webhook_api::v1::CreateWebhookResponse>();
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_CreateWebhookResponse_free(const google::protobuf::Map<std::string, ::webhook_api::v1::CreateWebhookResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_CreateWebhookResponse_clear(google::protobuf::Map<std::string, ::webhook_api::v1::CreateWebhookResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_CreateWebhookResponse_size(const google::protobuf::Map<std::string, ::webhook_api::v1::CreateWebhookResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_CreateWebhookResponse_insert(google::protobuf::Map<std::string, ::webhook_api::v1::CreateWebhookResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::CreateWebhookResponse value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_CreateWebhookResponse_get(const google::protobuf::Map<std::string, ::webhook_api::v1::CreateWebhookResponse>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::webhook_api::v1::CreateWebhookResponse** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::CreateWebhookResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_CreateWebhookResponse_remove(google::protobuf::Map<std::string, ::webhook_api::v1::CreateWebhookResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::CreateWebhookResponse * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_CreateWebhookResponse_iter(
      const google::protobuf::Map<std::string, ::webhook_api::v1::CreateWebhookResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_CreateWebhookResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::webhook_api::v1::CreateWebhookResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::webhook_api::v1::CreateWebhookResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// webhook_api.v1.ListWebhooksRequest
        // clang-format off
extern "C" {
void* __rust_proto_thunk__webhook_api_v1_ListWebhooksRequest_new() { return new ::webhook_api::v1::ListWebhooksRequest(); }
void __rust_proto_thunk__webhook_api_v1_ListWebhooksRequest_delete(void* ptr) { delete static_cast<::webhook_api::v1::ListWebhooksRequest*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__webhook_api_v1_ListWebhooksRequest_serialize(::webhook_api::v1::ListWebhooksRequest* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__webhook_api_v1_ListWebhooksRequest_parse(::webhook_api::v1::ListWebhooksRequest* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__webhook_api_v1_ListWebhooksRequest_copy_from(::webhook_api::v1::ListWebhooksRequest* dst, const ::webhook_api::v1::ListWebhooksRequest* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__webhook_api_v1_ListWebhooksRequest_repeated_len(google::protobuf::RepeatedPtrField<::webhook_api::v1::ListWebhooksRequest>* field) {
  return field->size();
}
const ::webhook_api::v1::ListWebhooksRequest& __rust_proto_thunk__webhook_api_v1_ListWebhooksRequest_repeated_get(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::ListWebhooksRequest>* field,
  size_t index) {
  return field->Get(index);
}
::webhook_api::v1::ListWebhooksRequest* __rust_proto_thunk__webhook_api_v1_ListWebhooksRequest_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::ListWebhooksRequest>* field,
  size_t index) {
  return field->Mutable(index);
}
::webhook_api::v1::ListWebhooksRequest* __rust_proto_thunk__webhook_api_v1_ListWebhooksRequest_repeated_add(google::protobuf::RepeatedPtrField<::webhook_api::v1::ListWebhooksRequest>* field) {
  return field->Add();
}
void __rust_proto_thunk__webhook_api_v1_ListWebhooksRequest_repeated_clear(google::protobuf::RepeatedPtrField<::webhook_api::v1::ListWebhooksRequest>* field) {
  field->Clear();
}
void __rust_proto_thunk__webhook_api_v1_ListWebhooksRequest_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::ListWebhooksRequest>& dst,
  const google::protobuf::RepeatedPtrField<::webhook_api::v1::ListWebhooksRequest>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__webhook_api_v1_ListWebhooksRequest_get_api_key(::webhook_api::v1::ListWebhooksRequest* msg) {
  absl::string_view val = msg->api_key();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__webhook_api_v1_ListWebhooksRequest_set_api_key(::webhook_api::v1::ListWebhooksRequest* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_api_key(absl::string_view(s.ptr, s.len));
}

::int32_t __rust_proto_thunk__webhook_api_v1_ListWebhooksRequest_get_page(::webhook_api::v1::ListWebhooksRequest* msg) { return msg->page(); }
void __rust_proto_thunk__webhook_api_v1_ListWebhooksRequest_set_page(::webhook_api::v1::ListWebhooksRequest* msg, ::int32_t val) {
  msg->set_page(val);
}

::int32_t __rust_proto_thunk__webhook_api_v1_ListWebhooksRequest_get_page_size(::webhook_api::v1::ListWebhooksRequest* msg) { return msg->page_size(); }
void __rust_proto_thunk__webhook_api_v1_ListWebhooksRequest_set_page_size(::webhook_api::v1::ListWebhooksRequest* msg, ::int32_t val) {
  msg->set_page_size(val);
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::webhook_api::v1::ListWebhooksRequest>* __rust_proto_thunk__Map_i32_webhook_api_v1_ListWebhooksRequest_new() {
    return new google::protobuf::Map<int32_t, ::webhook_api::v1::ListWebhooksRequest>();
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_ListWebhooksRequest_free(const google::protobuf::Map<int32_t, ::webhook_api::v1::ListWebhooksRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_ListWebhooksRequest_clear(google::protobuf::Map<int32_t, ::webhook_api::v1::ListWebhooksRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_webhook_api_v1_ListWebhooksRequest_size(const google::protobuf::Map<int32_t, ::webhook_api::v1::ListWebhooksRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_ListWebhooksRequest_insert(google::protobuf::Map<int32_t, ::webhook_api::v1::ListWebhooksRequest> * m,
                          int32_t key, ::webhook_api::v1::ListWebhooksRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_ListWebhooksRequest_get(const google::protobuf::Map<int32_t, ::webhook_api::v1::ListWebhooksRequest>* m,
                       int32_t key, const ::webhook_api::v1::ListWebhooksRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ListWebhooksRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_ListWebhooksRequest_remove(google::protobuf::Map<int32_t, ::webhook_api::v1::ListWebhooksRequest> * m,
                          int32_t key, ::webhook_api::v1::ListWebhooksRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_webhook_api_v1_ListWebhooksRequest_iter(
      const google::protobuf::Map<int32_t, ::webhook_api::v1::ListWebhooksRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_ListWebhooksRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::webhook_api::v1::ListWebhooksRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::webhook_api::v1::ListWebhooksRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::webhook_api::v1::ListWebhooksRequest>* __rust_proto_thunk__Map_u32_webhook_api_v1_ListWebhooksRequest_new() {
    return new google::protobuf::Map<uint32_t, ::webhook_api::v1::ListWebhooksRequest>();
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_ListWebhooksRequest_free(const google::protobuf::Map<uint32_t, ::webhook_api::v1::ListWebhooksRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_ListWebhooksRequest_clear(google::protobuf::Map<uint32_t, ::webhook_api::v1::ListWebhooksRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_webhook_api_v1_ListWebhooksRequest_size(const google::protobuf::Map<uint32_t, ::webhook_api::v1::ListWebhooksRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_ListWebhooksRequest_insert(google::protobuf::Map<uint32_t, ::webhook_api::v1::ListWebhooksRequest> * m,
                          uint32_t key, ::webhook_api::v1::ListWebhooksRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_ListWebhooksRequest_get(const google::protobuf::Map<uint32_t, ::webhook_api::v1::ListWebhooksRequest>* m,
                       uint32_t key, const ::webhook_api::v1::ListWebhooksRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ListWebhooksRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_ListWebhooksRequest_remove(google::protobuf::Map<uint32_t, ::webhook_api::v1::ListWebhooksRequest> * m,
                          uint32_t key, ::webhook_api::v1::ListWebhooksRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_webhook_api_v1_ListWebhooksRequest_iter(
      const google::protobuf::Map<uint32_t, ::webhook_api::v1::ListWebhooksRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_ListWebhooksRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::webhook_api::v1::ListWebhooksRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::webhook_api::v1::ListWebhooksRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::webhook_api::v1::ListWebhooksRequest>* __rust_proto_thunk__Map_i64_webhook_api_v1_ListWebhooksRequest_new() {
    return new google::protobuf::Map<int64_t, ::webhook_api::v1::ListWebhooksRequest>();
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_ListWebhooksRequest_free(const google::protobuf::Map<int64_t, ::webhook_api::v1::ListWebhooksRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_ListWebhooksRequest_clear(google::protobuf::Map<int64_t, ::webhook_api::v1::ListWebhooksRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_webhook_api_v1_ListWebhooksRequest_size(const google::protobuf::Map<int64_t, ::webhook_api::v1::ListWebhooksRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_ListWebhooksRequest_insert(google::protobuf::Map<int64_t, ::webhook_api::v1::ListWebhooksRequest> * m,
                          int64_t key, ::webhook_api::v1::ListWebhooksRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_ListWebhooksRequest_get(const google::protobuf::Map<int64_t, ::webhook_api::v1::ListWebhooksRequest>* m,
                       int64_t key, const ::webhook_api::v1::ListWebhooksRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ListWebhooksRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_ListWebhooksRequest_remove(google::protobuf::Map<int64_t, ::webhook_api::v1::ListWebhooksRequest> * m,
                          int64_t key, ::webhook_api::v1::ListWebhooksRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_webhook_api_v1_ListWebhooksRequest_iter(
      const google::protobuf::Map<int64_t, ::webhook_api::v1::ListWebhooksRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_ListWebhooksRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::webhook_api::v1::ListWebhooksRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::webhook_api::v1::ListWebhooksRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::webhook_api::v1::ListWebhooksRequest>* __rust_proto_thunk__Map_u64_webhook_api_v1_ListWebhooksRequest_new() {
    return new google::protobuf::Map<uint64_t, ::webhook_api::v1::ListWebhooksRequest>();
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_ListWebhooksRequest_free(const google::protobuf::Map<uint64_t, ::webhook_api::v1::ListWebhooksRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_ListWebhooksRequest_clear(google::protobuf::Map<uint64_t, ::webhook_api::v1::ListWebhooksRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_webhook_api_v1_ListWebhooksRequest_size(const google::protobuf::Map<uint64_t, ::webhook_api::v1::ListWebhooksRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_ListWebhooksRequest_insert(google::protobuf::Map<uint64_t, ::webhook_api::v1::ListWebhooksRequest> * m,
                          uint64_t key, ::webhook_api::v1::ListWebhooksRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_ListWebhooksRequest_get(const google::protobuf::Map<uint64_t, ::webhook_api::v1::ListWebhooksRequest>* m,
                       uint64_t key, const ::webhook_api::v1::ListWebhooksRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ListWebhooksRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_ListWebhooksRequest_remove(google::protobuf::Map<uint64_t, ::webhook_api::v1::ListWebhooksRequest> * m,
                          uint64_t key, ::webhook_api::v1::ListWebhooksRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_webhook_api_v1_ListWebhooksRequest_iter(
      const google::protobuf::Map<uint64_t, ::webhook_api::v1::ListWebhooksRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_ListWebhooksRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::webhook_api::v1::ListWebhooksRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::webhook_api::v1::ListWebhooksRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::webhook_api::v1::ListWebhooksRequest>* __rust_proto_thunk__Map_bool_webhook_api_v1_ListWebhooksRequest_new() {
    return new google::protobuf::Map<bool, ::webhook_api::v1::ListWebhooksRequest>();
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_ListWebhooksRequest_free(const google::protobuf::Map<bool, ::webhook_api::v1::ListWebhooksRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_ListWebhooksRequest_clear(google::protobuf::Map<bool, ::webhook_api::v1::ListWebhooksRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_webhook_api_v1_ListWebhooksRequest_size(const google::protobuf::Map<bool, ::webhook_api::v1::ListWebhooksRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_ListWebhooksRequest_insert(google::protobuf::Map<bool, ::webhook_api::v1::ListWebhooksRequest> * m,
                          bool key, ::webhook_api::v1::ListWebhooksRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_ListWebhooksRequest_get(const google::protobuf::Map<bool, ::webhook_api::v1::ListWebhooksRequest>* m,
                       bool key, const ::webhook_api::v1::ListWebhooksRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ListWebhooksRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_ListWebhooksRequest_remove(google::protobuf::Map<bool, ::webhook_api::v1::ListWebhooksRequest> * m,
                          bool key, ::webhook_api::v1::ListWebhooksRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_webhook_api_v1_ListWebhooksRequest_iter(
      const google::protobuf::Map<bool, ::webhook_api::v1::ListWebhooksRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_ListWebhooksRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::webhook_api::v1::ListWebhooksRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::webhook_api::v1::ListWebhooksRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::webhook_api::v1::ListWebhooksRequest>* __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListWebhooksRequest_new() {
    return new google::protobuf::Map<std::string, ::webhook_api::v1::ListWebhooksRequest>();
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListWebhooksRequest_free(const google::protobuf::Map<std::string, ::webhook_api::v1::ListWebhooksRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListWebhooksRequest_clear(google::protobuf::Map<std::string, ::webhook_api::v1::ListWebhooksRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListWebhooksRequest_size(const google::protobuf::Map<std::string, ::webhook_api::v1::ListWebhooksRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListWebhooksRequest_insert(google::protobuf::Map<std::string, ::webhook_api::v1::ListWebhooksRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::ListWebhooksRequest value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListWebhooksRequest_get(const google::protobuf::Map<std::string, ::webhook_api::v1::ListWebhooksRequest>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::webhook_api::v1::ListWebhooksRequest** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ListWebhooksRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListWebhooksRequest_remove(google::protobuf::Map<std::string, ::webhook_api::v1::ListWebhooksRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::ListWebhooksRequest * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListWebhooksRequest_iter(
      const google::protobuf::Map<std::string, ::webhook_api::v1::ListWebhooksRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListWebhooksRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::webhook_api::v1::ListWebhooksRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::webhook_api::v1::ListWebhooksRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// webhook_api.v1.ListWebhooksResponse
        // clang-format off
extern "C" {
void* __rust_proto_thunk__webhook_api_v1_ListWebhooksResponse_new() { return new ::webhook_api::v1::ListWebhooksResponse(); }
void __rust_proto_thunk__webhook_api_v1_ListWebhooksResponse_delete(void* ptr) { delete static_cast<::webhook_api::v1::ListWebhooksResponse*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__webhook_api_v1_ListWebhooksResponse_serialize(::webhook_api::v1::ListWebhooksResponse* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__webhook_api_v1_ListWebhooksResponse_parse(::webhook_api::v1::ListWebhooksResponse* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__webhook_api_v1_ListWebhooksResponse_copy_from(::webhook_api::v1::ListWebhooksResponse* dst, const ::webhook_api::v1::ListWebhooksResponse* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__webhook_api_v1_ListWebhooksResponse_repeated_len(google::protobuf::RepeatedPtrField<::webhook_api::v1::ListWebhooksResponse>* field) {
  return field->size();
}
const ::webhook_api::v1::ListWebhooksResponse& __rust_proto_thunk__webhook_api_v1_ListWebhooksResponse_repeated_get(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::ListWebhooksResponse>* field,
  size_t index) {
  return field->Get(index);
}
::webhook_api::v1::ListWebhooksResponse* __rust_proto_thunk__webhook_api_v1_ListWebhooksResponse_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::ListWebhooksResponse>* field,
  size_t index) {
  return field->Mutable(index);
}
::webhook_api::v1::ListWebhooksResponse* __rust_proto_thunk__webhook_api_v1_ListWebhooksResponse_repeated_add(google::protobuf::RepeatedPtrField<::webhook_api::v1::ListWebhooksResponse>* field) {
  return field->Add();
}
void __rust_proto_thunk__webhook_api_v1_ListWebhooksResponse_repeated_clear(google::protobuf::RepeatedPtrField<::webhook_api::v1::ListWebhooksResponse>* field) {
  field->Clear();
}
void __rust_proto_thunk__webhook_api_v1_ListWebhooksResponse_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::ListWebhooksResponse>& dst,
  const google::protobuf::RepeatedPtrField<::webhook_api::v1::ListWebhooksResponse>& src) {
  dst = src;
}

void __rust_proto_thunk__webhook_api_v1_ListWebhooksResponse_clear_webhooks(::webhook_api::v1::ListWebhooksResponse* msg) {
  msg->clear_webhooks();
}
google::protobuf::RepeatedPtrField<::webhook_api::v1::Webhook>* __rust_proto_thunk__webhook_api_v1_ListWebhooksResponse_get_mut_webhooks(::webhook_api::v1::ListWebhooksResponse* msg) {
  return msg->mutable_webhooks();
}
const google::protobuf::RepeatedPtrField<::webhook_api::v1::Webhook>* __rust_proto_thunk__webhook_api_v1_ListWebhooksResponse_get_webhooks(
    const ::webhook_api::v1::ListWebhooksResponse* msg) {
  return &msg->webhooks();
}

::uint32_t __rust_proto_thunk__webhook_api_v1_ListWebhooksResponse_get_total_owned_webhooks(::webhook_api::v1::ListWebhooksResponse* msg) { return msg->total_owned_webhooks(); }
void __rust_proto_thunk__webhook_api_v1_ListWebhooksResponse_set_total_owned_webhooks(::webhook_api::v1::ListWebhooksResponse* msg, ::uint32_t val) {
  msg->set_total_owned_webhooks(val);
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::webhook_api::v1::ListWebhooksResponse>* __rust_proto_thunk__Map_i32_webhook_api_v1_ListWebhooksResponse_new() {
    return new google::protobuf::Map<int32_t, ::webhook_api::v1::ListWebhooksResponse>();
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_ListWebhooksResponse_free(const google::protobuf::Map<int32_t, ::webhook_api::v1::ListWebhooksResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_ListWebhooksResponse_clear(google::protobuf::Map<int32_t, ::webhook_api::v1::ListWebhooksResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_webhook_api_v1_ListWebhooksResponse_size(const google::protobuf::Map<int32_t, ::webhook_api::v1::ListWebhooksResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_ListWebhooksResponse_insert(google::protobuf::Map<int32_t, ::webhook_api::v1::ListWebhooksResponse> * m,
                          int32_t key, ::webhook_api::v1::ListWebhooksResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_ListWebhooksResponse_get(const google::protobuf::Map<int32_t, ::webhook_api::v1::ListWebhooksResponse>* m,
                       int32_t key, const ::webhook_api::v1::ListWebhooksResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ListWebhooksResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_ListWebhooksResponse_remove(google::protobuf::Map<int32_t, ::webhook_api::v1::ListWebhooksResponse> * m,
                          int32_t key, ::webhook_api::v1::ListWebhooksResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_webhook_api_v1_ListWebhooksResponse_iter(
      const google::protobuf::Map<int32_t, ::webhook_api::v1::ListWebhooksResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_ListWebhooksResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::webhook_api::v1::ListWebhooksResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::webhook_api::v1::ListWebhooksResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::webhook_api::v1::ListWebhooksResponse>* __rust_proto_thunk__Map_u32_webhook_api_v1_ListWebhooksResponse_new() {
    return new google::protobuf::Map<uint32_t, ::webhook_api::v1::ListWebhooksResponse>();
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_ListWebhooksResponse_free(const google::protobuf::Map<uint32_t, ::webhook_api::v1::ListWebhooksResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_ListWebhooksResponse_clear(google::protobuf::Map<uint32_t, ::webhook_api::v1::ListWebhooksResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_webhook_api_v1_ListWebhooksResponse_size(const google::protobuf::Map<uint32_t, ::webhook_api::v1::ListWebhooksResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_ListWebhooksResponse_insert(google::protobuf::Map<uint32_t, ::webhook_api::v1::ListWebhooksResponse> * m,
                          uint32_t key, ::webhook_api::v1::ListWebhooksResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_ListWebhooksResponse_get(const google::protobuf::Map<uint32_t, ::webhook_api::v1::ListWebhooksResponse>* m,
                       uint32_t key, const ::webhook_api::v1::ListWebhooksResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ListWebhooksResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_ListWebhooksResponse_remove(google::protobuf::Map<uint32_t, ::webhook_api::v1::ListWebhooksResponse> * m,
                          uint32_t key, ::webhook_api::v1::ListWebhooksResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_webhook_api_v1_ListWebhooksResponse_iter(
      const google::protobuf::Map<uint32_t, ::webhook_api::v1::ListWebhooksResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_ListWebhooksResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::webhook_api::v1::ListWebhooksResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::webhook_api::v1::ListWebhooksResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::webhook_api::v1::ListWebhooksResponse>* __rust_proto_thunk__Map_i64_webhook_api_v1_ListWebhooksResponse_new() {
    return new google::protobuf::Map<int64_t, ::webhook_api::v1::ListWebhooksResponse>();
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_ListWebhooksResponse_free(const google::protobuf::Map<int64_t, ::webhook_api::v1::ListWebhooksResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_ListWebhooksResponse_clear(google::protobuf::Map<int64_t, ::webhook_api::v1::ListWebhooksResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_webhook_api_v1_ListWebhooksResponse_size(const google::protobuf::Map<int64_t, ::webhook_api::v1::ListWebhooksResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_ListWebhooksResponse_insert(google::protobuf::Map<int64_t, ::webhook_api::v1::ListWebhooksResponse> * m,
                          int64_t key, ::webhook_api::v1::ListWebhooksResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_ListWebhooksResponse_get(const google::protobuf::Map<int64_t, ::webhook_api::v1::ListWebhooksResponse>* m,
                       int64_t key, const ::webhook_api::v1::ListWebhooksResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ListWebhooksResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_ListWebhooksResponse_remove(google::protobuf::Map<int64_t, ::webhook_api::v1::ListWebhooksResponse> * m,
                          int64_t key, ::webhook_api::v1::ListWebhooksResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_webhook_api_v1_ListWebhooksResponse_iter(
      const google::protobuf::Map<int64_t, ::webhook_api::v1::ListWebhooksResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_ListWebhooksResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::webhook_api::v1::ListWebhooksResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::webhook_api::v1::ListWebhooksResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::webhook_api::v1::ListWebhooksResponse>* __rust_proto_thunk__Map_u64_webhook_api_v1_ListWebhooksResponse_new() {
    return new google::protobuf::Map<uint64_t, ::webhook_api::v1::ListWebhooksResponse>();
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_ListWebhooksResponse_free(const google::protobuf::Map<uint64_t, ::webhook_api::v1::ListWebhooksResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_ListWebhooksResponse_clear(google::protobuf::Map<uint64_t, ::webhook_api::v1::ListWebhooksResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_webhook_api_v1_ListWebhooksResponse_size(const google::protobuf::Map<uint64_t, ::webhook_api::v1::ListWebhooksResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_ListWebhooksResponse_insert(google::protobuf::Map<uint64_t, ::webhook_api::v1::ListWebhooksResponse> * m,
                          uint64_t key, ::webhook_api::v1::ListWebhooksResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_ListWebhooksResponse_get(const google::protobuf::Map<uint64_t, ::webhook_api::v1::ListWebhooksResponse>* m,
                       uint64_t key, const ::webhook_api::v1::ListWebhooksResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ListWebhooksResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_ListWebhooksResponse_remove(google::protobuf::Map<uint64_t, ::webhook_api::v1::ListWebhooksResponse> * m,
                          uint64_t key, ::webhook_api::v1::ListWebhooksResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_webhook_api_v1_ListWebhooksResponse_iter(
      const google::protobuf::Map<uint64_t, ::webhook_api::v1::ListWebhooksResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_ListWebhooksResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::webhook_api::v1::ListWebhooksResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::webhook_api::v1::ListWebhooksResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::webhook_api::v1::ListWebhooksResponse>* __rust_proto_thunk__Map_bool_webhook_api_v1_ListWebhooksResponse_new() {
    return new google::protobuf::Map<bool, ::webhook_api::v1::ListWebhooksResponse>();
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_ListWebhooksResponse_free(const google::protobuf::Map<bool, ::webhook_api::v1::ListWebhooksResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_ListWebhooksResponse_clear(google::protobuf::Map<bool, ::webhook_api::v1::ListWebhooksResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_webhook_api_v1_ListWebhooksResponse_size(const google::protobuf::Map<bool, ::webhook_api::v1::ListWebhooksResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_ListWebhooksResponse_insert(google::protobuf::Map<bool, ::webhook_api::v1::ListWebhooksResponse> * m,
                          bool key, ::webhook_api::v1::ListWebhooksResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_ListWebhooksResponse_get(const google::protobuf::Map<bool, ::webhook_api::v1::ListWebhooksResponse>* m,
                       bool key, const ::webhook_api::v1::ListWebhooksResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ListWebhooksResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_ListWebhooksResponse_remove(google::protobuf::Map<bool, ::webhook_api::v1::ListWebhooksResponse> * m,
                          bool key, ::webhook_api::v1::ListWebhooksResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_webhook_api_v1_ListWebhooksResponse_iter(
      const google::protobuf::Map<bool, ::webhook_api::v1::ListWebhooksResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_ListWebhooksResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::webhook_api::v1::ListWebhooksResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::webhook_api::v1::ListWebhooksResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::webhook_api::v1::ListWebhooksResponse>* __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListWebhooksResponse_new() {
    return new google::protobuf::Map<std::string, ::webhook_api::v1::ListWebhooksResponse>();
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListWebhooksResponse_free(const google::protobuf::Map<std::string, ::webhook_api::v1::ListWebhooksResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListWebhooksResponse_clear(google::protobuf::Map<std::string, ::webhook_api::v1::ListWebhooksResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListWebhooksResponse_size(const google::protobuf::Map<std::string, ::webhook_api::v1::ListWebhooksResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListWebhooksResponse_insert(google::protobuf::Map<std::string, ::webhook_api::v1::ListWebhooksResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::ListWebhooksResponse value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListWebhooksResponse_get(const google::protobuf::Map<std::string, ::webhook_api::v1::ListWebhooksResponse>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::webhook_api::v1::ListWebhooksResponse** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ListWebhooksResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListWebhooksResponse_remove(google::protobuf::Map<std::string, ::webhook_api::v1::ListWebhooksResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::ListWebhooksResponse * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListWebhooksResponse_iter(
      const google::protobuf::Map<std::string, ::webhook_api::v1::ListWebhooksResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListWebhooksResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::webhook_api::v1::ListWebhooksResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::webhook_api::v1::ListWebhooksResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// webhook_api.v1.DeleteWebhookRequest
        // clang-format off
extern "C" {
void* __rust_proto_thunk__webhook_api_v1_DeleteWebhookRequest_new() { return new ::webhook_api::v1::DeleteWebhookRequest(); }
void __rust_proto_thunk__webhook_api_v1_DeleteWebhookRequest_delete(void* ptr) { delete static_cast<::webhook_api::v1::DeleteWebhookRequest*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__webhook_api_v1_DeleteWebhookRequest_serialize(::webhook_api::v1::DeleteWebhookRequest* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__webhook_api_v1_DeleteWebhookRequest_parse(::webhook_api::v1::DeleteWebhookRequest* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__webhook_api_v1_DeleteWebhookRequest_copy_from(::webhook_api::v1::DeleteWebhookRequest* dst, const ::webhook_api::v1::DeleteWebhookRequest* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__webhook_api_v1_DeleteWebhookRequest_repeated_len(google::protobuf::RepeatedPtrField<::webhook_api::v1::DeleteWebhookRequest>* field) {
  return field->size();
}
const ::webhook_api::v1::DeleteWebhookRequest& __rust_proto_thunk__webhook_api_v1_DeleteWebhookRequest_repeated_get(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::DeleteWebhookRequest>* field,
  size_t index) {
  return field->Get(index);
}
::webhook_api::v1::DeleteWebhookRequest* __rust_proto_thunk__webhook_api_v1_DeleteWebhookRequest_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::DeleteWebhookRequest>* field,
  size_t index) {
  return field->Mutable(index);
}
::webhook_api::v1::DeleteWebhookRequest* __rust_proto_thunk__webhook_api_v1_DeleteWebhookRequest_repeated_add(google::protobuf::RepeatedPtrField<::webhook_api::v1::DeleteWebhookRequest>* field) {
  return field->Add();
}
void __rust_proto_thunk__webhook_api_v1_DeleteWebhookRequest_repeated_clear(google::protobuf::RepeatedPtrField<::webhook_api::v1::DeleteWebhookRequest>* field) {
  field->Clear();
}
void __rust_proto_thunk__webhook_api_v1_DeleteWebhookRequest_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::DeleteWebhookRequest>& dst,
  const google::protobuf::RepeatedPtrField<::webhook_api::v1::DeleteWebhookRequest>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__webhook_api_v1_DeleteWebhookRequest_get_api_key(::webhook_api::v1::DeleteWebhookRequest* msg) {
  absl::string_view val = msg->api_key();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__webhook_api_v1_DeleteWebhookRequest_set_api_key(::webhook_api::v1::DeleteWebhookRequest* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_api_key(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__webhook_api_v1_DeleteWebhookRequest_get_webhook_id(::webhook_api::v1::DeleteWebhookRequest* msg) {
  absl::string_view val = msg->webhook_id();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__webhook_api_v1_DeleteWebhookRequest_set_webhook_id(::webhook_api::v1::DeleteWebhookRequest* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_webhook_id(absl::string_view(s.ptr, s.len));
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::webhook_api::v1::DeleteWebhookRequest>* __rust_proto_thunk__Map_i32_webhook_api_v1_DeleteWebhookRequest_new() {
    return new google::protobuf::Map<int32_t, ::webhook_api::v1::DeleteWebhookRequest>();
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_DeleteWebhookRequest_free(const google::protobuf::Map<int32_t, ::webhook_api::v1::DeleteWebhookRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_DeleteWebhookRequest_clear(google::protobuf::Map<int32_t, ::webhook_api::v1::DeleteWebhookRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_webhook_api_v1_DeleteWebhookRequest_size(const google::protobuf::Map<int32_t, ::webhook_api::v1::DeleteWebhookRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_DeleteWebhookRequest_insert(google::protobuf::Map<int32_t, ::webhook_api::v1::DeleteWebhookRequest> * m,
                          int32_t key, ::webhook_api::v1::DeleteWebhookRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_DeleteWebhookRequest_get(const google::protobuf::Map<int32_t, ::webhook_api::v1::DeleteWebhookRequest>* m,
                       int32_t key, const ::webhook_api::v1::DeleteWebhookRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::DeleteWebhookRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_DeleteWebhookRequest_remove(google::protobuf::Map<int32_t, ::webhook_api::v1::DeleteWebhookRequest> * m,
                          int32_t key, ::webhook_api::v1::DeleteWebhookRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_webhook_api_v1_DeleteWebhookRequest_iter(
      const google::protobuf::Map<int32_t, ::webhook_api::v1::DeleteWebhookRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_DeleteWebhookRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::webhook_api::v1::DeleteWebhookRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::webhook_api::v1::DeleteWebhookRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::webhook_api::v1::DeleteWebhookRequest>* __rust_proto_thunk__Map_u32_webhook_api_v1_DeleteWebhookRequest_new() {
    return new google::protobuf::Map<uint32_t, ::webhook_api::v1::DeleteWebhookRequest>();
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_DeleteWebhookRequest_free(const google::protobuf::Map<uint32_t, ::webhook_api::v1::DeleteWebhookRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_DeleteWebhookRequest_clear(google::protobuf::Map<uint32_t, ::webhook_api::v1::DeleteWebhookRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_webhook_api_v1_DeleteWebhookRequest_size(const google::protobuf::Map<uint32_t, ::webhook_api::v1::DeleteWebhookRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_DeleteWebhookRequest_insert(google::protobuf::Map<uint32_t, ::webhook_api::v1::DeleteWebhookRequest> * m,
                          uint32_t key, ::webhook_api::v1::DeleteWebhookRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_DeleteWebhookRequest_get(const google::protobuf::Map<uint32_t, ::webhook_api::v1::DeleteWebhookRequest>* m,
                       uint32_t key, const ::webhook_api::v1::DeleteWebhookRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::DeleteWebhookRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_DeleteWebhookRequest_remove(google::protobuf::Map<uint32_t, ::webhook_api::v1::DeleteWebhookRequest> * m,
                          uint32_t key, ::webhook_api::v1::DeleteWebhookRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_webhook_api_v1_DeleteWebhookRequest_iter(
      const google::protobuf::Map<uint32_t, ::webhook_api::v1::DeleteWebhookRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_DeleteWebhookRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::webhook_api::v1::DeleteWebhookRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::webhook_api::v1::DeleteWebhookRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::webhook_api::v1::DeleteWebhookRequest>* __rust_proto_thunk__Map_i64_webhook_api_v1_DeleteWebhookRequest_new() {
    return new google::protobuf::Map<int64_t, ::webhook_api::v1::DeleteWebhookRequest>();
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_DeleteWebhookRequest_free(const google::protobuf::Map<int64_t, ::webhook_api::v1::DeleteWebhookRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_DeleteWebhookRequest_clear(google::protobuf::Map<int64_t, ::webhook_api::v1::DeleteWebhookRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_webhook_api_v1_DeleteWebhookRequest_size(const google::protobuf::Map<int64_t, ::webhook_api::v1::DeleteWebhookRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_DeleteWebhookRequest_insert(google::protobuf::Map<int64_t, ::webhook_api::v1::DeleteWebhookRequest> * m,
                          int64_t key, ::webhook_api::v1::DeleteWebhookRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_DeleteWebhookRequest_get(const google::protobuf::Map<int64_t, ::webhook_api::v1::DeleteWebhookRequest>* m,
                       int64_t key, const ::webhook_api::v1::DeleteWebhookRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::DeleteWebhookRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_DeleteWebhookRequest_remove(google::protobuf::Map<int64_t, ::webhook_api::v1::DeleteWebhookRequest> * m,
                          int64_t key, ::webhook_api::v1::DeleteWebhookRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_webhook_api_v1_DeleteWebhookRequest_iter(
      const google::protobuf::Map<int64_t, ::webhook_api::v1::DeleteWebhookRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_DeleteWebhookRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::webhook_api::v1::DeleteWebhookRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::webhook_api::v1::DeleteWebhookRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::webhook_api::v1::DeleteWebhookRequest>* __rust_proto_thunk__Map_u64_webhook_api_v1_DeleteWebhookRequest_new() {
    return new google::protobuf::Map<uint64_t, ::webhook_api::v1::DeleteWebhookRequest>();
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_DeleteWebhookRequest_free(const google::protobuf::Map<uint64_t, ::webhook_api::v1::DeleteWebhookRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_DeleteWebhookRequest_clear(google::protobuf::Map<uint64_t, ::webhook_api::v1::DeleteWebhookRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_webhook_api_v1_DeleteWebhookRequest_size(const google::protobuf::Map<uint64_t, ::webhook_api::v1::DeleteWebhookRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_DeleteWebhookRequest_insert(google::protobuf::Map<uint64_t, ::webhook_api::v1::DeleteWebhookRequest> * m,
                          uint64_t key, ::webhook_api::v1::DeleteWebhookRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_DeleteWebhookRequest_get(const google::protobuf::Map<uint64_t, ::webhook_api::v1::DeleteWebhookRequest>* m,
                       uint64_t key, const ::webhook_api::v1::DeleteWebhookRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::DeleteWebhookRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_DeleteWebhookRequest_remove(google::protobuf::Map<uint64_t, ::webhook_api::v1::DeleteWebhookRequest> * m,
                          uint64_t key, ::webhook_api::v1::DeleteWebhookRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_webhook_api_v1_DeleteWebhookRequest_iter(
      const google::protobuf::Map<uint64_t, ::webhook_api::v1::DeleteWebhookRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_DeleteWebhookRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::webhook_api::v1::DeleteWebhookRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::webhook_api::v1::DeleteWebhookRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::webhook_api::v1::DeleteWebhookRequest>* __rust_proto_thunk__Map_bool_webhook_api_v1_DeleteWebhookRequest_new() {
    return new google::protobuf::Map<bool, ::webhook_api::v1::DeleteWebhookRequest>();
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_DeleteWebhookRequest_free(const google::protobuf::Map<bool, ::webhook_api::v1::DeleteWebhookRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_DeleteWebhookRequest_clear(google::protobuf::Map<bool, ::webhook_api::v1::DeleteWebhookRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_webhook_api_v1_DeleteWebhookRequest_size(const google::protobuf::Map<bool, ::webhook_api::v1::DeleteWebhookRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_DeleteWebhookRequest_insert(google::protobuf::Map<bool, ::webhook_api::v1::DeleteWebhookRequest> * m,
                          bool key, ::webhook_api::v1::DeleteWebhookRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_DeleteWebhookRequest_get(const google::protobuf::Map<bool, ::webhook_api::v1::DeleteWebhookRequest>* m,
                       bool key, const ::webhook_api::v1::DeleteWebhookRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::DeleteWebhookRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_DeleteWebhookRequest_remove(google::protobuf::Map<bool, ::webhook_api::v1::DeleteWebhookRequest> * m,
                          bool key, ::webhook_api::v1::DeleteWebhookRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_webhook_api_v1_DeleteWebhookRequest_iter(
      const google::protobuf::Map<bool, ::webhook_api::v1::DeleteWebhookRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_DeleteWebhookRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::webhook_api::v1::DeleteWebhookRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::webhook_api::v1::DeleteWebhookRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::webhook_api::v1::DeleteWebhookRequest>* __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_DeleteWebhookRequest_new() {
    return new google::protobuf::Map<std::string, ::webhook_api::v1::DeleteWebhookRequest>();
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_DeleteWebhookRequest_free(const google::protobuf::Map<std::string, ::webhook_api::v1::DeleteWebhookRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_DeleteWebhookRequest_clear(google::protobuf::Map<std::string, ::webhook_api::v1::DeleteWebhookRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_DeleteWebhookRequest_size(const google::protobuf::Map<std::string, ::webhook_api::v1::DeleteWebhookRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_DeleteWebhookRequest_insert(google::protobuf::Map<std::string, ::webhook_api::v1::DeleteWebhookRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::DeleteWebhookRequest value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_DeleteWebhookRequest_get(const google::protobuf::Map<std::string, ::webhook_api::v1::DeleteWebhookRequest>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::webhook_api::v1::DeleteWebhookRequest** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::DeleteWebhookRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_DeleteWebhookRequest_remove(google::protobuf::Map<std::string, ::webhook_api::v1::DeleteWebhookRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::DeleteWebhookRequest * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_DeleteWebhookRequest_iter(
      const google::protobuf::Map<std::string, ::webhook_api::v1::DeleteWebhookRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_DeleteWebhookRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::webhook_api::v1::DeleteWebhookRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::webhook_api::v1::DeleteWebhookRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// webhook_api.v1.DeleteWebhookResponse
        // clang-format off
extern "C" {
void* __rust_proto_thunk__webhook_api_v1_DeleteWebhookResponse_new() { return new ::webhook_api::v1::DeleteWebhookResponse(); }
void __rust_proto_thunk__webhook_api_v1_DeleteWebhookResponse_delete(void* ptr) { delete static_cast<::webhook_api::v1::DeleteWebhookResponse*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__webhook_api_v1_DeleteWebhookResponse_serialize(::webhook_api::v1::DeleteWebhookResponse* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__webhook_api_v1_DeleteWebhookResponse_parse(::webhook_api::v1::DeleteWebhookResponse* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__webhook_api_v1_DeleteWebhookResponse_copy_from(::webhook_api::v1::DeleteWebhookResponse* dst, const ::webhook_api::v1::DeleteWebhookResponse* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__webhook_api_v1_DeleteWebhookResponse_repeated_len(google::protobuf::RepeatedPtrField<::webhook_api::v1::DeleteWebhookResponse>* field) {
  return field->size();
}
const ::webhook_api::v1::DeleteWebhookResponse& __rust_proto_thunk__webhook_api_v1_DeleteWebhookResponse_repeated_get(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::DeleteWebhookResponse>* field,
  size_t index) {
  return field->Get(index);
}
::webhook_api::v1::DeleteWebhookResponse* __rust_proto_thunk__webhook_api_v1_DeleteWebhookResponse_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::DeleteWebhookResponse>* field,
  size_t index) {
  return field->Mutable(index);
}
::webhook_api::v1::DeleteWebhookResponse* __rust_proto_thunk__webhook_api_v1_DeleteWebhookResponse_repeated_add(google::protobuf::RepeatedPtrField<::webhook_api::v1::DeleteWebhookResponse>* field) {
  return field->Add();
}
void __rust_proto_thunk__webhook_api_v1_DeleteWebhookResponse_repeated_clear(google::protobuf::RepeatedPtrField<::webhook_api::v1::DeleteWebhookResponse>* field) {
  field->Clear();
}
void __rust_proto_thunk__webhook_api_v1_DeleteWebhookResponse_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::DeleteWebhookResponse>& dst,
  const google::protobuf::RepeatedPtrField<::webhook_api::v1::DeleteWebhookResponse>& src) {
  dst = src;
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::webhook_api::v1::DeleteWebhookResponse>* __rust_proto_thunk__Map_i32_webhook_api_v1_DeleteWebhookResponse_new() {
    return new google::protobuf::Map<int32_t, ::webhook_api::v1::DeleteWebhookResponse>();
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_DeleteWebhookResponse_free(const google::protobuf::Map<int32_t, ::webhook_api::v1::DeleteWebhookResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_DeleteWebhookResponse_clear(google::protobuf::Map<int32_t, ::webhook_api::v1::DeleteWebhookResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_webhook_api_v1_DeleteWebhookResponse_size(const google::protobuf::Map<int32_t, ::webhook_api::v1::DeleteWebhookResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_DeleteWebhookResponse_insert(google::protobuf::Map<int32_t, ::webhook_api::v1::DeleteWebhookResponse> * m,
                          int32_t key, ::webhook_api::v1::DeleteWebhookResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_DeleteWebhookResponse_get(const google::protobuf::Map<int32_t, ::webhook_api::v1::DeleteWebhookResponse>* m,
                       int32_t key, const ::webhook_api::v1::DeleteWebhookResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::DeleteWebhookResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_DeleteWebhookResponse_remove(google::protobuf::Map<int32_t, ::webhook_api::v1::DeleteWebhookResponse> * m,
                          int32_t key, ::webhook_api::v1::DeleteWebhookResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_webhook_api_v1_DeleteWebhookResponse_iter(
      const google::protobuf::Map<int32_t, ::webhook_api::v1::DeleteWebhookResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_DeleteWebhookResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::webhook_api::v1::DeleteWebhookResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::webhook_api::v1::DeleteWebhookResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::webhook_api::v1::DeleteWebhookResponse>* __rust_proto_thunk__Map_u32_webhook_api_v1_DeleteWebhookResponse_new() {
    return new google::protobuf::Map<uint32_t, ::webhook_api::v1::DeleteWebhookResponse>();
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_DeleteWebhookResponse_free(const google::protobuf::Map<uint32_t, ::webhook_api::v1::DeleteWebhookResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_DeleteWebhookResponse_clear(google::protobuf::Map<uint32_t, ::webhook_api::v1::DeleteWebhookResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_webhook_api_v1_DeleteWebhookResponse_size(const google::protobuf::Map<uint32_t, ::webhook_api::v1::DeleteWebhookResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_DeleteWebhookResponse_insert(google::protobuf::Map<uint32_t, ::webhook_api::v1::DeleteWebhookResponse> * m,
                          uint32_t key, ::webhook_api::v1::DeleteWebhookResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_DeleteWebhookResponse_get(const google::protobuf::Map<uint32_t, ::webhook_api::v1::DeleteWebhookResponse>* m,
                       uint32_t key, const ::webhook_api::v1::DeleteWebhookResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::DeleteWebhookResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_DeleteWebhookResponse_remove(google::protobuf::Map<uint32_t, ::webhook_api::v1::DeleteWebhookResponse> * m,
                          uint32_t key, ::webhook_api::v1::DeleteWebhookResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_webhook_api_v1_DeleteWebhookResponse_iter(
      const google::protobuf::Map<uint32_t, ::webhook_api::v1::DeleteWebhookResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_DeleteWebhookResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::webhook_api::v1::DeleteWebhookResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::webhook_api::v1::DeleteWebhookResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::webhook_api::v1::DeleteWebhookResponse>* __rust_proto_thunk__Map_i64_webhook_api_v1_DeleteWebhookResponse_new() {
    return new google::protobuf::Map<int64_t, ::webhook_api::v1::DeleteWebhookResponse>();
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_DeleteWebhookResponse_free(const google::protobuf::Map<int64_t, ::webhook_api::v1::DeleteWebhookResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_DeleteWebhookResponse_clear(google::protobuf::Map<int64_t, ::webhook_api::v1::DeleteWebhookResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_webhook_api_v1_DeleteWebhookResponse_size(const google::protobuf::Map<int64_t, ::webhook_api::v1::DeleteWebhookResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_DeleteWebhookResponse_insert(google::protobuf::Map<int64_t, ::webhook_api::v1::DeleteWebhookResponse> * m,
                          int64_t key, ::webhook_api::v1::DeleteWebhookResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_DeleteWebhookResponse_get(const google::protobuf::Map<int64_t, ::webhook_api::v1::DeleteWebhookResponse>* m,
                       int64_t key, const ::webhook_api::v1::DeleteWebhookResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::DeleteWebhookResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_DeleteWebhookResponse_remove(google::protobuf::Map<int64_t, ::webhook_api::v1::DeleteWebhookResponse> * m,
                          int64_t key, ::webhook_api::v1::DeleteWebhookResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_webhook_api_v1_DeleteWebhookResponse_iter(
      const google::protobuf::Map<int64_t, ::webhook_api::v1::DeleteWebhookResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_DeleteWebhookResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::webhook_api::v1::DeleteWebhookResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::webhook_api::v1::DeleteWebhookResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::webhook_api::v1::DeleteWebhookResponse>* __rust_proto_thunk__Map_u64_webhook_api_v1_DeleteWebhookResponse_new() {
    return new google::protobuf::Map<uint64_t, ::webhook_api::v1::DeleteWebhookResponse>();
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_DeleteWebhookResponse_free(const google::protobuf::Map<uint64_t, ::webhook_api::v1::DeleteWebhookResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_DeleteWebhookResponse_clear(google::protobuf::Map<uint64_t, ::webhook_api::v1::DeleteWebhookResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_webhook_api_v1_DeleteWebhookResponse_size(const google::protobuf::Map<uint64_t, ::webhook_api::v1::DeleteWebhookResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_DeleteWebhookResponse_insert(google::protobuf::Map<uint64_t, ::webhook_api::v1::DeleteWebhookResponse> * m,
                          uint64_t key, ::webhook_api::v1::DeleteWebhookResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_DeleteWebhookResponse_get(const google::protobuf::Map<uint64_t, ::webhook_api::v1::DeleteWebhookResponse>* m,
                       uint64_t key, const ::webhook_api::v1::DeleteWebhookResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::DeleteWebhookResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_DeleteWebhookResponse_remove(google::protobuf::Map<uint64_t, ::webhook_api::v1::DeleteWebhookResponse> * m,
                          uint64_t key, ::webhook_api::v1::DeleteWebhookResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_webhook_api_v1_DeleteWebhookResponse_iter(
      const google::protobuf::Map<uint64_t, ::webhook_api::v1::DeleteWebhookResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_DeleteWebhookResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::webhook_api::v1::DeleteWebhookResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::webhook_api::v1::DeleteWebhookResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::webhook_api::v1::DeleteWebhookResponse>* __rust_proto_thunk__Map_bool_webhook_api_v1_DeleteWebhookResponse_new() {
    return new google::protobuf::Map<bool, ::webhook_api::v1::DeleteWebhookResponse>();
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_DeleteWebhookResponse_free(const google::protobuf::Map<bool, ::webhook_api::v1::DeleteWebhookResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_DeleteWebhookResponse_clear(google::protobuf::Map<bool, ::webhook_api::v1::DeleteWebhookResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_webhook_api_v1_DeleteWebhookResponse_size(const google::protobuf::Map<bool, ::webhook_api::v1::DeleteWebhookResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_DeleteWebhookResponse_insert(google::protobuf::Map<bool, ::webhook_api::v1::DeleteWebhookResponse> * m,
                          bool key, ::webhook_api::v1::DeleteWebhookResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_DeleteWebhookResponse_get(const google::protobuf::Map<bool, ::webhook_api::v1::DeleteWebhookResponse>* m,
                       bool key, const ::webhook_api::v1::DeleteWebhookResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::DeleteWebhookResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_DeleteWebhookResponse_remove(google::protobuf::Map<bool, ::webhook_api::v1::DeleteWebhookResponse> * m,
                          bool key, ::webhook_api::v1::DeleteWebhookResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_webhook_api_v1_DeleteWebhookResponse_iter(
      const google::protobuf::Map<bool, ::webhook_api::v1::DeleteWebhookResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_DeleteWebhookResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::webhook_api::v1::DeleteWebhookResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::webhook_api::v1::DeleteWebhookResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::webhook_api::v1::DeleteWebhookResponse>* __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_DeleteWebhookResponse_new() {
    return new google::protobuf::Map<std::string, ::webhook_api::v1::DeleteWebhookResponse>();
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_DeleteWebhookResponse_free(const google::protobuf::Map<std::string, ::webhook_api::v1::DeleteWebhookResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_DeleteWebhookResponse_clear(google::protobuf::Map<std::string, ::webhook_api::v1::DeleteWebhookResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_DeleteWebhookResponse_size(const google::protobuf::Map<std::string, ::webhook_api::v1::DeleteWebhookResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_DeleteWebhookResponse_insert(google::protobuf::Map<std::string, ::webhook_api::v1::DeleteWebhookResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::DeleteWebhookResponse value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_DeleteWebhookResponse_get(const google::protobuf::Map<std::string, ::webhook_api::v1::DeleteWebhookResponse>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::webhook_api::v1::DeleteWebhookResponse** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::DeleteWebhookResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_DeleteWebhookResponse_remove(google::protobuf::Map<std::string, ::webhook_api::v1::DeleteWebhookResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::DeleteWebhookResponse * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_DeleteWebhookResponse_iter(
      const google::protobuf::Map<std::string, ::webhook_api::v1::DeleteWebhookResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_DeleteWebhookResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::webhook_api::v1::DeleteWebhookResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::webhook_api::v1::DeleteWebhookResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// webhook_api.v1.EventRequest
        // clang-format off
extern "C" {
void* __rust_proto_thunk__webhook_api_v1_EventRequest_new() { return new ::webhook_api::v1::EventRequest(); }
void __rust_proto_thunk__webhook_api_v1_EventRequest_delete(void* ptr) { delete static_cast<::webhook_api::v1::EventRequest*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__webhook_api_v1_EventRequest_serialize(::webhook_api::v1::EventRequest* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__webhook_api_v1_EventRequest_parse(::webhook_api::v1::EventRequest* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__webhook_api_v1_EventRequest_copy_from(::webhook_api::v1::EventRequest* dst, const ::webhook_api::v1::EventRequest* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__webhook_api_v1_EventRequest_repeated_len(google::protobuf::RepeatedPtrField<::webhook_api::v1::EventRequest>* field) {
  return field->size();
}
const ::webhook_api::v1::EventRequest& __rust_proto_thunk__webhook_api_v1_EventRequest_repeated_get(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::EventRequest>* field,
  size_t index) {
  return field->Get(index);
}
::webhook_api::v1::EventRequest* __rust_proto_thunk__webhook_api_v1_EventRequest_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::EventRequest>* field,
  size_t index) {
  return field->Mutable(index);
}
::webhook_api::v1::EventRequest* __rust_proto_thunk__webhook_api_v1_EventRequest_repeated_add(google::protobuf::RepeatedPtrField<::webhook_api::v1::EventRequest>* field) {
  return field->Add();
}
void __rust_proto_thunk__webhook_api_v1_EventRequest_repeated_clear(google::protobuf::RepeatedPtrField<::webhook_api::v1::EventRequest>* field) {
  field->Clear();
}
void __rust_proto_thunk__webhook_api_v1_EventRequest_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::EventRequest>& dst,
  const google::protobuf::RepeatedPtrField<::webhook_api::v1::EventRequest>& src) {
  dst = src;
}

const void* __rust_proto_thunk__webhook_api_v1_EventRequest_get_event(::webhook_api::v1::EventRequest* msg) {
  return static_cast<const void*>(&msg->event());
}
void* __rust_proto_thunk__webhook_api_v1_EventRequest_get_mut_event(::webhook_api::v1::EventRequest* msg) {
  return static_cast<void*>(msg->mutable_event());
}
void __rust_proto_thunk__webhook_api_v1_EventRequest_clear_event(::webhook_api::v1::EventRequest* msg) { msg->clear_event(); }
bool __rust_proto_thunk__webhook_api_v1_EventRequest_has_event(::webhook_api::v1::EventRequest* msg) { return msg->has_event(); }


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::webhook_api::v1::EventRequest>* __rust_proto_thunk__Map_i32_webhook_api_v1_EventRequest_new() {
    return new google::protobuf::Map<int32_t, ::webhook_api::v1::EventRequest>();
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_EventRequest_free(const google::protobuf::Map<int32_t, ::webhook_api::v1::EventRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_EventRequest_clear(google::protobuf::Map<int32_t, ::webhook_api::v1::EventRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_webhook_api_v1_EventRequest_size(const google::protobuf::Map<int32_t, ::webhook_api::v1::EventRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_EventRequest_insert(google::protobuf::Map<int32_t, ::webhook_api::v1::EventRequest> * m,
                          int32_t key, ::webhook_api::v1::EventRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_EventRequest_get(const google::protobuf::Map<int32_t, ::webhook_api::v1::EventRequest>* m,
                       int32_t key, const ::webhook_api::v1::EventRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::EventRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_EventRequest_remove(google::protobuf::Map<int32_t, ::webhook_api::v1::EventRequest> * m,
                          int32_t key, ::webhook_api::v1::EventRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_webhook_api_v1_EventRequest_iter(
      const google::protobuf::Map<int32_t, ::webhook_api::v1::EventRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_EventRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::webhook_api::v1::EventRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::webhook_api::v1::EventRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::webhook_api::v1::EventRequest>* __rust_proto_thunk__Map_u32_webhook_api_v1_EventRequest_new() {
    return new google::protobuf::Map<uint32_t, ::webhook_api::v1::EventRequest>();
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_EventRequest_free(const google::protobuf::Map<uint32_t, ::webhook_api::v1::EventRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_EventRequest_clear(google::protobuf::Map<uint32_t, ::webhook_api::v1::EventRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_webhook_api_v1_EventRequest_size(const google::protobuf::Map<uint32_t, ::webhook_api::v1::EventRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_EventRequest_insert(google::protobuf::Map<uint32_t, ::webhook_api::v1::EventRequest> * m,
                          uint32_t key, ::webhook_api::v1::EventRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_EventRequest_get(const google::protobuf::Map<uint32_t, ::webhook_api::v1::EventRequest>* m,
                       uint32_t key, const ::webhook_api::v1::EventRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::EventRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_EventRequest_remove(google::protobuf::Map<uint32_t, ::webhook_api::v1::EventRequest> * m,
                          uint32_t key, ::webhook_api::v1::EventRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_webhook_api_v1_EventRequest_iter(
      const google::protobuf::Map<uint32_t, ::webhook_api::v1::EventRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_EventRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::webhook_api::v1::EventRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::webhook_api::v1::EventRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::webhook_api::v1::EventRequest>* __rust_proto_thunk__Map_i64_webhook_api_v1_EventRequest_new() {
    return new google::protobuf::Map<int64_t, ::webhook_api::v1::EventRequest>();
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_EventRequest_free(const google::protobuf::Map<int64_t, ::webhook_api::v1::EventRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_EventRequest_clear(google::protobuf::Map<int64_t, ::webhook_api::v1::EventRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_webhook_api_v1_EventRequest_size(const google::protobuf::Map<int64_t, ::webhook_api::v1::EventRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_EventRequest_insert(google::protobuf::Map<int64_t, ::webhook_api::v1::EventRequest> * m,
                          int64_t key, ::webhook_api::v1::EventRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_EventRequest_get(const google::protobuf::Map<int64_t, ::webhook_api::v1::EventRequest>* m,
                       int64_t key, const ::webhook_api::v1::EventRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::EventRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_EventRequest_remove(google::protobuf::Map<int64_t, ::webhook_api::v1::EventRequest> * m,
                          int64_t key, ::webhook_api::v1::EventRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_webhook_api_v1_EventRequest_iter(
      const google::protobuf::Map<int64_t, ::webhook_api::v1::EventRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_EventRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::webhook_api::v1::EventRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::webhook_api::v1::EventRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::webhook_api::v1::EventRequest>* __rust_proto_thunk__Map_u64_webhook_api_v1_EventRequest_new() {
    return new google::protobuf::Map<uint64_t, ::webhook_api::v1::EventRequest>();
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_EventRequest_free(const google::protobuf::Map<uint64_t, ::webhook_api::v1::EventRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_EventRequest_clear(google::protobuf::Map<uint64_t, ::webhook_api::v1::EventRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_webhook_api_v1_EventRequest_size(const google::protobuf::Map<uint64_t, ::webhook_api::v1::EventRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_EventRequest_insert(google::protobuf::Map<uint64_t, ::webhook_api::v1::EventRequest> * m,
                          uint64_t key, ::webhook_api::v1::EventRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_EventRequest_get(const google::protobuf::Map<uint64_t, ::webhook_api::v1::EventRequest>* m,
                       uint64_t key, const ::webhook_api::v1::EventRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::EventRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_EventRequest_remove(google::protobuf::Map<uint64_t, ::webhook_api::v1::EventRequest> * m,
                          uint64_t key, ::webhook_api::v1::EventRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_webhook_api_v1_EventRequest_iter(
      const google::protobuf::Map<uint64_t, ::webhook_api::v1::EventRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_EventRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::webhook_api::v1::EventRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::webhook_api::v1::EventRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::webhook_api::v1::EventRequest>* __rust_proto_thunk__Map_bool_webhook_api_v1_EventRequest_new() {
    return new google::protobuf::Map<bool, ::webhook_api::v1::EventRequest>();
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_EventRequest_free(const google::protobuf::Map<bool, ::webhook_api::v1::EventRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_EventRequest_clear(google::protobuf::Map<bool, ::webhook_api::v1::EventRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_webhook_api_v1_EventRequest_size(const google::protobuf::Map<bool, ::webhook_api::v1::EventRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_EventRequest_insert(google::protobuf::Map<bool, ::webhook_api::v1::EventRequest> * m,
                          bool key, ::webhook_api::v1::EventRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_EventRequest_get(const google::protobuf::Map<bool, ::webhook_api::v1::EventRequest>* m,
                       bool key, const ::webhook_api::v1::EventRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::EventRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_EventRequest_remove(google::protobuf::Map<bool, ::webhook_api::v1::EventRequest> * m,
                          bool key, ::webhook_api::v1::EventRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_webhook_api_v1_EventRequest_iter(
      const google::protobuf::Map<bool, ::webhook_api::v1::EventRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_EventRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::webhook_api::v1::EventRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::webhook_api::v1::EventRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::webhook_api::v1::EventRequest>* __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_EventRequest_new() {
    return new google::protobuf::Map<std::string, ::webhook_api::v1::EventRequest>();
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_EventRequest_free(const google::protobuf::Map<std::string, ::webhook_api::v1::EventRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_EventRequest_clear(google::protobuf::Map<std::string, ::webhook_api::v1::EventRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_EventRequest_size(const google::protobuf::Map<std::string, ::webhook_api::v1::EventRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_EventRequest_insert(google::protobuf::Map<std::string, ::webhook_api::v1::EventRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::EventRequest value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_EventRequest_get(const google::protobuf::Map<std::string, ::webhook_api::v1::EventRequest>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::webhook_api::v1::EventRequest** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::EventRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_EventRequest_remove(google::protobuf::Map<std::string, ::webhook_api::v1::EventRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::EventRequest * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_EventRequest_iter(
      const google::protobuf::Map<std::string, ::webhook_api::v1::EventRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_EventRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::webhook_api::v1::EventRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::webhook_api::v1::EventRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// webhook_api.v1.EventResponse
        // clang-format off
extern "C" {
void* __rust_proto_thunk__webhook_api_v1_EventResponse_new() { return new ::webhook_api::v1::EventResponse(); }
void __rust_proto_thunk__webhook_api_v1_EventResponse_delete(void* ptr) { delete static_cast<::webhook_api::v1::EventResponse*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__webhook_api_v1_EventResponse_serialize(::webhook_api::v1::EventResponse* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__webhook_api_v1_EventResponse_parse(::webhook_api::v1::EventResponse* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__webhook_api_v1_EventResponse_copy_from(::webhook_api::v1::EventResponse* dst, const ::webhook_api::v1::EventResponse* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__webhook_api_v1_EventResponse_repeated_len(google::protobuf::RepeatedPtrField<::webhook_api::v1::EventResponse>* field) {
  return field->size();
}
const ::webhook_api::v1::EventResponse& __rust_proto_thunk__webhook_api_v1_EventResponse_repeated_get(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::EventResponse>* field,
  size_t index) {
  return field->Get(index);
}
::webhook_api::v1::EventResponse* __rust_proto_thunk__webhook_api_v1_EventResponse_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::EventResponse>* field,
  size_t index) {
  return field->Mutable(index);
}
::webhook_api::v1::EventResponse* __rust_proto_thunk__webhook_api_v1_EventResponse_repeated_add(google::protobuf::RepeatedPtrField<::webhook_api::v1::EventResponse>* field) {
  return field->Add();
}
void __rust_proto_thunk__webhook_api_v1_EventResponse_repeated_clear(google::protobuf::RepeatedPtrField<::webhook_api::v1::EventResponse>* field) {
  field->Clear();
}
void __rust_proto_thunk__webhook_api_v1_EventResponse_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::EventResponse>& dst,
  const google::protobuf::RepeatedPtrField<::webhook_api::v1::EventResponse>& src) {
  dst = src;
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::webhook_api::v1::EventResponse>* __rust_proto_thunk__Map_i32_webhook_api_v1_EventResponse_new() {
    return new google::protobuf::Map<int32_t, ::webhook_api::v1::EventResponse>();
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_EventResponse_free(const google::protobuf::Map<int32_t, ::webhook_api::v1::EventResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_EventResponse_clear(google::protobuf::Map<int32_t, ::webhook_api::v1::EventResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_webhook_api_v1_EventResponse_size(const google::protobuf::Map<int32_t, ::webhook_api::v1::EventResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_EventResponse_insert(google::protobuf::Map<int32_t, ::webhook_api::v1::EventResponse> * m,
                          int32_t key, ::webhook_api::v1::EventResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_EventResponse_get(const google::protobuf::Map<int32_t, ::webhook_api::v1::EventResponse>* m,
                       int32_t key, const ::webhook_api::v1::EventResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::EventResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_EventResponse_remove(google::protobuf::Map<int32_t, ::webhook_api::v1::EventResponse> * m,
                          int32_t key, ::webhook_api::v1::EventResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_webhook_api_v1_EventResponse_iter(
      const google::protobuf::Map<int32_t, ::webhook_api::v1::EventResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_EventResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::webhook_api::v1::EventResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::webhook_api::v1::EventResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::webhook_api::v1::EventResponse>* __rust_proto_thunk__Map_u32_webhook_api_v1_EventResponse_new() {
    return new google::protobuf::Map<uint32_t, ::webhook_api::v1::EventResponse>();
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_EventResponse_free(const google::protobuf::Map<uint32_t, ::webhook_api::v1::EventResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_EventResponse_clear(google::protobuf::Map<uint32_t, ::webhook_api::v1::EventResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_webhook_api_v1_EventResponse_size(const google::protobuf::Map<uint32_t, ::webhook_api::v1::EventResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_EventResponse_insert(google::protobuf::Map<uint32_t, ::webhook_api::v1::EventResponse> * m,
                          uint32_t key, ::webhook_api::v1::EventResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_EventResponse_get(const google::protobuf::Map<uint32_t, ::webhook_api::v1::EventResponse>* m,
                       uint32_t key, const ::webhook_api::v1::EventResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::EventResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_EventResponse_remove(google::protobuf::Map<uint32_t, ::webhook_api::v1::EventResponse> * m,
                          uint32_t key, ::webhook_api::v1::EventResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_webhook_api_v1_EventResponse_iter(
      const google::protobuf::Map<uint32_t, ::webhook_api::v1::EventResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_EventResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::webhook_api::v1::EventResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::webhook_api::v1::EventResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::webhook_api::v1::EventResponse>* __rust_proto_thunk__Map_i64_webhook_api_v1_EventResponse_new() {
    return new google::protobuf::Map<int64_t, ::webhook_api::v1::EventResponse>();
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_EventResponse_free(const google::protobuf::Map<int64_t, ::webhook_api::v1::EventResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_EventResponse_clear(google::protobuf::Map<int64_t, ::webhook_api::v1::EventResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_webhook_api_v1_EventResponse_size(const google::protobuf::Map<int64_t, ::webhook_api::v1::EventResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_EventResponse_insert(google::protobuf::Map<int64_t, ::webhook_api::v1::EventResponse> * m,
                          int64_t key, ::webhook_api::v1::EventResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_EventResponse_get(const google::protobuf::Map<int64_t, ::webhook_api::v1::EventResponse>* m,
                       int64_t key, const ::webhook_api::v1::EventResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::EventResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_EventResponse_remove(google::protobuf::Map<int64_t, ::webhook_api::v1::EventResponse> * m,
                          int64_t key, ::webhook_api::v1::EventResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_webhook_api_v1_EventResponse_iter(
      const google::protobuf::Map<int64_t, ::webhook_api::v1::EventResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_EventResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::webhook_api::v1::EventResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::webhook_api::v1::EventResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::webhook_api::v1::EventResponse>* __rust_proto_thunk__Map_u64_webhook_api_v1_EventResponse_new() {
    return new google::protobuf::Map<uint64_t, ::webhook_api::v1::EventResponse>();
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_EventResponse_free(const google::protobuf::Map<uint64_t, ::webhook_api::v1::EventResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_EventResponse_clear(google::protobuf::Map<uint64_t, ::webhook_api::v1::EventResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_webhook_api_v1_EventResponse_size(const google::protobuf::Map<uint64_t, ::webhook_api::v1::EventResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_EventResponse_insert(google::protobuf::Map<uint64_t, ::webhook_api::v1::EventResponse> * m,
                          uint64_t key, ::webhook_api::v1::EventResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_EventResponse_get(const google::protobuf::Map<uint64_t, ::webhook_api::v1::EventResponse>* m,
                       uint64_t key, const ::webhook_api::v1::EventResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::EventResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_EventResponse_remove(google::protobuf::Map<uint64_t, ::webhook_api::v1::EventResponse> * m,
                          uint64_t key, ::webhook_api::v1::EventResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_webhook_api_v1_EventResponse_iter(
      const google::protobuf::Map<uint64_t, ::webhook_api::v1::EventResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_EventResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::webhook_api::v1::EventResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::webhook_api::v1::EventResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::webhook_api::v1::EventResponse>* __rust_proto_thunk__Map_bool_webhook_api_v1_EventResponse_new() {
    return new google::protobuf::Map<bool, ::webhook_api::v1::EventResponse>();
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_EventResponse_free(const google::protobuf::Map<bool, ::webhook_api::v1::EventResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_EventResponse_clear(google::protobuf::Map<bool, ::webhook_api::v1::EventResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_webhook_api_v1_EventResponse_size(const google::protobuf::Map<bool, ::webhook_api::v1::EventResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_EventResponse_insert(google::protobuf::Map<bool, ::webhook_api::v1::EventResponse> * m,
                          bool key, ::webhook_api::v1::EventResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_EventResponse_get(const google::protobuf::Map<bool, ::webhook_api::v1::EventResponse>* m,
                       bool key, const ::webhook_api::v1::EventResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::EventResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_EventResponse_remove(google::protobuf::Map<bool, ::webhook_api::v1::EventResponse> * m,
                          bool key, ::webhook_api::v1::EventResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_webhook_api_v1_EventResponse_iter(
      const google::protobuf::Map<bool, ::webhook_api::v1::EventResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_EventResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::webhook_api::v1::EventResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::webhook_api::v1::EventResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::webhook_api::v1::EventResponse>* __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_EventResponse_new() {
    return new google::protobuf::Map<std::string, ::webhook_api::v1::EventResponse>();
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_EventResponse_free(const google::protobuf::Map<std::string, ::webhook_api::v1::EventResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_EventResponse_clear(google::protobuf::Map<std::string, ::webhook_api::v1::EventResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_EventResponse_size(const google::protobuf::Map<std::string, ::webhook_api::v1::EventResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_EventResponse_insert(google::protobuf::Map<std::string, ::webhook_api::v1::EventResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::EventResponse value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_EventResponse_get(const google::protobuf::Map<std::string, ::webhook_api::v1::EventResponse>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::webhook_api::v1::EventResponse** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::EventResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_EventResponse_remove(google::protobuf::Map<std::string, ::webhook_api::v1::EventResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::EventResponse * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_EventResponse_iter(
      const google::protobuf::Map<std::string, ::webhook_api::v1::EventResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_EventResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::webhook_api::v1::EventResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::webhook_api::v1::EventResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// webhook_api.v1.ListEventsRequest
        // clang-format off
extern "C" {
void* __rust_proto_thunk__webhook_api_v1_ListEventsRequest_new() { return new ::webhook_api::v1::ListEventsRequest(); }
void __rust_proto_thunk__webhook_api_v1_ListEventsRequest_delete(void* ptr) { delete static_cast<::webhook_api::v1::ListEventsRequest*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__webhook_api_v1_ListEventsRequest_serialize(::webhook_api::v1::ListEventsRequest* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__webhook_api_v1_ListEventsRequest_parse(::webhook_api::v1::ListEventsRequest* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__webhook_api_v1_ListEventsRequest_copy_from(::webhook_api::v1::ListEventsRequest* dst, const ::webhook_api::v1::ListEventsRequest* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__webhook_api_v1_ListEventsRequest_repeated_len(google::protobuf::RepeatedPtrField<::webhook_api::v1::ListEventsRequest>* field) {
  return field->size();
}
const ::webhook_api::v1::ListEventsRequest& __rust_proto_thunk__webhook_api_v1_ListEventsRequest_repeated_get(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::ListEventsRequest>* field,
  size_t index) {
  return field->Get(index);
}
::webhook_api::v1::ListEventsRequest* __rust_proto_thunk__webhook_api_v1_ListEventsRequest_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::ListEventsRequest>* field,
  size_t index) {
  return field->Mutable(index);
}
::webhook_api::v1::ListEventsRequest* __rust_proto_thunk__webhook_api_v1_ListEventsRequest_repeated_add(google::protobuf::RepeatedPtrField<::webhook_api::v1::ListEventsRequest>* field) {
  return field->Add();
}
void __rust_proto_thunk__webhook_api_v1_ListEventsRequest_repeated_clear(google::protobuf::RepeatedPtrField<::webhook_api::v1::ListEventsRequest>* field) {
  field->Clear();
}
void __rust_proto_thunk__webhook_api_v1_ListEventsRequest_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::ListEventsRequest>& dst,
  const google::protobuf::RepeatedPtrField<::webhook_api::v1::ListEventsRequest>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__webhook_api_v1_ListEventsRequest_get_api_key(::webhook_api::v1::ListEventsRequest* msg) {
  absl::string_view val = msg->api_key();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__webhook_api_v1_ListEventsRequest_set_api_key(::webhook_api::v1::ListEventsRequest* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_api_key(absl::string_view(s.ptr, s.len));
}

::int32_t __rust_proto_thunk__webhook_api_v1_ListEventsRequest_get_page(::webhook_api::v1::ListEventsRequest* msg) { return msg->page(); }
void __rust_proto_thunk__webhook_api_v1_ListEventsRequest_set_page(::webhook_api::v1::ListEventsRequest* msg, ::int32_t val) {
  msg->set_page(val);
}

::int32_t __rust_proto_thunk__webhook_api_v1_ListEventsRequest_get_page_size(::webhook_api::v1::ListEventsRequest* msg) { return msg->page_size(); }
void __rust_proto_thunk__webhook_api_v1_ListEventsRequest_set_page_size(::webhook_api::v1::ListEventsRequest* msg, ::int32_t val) {
  msg->set_page_size(val);
}

const void* __rust_proto_thunk__webhook_api_v1_ListEventsRequest_get_filters(::webhook_api::v1::ListEventsRequest* msg) {
  return static_cast<const void*>(&msg->filters());
}
void* __rust_proto_thunk__webhook_api_v1_ListEventsRequest_get_mut_filters(::webhook_api::v1::ListEventsRequest* msg) {
  return static_cast<void*>(msg->mutable_filters());
}
void __rust_proto_thunk__webhook_api_v1_ListEventsRequest_clear_filters(::webhook_api::v1::ListEventsRequest* msg) { msg->clear_filters(); }
bool __rust_proto_thunk__webhook_api_v1_ListEventsRequest_has_filters(::webhook_api::v1::ListEventsRequest* msg) { return msg->has_filters(); }

const void* __rust_proto_thunk__webhook_api_v1_ListEventsRequest_get_sort_by(::webhook_api::v1::ListEventsRequest* msg) {
  return static_cast<const void*>(&msg->sort_by());
}
void* __rust_proto_thunk__webhook_api_v1_ListEventsRequest_get_mut_sort_by(::webhook_api::v1::ListEventsRequest* msg) {
  return static_cast<void*>(msg->mutable_sort_by());
}
void __rust_proto_thunk__webhook_api_v1_ListEventsRequest_clear_sort_by(::webhook_api::v1::ListEventsRequest* msg) { msg->clear_sort_by(); }
bool __rust_proto_thunk__webhook_api_v1_ListEventsRequest_has_sort_by(::webhook_api::v1::ListEventsRequest* msg) { return msg->has_sort_by(); }


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::webhook_api::v1::ListEventsRequest>* __rust_proto_thunk__Map_i32_webhook_api_v1_ListEventsRequest_new() {
    return new google::protobuf::Map<int32_t, ::webhook_api::v1::ListEventsRequest>();
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_ListEventsRequest_free(const google::protobuf::Map<int32_t, ::webhook_api::v1::ListEventsRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_ListEventsRequest_clear(google::protobuf::Map<int32_t, ::webhook_api::v1::ListEventsRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_webhook_api_v1_ListEventsRequest_size(const google::protobuf::Map<int32_t, ::webhook_api::v1::ListEventsRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_ListEventsRequest_insert(google::protobuf::Map<int32_t, ::webhook_api::v1::ListEventsRequest> * m,
                          int32_t key, ::webhook_api::v1::ListEventsRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_ListEventsRequest_get(const google::protobuf::Map<int32_t, ::webhook_api::v1::ListEventsRequest>* m,
                       int32_t key, const ::webhook_api::v1::ListEventsRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ListEventsRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_ListEventsRequest_remove(google::protobuf::Map<int32_t, ::webhook_api::v1::ListEventsRequest> * m,
                          int32_t key, ::webhook_api::v1::ListEventsRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_webhook_api_v1_ListEventsRequest_iter(
      const google::protobuf::Map<int32_t, ::webhook_api::v1::ListEventsRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_ListEventsRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::webhook_api::v1::ListEventsRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::webhook_api::v1::ListEventsRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::webhook_api::v1::ListEventsRequest>* __rust_proto_thunk__Map_u32_webhook_api_v1_ListEventsRequest_new() {
    return new google::protobuf::Map<uint32_t, ::webhook_api::v1::ListEventsRequest>();
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_ListEventsRequest_free(const google::protobuf::Map<uint32_t, ::webhook_api::v1::ListEventsRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_ListEventsRequest_clear(google::protobuf::Map<uint32_t, ::webhook_api::v1::ListEventsRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_webhook_api_v1_ListEventsRequest_size(const google::protobuf::Map<uint32_t, ::webhook_api::v1::ListEventsRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_ListEventsRequest_insert(google::protobuf::Map<uint32_t, ::webhook_api::v1::ListEventsRequest> * m,
                          uint32_t key, ::webhook_api::v1::ListEventsRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_ListEventsRequest_get(const google::protobuf::Map<uint32_t, ::webhook_api::v1::ListEventsRequest>* m,
                       uint32_t key, const ::webhook_api::v1::ListEventsRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ListEventsRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_ListEventsRequest_remove(google::protobuf::Map<uint32_t, ::webhook_api::v1::ListEventsRequest> * m,
                          uint32_t key, ::webhook_api::v1::ListEventsRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_webhook_api_v1_ListEventsRequest_iter(
      const google::protobuf::Map<uint32_t, ::webhook_api::v1::ListEventsRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_ListEventsRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::webhook_api::v1::ListEventsRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::webhook_api::v1::ListEventsRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::webhook_api::v1::ListEventsRequest>* __rust_proto_thunk__Map_i64_webhook_api_v1_ListEventsRequest_new() {
    return new google::protobuf::Map<int64_t, ::webhook_api::v1::ListEventsRequest>();
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_ListEventsRequest_free(const google::protobuf::Map<int64_t, ::webhook_api::v1::ListEventsRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_ListEventsRequest_clear(google::protobuf::Map<int64_t, ::webhook_api::v1::ListEventsRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_webhook_api_v1_ListEventsRequest_size(const google::protobuf::Map<int64_t, ::webhook_api::v1::ListEventsRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_ListEventsRequest_insert(google::protobuf::Map<int64_t, ::webhook_api::v1::ListEventsRequest> * m,
                          int64_t key, ::webhook_api::v1::ListEventsRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_ListEventsRequest_get(const google::protobuf::Map<int64_t, ::webhook_api::v1::ListEventsRequest>* m,
                       int64_t key, const ::webhook_api::v1::ListEventsRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ListEventsRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_ListEventsRequest_remove(google::protobuf::Map<int64_t, ::webhook_api::v1::ListEventsRequest> * m,
                          int64_t key, ::webhook_api::v1::ListEventsRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_webhook_api_v1_ListEventsRequest_iter(
      const google::protobuf::Map<int64_t, ::webhook_api::v1::ListEventsRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_ListEventsRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::webhook_api::v1::ListEventsRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::webhook_api::v1::ListEventsRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::webhook_api::v1::ListEventsRequest>* __rust_proto_thunk__Map_u64_webhook_api_v1_ListEventsRequest_new() {
    return new google::protobuf::Map<uint64_t, ::webhook_api::v1::ListEventsRequest>();
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_ListEventsRequest_free(const google::protobuf::Map<uint64_t, ::webhook_api::v1::ListEventsRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_ListEventsRequest_clear(google::protobuf::Map<uint64_t, ::webhook_api::v1::ListEventsRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_webhook_api_v1_ListEventsRequest_size(const google::protobuf::Map<uint64_t, ::webhook_api::v1::ListEventsRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_ListEventsRequest_insert(google::protobuf::Map<uint64_t, ::webhook_api::v1::ListEventsRequest> * m,
                          uint64_t key, ::webhook_api::v1::ListEventsRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_ListEventsRequest_get(const google::protobuf::Map<uint64_t, ::webhook_api::v1::ListEventsRequest>* m,
                       uint64_t key, const ::webhook_api::v1::ListEventsRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ListEventsRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_ListEventsRequest_remove(google::protobuf::Map<uint64_t, ::webhook_api::v1::ListEventsRequest> * m,
                          uint64_t key, ::webhook_api::v1::ListEventsRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_webhook_api_v1_ListEventsRequest_iter(
      const google::protobuf::Map<uint64_t, ::webhook_api::v1::ListEventsRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_ListEventsRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::webhook_api::v1::ListEventsRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::webhook_api::v1::ListEventsRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::webhook_api::v1::ListEventsRequest>* __rust_proto_thunk__Map_bool_webhook_api_v1_ListEventsRequest_new() {
    return new google::protobuf::Map<bool, ::webhook_api::v1::ListEventsRequest>();
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_ListEventsRequest_free(const google::protobuf::Map<bool, ::webhook_api::v1::ListEventsRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_ListEventsRequest_clear(google::protobuf::Map<bool, ::webhook_api::v1::ListEventsRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_webhook_api_v1_ListEventsRequest_size(const google::protobuf::Map<bool, ::webhook_api::v1::ListEventsRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_ListEventsRequest_insert(google::protobuf::Map<bool, ::webhook_api::v1::ListEventsRequest> * m,
                          bool key, ::webhook_api::v1::ListEventsRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_ListEventsRequest_get(const google::protobuf::Map<bool, ::webhook_api::v1::ListEventsRequest>* m,
                       bool key, const ::webhook_api::v1::ListEventsRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ListEventsRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_ListEventsRequest_remove(google::protobuf::Map<bool, ::webhook_api::v1::ListEventsRequest> * m,
                          bool key, ::webhook_api::v1::ListEventsRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_webhook_api_v1_ListEventsRequest_iter(
      const google::protobuf::Map<bool, ::webhook_api::v1::ListEventsRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_ListEventsRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::webhook_api::v1::ListEventsRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::webhook_api::v1::ListEventsRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::webhook_api::v1::ListEventsRequest>* __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListEventsRequest_new() {
    return new google::protobuf::Map<std::string, ::webhook_api::v1::ListEventsRequest>();
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListEventsRequest_free(const google::protobuf::Map<std::string, ::webhook_api::v1::ListEventsRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListEventsRequest_clear(google::protobuf::Map<std::string, ::webhook_api::v1::ListEventsRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListEventsRequest_size(const google::protobuf::Map<std::string, ::webhook_api::v1::ListEventsRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListEventsRequest_insert(google::protobuf::Map<std::string, ::webhook_api::v1::ListEventsRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::ListEventsRequest value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListEventsRequest_get(const google::protobuf::Map<std::string, ::webhook_api::v1::ListEventsRequest>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::webhook_api::v1::ListEventsRequest** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ListEventsRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListEventsRequest_remove(google::protobuf::Map<std::string, ::webhook_api::v1::ListEventsRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::ListEventsRequest * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListEventsRequest_iter(
      const google::protobuf::Map<std::string, ::webhook_api::v1::ListEventsRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListEventsRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::webhook_api::v1::ListEventsRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::webhook_api::v1::ListEventsRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// webhook_api.v1.ListEventsResponse
        // clang-format off
extern "C" {
void* __rust_proto_thunk__webhook_api_v1_ListEventsResponse_new() { return new ::webhook_api::v1::ListEventsResponse(); }
void __rust_proto_thunk__webhook_api_v1_ListEventsResponse_delete(void* ptr) { delete static_cast<::webhook_api::v1::ListEventsResponse*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__webhook_api_v1_ListEventsResponse_serialize(::webhook_api::v1::ListEventsResponse* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__webhook_api_v1_ListEventsResponse_parse(::webhook_api::v1::ListEventsResponse* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__webhook_api_v1_ListEventsResponse_copy_from(::webhook_api::v1::ListEventsResponse* dst, const ::webhook_api::v1::ListEventsResponse* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__webhook_api_v1_ListEventsResponse_repeated_len(google::protobuf::RepeatedPtrField<::webhook_api::v1::ListEventsResponse>* field) {
  return field->size();
}
const ::webhook_api::v1::ListEventsResponse& __rust_proto_thunk__webhook_api_v1_ListEventsResponse_repeated_get(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::ListEventsResponse>* field,
  size_t index) {
  return field->Get(index);
}
::webhook_api::v1::ListEventsResponse* __rust_proto_thunk__webhook_api_v1_ListEventsResponse_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::ListEventsResponse>* field,
  size_t index) {
  return field->Mutable(index);
}
::webhook_api::v1::ListEventsResponse* __rust_proto_thunk__webhook_api_v1_ListEventsResponse_repeated_add(google::protobuf::RepeatedPtrField<::webhook_api::v1::ListEventsResponse>* field) {
  return field->Add();
}
void __rust_proto_thunk__webhook_api_v1_ListEventsResponse_repeated_clear(google::protobuf::RepeatedPtrField<::webhook_api::v1::ListEventsResponse>* field) {
  field->Clear();
}
void __rust_proto_thunk__webhook_api_v1_ListEventsResponse_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::ListEventsResponse>& dst,
  const google::protobuf::RepeatedPtrField<::webhook_api::v1::ListEventsResponse>& src) {
  dst = src;
}

void __rust_proto_thunk__webhook_api_v1_ListEventsResponse_clear_events(::webhook_api::v1::ListEventsResponse* msg) {
  msg->clear_events();
}
google::protobuf::RepeatedPtrField<::webhook_api::v1::Event>* __rust_proto_thunk__webhook_api_v1_ListEventsResponse_get_mut_events(::webhook_api::v1::ListEventsResponse* msg) {
  return msg->mutable_events();
}
const google::protobuf::RepeatedPtrField<::webhook_api::v1::Event>* __rust_proto_thunk__webhook_api_v1_ListEventsResponse_get_events(
    const ::webhook_api::v1::ListEventsResponse* msg) {
  return &msg->events();
}

::uint32_t __rust_proto_thunk__webhook_api_v1_ListEventsResponse_get_total_owned_events(::webhook_api::v1::ListEventsResponse* msg) { return msg->total_owned_events(); }
void __rust_proto_thunk__webhook_api_v1_ListEventsResponse_set_total_owned_events(::webhook_api::v1::ListEventsResponse* msg, ::uint32_t val) {
  msg->set_total_owned_events(val);
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::webhook_api::v1::ListEventsResponse>* __rust_proto_thunk__Map_i32_webhook_api_v1_ListEventsResponse_new() {
    return new google::protobuf::Map<int32_t, ::webhook_api::v1::ListEventsResponse>();
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_ListEventsResponse_free(const google::protobuf::Map<int32_t, ::webhook_api::v1::ListEventsResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_ListEventsResponse_clear(google::protobuf::Map<int32_t, ::webhook_api::v1::ListEventsResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_webhook_api_v1_ListEventsResponse_size(const google::protobuf::Map<int32_t, ::webhook_api::v1::ListEventsResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_ListEventsResponse_insert(google::protobuf::Map<int32_t, ::webhook_api::v1::ListEventsResponse> * m,
                          int32_t key, ::webhook_api::v1::ListEventsResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_ListEventsResponse_get(const google::protobuf::Map<int32_t, ::webhook_api::v1::ListEventsResponse>* m,
                       int32_t key, const ::webhook_api::v1::ListEventsResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ListEventsResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_ListEventsResponse_remove(google::protobuf::Map<int32_t, ::webhook_api::v1::ListEventsResponse> * m,
                          int32_t key, ::webhook_api::v1::ListEventsResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_webhook_api_v1_ListEventsResponse_iter(
      const google::protobuf::Map<int32_t, ::webhook_api::v1::ListEventsResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_ListEventsResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::webhook_api::v1::ListEventsResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::webhook_api::v1::ListEventsResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::webhook_api::v1::ListEventsResponse>* __rust_proto_thunk__Map_u32_webhook_api_v1_ListEventsResponse_new() {
    return new google::protobuf::Map<uint32_t, ::webhook_api::v1::ListEventsResponse>();
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_ListEventsResponse_free(const google::protobuf::Map<uint32_t, ::webhook_api::v1::ListEventsResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_ListEventsResponse_clear(google::protobuf::Map<uint32_t, ::webhook_api::v1::ListEventsResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_webhook_api_v1_ListEventsResponse_size(const google::protobuf::Map<uint32_t, ::webhook_api::v1::ListEventsResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_ListEventsResponse_insert(google::protobuf::Map<uint32_t, ::webhook_api::v1::ListEventsResponse> * m,
                          uint32_t key, ::webhook_api::v1::ListEventsResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_ListEventsResponse_get(const google::protobuf::Map<uint32_t, ::webhook_api::v1::ListEventsResponse>* m,
                       uint32_t key, const ::webhook_api::v1::ListEventsResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ListEventsResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_ListEventsResponse_remove(google::protobuf::Map<uint32_t, ::webhook_api::v1::ListEventsResponse> * m,
                          uint32_t key, ::webhook_api::v1::ListEventsResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_webhook_api_v1_ListEventsResponse_iter(
      const google::protobuf::Map<uint32_t, ::webhook_api::v1::ListEventsResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_ListEventsResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::webhook_api::v1::ListEventsResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::webhook_api::v1::ListEventsResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::webhook_api::v1::ListEventsResponse>* __rust_proto_thunk__Map_i64_webhook_api_v1_ListEventsResponse_new() {
    return new google::protobuf::Map<int64_t, ::webhook_api::v1::ListEventsResponse>();
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_ListEventsResponse_free(const google::protobuf::Map<int64_t, ::webhook_api::v1::ListEventsResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_ListEventsResponse_clear(google::protobuf::Map<int64_t, ::webhook_api::v1::ListEventsResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_webhook_api_v1_ListEventsResponse_size(const google::protobuf::Map<int64_t, ::webhook_api::v1::ListEventsResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_ListEventsResponse_insert(google::protobuf::Map<int64_t, ::webhook_api::v1::ListEventsResponse> * m,
                          int64_t key, ::webhook_api::v1::ListEventsResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_ListEventsResponse_get(const google::protobuf::Map<int64_t, ::webhook_api::v1::ListEventsResponse>* m,
                       int64_t key, const ::webhook_api::v1::ListEventsResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ListEventsResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_ListEventsResponse_remove(google::protobuf::Map<int64_t, ::webhook_api::v1::ListEventsResponse> * m,
                          int64_t key, ::webhook_api::v1::ListEventsResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_webhook_api_v1_ListEventsResponse_iter(
      const google::protobuf::Map<int64_t, ::webhook_api::v1::ListEventsResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_ListEventsResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::webhook_api::v1::ListEventsResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::webhook_api::v1::ListEventsResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::webhook_api::v1::ListEventsResponse>* __rust_proto_thunk__Map_u64_webhook_api_v1_ListEventsResponse_new() {
    return new google::protobuf::Map<uint64_t, ::webhook_api::v1::ListEventsResponse>();
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_ListEventsResponse_free(const google::protobuf::Map<uint64_t, ::webhook_api::v1::ListEventsResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_ListEventsResponse_clear(google::protobuf::Map<uint64_t, ::webhook_api::v1::ListEventsResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_webhook_api_v1_ListEventsResponse_size(const google::protobuf::Map<uint64_t, ::webhook_api::v1::ListEventsResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_ListEventsResponse_insert(google::protobuf::Map<uint64_t, ::webhook_api::v1::ListEventsResponse> * m,
                          uint64_t key, ::webhook_api::v1::ListEventsResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_ListEventsResponse_get(const google::protobuf::Map<uint64_t, ::webhook_api::v1::ListEventsResponse>* m,
                       uint64_t key, const ::webhook_api::v1::ListEventsResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ListEventsResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_ListEventsResponse_remove(google::protobuf::Map<uint64_t, ::webhook_api::v1::ListEventsResponse> * m,
                          uint64_t key, ::webhook_api::v1::ListEventsResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_webhook_api_v1_ListEventsResponse_iter(
      const google::protobuf::Map<uint64_t, ::webhook_api::v1::ListEventsResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_ListEventsResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::webhook_api::v1::ListEventsResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::webhook_api::v1::ListEventsResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::webhook_api::v1::ListEventsResponse>* __rust_proto_thunk__Map_bool_webhook_api_v1_ListEventsResponse_new() {
    return new google::protobuf::Map<bool, ::webhook_api::v1::ListEventsResponse>();
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_ListEventsResponse_free(const google::protobuf::Map<bool, ::webhook_api::v1::ListEventsResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_ListEventsResponse_clear(google::protobuf::Map<bool, ::webhook_api::v1::ListEventsResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_webhook_api_v1_ListEventsResponse_size(const google::protobuf::Map<bool, ::webhook_api::v1::ListEventsResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_ListEventsResponse_insert(google::protobuf::Map<bool, ::webhook_api::v1::ListEventsResponse> * m,
                          bool key, ::webhook_api::v1::ListEventsResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_ListEventsResponse_get(const google::protobuf::Map<bool, ::webhook_api::v1::ListEventsResponse>* m,
                       bool key, const ::webhook_api::v1::ListEventsResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ListEventsResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_ListEventsResponse_remove(google::protobuf::Map<bool, ::webhook_api::v1::ListEventsResponse> * m,
                          bool key, ::webhook_api::v1::ListEventsResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_webhook_api_v1_ListEventsResponse_iter(
      const google::protobuf::Map<bool, ::webhook_api::v1::ListEventsResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_ListEventsResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::webhook_api::v1::ListEventsResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::webhook_api::v1::ListEventsResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::webhook_api::v1::ListEventsResponse>* __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListEventsResponse_new() {
    return new google::protobuf::Map<std::string, ::webhook_api::v1::ListEventsResponse>();
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListEventsResponse_free(const google::protobuf::Map<std::string, ::webhook_api::v1::ListEventsResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListEventsResponse_clear(google::protobuf::Map<std::string, ::webhook_api::v1::ListEventsResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListEventsResponse_size(const google::protobuf::Map<std::string, ::webhook_api::v1::ListEventsResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListEventsResponse_insert(google::protobuf::Map<std::string, ::webhook_api::v1::ListEventsResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::ListEventsResponse value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListEventsResponse_get(const google::protobuf::Map<std::string, ::webhook_api::v1::ListEventsResponse>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::webhook_api::v1::ListEventsResponse** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ListEventsResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListEventsResponse_remove(google::protobuf::Map<std::string, ::webhook_api::v1::ListEventsResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::ListEventsResponse * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListEventsResponse_iter(
      const google::protobuf::Map<std::string, ::webhook_api::v1::ListEventsResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListEventsResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::webhook_api::v1::ListEventsResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::webhook_api::v1::ListEventsResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// webhook_api.v1.RetryEventRequest
        // clang-format off
extern "C" {
void* __rust_proto_thunk__webhook_api_v1_RetryEventRequest_new() { return new ::webhook_api::v1::RetryEventRequest(); }
void __rust_proto_thunk__webhook_api_v1_RetryEventRequest_delete(void* ptr) { delete static_cast<::webhook_api::v1::RetryEventRequest*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__webhook_api_v1_RetryEventRequest_serialize(::webhook_api::v1::RetryEventRequest* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__webhook_api_v1_RetryEventRequest_parse(::webhook_api::v1::RetryEventRequest* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__webhook_api_v1_RetryEventRequest_copy_from(::webhook_api::v1::RetryEventRequest* dst, const ::webhook_api::v1::RetryEventRequest* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__webhook_api_v1_RetryEventRequest_repeated_len(google::protobuf::RepeatedPtrField<::webhook_api::v1::RetryEventRequest>* field) {
  return field->size();
}
const ::webhook_api::v1::RetryEventRequest& __rust_proto_thunk__webhook_api_v1_RetryEventRequest_repeated_get(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::RetryEventRequest>* field,
  size_t index) {
  return field->Get(index);
}
::webhook_api::v1::RetryEventRequest* __rust_proto_thunk__webhook_api_v1_RetryEventRequest_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::RetryEventRequest>* field,
  size_t index) {
  return field->Mutable(index);
}
::webhook_api::v1::RetryEventRequest* __rust_proto_thunk__webhook_api_v1_RetryEventRequest_repeated_add(google::protobuf::RepeatedPtrField<::webhook_api::v1::RetryEventRequest>* field) {
  return field->Add();
}
void __rust_proto_thunk__webhook_api_v1_RetryEventRequest_repeated_clear(google::protobuf::RepeatedPtrField<::webhook_api::v1::RetryEventRequest>* field) {
  field->Clear();
}
void __rust_proto_thunk__webhook_api_v1_RetryEventRequest_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::RetryEventRequest>& dst,
  const google::protobuf::RepeatedPtrField<::webhook_api::v1::RetryEventRequest>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__webhook_api_v1_RetryEventRequest_get_api_key(::webhook_api::v1::RetryEventRequest* msg) {
  absl::string_view val = msg->api_key();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__webhook_api_v1_RetryEventRequest_set_api_key(::webhook_api::v1::RetryEventRequest* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_api_key(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__webhook_api_v1_RetryEventRequest_get_event_id(::webhook_api::v1::RetryEventRequest* msg) {
  absl::string_view val = msg->event_id();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__webhook_api_v1_RetryEventRequest_set_event_id(::webhook_api::v1::RetryEventRequest* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_event_id(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__webhook_api_v1_RetryEventRequest_get_webhook_id(::webhook_api::v1::RetryEventRequest* msg) {
  absl::string_view val = msg->webhook_id();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__webhook_api_v1_RetryEventRequest_set_webhook_id(::webhook_api::v1::RetryEventRequest* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_webhook_id(absl::string_view(s.ptr, s.len));
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::webhook_api::v1::RetryEventRequest>* __rust_proto_thunk__Map_i32_webhook_api_v1_RetryEventRequest_new() {
    return new google::protobuf::Map<int32_t, ::webhook_api::v1::RetryEventRequest>();
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_RetryEventRequest_free(const google::protobuf::Map<int32_t, ::webhook_api::v1::RetryEventRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_RetryEventRequest_clear(google::protobuf::Map<int32_t, ::webhook_api::v1::RetryEventRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_webhook_api_v1_RetryEventRequest_size(const google::protobuf::Map<int32_t, ::webhook_api::v1::RetryEventRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_RetryEventRequest_insert(google::protobuf::Map<int32_t, ::webhook_api::v1::RetryEventRequest> * m,
                          int32_t key, ::webhook_api::v1::RetryEventRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_RetryEventRequest_get(const google::protobuf::Map<int32_t, ::webhook_api::v1::RetryEventRequest>* m,
                       int32_t key, const ::webhook_api::v1::RetryEventRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::RetryEventRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_RetryEventRequest_remove(google::protobuf::Map<int32_t, ::webhook_api::v1::RetryEventRequest> * m,
                          int32_t key, ::webhook_api::v1::RetryEventRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_webhook_api_v1_RetryEventRequest_iter(
      const google::protobuf::Map<int32_t, ::webhook_api::v1::RetryEventRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_RetryEventRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::webhook_api::v1::RetryEventRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::webhook_api::v1::RetryEventRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::webhook_api::v1::RetryEventRequest>* __rust_proto_thunk__Map_u32_webhook_api_v1_RetryEventRequest_new() {
    return new google::protobuf::Map<uint32_t, ::webhook_api::v1::RetryEventRequest>();
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_RetryEventRequest_free(const google::protobuf::Map<uint32_t, ::webhook_api::v1::RetryEventRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_RetryEventRequest_clear(google::protobuf::Map<uint32_t, ::webhook_api::v1::RetryEventRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_webhook_api_v1_RetryEventRequest_size(const google::protobuf::Map<uint32_t, ::webhook_api::v1::RetryEventRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_RetryEventRequest_insert(google::protobuf::Map<uint32_t, ::webhook_api::v1::RetryEventRequest> * m,
                          uint32_t key, ::webhook_api::v1::RetryEventRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_RetryEventRequest_get(const google::protobuf::Map<uint32_t, ::webhook_api::v1::RetryEventRequest>* m,
                       uint32_t key, const ::webhook_api::v1::RetryEventRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::RetryEventRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_RetryEventRequest_remove(google::protobuf::Map<uint32_t, ::webhook_api::v1::RetryEventRequest> * m,
                          uint32_t key, ::webhook_api::v1::RetryEventRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_webhook_api_v1_RetryEventRequest_iter(
      const google::protobuf::Map<uint32_t, ::webhook_api::v1::RetryEventRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_RetryEventRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::webhook_api::v1::RetryEventRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::webhook_api::v1::RetryEventRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::webhook_api::v1::RetryEventRequest>* __rust_proto_thunk__Map_i64_webhook_api_v1_RetryEventRequest_new() {
    return new google::protobuf::Map<int64_t, ::webhook_api::v1::RetryEventRequest>();
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_RetryEventRequest_free(const google::protobuf::Map<int64_t, ::webhook_api::v1::RetryEventRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_RetryEventRequest_clear(google::protobuf::Map<int64_t, ::webhook_api::v1::RetryEventRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_webhook_api_v1_RetryEventRequest_size(const google::protobuf::Map<int64_t, ::webhook_api::v1::RetryEventRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_RetryEventRequest_insert(google::protobuf::Map<int64_t, ::webhook_api::v1::RetryEventRequest> * m,
                          int64_t key, ::webhook_api::v1::RetryEventRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_RetryEventRequest_get(const google::protobuf::Map<int64_t, ::webhook_api::v1::RetryEventRequest>* m,
                       int64_t key, const ::webhook_api::v1::RetryEventRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::RetryEventRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_RetryEventRequest_remove(google::protobuf::Map<int64_t, ::webhook_api::v1::RetryEventRequest> * m,
                          int64_t key, ::webhook_api::v1::RetryEventRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_webhook_api_v1_RetryEventRequest_iter(
      const google::protobuf::Map<int64_t, ::webhook_api::v1::RetryEventRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_RetryEventRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::webhook_api::v1::RetryEventRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::webhook_api::v1::RetryEventRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::webhook_api::v1::RetryEventRequest>* __rust_proto_thunk__Map_u64_webhook_api_v1_RetryEventRequest_new() {
    return new google::protobuf::Map<uint64_t, ::webhook_api::v1::RetryEventRequest>();
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_RetryEventRequest_free(const google::protobuf::Map<uint64_t, ::webhook_api::v1::RetryEventRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_RetryEventRequest_clear(google::protobuf::Map<uint64_t, ::webhook_api::v1::RetryEventRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_webhook_api_v1_RetryEventRequest_size(const google::protobuf::Map<uint64_t, ::webhook_api::v1::RetryEventRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_RetryEventRequest_insert(google::protobuf::Map<uint64_t, ::webhook_api::v1::RetryEventRequest> * m,
                          uint64_t key, ::webhook_api::v1::RetryEventRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_RetryEventRequest_get(const google::protobuf::Map<uint64_t, ::webhook_api::v1::RetryEventRequest>* m,
                       uint64_t key, const ::webhook_api::v1::RetryEventRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::RetryEventRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_RetryEventRequest_remove(google::protobuf::Map<uint64_t, ::webhook_api::v1::RetryEventRequest> * m,
                          uint64_t key, ::webhook_api::v1::RetryEventRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_webhook_api_v1_RetryEventRequest_iter(
      const google::protobuf::Map<uint64_t, ::webhook_api::v1::RetryEventRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_RetryEventRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::webhook_api::v1::RetryEventRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::webhook_api::v1::RetryEventRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::webhook_api::v1::RetryEventRequest>* __rust_proto_thunk__Map_bool_webhook_api_v1_RetryEventRequest_new() {
    return new google::protobuf::Map<bool, ::webhook_api::v1::RetryEventRequest>();
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_RetryEventRequest_free(const google::protobuf::Map<bool, ::webhook_api::v1::RetryEventRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_RetryEventRequest_clear(google::protobuf::Map<bool, ::webhook_api::v1::RetryEventRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_webhook_api_v1_RetryEventRequest_size(const google::protobuf::Map<bool, ::webhook_api::v1::RetryEventRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_RetryEventRequest_insert(google::protobuf::Map<bool, ::webhook_api::v1::RetryEventRequest> * m,
                          bool key, ::webhook_api::v1::RetryEventRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_RetryEventRequest_get(const google::protobuf::Map<bool, ::webhook_api::v1::RetryEventRequest>* m,
                       bool key, const ::webhook_api::v1::RetryEventRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::RetryEventRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_RetryEventRequest_remove(google::protobuf::Map<bool, ::webhook_api::v1::RetryEventRequest> * m,
                          bool key, ::webhook_api::v1::RetryEventRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_webhook_api_v1_RetryEventRequest_iter(
      const google::protobuf::Map<bool, ::webhook_api::v1::RetryEventRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_RetryEventRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::webhook_api::v1::RetryEventRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::webhook_api::v1::RetryEventRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::webhook_api::v1::RetryEventRequest>* __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_RetryEventRequest_new() {
    return new google::protobuf::Map<std::string, ::webhook_api::v1::RetryEventRequest>();
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_RetryEventRequest_free(const google::protobuf::Map<std::string, ::webhook_api::v1::RetryEventRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_RetryEventRequest_clear(google::protobuf::Map<std::string, ::webhook_api::v1::RetryEventRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_RetryEventRequest_size(const google::protobuf::Map<std::string, ::webhook_api::v1::RetryEventRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_RetryEventRequest_insert(google::protobuf::Map<std::string, ::webhook_api::v1::RetryEventRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::RetryEventRequest value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_RetryEventRequest_get(const google::protobuf::Map<std::string, ::webhook_api::v1::RetryEventRequest>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::webhook_api::v1::RetryEventRequest** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::RetryEventRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_RetryEventRequest_remove(google::protobuf::Map<std::string, ::webhook_api::v1::RetryEventRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::RetryEventRequest * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_RetryEventRequest_iter(
      const google::protobuf::Map<std::string, ::webhook_api::v1::RetryEventRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_RetryEventRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::webhook_api::v1::RetryEventRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::webhook_api::v1::RetryEventRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// webhook_api.v1.RetryEventResponse
        // clang-format off
extern "C" {
void* __rust_proto_thunk__webhook_api_v1_RetryEventResponse_new() { return new ::webhook_api::v1::RetryEventResponse(); }
void __rust_proto_thunk__webhook_api_v1_RetryEventResponse_delete(void* ptr) { delete static_cast<::webhook_api::v1::RetryEventResponse*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__webhook_api_v1_RetryEventResponse_serialize(::webhook_api::v1::RetryEventResponse* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__webhook_api_v1_RetryEventResponse_parse(::webhook_api::v1::RetryEventResponse* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__webhook_api_v1_RetryEventResponse_copy_from(::webhook_api::v1::RetryEventResponse* dst, const ::webhook_api::v1::RetryEventResponse* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__webhook_api_v1_RetryEventResponse_repeated_len(google::protobuf::RepeatedPtrField<::webhook_api::v1::RetryEventResponse>* field) {
  return field->size();
}
const ::webhook_api::v1::RetryEventResponse& __rust_proto_thunk__webhook_api_v1_RetryEventResponse_repeated_get(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::RetryEventResponse>* field,
  size_t index) {
  return field->Get(index);
}
::webhook_api::v1::RetryEventResponse* __rust_proto_thunk__webhook_api_v1_RetryEventResponse_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::RetryEventResponse>* field,
  size_t index) {
  return field->Mutable(index);
}
::webhook_api::v1::RetryEventResponse* __rust_proto_thunk__webhook_api_v1_RetryEventResponse_repeated_add(google::protobuf::RepeatedPtrField<::webhook_api::v1::RetryEventResponse>* field) {
  return field->Add();
}
void __rust_proto_thunk__webhook_api_v1_RetryEventResponse_repeated_clear(google::protobuf::RepeatedPtrField<::webhook_api::v1::RetryEventResponse>* field) {
  field->Clear();
}
void __rust_proto_thunk__webhook_api_v1_RetryEventResponse_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::RetryEventResponse>& dst,
  const google::protobuf::RepeatedPtrField<::webhook_api::v1::RetryEventResponse>& src) {
  dst = src;
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::webhook_api::v1::RetryEventResponse>* __rust_proto_thunk__Map_i32_webhook_api_v1_RetryEventResponse_new() {
    return new google::protobuf::Map<int32_t, ::webhook_api::v1::RetryEventResponse>();
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_RetryEventResponse_free(const google::protobuf::Map<int32_t, ::webhook_api::v1::RetryEventResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_RetryEventResponse_clear(google::protobuf::Map<int32_t, ::webhook_api::v1::RetryEventResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_webhook_api_v1_RetryEventResponse_size(const google::protobuf::Map<int32_t, ::webhook_api::v1::RetryEventResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_RetryEventResponse_insert(google::protobuf::Map<int32_t, ::webhook_api::v1::RetryEventResponse> * m,
                          int32_t key, ::webhook_api::v1::RetryEventResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_RetryEventResponse_get(const google::protobuf::Map<int32_t, ::webhook_api::v1::RetryEventResponse>* m,
                       int32_t key, const ::webhook_api::v1::RetryEventResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::RetryEventResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_RetryEventResponse_remove(google::protobuf::Map<int32_t, ::webhook_api::v1::RetryEventResponse> * m,
                          int32_t key, ::webhook_api::v1::RetryEventResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_webhook_api_v1_RetryEventResponse_iter(
      const google::protobuf::Map<int32_t, ::webhook_api::v1::RetryEventResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_RetryEventResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::webhook_api::v1::RetryEventResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::webhook_api::v1::RetryEventResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::webhook_api::v1::RetryEventResponse>* __rust_proto_thunk__Map_u32_webhook_api_v1_RetryEventResponse_new() {
    return new google::protobuf::Map<uint32_t, ::webhook_api::v1::RetryEventResponse>();
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_RetryEventResponse_free(const google::protobuf::Map<uint32_t, ::webhook_api::v1::RetryEventResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_RetryEventResponse_clear(google::protobuf::Map<uint32_t, ::webhook_api::v1::RetryEventResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_webhook_api_v1_RetryEventResponse_size(const google::protobuf::Map<uint32_t, ::webhook_api::v1::RetryEventResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_RetryEventResponse_insert(google::protobuf::Map<uint32_t, ::webhook_api::v1::RetryEventResponse> * m,
                          uint32_t key, ::webhook_api::v1::RetryEventResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_RetryEventResponse_get(const google::protobuf::Map<uint32_t, ::webhook_api::v1::RetryEventResponse>* m,
                       uint32_t key, const ::webhook_api::v1::RetryEventResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::RetryEventResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_RetryEventResponse_remove(google::protobuf::Map<uint32_t, ::webhook_api::v1::RetryEventResponse> * m,
                          uint32_t key, ::webhook_api::v1::RetryEventResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_webhook_api_v1_RetryEventResponse_iter(
      const google::protobuf::Map<uint32_t, ::webhook_api::v1::RetryEventResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_RetryEventResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::webhook_api::v1::RetryEventResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::webhook_api::v1::RetryEventResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::webhook_api::v1::RetryEventResponse>* __rust_proto_thunk__Map_i64_webhook_api_v1_RetryEventResponse_new() {
    return new google::protobuf::Map<int64_t, ::webhook_api::v1::RetryEventResponse>();
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_RetryEventResponse_free(const google::protobuf::Map<int64_t, ::webhook_api::v1::RetryEventResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_RetryEventResponse_clear(google::protobuf::Map<int64_t, ::webhook_api::v1::RetryEventResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_webhook_api_v1_RetryEventResponse_size(const google::protobuf::Map<int64_t, ::webhook_api::v1::RetryEventResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_RetryEventResponse_insert(google::protobuf::Map<int64_t, ::webhook_api::v1::RetryEventResponse> * m,
                          int64_t key, ::webhook_api::v1::RetryEventResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_RetryEventResponse_get(const google::protobuf::Map<int64_t, ::webhook_api::v1::RetryEventResponse>* m,
                       int64_t key, const ::webhook_api::v1::RetryEventResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::RetryEventResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_RetryEventResponse_remove(google::protobuf::Map<int64_t, ::webhook_api::v1::RetryEventResponse> * m,
                          int64_t key, ::webhook_api::v1::RetryEventResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_webhook_api_v1_RetryEventResponse_iter(
      const google::protobuf::Map<int64_t, ::webhook_api::v1::RetryEventResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_RetryEventResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::webhook_api::v1::RetryEventResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::webhook_api::v1::RetryEventResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::webhook_api::v1::RetryEventResponse>* __rust_proto_thunk__Map_u64_webhook_api_v1_RetryEventResponse_new() {
    return new google::protobuf::Map<uint64_t, ::webhook_api::v1::RetryEventResponse>();
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_RetryEventResponse_free(const google::protobuf::Map<uint64_t, ::webhook_api::v1::RetryEventResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_RetryEventResponse_clear(google::protobuf::Map<uint64_t, ::webhook_api::v1::RetryEventResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_webhook_api_v1_RetryEventResponse_size(const google::protobuf::Map<uint64_t, ::webhook_api::v1::RetryEventResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_RetryEventResponse_insert(google::protobuf::Map<uint64_t, ::webhook_api::v1::RetryEventResponse> * m,
                          uint64_t key, ::webhook_api::v1::RetryEventResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_RetryEventResponse_get(const google::protobuf::Map<uint64_t, ::webhook_api::v1::RetryEventResponse>* m,
                       uint64_t key, const ::webhook_api::v1::RetryEventResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::RetryEventResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_RetryEventResponse_remove(google::protobuf::Map<uint64_t, ::webhook_api::v1::RetryEventResponse> * m,
                          uint64_t key, ::webhook_api::v1::RetryEventResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_webhook_api_v1_RetryEventResponse_iter(
      const google::protobuf::Map<uint64_t, ::webhook_api::v1::RetryEventResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_RetryEventResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::webhook_api::v1::RetryEventResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::webhook_api::v1::RetryEventResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::webhook_api::v1::RetryEventResponse>* __rust_proto_thunk__Map_bool_webhook_api_v1_RetryEventResponse_new() {
    return new google::protobuf::Map<bool, ::webhook_api::v1::RetryEventResponse>();
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_RetryEventResponse_free(const google::protobuf::Map<bool, ::webhook_api::v1::RetryEventResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_RetryEventResponse_clear(google::protobuf::Map<bool, ::webhook_api::v1::RetryEventResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_webhook_api_v1_RetryEventResponse_size(const google::protobuf::Map<bool, ::webhook_api::v1::RetryEventResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_RetryEventResponse_insert(google::protobuf::Map<bool, ::webhook_api::v1::RetryEventResponse> * m,
                          bool key, ::webhook_api::v1::RetryEventResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_RetryEventResponse_get(const google::protobuf::Map<bool, ::webhook_api::v1::RetryEventResponse>* m,
                       bool key, const ::webhook_api::v1::RetryEventResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::RetryEventResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_RetryEventResponse_remove(google::protobuf::Map<bool, ::webhook_api::v1::RetryEventResponse> * m,
                          bool key, ::webhook_api::v1::RetryEventResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_webhook_api_v1_RetryEventResponse_iter(
      const google::protobuf::Map<bool, ::webhook_api::v1::RetryEventResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_RetryEventResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::webhook_api::v1::RetryEventResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::webhook_api::v1::RetryEventResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::webhook_api::v1::RetryEventResponse>* __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_RetryEventResponse_new() {
    return new google::protobuf::Map<std::string, ::webhook_api::v1::RetryEventResponse>();
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_RetryEventResponse_free(const google::protobuf::Map<std::string, ::webhook_api::v1::RetryEventResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_RetryEventResponse_clear(google::protobuf::Map<std::string, ::webhook_api::v1::RetryEventResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_RetryEventResponse_size(const google::protobuf::Map<std::string, ::webhook_api::v1::RetryEventResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_RetryEventResponse_insert(google::protobuf::Map<std::string, ::webhook_api::v1::RetryEventResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::RetryEventResponse value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_RetryEventResponse_get(const google::protobuf::Map<std::string, ::webhook_api::v1::RetryEventResponse>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::webhook_api::v1::RetryEventResponse** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::RetryEventResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_RetryEventResponse_remove(google::protobuf::Map<std::string, ::webhook_api::v1::RetryEventResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::RetryEventResponse * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_RetryEventResponse_iter(
      const google::protobuf::Map<std::string, ::webhook_api::v1::RetryEventResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_RetryEventResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::webhook_api::v1::RetryEventResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::webhook_api::v1::RetryEventResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// webhook_api.v1.PingRequest
        // clang-format off
extern "C" {
void* __rust_proto_thunk__webhook_api_v1_PingRequest_new() { return new ::webhook_api::v1::PingRequest(); }
void __rust_proto_thunk__webhook_api_v1_PingRequest_delete(void* ptr) { delete static_cast<::webhook_api::v1::PingRequest*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__webhook_api_v1_PingRequest_serialize(::webhook_api::v1::PingRequest* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__webhook_api_v1_PingRequest_parse(::webhook_api::v1::PingRequest* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__webhook_api_v1_PingRequest_copy_from(::webhook_api::v1::PingRequest* dst, const ::webhook_api::v1::PingRequest* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__webhook_api_v1_PingRequest_repeated_len(google::protobuf::RepeatedPtrField<::webhook_api::v1::PingRequest>* field) {
  return field->size();
}
const ::webhook_api::v1::PingRequest& __rust_proto_thunk__webhook_api_v1_PingRequest_repeated_get(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::PingRequest>* field,
  size_t index) {
  return field->Get(index);
}
::webhook_api::v1::PingRequest* __rust_proto_thunk__webhook_api_v1_PingRequest_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::PingRequest>* field,
  size_t index) {
  return field->Mutable(index);
}
::webhook_api::v1::PingRequest* __rust_proto_thunk__webhook_api_v1_PingRequest_repeated_add(google::protobuf::RepeatedPtrField<::webhook_api::v1::PingRequest>* field) {
  return field->Add();
}
void __rust_proto_thunk__webhook_api_v1_PingRequest_repeated_clear(google::protobuf::RepeatedPtrField<::webhook_api::v1::PingRequest>* field) {
  field->Clear();
}
void __rust_proto_thunk__webhook_api_v1_PingRequest_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::PingRequest>& dst,
  const google::protobuf::RepeatedPtrField<::webhook_api::v1::PingRequest>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__webhook_api_v1_PingRequest_get_api_key(::webhook_api::v1::PingRequest* msg) {
  absl::string_view val = msg->api_key();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__webhook_api_v1_PingRequest_set_api_key(::webhook_api::v1::PingRequest* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_api_key(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__webhook_api_v1_PingRequest_get_message(::webhook_api::v1::PingRequest* msg) {
  absl::string_view val = msg->message();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__webhook_api_v1_PingRequest_set_message(::webhook_api::v1::PingRequest* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_message(absl::string_view(s.ptr, s.len));
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::webhook_api::v1::PingRequest>* __rust_proto_thunk__Map_i32_webhook_api_v1_PingRequest_new() {
    return new google::protobuf::Map<int32_t, ::webhook_api::v1::PingRequest>();
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_PingRequest_free(const google::protobuf::Map<int32_t, ::webhook_api::v1::PingRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_PingRequest_clear(google::protobuf::Map<int32_t, ::webhook_api::v1::PingRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_webhook_api_v1_PingRequest_size(const google::protobuf::Map<int32_t, ::webhook_api::v1::PingRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_PingRequest_insert(google::protobuf::Map<int32_t, ::webhook_api::v1::PingRequest> * m,
                          int32_t key, ::webhook_api::v1::PingRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_PingRequest_get(const google::protobuf::Map<int32_t, ::webhook_api::v1::PingRequest>* m,
                       int32_t key, const ::webhook_api::v1::PingRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::PingRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_PingRequest_remove(google::protobuf::Map<int32_t, ::webhook_api::v1::PingRequest> * m,
                          int32_t key, ::webhook_api::v1::PingRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_webhook_api_v1_PingRequest_iter(
      const google::protobuf::Map<int32_t, ::webhook_api::v1::PingRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_PingRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::webhook_api::v1::PingRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::webhook_api::v1::PingRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::webhook_api::v1::PingRequest>* __rust_proto_thunk__Map_u32_webhook_api_v1_PingRequest_new() {
    return new google::protobuf::Map<uint32_t, ::webhook_api::v1::PingRequest>();
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_PingRequest_free(const google::protobuf::Map<uint32_t, ::webhook_api::v1::PingRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_PingRequest_clear(google::protobuf::Map<uint32_t, ::webhook_api::v1::PingRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_webhook_api_v1_PingRequest_size(const google::protobuf::Map<uint32_t, ::webhook_api::v1::PingRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_PingRequest_insert(google::protobuf::Map<uint32_t, ::webhook_api::v1::PingRequest> * m,
                          uint32_t key, ::webhook_api::v1::PingRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_PingRequest_get(const google::protobuf::Map<uint32_t, ::webhook_api::v1::PingRequest>* m,
                       uint32_t key, const ::webhook_api::v1::PingRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::PingRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_PingRequest_remove(google::protobuf::Map<uint32_t, ::webhook_api::v1::PingRequest> * m,
                          uint32_t key, ::webhook_api::v1::PingRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_webhook_api_v1_PingRequest_iter(
      const google::protobuf::Map<uint32_t, ::webhook_api::v1::PingRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_PingRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::webhook_api::v1::PingRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::webhook_api::v1::PingRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::webhook_api::v1::PingRequest>* __rust_proto_thunk__Map_i64_webhook_api_v1_PingRequest_new() {
    return new google::protobuf::Map<int64_t, ::webhook_api::v1::PingRequest>();
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_PingRequest_free(const google::protobuf::Map<int64_t, ::webhook_api::v1::PingRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_PingRequest_clear(google::protobuf::Map<int64_t, ::webhook_api::v1::PingRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_webhook_api_v1_PingRequest_size(const google::protobuf::Map<int64_t, ::webhook_api::v1::PingRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_PingRequest_insert(google::protobuf::Map<int64_t, ::webhook_api::v1::PingRequest> * m,
                          int64_t key, ::webhook_api::v1::PingRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_PingRequest_get(const google::protobuf::Map<int64_t, ::webhook_api::v1::PingRequest>* m,
                       int64_t key, const ::webhook_api::v1::PingRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::PingRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_PingRequest_remove(google::protobuf::Map<int64_t, ::webhook_api::v1::PingRequest> * m,
                          int64_t key, ::webhook_api::v1::PingRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_webhook_api_v1_PingRequest_iter(
      const google::protobuf::Map<int64_t, ::webhook_api::v1::PingRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_PingRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::webhook_api::v1::PingRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::webhook_api::v1::PingRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::webhook_api::v1::PingRequest>* __rust_proto_thunk__Map_u64_webhook_api_v1_PingRequest_new() {
    return new google::protobuf::Map<uint64_t, ::webhook_api::v1::PingRequest>();
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_PingRequest_free(const google::protobuf::Map<uint64_t, ::webhook_api::v1::PingRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_PingRequest_clear(google::protobuf::Map<uint64_t, ::webhook_api::v1::PingRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_webhook_api_v1_PingRequest_size(const google::protobuf::Map<uint64_t, ::webhook_api::v1::PingRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_PingRequest_insert(google::protobuf::Map<uint64_t, ::webhook_api::v1::PingRequest> * m,
                          uint64_t key, ::webhook_api::v1::PingRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_PingRequest_get(const google::protobuf::Map<uint64_t, ::webhook_api::v1::PingRequest>* m,
                       uint64_t key, const ::webhook_api::v1::PingRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::PingRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_PingRequest_remove(google::protobuf::Map<uint64_t, ::webhook_api::v1::PingRequest> * m,
                          uint64_t key, ::webhook_api::v1::PingRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_webhook_api_v1_PingRequest_iter(
      const google::protobuf::Map<uint64_t, ::webhook_api::v1::PingRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_PingRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::webhook_api::v1::PingRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::webhook_api::v1::PingRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::webhook_api::v1::PingRequest>* __rust_proto_thunk__Map_bool_webhook_api_v1_PingRequest_new() {
    return new google::protobuf::Map<bool, ::webhook_api::v1::PingRequest>();
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_PingRequest_free(const google::protobuf::Map<bool, ::webhook_api::v1::PingRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_PingRequest_clear(google::protobuf::Map<bool, ::webhook_api::v1::PingRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_webhook_api_v1_PingRequest_size(const google::protobuf::Map<bool, ::webhook_api::v1::PingRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_PingRequest_insert(google::protobuf::Map<bool, ::webhook_api::v1::PingRequest> * m,
                          bool key, ::webhook_api::v1::PingRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_PingRequest_get(const google::protobuf::Map<bool, ::webhook_api::v1::PingRequest>* m,
                       bool key, const ::webhook_api::v1::PingRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::PingRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_PingRequest_remove(google::protobuf::Map<bool, ::webhook_api::v1::PingRequest> * m,
                          bool key, ::webhook_api::v1::PingRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_webhook_api_v1_PingRequest_iter(
      const google::protobuf::Map<bool, ::webhook_api::v1::PingRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_PingRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::webhook_api::v1::PingRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::webhook_api::v1::PingRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::webhook_api::v1::PingRequest>* __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_PingRequest_new() {
    return new google::protobuf::Map<std::string, ::webhook_api::v1::PingRequest>();
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_PingRequest_free(const google::protobuf::Map<std::string, ::webhook_api::v1::PingRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_PingRequest_clear(google::protobuf::Map<std::string, ::webhook_api::v1::PingRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_PingRequest_size(const google::protobuf::Map<std::string, ::webhook_api::v1::PingRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_PingRequest_insert(google::protobuf::Map<std::string, ::webhook_api::v1::PingRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::PingRequest value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_PingRequest_get(const google::protobuf::Map<std::string, ::webhook_api::v1::PingRequest>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::webhook_api::v1::PingRequest** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::PingRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_PingRequest_remove(google::protobuf::Map<std::string, ::webhook_api::v1::PingRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::PingRequest * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_PingRequest_iter(
      const google::protobuf::Map<std::string, ::webhook_api::v1::PingRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_PingRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::webhook_api::v1::PingRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::webhook_api::v1::PingRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// webhook_api.v1.PingResponse
        // clang-format off
extern "C" {
void* __rust_proto_thunk__webhook_api_v1_PingResponse_new() { return new ::webhook_api::v1::PingResponse(); }
void __rust_proto_thunk__webhook_api_v1_PingResponse_delete(void* ptr) { delete static_cast<::webhook_api::v1::PingResponse*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__webhook_api_v1_PingResponse_serialize(::webhook_api::v1::PingResponse* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__webhook_api_v1_PingResponse_parse(::webhook_api::v1::PingResponse* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__webhook_api_v1_PingResponse_copy_from(::webhook_api::v1::PingResponse* dst, const ::webhook_api::v1::PingResponse* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__webhook_api_v1_PingResponse_repeated_len(google::protobuf::RepeatedPtrField<::webhook_api::v1::PingResponse>* field) {
  return field->size();
}
const ::webhook_api::v1::PingResponse& __rust_proto_thunk__webhook_api_v1_PingResponse_repeated_get(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::PingResponse>* field,
  size_t index) {
  return field->Get(index);
}
::webhook_api::v1::PingResponse* __rust_proto_thunk__webhook_api_v1_PingResponse_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::PingResponse>* field,
  size_t index) {
  return field->Mutable(index);
}
::webhook_api::v1::PingResponse* __rust_proto_thunk__webhook_api_v1_PingResponse_repeated_add(google::protobuf::RepeatedPtrField<::webhook_api::v1::PingResponse>* field) {
  return field->Add();
}
void __rust_proto_thunk__webhook_api_v1_PingResponse_repeated_clear(google::protobuf::RepeatedPtrField<::webhook_api::v1::PingResponse>* field) {
  field->Clear();
}
void __rust_proto_thunk__webhook_api_v1_PingResponse_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::PingResponse>& dst,
  const google::protobuf::RepeatedPtrField<::webhook_api::v1::PingResponse>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__webhook_api_v1_PingResponse_get_message(::webhook_api::v1::PingResponse* msg) {
  absl::string_view val = msg->message();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__webhook_api_v1_PingResponse_set_message(::webhook_api::v1::PingResponse* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_message(absl::string_view(s.ptr, s.len));
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::webhook_api::v1::PingResponse>* __rust_proto_thunk__Map_i32_webhook_api_v1_PingResponse_new() {
    return new google::protobuf::Map<int32_t, ::webhook_api::v1::PingResponse>();
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_PingResponse_free(const google::protobuf::Map<int32_t, ::webhook_api::v1::PingResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_PingResponse_clear(google::protobuf::Map<int32_t, ::webhook_api::v1::PingResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_webhook_api_v1_PingResponse_size(const google::protobuf::Map<int32_t, ::webhook_api::v1::PingResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_PingResponse_insert(google::protobuf::Map<int32_t, ::webhook_api::v1::PingResponse> * m,
                          int32_t key, ::webhook_api::v1::PingResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_PingResponse_get(const google::protobuf::Map<int32_t, ::webhook_api::v1::PingResponse>* m,
                       int32_t key, const ::webhook_api::v1::PingResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::PingResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_PingResponse_remove(google::protobuf::Map<int32_t, ::webhook_api::v1::PingResponse> * m,
                          int32_t key, ::webhook_api::v1::PingResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_webhook_api_v1_PingResponse_iter(
      const google::protobuf::Map<int32_t, ::webhook_api::v1::PingResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_PingResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::webhook_api::v1::PingResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::webhook_api::v1::PingResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::webhook_api::v1::PingResponse>* __rust_proto_thunk__Map_u32_webhook_api_v1_PingResponse_new() {
    return new google::protobuf::Map<uint32_t, ::webhook_api::v1::PingResponse>();
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_PingResponse_free(const google::protobuf::Map<uint32_t, ::webhook_api::v1::PingResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_PingResponse_clear(google::protobuf::Map<uint32_t, ::webhook_api::v1::PingResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_webhook_api_v1_PingResponse_size(const google::protobuf::Map<uint32_t, ::webhook_api::v1::PingResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_PingResponse_insert(google::protobuf::Map<uint32_t, ::webhook_api::v1::PingResponse> * m,
                          uint32_t key, ::webhook_api::v1::PingResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_PingResponse_get(const google::protobuf::Map<uint32_t, ::webhook_api::v1::PingResponse>* m,
                       uint32_t key, const ::webhook_api::v1::PingResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::PingResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_PingResponse_remove(google::protobuf::Map<uint32_t, ::webhook_api::v1::PingResponse> * m,
                          uint32_t key, ::webhook_api::v1::PingResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_webhook_api_v1_PingResponse_iter(
      const google::protobuf::Map<uint32_t, ::webhook_api::v1::PingResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_PingResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::webhook_api::v1::PingResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::webhook_api::v1::PingResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::webhook_api::v1::PingResponse>* __rust_proto_thunk__Map_i64_webhook_api_v1_PingResponse_new() {
    return new google::protobuf::Map<int64_t, ::webhook_api::v1::PingResponse>();
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_PingResponse_free(const google::protobuf::Map<int64_t, ::webhook_api::v1::PingResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_PingResponse_clear(google::protobuf::Map<int64_t, ::webhook_api::v1::PingResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_webhook_api_v1_PingResponse_size(const google::protobuf::Map<int64_t, ::webhook_api::v1::PingResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_PingResponse_insert(google::protobuf::Map<int64_t, ::webhook_api::v1::PingResponse> * m,
                          int64_t key, ::webhook_api::v1::PingResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_PingResponse_get(const google::protobuf::Map<int64_t, ::webhook_api::v1::PingResponse>* m,
                       int64_t key, const ::webhook_api::v1::PingResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::PingResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_PingResponse_remove(google::protobuf::Map<int64_t, ::webhook_api::v1::PingResponse> * m,
                          int64_t key, ::webhook_api::v1::PingResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_webhook_api_v1_PingResponse_iter(
      const google::protobuf::Map<int64_t, ::webhook_api::v1::PingResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_PingResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::webhook_api::v1::PingResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::webhook_api::v1::PingResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::webhook_api::v1::PingResponse>* __rust_proto_thunk__Map_u64_webhook_api_v1_PingResponse_new() {
    return new google::protobuf::Map<uint64_t, ::webhook_api::v1::PingResponse>();
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_PingResponse_free(const google::protobuf::Map<uint64_t, ::webhook_api::v1::PingResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_PingResponse_clear(google::protobuf::Map<uint64_t, ::webhook_api::v1::PingResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_webhook_api_v1_PingResponse_size(const google::protobuf::Map<uint64_t, ::webhook_api::v1::PingResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_PingResponse_insert(google::protobuf::Map<uint64_t, ::webhook_api::v1::PingResponse> * m,
                          uint64_t key, ::webhook_api::v1::PingResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_PingResponse_get(const google::protobuf::Map<uint64_t, ::webhook_api::v1::PingResponse>* m,
                       uint64_t key, const ::webhook_api::v1::PingResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::PingResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_PingResponse_remove(google::protobuf::Map<uint64_t, ::webhook_api::v1::PingResponse> * m,
                          uint64_t key, ::webhook_api::v1::PingResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_webhook_api_v1_PingResponse_iter(
      const google::protobuf::Map<uint64_t, ::webhook_api::v1::PingResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_PingResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::webhook_api::v1::PingResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::webhook_api::v1::PingResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::webhook_api::v1::PingResponse>* __rust_proto_thunk__Map_bool_webhook_api_v1_PingResponse_new() {
    return new google::protobuf::Map<bool, ::webhook_api::v1::PingResponse>();
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_PingResponse_free(const google::protobuf::Map<bool, ::webhook_api::v1::PingResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_PingResponse_clear(google::protobuf::Map<bool, ::webhook_api::v1::PingResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_webhook_api_v1_PingResponse_size(const google::protobuf::Map<bool, ::webhook_api::v1::PingResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_PingResponse_insert(google::protobuf::Map<bool, ::webhook_api::v1::PingResponse> * m,
                          bool key, ::webhook_api::v1::PingResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_PingResponse_get(const google::protobuf::Map<bool, ::webhook_api::v1::PingResponse>* m,
                       bool key, const ::webhook_api::v1::PingResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::PingResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_PingResponse_remove(google::protobuf::Map<bool, ::webhook_api::v1::PingResponse> * m,
                          bool key, ::webhook_api::v1::PingResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_webhook_api_v1_PingResponse_iter(
      const google::protobuf::Map<bool, ::webhook_api::v1::PingResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_PingResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::webhook_api::v1::PingResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::webhook_api::v1::PingResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::webhook_api::v1::PingResponse>* __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_PingResponse_new() {
    return new google::protobuf::Map<std::string, ::webhook_api::v1::PingResponse>();
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_PingResponse_free(const google::protobuf::Map<std::string, ::webhook_api::v1::PingResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_PingResponse_clear(google::protobuf::Map<std::string, ::webhook_api::v1::PingResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_PingResponse_size(const google::protobuf::Map<std::string, ::webhook_api::v1::PingResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_PingResponse_insert(google::protobuf::Map<std::string, ::webhook_api::v1::PingResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::PingResponse value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_PingResponse_get(const google::protobuf::Map<std::string, ::webhook_api::v1::PingResponse>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::webhook_api::v1::PingResponse** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::PingResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_PingResponse_remove(google::protobuf::Map<std::string, ::webhook_api::v1::PingResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::PingResponse * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_PingResponse_iter(
      const google::protobuf::Map<std::string, ::webhook_api::v1::PingResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_PingResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::webhook_api::v1::PingResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::webhook_api::v1::PingResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// webhook_api.v1.ListDeliveryHistoryRequest
        // clang-format off
extern "C" {
void* __rust_proto_thunk__webhook_api_v1_ListDeliveryHistoryRequest_new() { return new ::webhook_api::v1::ListDeliveryHistoryRequest(); }
void __rust_proto_thunk__webhook_api_v1_ListDeliveryHistoryRequest_delete(void* ptr) { delete static_cast<::webhook_api::v1::ListDeliveryHistoryRequest*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__webhook_api_v1_ListDeliveryHistoryRequest_serialize(::webhook_api::v1::ListDeliveryHistoryRequest* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__webhook_api_v1_ListDeliveryHistoryRequest_parse(::webhook_api::v1::ListDeliveryHistoryRequest* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__webhook_api_v1_ListDeliveryHistoryRequest_copy_from(::webhook_api::v1::ListDeliveryHistoryRequest* dst, const ::webhook_api::v1::ListDeliveryHistoryRequest* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__webhook_api_v1_ListDeliveryHistoryRequest_repeated_len(google::protobuf::RepeatedPtrField<::webhook_api::v1::ListDeliveryHistoryRequest>* field) {
  return field->size();
}
const ::webhook_api::v1::ListDeliveryHistoryRequest& __rust_proto_thunk__webhook_api_v1_ListDeliveryHistoryRequest_repeated_get(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::ListDeliveryHistoryRequest>* field,
  size_t index) {
  return field->Get(index);
}
::webhook_api::v1::ListDeliveryHistoryRequest* __rust_proto_thunk__webhook_api_v1_ListDeliveryHistoryRequest_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::ListDeliveryHistoryRequest>* field,
  size_t index) {
  return field->Mutable(index);
}
::webhook_api::v1::ListDeliveryHistoryRequest* __rust_proto_thunk__webhook_api_v1_ListDeliveryHistoryRequest_repeated_add(google::protobuf::RepeatedPtrField<::webhook_api::v1::ListDeliveryHistoryRequest>* field) {
  return field->Add();
}
void __rust_proto_thunk__webhook_api_v1_ListDeliveryHistoryRequest_repeated_clear(google::protobuf::RepeatedPtrField<::webhook_api::v1::ListDeliveryHistoryRequest>* field) {
  field->Clear();
}
void __rust_proto_thunk__webhook_api_v1_ListDeliveryHistoryRequest_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::ListDeliveryHistoryRequest>& dst,
  const google::protobuf::RepeatedPtrField<::webhook_api::v1::ListDeliveryHistoryRequest>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__webhook_api_v1_ListDeliveryHistoryRequest_get_api_key(::webhook_api::v1::ListDeliveryHistoryRequest* msg) {
  absl::string_view val = msg->api_key();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__webhook_api_v1_ListDeliveryHistoryRequest_set_api_key(::webhook_api::v1::ListDeliveryHistoryRequest* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_api_key(absl::string_view(s.ptr, s.len));
}

::int32_t __rust_proto_thunk__webhook_api_v1_ListDeliveryHistoryRequest_get_page(::webhook_api::v1::ListDeliveryHistoryRequest* msg) { return msg->page(); }
void __rust_proto_thunk__webhook_api_v1_ListDeliveryHistoryRequest_set_page(::webhook_api::v1::ListDeliveryHistoryRequest* msg, ::int32_t val) {
  msg->set_page(val);
}

::int32_t __rust_proto_thunk__webhook_api_v1_ListDeliveryHistoryRequest_get_page_size(::webhook_api::v1::ListDeliveryHistoryRequest* msg) { return msg->page_size(); }
void __rust_proto_thunk__webhook_api_v1_ListDeliveryHistoryRequest_set_page_size(::webhook_api::v1::ListDeliveryHistoryRequest* msg, ::int32_t val) {
  msg->set_page_size(val);
}

const void* __rust_proto_thunk__webhook_api_v1_ListDeliveryHistoryRequest_get_filters(::webhook_api::v1::ListDeliveryHistoryRequest* msg) {
  return static_cast<const void*>(&msg->filters());
}
void* __rust_proto_thunk__webhook_api_v1_ListDeliveryHistoryRequest_get_mut_filters(::webhook_api::v1::ListDeliveryHistoryRequest* msg) {
  return static_cast<void*>(msg->mutable_filters());
}
void __rust_proto_thunk__webhook_api_v1_ListDeliveryHistoryRequest_clear_filters(::webhook_api::v1::ListDeliveryHistoryRequest* msg) { msg->clear_filters(); }
bool __rust_proto_thunk__webhook_api_v1_ListDeliveryHistoryRequest_has_filters(::webhook_api::v1::ListDeliveryHistoryRequest* msg) { return msg->has_filters(); }

const void* __rust_proto_thunk__webhook_api_v1_ListDeliveryHistoryRequest_get_sort_by(::webhook_api::v1::ListDeliveryHistoryRequest* msg) {
  return static_cast<const void*>(&msg->sort_by());
}
void* __rust_proto_thunk__webhook_api_v1_ListDeliveryHistoryRequest_get_mut_sort_by(::webhook_api::v1::ListDeliveryHistoryRequest* msg) {
  return static_cast<void*>(msg->mutable_sort_by());
}
void __rust_proto_thunk__webhook_api_v1_ListDeliveryHistoryRequest_clear_sort_by(::webhook_api::v1::ListDeliveryHistoryRequest* msg) { msg->clear_sort_by(); }
bool __rust_proto_thunk__webhook_api_v1_ListDeliveryHistoryRequest_has_sort_by(::webhook_api::v1::ListDeliveryHistoryRequest* msg) { return msg->has_sort_by(); }


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::webhook_api::v1::ListDeliveryHistoryRequest>* __rust_proto_thunk__Map_i32_webhook_api_v1_ListDeliveryHistoryRequest_new() {
    return new google::protobuf::Map<int32_t, ::webhook_api::v1::ListDeliveryHistoryRequest>();
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_ListDeliveryHistoryRequest_free(const google::protobuf::Map<int32_t, ::webhook_api::v1::ListDeliveryHistoryRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_ListDeliveryHistoryRequest_clear(google::protobuf::Map<int32_t, ::webhook_api::v1::ListDeliveryHistoryRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_webhook_api_v1_ListDeliveryHistoryRequest_size(const google::protobuf::Map<int32_t, ::webhook_api::v1::ListDeliveryHistoryRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_ListDeliveryHistoryRequest_insert(google::protobuf::Map<int32_t, ::webhook_api::v1::ListDeliveryHistoryRequest> * m,
                          int32_t key, ::webhook_api::v1::ListDeliveryHistoryRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_ListDeliveryHistoryRequest_get(const google::protobuf::Map<int32_t, ::webhook_api::v1::ListDeliveryHistoryRequest>* m,
                       int32_t key, const ::webhook_api::v1::ListDeliveryHistoryRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ListDeliveryHistoryRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_ListDeliveryHistoryRequest_remove(google::protobuf::Map<int32_t, ::webhook_api::v1::ListDeliveryHistoryRequest> * m,
                          int32_t key, ::webhook_api::v1::ListDeliveryHistoryRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_webhook_api_v1_ListDeliveryHistoryRequest_iter(
      const google::protobuf::Map<int32_t, ::webhook_api::v1::ListDeliveryHistoryRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_ListDeliveryHistoryRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::webhook_api::v1::ListDeliveryHistoryRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::webhook_api::v1::ListDeliveryHistoryRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::webhook_api::v1::ListDeliveryHistoryRequest>* __rust_proto_thunk__Map_u32_webhook_api_v1_ListDeliveryHistoryRequest_new() {
    return new google::protobuf::Map<uint32_t, ::webhook_api::v1::ListDeliveryHistoryRequest>();
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_ListDeliveryHistoryRequest_free(const google::protobuf::Map<uint32_t, ::webhook_api::v1::ListDeliveryHistoryRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_ListDeliveryHistoryRequest_clear(google::protobuf::Map<uint32_t, ::webhook_api::v1::ListDeliveryHistoryRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_webhook_api_v1_ListDeliveryHistoryRequest_size(const google::protobuf::Map<uint32_t, ::webhook_api::v1::ListDeliveryHistoryRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_ListDeliveryHistoryRequest_insert(google::protobuf::Map<uint32_t, ::webhook_api::v1::ListDeliveryHistoryRequest> * m,
                          uint32_t key, ::webhook_api::v1::ListDeliveryHistoryRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_ListDeliveryHistoryRequest_get(const google::protobuf::Map<uint32_t, ::webhook_api::v1::ListDeliveryHistoryRequest>* m,
                       uint32_t key, const ::webhook_api::v1::ListDeliveryHistoryRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ListDeliveryHistoryRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_ListDeliveryHistoryRequest_remove(google::protobuf::Map<uint32_t, ::webhook_api::v1::ListDeliveryHistoryRequest> * m,
                          uint32_t key, ::webhook_api::v1::ListDeliveryHistoryRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_webhook_api_v1_ListDeliveryHistoryRequest_iter(
      const google::protobuf::Map<uint32_t, ::webhook_api::v1::ListDeliveryHistoryRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_ListDeliveryHistoryRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::webhook_api::v1::ListDeliveryHistoryRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::webhook_api::v1::ListDeliveryHistoryRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::webhook_api::v1::ListDeliveryHistoryRequest>* __rust_proto_thunk__Map_i64_webhook_api_v1_ListDeliveryHistoryRequest_new() {
    return new google::protobuf::Map<int64_t, ::webhook_api::v1::ListDeliveryHistoryRequest>();
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_ListDeliveryHistoryRequest_free(const google::protobuf::Map<int64_t, ::webhook_api::v1::ListDeliveryHistoryRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_ListDeliveryHistoryRequest_clear(google::protobuf::Map<int64_t, ::webhook_api::v1::ListDeliveryHistoryRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_webhook_api_v1_ListDeliveryHistoryRequest_size(const google::protobuf::Map<int64_t, ::webhook_api::v1::ListDeliveryHistoryRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_ListDeliveryHistoryRequest_insert(google::protobuf::Map<int64_t, ::webhook_api::v1::ListDeliveryHistoryRequest> * m,
                          int64_t key, ::webhook_api::v1::ListDeliveryHistoryRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_ListDeliveryHistoryRequest_get(const google::protobuf::Map<int64_t, ::webhook_api::v1::ListDeliveryHistoryRequest>* m,
                       int64_t key, const ::webhook_api::v1::ListDeliveryHistoryRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ListDeliveryHistoryRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_ListDeliveryHistoryRequest_remove(google::protobuf::Map<int64_t, ::webhook_api::v1::ListDeliveryHistoryRequest> * m,
                          int64_t key, ::webhook_api::v1::ListDeliveryHistoryRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_webhook_api_v1_ListDeliveryHistoryRequest_iter(
      const google::protobuf::Map<int64_t, ::webhook_api::v1::ListDeliveryHistoryRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_ListDeliveryHistoryRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::webhook_api::v1::ListDeliveryHistoryRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::webhook_api::v1::ListDeliveryHistoryRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::webhook_api::v1::ListDeliveryHistoryRequest>* __rust_proto_thunk__Map_u64_webhook_api_v1_ListDeliveryHistoryRequest_new() {
    return new google::protobuf::Map<uint64_t, ::webhook_api::v1::ListDeliveryHistoryRequest>();
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_ListDeliveryHistoryRequest_free(const google::protobuf::Map<uint64_t, ::webhook_api::v1::ListDeliveryHistoryRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_ListDeliveryHistoryRequest_clear(google::protobuf::Map<uint64_t, ::webhook_api::v1::ListDeliveryHistoryRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_webhook_api_v1_ListDeliveryHistoryRequest_size(const google::protobuf::Map<uint64_t, ::webhook_api::v1::ListDeliveryHistoryRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_ListDeliveryHistoryRequest_insert(google::protobuf::Map<uint64_t, ::webhook_api::v1::ListDeliveryHistoryRequest> * m,
                          uint64_t key, ::webhook_api::v1::ListDeliveryHistoryRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_ListDeliveryHistoryRequest_get(const google::protobuf::Map<uint64_t, ::webhook_api::v1::ListDeliveryHistoryRequest>* m,
                       uint64_t key, const ::webhook_api::v1::ListDeliveryHistoryRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ListDeliveryHistoryRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_ListDeliveryHistoryRequest_remove(google::protobuf::Map<uint64_t, ::webhook_api::v1::ListDeliveryHistoryRequest> * m,
                          uint64_t key, ::webhook_api::v1::ListDeliveryHistoryRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_webhook_api_v1_ListDeliveryHistoryRequest_iter(
      const google::protobuf::Map<uint64_t, ::webhook_api::v1::ListDeliveryHistoryRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_ListDeliveryHistoryRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::webhook_api::v1::ListDeliveryHistoryRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::webhook_api::v1::ListDeliveryHistoryRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::webhook_api::v1::ListDeliveryHistoryRequest>* __rust_proto_thunk__Map_bool_webhook_api_v1_ListDeliveryHistoryRequest_new() {
    return new google::protobuf::Map<bool, ::webhook_api::v1::ListDeliveryHistoryRequest>();
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_ListDeliveryHistoryRequest_free(const google::protobuf::Map<bool, ::webhook_api::v1::ListDeliveryHistoryRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_ListDeliveryHistoryRequest_clear(google::protobuf::Map<bool, ::webhook_api::v1::ListDeliveryHistoryRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_webhook_api_v1_ListDeliveryHistoryRequest_size(const google::protobuf::Map<bool, ::webhook_api::v1::ListDeliveryHistoryRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_ListDeliveryHistoryRequest_insert(google::protobuf::Map<bool, ::webhook_api::v1::ListDeliveryHistoryRequest> * m,
                          bool key, ::webhook_api::v1::ListDeliveryHistoryRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_ListDeliveryHistoryRequest_get(const google::protobuf::Map<bool, ::webhook_api::v1::ListDeliveryHistoryRequest>* m,
                       bool key, const ::webhook_api::v1::ListDeliveryHistoryRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ListDeliveryHistoryRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_ListDeliveryHistoryRequest_remove(google::protobuf::Map<bool, ::webhook_api::v1::ListDeliveryHistoryRequest> * m,
                          bool key, ::webhook_api::v1::ListDeliveryHistoryRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_webhook_api_v1_ListDeliveryHistoryRequest_iter(
      const google::protobuf::Map<bool, ::webhook_api::v1::ListDeliveryHistoryRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_ListDeliveryHistoryRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::webhook_api::v1::ListDeliveryHistoryRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::webhook_api::v1::ListDeliveryHistoryRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::webhook_api::v1::ListDeliveryHistoryRequest>* __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListDeliveryHistoryRequest_new() {
    return new google::protobuf::Map<std::string, ::webhook_api::v1::ListDeliveryHistoryRequest>();
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListDeliveryHistoryRequest_free(const google::protobuf::Map<std::string, ::webhook_api::v1::ListDeliveryHistoryRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListDeliveryHistoryRequest_clear(google::protobuf::Map<std::string, ::webhook_api::v1::ListDeliveryHistoryRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListDeliveryHistoryRequest_size(const google::protobuf::Map<std::string, ::webhook_api::v1::ListDeliveryHistoryRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListDeliveryHistoryRequest_insert(google::protobuf::Map<std::string, ::webhook_api::v1::ListDeliveryHistoryRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::ListDeliveryHistoryRequest value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListDeliveryHistoryRequest_get(const google::protobuf::Map<std::string, ::webhook_api::v1::ListDeliveryHistoryRequest>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::webhook_api::v1::ListDeliveryHistoryRequest** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ListDeliveryHistoryRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListDeliveryHistoryRequest_remove(google::protobuf::Map<std::string, ::webhook_api::v1::ListDeliveryHistoryRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::ListDeliveryHistoryRequest * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListDeliveryHistoryRequest_iter(
      const google::protobuf::Map<std::string, ::webhook_api::v1::ListDeliveryHistoryRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListDeliveryHistoryRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::webhook_api::v1::ListDeliveryHistoryRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::webhook_api::v1::ListDeliveryHistoryRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// webhook_api.v1.ListDeliveryHistoryResponse
        // clang-format off
extern "C" {
void* __rust_proto_thunk__webhook_api_v1_ListDeliveryHistoryResponse_new() { return new ::webhook_api::v1::ListDeliveryHistoryResponse(); }
void __rust_proto_thunk__webhook_api_v1_ListDeliveryHistoryResponse_delete(void* ptr) { delete static_cast<::webhook_api::v1::ListDeliveryHistoryResponse*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__webhook_api_v1_ListDeliveryHistoryResponse_serialize(::webhook_api::v1::ListDeliveryHistoryResponse* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__webhook_api_v1_ListDeliveryHistoryResponse_parse(::webhook_api::v1::ListDeliveryHistoryResponse* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__webhook_api_v1_ListDeliveryHistoryResponse_copy_from(::webhook_api::v1::ListDeliveryHistoryResponse* dst, const ::webhook_api::v1::ListDeliveryHistoryResponse* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__webhook_api_v1_ListDeliveryHistoryResponse_repeated_len(google::protobuf::RepeatedPtrField<::webhook_api::v1::ListDeliveryHistoryResponse>* field) {
  return field->size();
}
const ::webhook_api::v1::ListDeliveryHistoryResponse& __rust_proto_thunk__webhook_api_v1_ListDeliveryHistoryResponse_repeated_get(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::ListDeliveryHistoryResponse>* field,
  size_t index) {
  return field->Get(index);
}
::webhook_api::v1::ListDeliveryHistoryResponse* __rust_proto_thunk__webhook_api_v1_ListDeliveryHistoryResponse_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::ListDeliveryHistoryResponse>* field,
  size_t index) {
  return field->Mutable(index);
}
::webhook_api::v1::ListDeliveryHistoryResponse* __rust_proto_thunk__webhook_api_v1_ListDeliveryHistoryResponse_repeated_add(google::protobuf::RepeatedPtrField<::webhook_api::v1::ListDeliveryHistoryResponse>* field) {
  return field->Add();
}
void __rust_proto_thunk__webhook_api_v1_ListDeliveryHistoryResponse_repeated_clear(google::protobuf::RepeatedPtrField<::webhook_api::v1::ListDeliveryHistoryResponse>* field) {
  field->Clear();
}
void __rust_proto_thunk__webhook_api_v1_ListDeliveryHistoryResponse_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::ListDeliveryHistoryResponse>& dst,
  const google::protobuf::RepeatedPtrField<::webhook_api::v1::ListDeliveryHistoryResponse>& src) {
  dst = src;
}

void __rust_proto_thunk__webhook_api_v1_ListDeliveryHistoryResponse_clear_delivery_history(::webhook_api::v1::ListDeliveryHistoryResponse* msg) {
  msg->clear_delivery_history();
}
google::protobuf::RepeatedPtrField<::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory>* __rust_proto_thunk__webhook_api_v1_ListDeliveryHistoryResponse_get_mut_delivery_history(::webhook_api::v1::ListDeliveryHistoryResponse* msg) {
  return msg->mutable_delivery_history();
}
const google::protobuf::RepeatedPtrField<::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory>* __rust_proto_thunk__webhook_api_v1_ListDeliveryHistoryResponse_get_delivery_history(
    const ::webhook_api::v1::ListDeliveryHistoryResponse* msg) {
  return &msg->delivery_history();
}

::uint32_t __rust_proto_thunk__webhook_api_v1_ListDeliveryHistoryResponse_get_total_owned_delivery_history(::webhook_api::v1::ListDeliveryHistoryResponse* msg) { return msg->total_owned_delivery_history(); }
void __rust_proto_thunk__webhook_api_v1_ListDeliveryHistoryResponse_set_total_owned_delivery_history(::webhook_api::v1::ListDeliveryHistoryResponse* msg, ::uint32_t val) {
  msg->set_total_owned_delivery_history(val);
}


}  // extern "C"
// clang-format on

        // clang-format off
extern "C" {
void* __rust_proto_thunk__webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_new() { return new ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory(); }
void __rust_proto_thunk__webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_delete(void* ptr) { delete static_cast<::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_serialize(::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_parse(::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_copy_from(::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory* dst, const ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_repeated_len(google::protobuf::RepeatedPtrField<::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory>* field) {
  return field->size();
}
const ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory& __rust_proto_thunk__webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_repeated_get(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory>* field,
  size_t index) {
  return field->Get(index);
}
::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory* __rust_proto_thunk__webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory>* field,
  size_t index) {
  return field->Mutable(index);
}
::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory* __rust_proto_thunk__webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_repeated_add(google::protobuf::RepeatedPtrField<::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory>* field) {
  return field->Add();
}
void __rust_proto_thunk__webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_repeated_clear(google::protobuf::RepeatedPtrField<::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory>* field) {
  field->Clear();
}
void __rust_proto_thunk__webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory>& dst,
  const google::protobuf::RepeatedPtrField<::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_get_id(::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory* msg) {
  absl::string_view val = msg->id();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_set_id(::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_id(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_get_webhook_id(::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory* msg) {
  absl::string_view val = msg->webhook_id();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_set_webhook_id(::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_webhook_id(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_get_event_id(::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory* msg) {
  absl::string_view val = msg->event_id();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_set_event_id(::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_event_id(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_get_owner_id(::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory* msg) {
  absl::string_view val = msg->owner_id();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_set_owner_id(::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_owner_id(absl::string_view(s.ptr, s.len));
}

::webhook_api::v1::DeliveryStage __rust_proto_thunk__webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_get_delivery_stage(::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory* msg) { return msg->delivery_stage(); }
void __rust_proto_thunk__webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_set_delivery_stage(::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory* msg, ::webhook_api::v1::DeliveryStage val) {
  msg->set_delivery_stage(val);
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_get_created(::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory* msg) {
  absl::string_view val = msg->created();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_set_created(::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_created(absl::string_view(s.ptr, s.len));
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory>* __rust_proto_thunk__Map_i32_webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_new() {
    return new google::protobuf::Map<int32_t, ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory>();
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_free(const google::protobuf::Map<int32_t, ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_clear(google::protobuf::Map<int32_t, ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_size(const google::protobuf::Map<int32_t, ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_insert(google::protobuf::Map<int32_t, ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory> * m,
                          int32_t key, ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_get(const google::protobuf::Map<int32_t, ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory>* m,
                       int32_t key, const ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_remove(google::protobuf::Map<int32_t, ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory> * m,
                          int32_t key, ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_iter(
      const google::protobuf::Map<int32_t, ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory>* __rust_proto_thunk__Map_u32_webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_new() {
    return new google::protobuf::Map<uint32_t, ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory>();
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_free(const google::protobuf::Map<uint32_t, ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_clear(google::protobuf::Map<uint32_t, ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_size(const google::protobuf::Map<uint32_t, ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_insert(google::protobuf::Map<uint32_t, ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory> * m,
                          uint32_t key, ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_get(const google::protobuf::Map<uint32_t, ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory>* m,
                       uint32_t key, const ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_remove(google::protobuf::Map<uint32_t, ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory> * m,
                          uint32_t key, ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_iter(
      const google::protobuf::Map<uint32_t, ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory>* __rust_proto_thunk__Map_i64_webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_new() {
    return new google::protobuf::Map<int64_t, ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory>();
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_free(const google::protobuf::Map<int64_t, ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_clear(google::protobuf::Map<int64_t, ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_size(const google::protobuf::Map<int64_t, ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_insert(google::protobuf::Map<int64_t, ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory> * m,
                          int64_t key, ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_get(const google::protobuf::Map<int64_t, ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory>* m,
                       int64_t key, const ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_remove(google::protobuf::Map<int64_t, ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory> * m,
                          int64_t key, ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_iter(
      const google::protobuf::Map<int64_t, ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory>* __rust_proto_thunk__Map_u64_webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_new() {
    return new google::protobuf::Map<uint64_t, ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory>();
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_free(const google::protobuf::Map<uint64_t, ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_clear(google::protobuf::Map<uint64_t, ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_size(const google::protobuf::Map<uint64_t, ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_insert(google::protobuf::Map<uint64_t, ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory> * m,
                          uint64_t key, ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_get(const google::protobuf::Map<uint64_t, ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory>* m,
                       uint64_t key, const ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_remove(google::protobuf::Map<uint64_t, ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory> * m,
                          uint64_t key, ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_iter(
      const google::protobuf::Map<uint64_t, ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory>* __rust_proto_thunk__Map_bool_webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_new() {
    return new google::protobuf::Map<bool, ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory>();
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_free(const google::protobuf::Map<bool, ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_clear(google::protobuf::Map<bool, ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_size(const google::protobuf::Map<bool, ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_insert(google::protobuf::Map<bool, ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory> * m,
                          bool key, ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_get(const google::protobuf::Map<bool, ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory>* m,
                       bool key, const ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_remove(google::protobuf::Map<bool, ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory> * m,
                          bool key, ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_iter(
      const google::protobuf::Map<bool, ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory>* __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_new() {
    return new google::protobuf::Map<std::string, ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory>();
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_free(const google::protobuf::Map<std::string, ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_clear(google::protobuf::Map<std::string, ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_size(const google::protobuf::Map<std::string, ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_insert(google::protobuf::Map<std::string, ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_get(const google::protobuf::Map<std::string, ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_remove(google::protobuf::Map<std::string, ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_iter(
      const google::protobuf::Map<std::string, ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListDeliveryHistoryResponse_DeliveryHistory_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::webhook_api::v1::ListDeliveryHistoryResponse_DeliveryHistory>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int32_t, ::webhook_api::v1::ListDeliveryHistoryResponse>* __rust_proto_thunk__Map_i32_webhook_api_v1_ListDeliveryHistoryResponse_new() {
    return new google::protobuf::Map<int32_t, ::webhook_api::v1::ListDeliveryHistoryResponse>();
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_ListDeliveryHistoryResponse_free(const google::protobuf::Map<int32_t, ::webhook_api::v1::ListDeliveryHistoryResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_ListDeliveryHistoryResponse_clear(google::protobuf::Map<int32_t, ::webhook_api::v1::ListDeliveryHistoryResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_webhook_api_v1_ListDeliveryHistoryResponse_size(const google::protobuf::Map<int32_t, ::webhook_api::v1::ListDeliveryHistoryResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_ListDeliveryHistoryResponse_insert(google::protobuf::Map<int32_t, ::webhook_api::v1::ListDeliveryHistoryResponse> * m,
                          int32_t key, ::webhook_api::v1::ListDeliveryHistoryResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_ListDeliveryHistoryResponse_get(const google::protobuf::Map<int32_t, ::webhook_api::v1::ListDeliveryHistoryResponse>* m,
                       int32_t key, const ::webhook_api::v1::ListDeliveryHistoryResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ListDeliveryHistoryResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_ListDeliveryHistoryResponse_remove(google::protobuf::Map<int32_t, ::webhook_api::v1::ListDeliveryHistoryResponse> * m,
                          int32_t key, ::webhook_api::v1::ListDeliveryHistoryResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_webhook_api_v1_ListDeliveryHistoryResponse_iter(
      const google::protobuf::Map<int32_t, ::webhook_api::v1::ListDeliveryHistoryResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_ListDeliveryHistoryResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::webhook_api::v1::ListDeliveryHistoryResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::webhook_api::v1::ListDeliveryHistoryResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::webhook_api::v1::ListDeliveryHistoryResponse>* __rust_proto_thunk__Map_u32_webhook_api_v1_ListDeliveryHistoryResponse_new() {
    return new google::protobuf::Map<uint32_t, ::webhook_api::v1::ListDeliveryHistoryResponse>();
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_ListDeliveryHistoryResponse_free(const google::protobuf::Map<uint32_t, ::webhook_api::v1::ListDeliveryHistoryResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_ListDeliveryHistoryResponse_clear(google::protobuf::Map<uint32_t, ::webhook_api::v1::ListDeliveryHistoryResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_webhook_api_v1_ListDeliveryHistoryResponse_size(const google::protobuf::Map<uint32_t, ::webhook_api::v1::ListDeliveryHistoryResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_ListDeliveryHistoryResponse_insert(google::protobuf::Map<uint32_t, ::webhook_api::v1::ListDeliveryHistoryResponse> * m,
                          uint32_t key, ::webhook_api::v1::ListDeliveryHistoryResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_ListDeliveryHistoryResponse_get(const google::protobuf::Map<uint32_t, ::webhook_api::v1::ListDeliveryHistoryResponse>* m,
                       uint32_t key, const ::webhook_api::v1::ListDeliveryHistoryResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ListDeliveryHistoryResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_ListDeliveryHistoryResponse_remove(google::protobuf::Map<uint32_t, ::webhook_api::v1::ListDeliveryHistoryResponse> * m,
                          uint32_t key, ::webhook_api::v1::ListDeliveryHistoryResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_webhook_api_v1_ListDeliveryHistoryResponse_iter(
      const google::protobuf::Map<uint32_t, ::webhook_api::v1::ListDeliveryHistoryResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_ListDeliveryHistoryResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::webhook_api::v1::ListDeliveryHistoryResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::webhook_api::v1::ListDeliveryHistoryResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::webhook_api::v1::ListDeliveryHistoryResponse>* __rust_proto_thunk__Map_i64_webhook_api_v1_ListDeliveryHistoryResponse_new() {
    return new google::protobuf::Map<int64_t, ::webhook_api::v1::ListDeliveryHistoryResponse>();
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_ListDeliveryHistoryResponse_free(const google::protobuf::Map<int64_t, ::webhook_api::v1::ListDeliveryHistoryResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_ListDeliveryHistoryResponse_clear(google::protobuf::Map<int64_t, ::webhook_api::v1::ListDeliveryHistoryResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_webhook_api_v1_ListDeliveryHistoryResponse_size(const google::protobuf::Map<int64_t, ::webhook_api::v1::ListDeliveryHistoryResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_ListDeliveryHistoryResponse_insert(google::protobuf::Map<int64_t, ::webhook_api::v1::ListDeliveryHistoryResponse> * m,
                          int64_t key, ::webhook_api::v1::ListDeliveryHistoryResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_ListDeliveryHistoryResponse_get(const google::protobuf::Map<int64_t, ::webhook_api::v1::ListDeliveryHistoryResponse>* m,
                       int64_t key, const ::webhook_api::v1::ListDeliveryHistoryResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ListDeliveryHistoryResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_ListDeliveryHistoryResponse_remove(google::protobuf::Map<int64_t, ::webhook_api::v1::ListDeliveryHistoryResponse> * m,
                          int64_t key, ::webhook_api::v1::ListDeliveryHistoryResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_webhook_api_v1_ListDeliveryHistoryResponse_iter(
      const google::protobuf::Map<int64_t, ::webhook_api::v1::ListDeliveryHistoryResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_ListDeliveryHistoryResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::webhook_api::v1::ListDeliveryHistoryResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::webhook_api::v1::ListDeliveryHistoryResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::webhook_api::v1::ListDeliveryHistoryResponse>* __rust_proto_thunk__Map_u64_webhook_api_v1_ListDeliveryHistoryResponse_new() {
    return new google::protobuf::Map<uint64_t, ::webhook_api::v1::ListDeliveryHistoryResponse>();
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_ListDeliveryHistoryResponse_free(const google::protobuf::Map<uint64_t, ::webhook_api::v1::ListDeliveryHistoryResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_ListDeliveryHistoryResponse_clear(google::protobuf::Map<uint64_t, ::webhook_api::v1::ListDeliveryHistoryResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_webhook_api_v1_ListDeliveryHistoryResponse_size(const google::protobuf::Map<uint64_t, ::webhook_api::v1::ListDeliveryHistoryResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_ListDeliveryHistoryResponse_insert(google::protobuf::Map<uint64_t, ::webhook_api::v1::ListDeliveryHistoryResponse> * m,
                          uint64_t key, ::webhook_api::v1::ListDeliveryHistoryResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_ListDeliveryHistoryResponse_get(const google::protobuf::Map<uint64_t, ::webhook_api::v1::ListDeliveryHistoryResponse>* m,
                       uint64_t key, const ::webhook_api::v1::ListDeliveryHistoryResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ListDeliveryHistoryResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_ListDeliveryHistoryResponse_remove(google::protobuf::Map<uint64_t, ::webhook_api::v1::ListDeliveryHistoryResponse> * m,
                          uint64_t key, ::webhook_api::v1::ListDeliveryHistoryResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_webhook_api_v1_ListDeliveryHistoryResponse_iter(
      const google::protobuf::Map<uint64_t, ::webhook_api::v1::ListDeliveryHistoryResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_ListDeliveryHistoryResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::webhook_api::v1::ListDeliveryHistoryResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::webhook_api::v1::ListDeliveryHistoryResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::webhook_api::v1::ListDeliveryHistoryResponse>* __rust_proto_thunk__Map_bool_webhook_api_v1_ListDeliveryHistoryResponse_new() {
    return new google::protobuf::Map<bool, ::webhook_api::v1::ListDeliveryHistoryResponse>();
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_ListDeliveryHistoryResponse_free(const google::protobuf::Map<bool, ::webhook_api::v1::ListDeliveryHistoryResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_ListDeliveryHistoryResponse_clear(google::protobuf::Map<bool, ::webhook_api::v1::ListDeliveryHistoryResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_webhook_api_v1_ListDeliveryHistoryResponse_size(const google::protobuf::Map<bool, ::webhook_api::v1::ListDeliveryHistoryResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_ListDeliveryHistoryResponse_insert(google::protobuf::Map<bool, ::webhook_api::v1::ListDeliveryHistoryResponse> * m,
                          bool key, ::webhook_api::v1::ListDeliveryHistoryResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_ListDeliveryHistoryResponse_get(const google::protobuf::Map<bool, ::webhook_api::v1::ListDeliveryHistoryResponse>* m,
                       bool key, const ::webhook_api::v1::ListDeliveryHistoryResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ListDeliveryHistoryResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_ListDeliveryHistoryResponse_remove(google::protobuf::Map<bool, ::webhook_api::v1::ListDeliveryHistoryResponse> * m,
                          bool key, ::webhook_api::v1::ListDeliveryHistoryResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_webhook_api_v1_ListDeliveryHistoryResponse_iter(
      const google::protobuf::Map<bool, ::webhook_api::v1::ListDeliveryHistoryResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_ListDeliveryHistoryResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::webhook_api::v1::ListDeliveryHistoryResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::webhook_api::v1::ListDeliveryHistoryResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::webhook_api::v1::ListDeliveryHistoryResponse>* __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListDeliveryHistoryResponse_new() {
    return new google::protobuf::Map<std::string, ::webhook_api::v1::ListDeliveryHistoryResponse>();
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListDeliveryHistoryResponse_free(const google::protobuf::Map<std::string, ::webhook_api::v1::ListDeliveryHistoryResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListDeliveryHistoryResponse_clear(google::protobuf::Map<std::string, ::webhook_api::v1::ListDeliveryHistoryResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListDeliveryHistoryResponse_size(const google::protobuf::Map<std::string, ::webhook_api::v1::ListDeliveryHistoryResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListDeliveryHistoryResponse_insert(google::protobuf::Map<std::string, ::webhook_api::v1::ListDeliveryHistoryResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::ListDeliveryHistoryResponse value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListDeliveryHistoryResponse_get(const google::protobuf::Map<std::string, ::webhook_api::v1::ListDeliveryHistoryResponse>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::webhook_api::v1::ListDeliveryHistoryResponse** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ListDeliveryHistoryResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListDeliveryHistoryResponse_remove(google::protobuf::Map<std::string, ::webhook_api::v1::ListDeliveryHistoryResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::ListDeliveryHistoryResponse * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListDeliveryHistoryResponse_iter(
      const google::protobuf::Map<std::string, ::webhook_api::v1::ListDeliveryHistoryResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListDeliveryHistoryResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::webhook_api::v1::ListDeliveryHistoryResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::webhook_api::v1::ListDeliveryHistoryResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// webhook_api.v1.ScheduledReaction
        // clang-format off
extern "C" {
void* __rust_proto_thunk__webhook_api_v1_ScheduledReaction_new() { return new ::webhook_api::v1::ScheduledReaction(); }
void __rust_proto_thunk__webhook_api_v1_ScheduledReaction_delete(void* ptr) { delete static_cast<::webhook_api::v1::ScheduledReaction*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__webhook_api_v1_ScheduledReaction_serialize(::webhook_api::v1::ScheduledReaction* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__webhook_api_v1_ScheduledReaction_parse(::webhook_api::v1::ScheduledReaction* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__webhook_api_v1_ScheduledReaction_copy_from(::webhook_api::v1::ScheduledReaction* dst, const ::webhook_api::v1::ScheduledReaction* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__webhook_api_v1_ScheduledReaction_repeated_len(google::protobuf::RepeatedPtrField<::webhook_api::v1::ScheduledReaction>* field) {
  return field->size();
}
const ::webhook_api::v1::ScheduledReaction& __rust_proto_thunk__webhook_api_v1_ScheduledReaction_repeated_get(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::ScheduledReaction>* field,
  size_t index) {
  return field->Get(index);
}
::webhook_api::v1::ScheduledReaction* __rust_proto_thunk__webhook_api_v1_ScheduledReaction_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::ScheduledReaction>* field,
  size_t index) {
  return field->Mutable(index);
}
::webhook_api::v1::ScheduledReaction* __rust_proto_thunk__webhook_api_v1_ScheduledReaction_repeated_add(google::protobuf::RepeatedPtrField<::webhook_api::v1::ScheduledReaction>* field) {
  return field->Add();
}
void __rust_proto_thunk__webhook_api_v1_ScheduledReaction_repeated_clear(google::protobuf::RepeatedPtrField<::webhook_api::v1::ScheduledReaction>* field) {
  field->Clear();
}
void __rust_proto_thunk__webhook_api_v1_ScheduledReaction_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::ScheduledReaction>& dst,
  const google::protobuf::RepeatedPtrField<::webhook_api::v1::ScheduledReaction>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__webhook_api_v1_ScheduledReaction_get_id(::webhook_api::v1::ScheduledReaction* msg) {
  absl::string_view val = msg->id();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__webhook_api_v1_ScheduledReaction_set_id(::webhook_api::v1::ScheduledReaction* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_id(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__webhook_api_v1_ScheduledReaction_get_name(::webhook_api::v1::ScheduledReaction* msg) {
  absl::string_view val = msg->name();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__webhook_api_v1_ScheduledReaction_set_name(::webhook_api::v1::ScheduledReaction* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_name(absl::string_view(s.ptr, s.len));
}

::webhook_api::v1::ScheduleReactionType __rust_proto_thunk__webhook_api_v1_ScheduledReaction_get_reaction_type(::webhook_api::v1::ScheduledReaction* msg) { return msg->reaction_type(); }
void __rust_proto_thunk__webhook_api_v1_ScheduledReaction_set_reaction_type(::webhook_api::v1::ScheduledReaction* msg, ::webhook_api::v1::ScheduleReactionType val) {
  msg->set_reaction_type(val);
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__webhook_api_v1_ScheduledReaction_get_cron_tab(::webhook_api::v1::ScheduledReaction* msg) {
  absl::string_view val = msg->cron_tab();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__webhook_api_v1_ScheduledReaction_set_cron_tab(::webhook_api::v1::ScheduledReaction* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_cron_tab(absl::string_view(s.ptr, s.len));
}

const void* __rust_proto_thunk__webhook_api_v1_ScheduledReaction_get_create_contract_payload(::webhook_api::v1::ScheduledReaction* msg) {
  return static_cast<const void*>(&msg->create_contract_payload());
}
void* __rust_proto_thunk__webhook_api_v1_ScheduledReaction_get_mut_create_contract_payload(::webhook_api::v1::ScheduledReaction* msg) {
  return static_cast<void*>(msg->mutable_create_contract_payload());
}
void __rust_proto_thunk__webhook_api_v1_ScheduledReaction_clear_create_contract_payload(::webhook_api::v1::ScheduledReaction* msg) { msg->clear_create_contract_payload(); }
bool __rust_proto_thunk__webhook_api_v1_ScheduledReaction_has_create_contract_payload(::webhook_api::v1::ScheduledReaction* msg) { return msg->has_create_contract_payload(); }


::webhook_api::v1::ScheduledReaction::PayloadCase __rust_proto_thunk__webhook_api_v1_ScheduledReaction_case_payload(::webhook_api::v1::ScheduledReaction* msg) {
  return msg->payload_case();
}
}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::webhook_api::v1::ScheduledReaction>* __rust_proto_thunk__Map_i32_webhook_api_v1_ScheduledReaction_new() {
    return new google::protobuf::Map<int32_t, ::webhook_api::v1::ScheduledReaction>();
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_ScheduledReaction_free(const google::protobuf::Map<int32_t, ::webhook_api::v1::ScheduledReaction>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_ScheduledReaction_clear(google::protobuf::Map<int32_t, ::webhook_api::v1::ScheduledReaction> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_webhook_api_v1_ScheduledReaction_size(const google::protobuf::Map<int32_t, ::webhook_api::v1::ScheduledReaction>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_ScheduledReaction_insert(google::protobuf::Map<int32_t, ::webhook_api::v1::ScheduledReaction> * m,
                          int32_t key, ::webhook_api::v1::ScheduledReaction value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_ScheduledReaction_get(const google::protobuf::Map<int32_t, ::webhook_api::v1::ScheduledReaction>* m,
                       int32_t key, const ::webhook_api::v1::ScheduledReaction** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ScheduledReaction* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_ScheduledReaction_remove(google::protobuf::Map<int32_t, ::webhook_api::v1::ScheduledReaction> * m,
                          int32_t key, ::webhook_api::v1::ScheduledReaction * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_webhook_api_v1_ScheduledReaction_iter(
      const google::protobuf::Map<int32_t, ::webhook_api::v1::ScheduledReaction>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_ScheduledReaction_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::webhook_api::v1::ScheduledReaction** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::webhook_api::v1::ScheduledReaction>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::webhook_api::v1::ScheduledReaction>* __rust_proto_thunk__Map_u32_webhook_api_v1_ScheduledReaction_new() {
    return new google::protobuf::Map<uint32_t, ::webhook_api::v1::ScheduledReaction>();
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_ScheduledReaction_free(const google::protobuf::Map<uint32_t, ::webhook_api::v1::ScheduledReaction>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_ScheduledReaction_clear(google::protobuf::Map<uint32_t, ::webhook_api::v1::ScheduledReaction> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_webhook_api_v1_ScheduledReaction_size(const google::protobuf::Map<uint32_t, ::webhook_api::v1::ScheduledReaction>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_ScheduledReaction_insert(google::protobuf::Map<uint32_t, ::webhook_api::v1::ScheduledReaction> * m,
                          uint32_t key, ::webhook_api::v1::ScheduledReaction value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_ScheduledReaction_get(const google::protobuf::Map<uint32_t, ::webhook_api::v1::ScheduledReaction>* m,
                       uint32_t key, const ::webhook_api::v1::ScheduledReaction** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ScheduledReaction* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_ScheduledReaction_remove(google::protobuf::Map<uint32_t, ::webhook_api::v1::ScheduledReaction> * m,
                          uint32_t key, ::webhook_api::v1::ScheduledReaction * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_webhook_api_v1_ScheduledReaction_iter(
      const google::protobuf::Map<uint32_t, ::webhook_api::v1::ScheduledReaction>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_ScheduledReaction_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::webhook_api::v1::ScheduledReaction** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::webhook_api::v1::ScheduledReaction>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::webhook_api::v1::ScheduledReaction>* __rust_proto_thunk__Map_i64_webhook_api_v1_ScheduledReaction_new() {
    return new google::protobuf::Map<int64_t, ::webhook_api::v1::ScheduledReaction>();
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_ScheduledReaction_free(const google::protobuf::Map<int64_t, ::webhook_api::v1::ScheduledReaction>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_ScheduledReaction_clear(google::protobuf::Map<int64_t, ::webhook_api::v1::ScheduledReaction> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_webhook_api_v1_ScheduledReaction_size(const google::protobuf::Map<int64_t, ::webhook_api::v1::ScheduledReaction>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_ScheduledReaction_insert(google::protobuf::Map<int64_t, ::webhook_api::v1::ScheduledReaction> * m,
                          int64_t key, ::webhook_api::v1::ScheduledReaction value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_ScheduledReaction_get(const google::protobuf::Map<int64_t, ::webhook_api::v1::ScheduledReaction>* m,
                       int64_t key, const ::webhook_api::v1::ScheduledReaction** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ScheduledReaction* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_ScheduledReaction_remove(google::protobuf::Map<int64_t, ::webhook_api::v1::ScheduledReaction> * m,
                          int64_t key, ::webhook_api::v1::ScheduledReaction * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_webhook_api_v1_ScheduledReaction_iter(
      const google::protobuf::Map<int64_t, ::webhook_api::v1::ScheduledReaction>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_ScheduledReaction_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::webhook_api::v1::ScheduledReaction** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::webhook_api::v1::ScheduledReaction>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::webhook_api::v1::ScheduledReaction>* __rust_proto_thunk__Map_u64_webhook_api_v1_ScheduledReaction_new() {
    return new google::protobuf::Map<uint64_t, ::webhook_api::v1::ScheduledReaction>();
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_ScheduledReaction_free(const google::protobuf::Map<uint64_t, ::webhook_api::v1::ScheduledReaction>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_ScheduledReaction_clear(google::protobuf::Map<uint64_t, ::webhook_api::v1::ScheduledReaction> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_webhook_api_v1_ScheduledReaction_size(const google::protobuf::Map<uint64_t, ::webhook_api::v1::ScheduledReaction>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_ScheduledReaction_insert(google::protobuf::Map<uint64_t, ::webhook_api::v1::ScheduledReaction> * m,
                          uint64_t key, ::webhook_api::v1::ScheduledReaction value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_ScheduledReaction_get(const google::protobuf::Map<uint64_t, ::webhook_api::v1::ScheduledReaction>* m,
                       uint64_t key, const ::webhook_api::v1::ScheduledReaction** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ScheduledReaction* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_ScheduledReaction_remove(google::protobuf::Map<uint64_t, ::webhook_api::v1::ScheduledReaction> * m,
                          uint64_t key, ::webhook_api::v1::ScheduledReaction * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_webhook_api_v1_ScheduledReaction_iter(
      const google::protobuf::Map<uint64_t, ::webhook_api::v1::ScheduledReaction>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_ScheduledReaction_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::webhook_api::v1::ScheduledReaction** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::webhook_api::v1::ScheduledReaction>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::webhook_api::v1::ScheduledReaction>* __rust_proto_thunk__Map_bool_webhook_api_v1_ScheduledReaction_new() {
    return new google::protobuf::Map<bool, ::webhook_api::v1::ScheduledReaction>();
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_ScheduledReaction_free(const google::protobuf::Map<bool, ::webhook_api::v1::ScheduledReaction>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_ScheduledReaction_clear(google::protobuf::Map<bool, ::webhook_api::v1::ScheduledReaction> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_webhook_api_v1_ScheduledReaction_size(const google::protobuf::Map<bool, ::webhook_api::v1::ScheduledReaction>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_ScheduledReaction_insert(google::protobuf::Map<bool, ::webhook_api::v1::ScheduledReaction> * m,
                          bool key, ::webhook_api::v1::ScheduledReaction value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_ScheduledReaction_get(const google::protobuf::Map<bool, ::webhook_api::v1::ScheduledReaction>* m,
                       bool key, const ::webhook_api::v1::ScheduledReaction** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ScheduledReaction* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_ScheduledReaction_remove(google::protobuf::Map<bool, ::webhook_api::v1::ScheduledReaction> * m,
                          bool key, ::webhook_api::v1::ScheduledReaction * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_webhook_api_v1_ScheduledReaction_iter(
      const google::protobuf::Map<bool, ::webhook_api::v1::ScheduledReaction>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_ScheduledReaction_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::webhook_api::v1::ScheduledReaction** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::webhook_api::v1::ScheduledReaction>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::webhook_api::v1::ScheduledReaction>* __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ScheduledReaction_new() {
    return new google::protobuf::Map<std::string, ::webhook_api::v1::ScheduledReaction>();
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ScheduledReaction_free(const google::protobuf::Map<std::string, ::webhook_api::v1::ScheduledReaction>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ScheduledReaction_clear(google::protobuf::Map<std::string, ::webhook_api::v1::ScheduledReaction> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ScheduledReaction_size(const google::protobuf::Map<std::string, ::webhook_api::v1::ScheduledReaction>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ScheduledReaction_insert(google::protobuf::Map<std::string, ::webhook_api::v1::ScheduledReaction> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::ScheduledReaction value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ScheduledReaction_get(const google::protobuf::Map<std::string, ::webhook_api::v1::ScheduledReaction>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::webhook_api::v1::ScheduledReaction** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ScheduledReaction* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ScheduledReaction_remove(google::protobuf::Map<std::string, ::webhook_api::v1::ScheduledReaction> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::ScheduledReaction * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ScheduledReaction_iter(
      const google::protobuf::Map<std::string, ::webhook_api::v1::ScheduledReaction>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ScheduledReaction_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::webhook_api::v1::ScheduledReaction** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::webhook_api::v1::ScheduledReaction>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// webhook_api.v1.CreateContractReactionPayload
        // clang-format off
extern "C" {
void* __rust_proto_thunk__webhook_api_v1_CreateContractReactionPayload_new() { return new ::webhook_api::v1::CreateContractReactionPayload(); }
void __rust_proto_thunk__webhook_api_v1_CreateContractReactionPayload_delete(void* ptr) { delete static_cast<::webhook_api::v1::CreateContractReactionPayload*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__webhook_api_v1_CreateContractReactionPayload_serialize(::webhook_api::v1::CreateContractReactionPayload* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__webhook_api_v1_CreateContractReactionPayload_parse(::webhook_api::v1::CreateContractReactionPayload* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__webhook_api_v1_CreateContractReactionPayload_copy_from(::webhook_api::v1::CreateContractReactionPayload* dst, const ::webhook_api::v1::CreateContractReactionPayload* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__webhook_api_v1_CreateContractReactionPayload_repeated_len(google::protobuf::RepeatedPtrField<::webhook_api::v1::CreateContractReactionPayload>* field) {
  return field->size();
}
const ::webhook_api::v1::CreateContractReactionPayload& __rust_proto_thunk__webhook_api_v1_CreateContractReactionPayload_repeated_get(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::CreateContractReactionPayload>* field,
  size_t index) {
  return field->Get(index);
}
::webhook_api::v1::CreateContractReactionPayload* __rust_proto_thunk__webhook_api_v1_CreateContractReactionPayload_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::CreateContractReactionPayload>* field,
  size_t index) {
  return field->Mutable(index);
}
::webhook_api::v1::CreateContractReactionPayload* __rust_proto_thunk__webhook_api_v1_CreateContractReactionPayload_repeated_add(google::protobuf::RepeatedPtrField<::webhook_api::v1::CreateContractReactionPayload>* field) {
  return field->Add();
}
void __rust_proto_thunk__webhook_api_v1_CreateContractReactionPayload_repeated_clear(google::protobuf::RepeatedPtrField<::webhook_api::v1::CreateContractReactionPayload>* field) {
  field->Clear();
}
void __rust_proto_thunk__webhook_api_v1_CreateContractReactionPayload_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::CreateContractReactionPayload>& dst,
  const google::protobuf::RepeatedPtrField<::webhook_api::v1::CreateContractReactionPayload>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__webhook_api_v1_CreateContractReactionPayload_get_sender_wallet_id(::webhook_api::v1::CreateContractReactionPayload* msg) {
  absl::string_view val = msg->sender_wallet_id();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__webhook_api_v1_CreateContractReactionPayload_set_sender_wallet_id(::webhook_api::v1::CreateContractReactionPayload* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_sender_wallet_id(absl::string_view(s.ptr, s.len));
}

void __rust_proto_thunk__webhook_api_v1_CreateContractReactionPayload_clear_commitments(::webhook_api::v1::CreateContractReactionPayload* msg) {
  msg->clear_commitments();
}
google::protobuf::RepeatedPtrField<::common::Commitment>* __rust_proto_thunk__webhook_api_v1_CreateContractReactionPayload_get_mut_commitments(::webhook_api::v1::CreateContractReactionPayload* msg) {
  return msg->mutable_commitments();
}
const google::protobuf::RepeatedPtrField<::common::Commitment>* __rust_proto_thunk__webhook_api_v1_CreateContractReactionPayload_get_commitments(
    const ::webhook_api::v1::CreateContractReactionPayload* msg) {
  return &msg->commitments();
}

void __rust_proto_thunk__webhook_api_v1_CreateContractReactionPayload_clear_conditions(::webhook_api::v1::CreateContractReactionPayload* msg) {
  msg->clear_conditions();
}
google::protobuf::RepeatedPtrField<::common::Condition>* __rust_proto_thunk__webhook_api_v1_CreateContractReactionPayload_get_mut_conditions(::webhook_api::v1::CreateContractReactionPayload* msg) {
  return msg->mutable_conditions();
}
const google::protobuf::RepeatedPtrField<::common::Condition>* __rust_proto_thunk__webhook_api_v1_CreateContractReactionPayload_get_conditions(
    const ::webhook_api::v1::CreateContractReactionPayload* msg) {
  return &msg->conditions();
}

::uint32_t __rust_proto_thunk__webhook_api_v1_CreateContractReactionPayload_get_timeout_secs(::webhook_api::v1::CreateContractReactionPayload* msg) { return msg->timeout_secs(); }
void __rust_proto_thunk__webhook_api_v1_CreateContractReactionPayload_set_timeout_secs(::webhook_api::v1::CreateContractReactionPayload* msg, ::uint32_t val) {
  msg->set_timeout_secs(val);
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__webhook_api_v1_CreateContractReactionPayload_get_memo(::webhook_api::v1::CreateContractReactionPayload* msg) {
  absl::string_view val = msg->memo();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__webhook_api_v1_CreateContractReactionPayload_set_memo(::webhook_api::v1::CreateContractReactionPayload* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_memo(absl::string_view(s.ptr, s.len));
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::webhook_api::v1::CreateContractReactionPayload>* __rust_proto_thunk__Map_i32_webhook_api_v1_CreateContractReactionPayload_new() {
    return new google::protobuf::Map<int32_t, ::webhook_api::v1::CreateContractReactionPayload>();
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_CreateContractReactionPayload_free(const google::protobuf::Map<int32_t, ::webhook_api::v1::CreateContractReactionPayload>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_CreateContractReactionPayload_clear(google::protobuf::Map<int32_t, ::webhook_api::v1::CreateContractReactionPayload> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_webhook_api_v1_CreateContractReactionPayload_size(const google::protobuf::Map<int32_t, ::webhook_api::v1::CreateContractReactionPayload>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_CreateContractReactionPayload_insert(google::protobuf::Map<int32_t, ::webhook_api::v1::CreateContractReactionPayload> * m,
                          int32_t key, ::webhook_api::v1::CreateContractReactionPayload value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_CreateContractReactionPayload_get(const google::protobuf::Map<int32_t, ::webhook_api::v1::CreateContractReactionPayload>* m,
                       int32_t key, const ::webhook_api::v1::CreateContractReactionPayload** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::CreateContractReactionPayload* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_CreateContractReactionPayload_remove(google::protobuf::Map<int32_t, ::webhook_api::v1::CreateContractReactionPayload> * m,
                          int32_t key, ::webhook_api::v1::CreateContractReactionPayload * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_webhook_api_v1_CreateContractReactionPayload_iter(
      const google::protobuf::Map<int32_t, ::webhook_api::v1::CreateContractReactionPayload>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_CreateContractReactionPayload_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::webhook_api::v1::CreateContractReactionPayload** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::webhook_api::v1::CreateContractReactionPayload>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::webhook_api::v1::CreateContractReactionPayload>* __rust_proto_thunk__Map_u32_webhook_api_v1_CreateContractReactionPayload_new() {
    return new google::protobuf::Map<uint32_t, ::webhook_api::v1::CreateContractReactionPayload>();
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_CreateContractReactionPayload_free(const google::protobuf::Map<uint32_t, ::webhook_api::v1::CreateContractReactionPayload>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_CreateContractReactionPayload_clear(google::protobuf::Map<uint32_t, ::webhook_api::v1::CreateContractReactionPayload> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_webhook_api_v1_CreateContractReactionPayload_size(const google::protobuf::Map<uint32_t, ::webhook_api::v1::CreateContractReactionPayload>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_CreateContractReactionPayload_insert(google::protobuf::Map<uint32_t, ::webhook_api::v1::CreateContractReactionPayload> * m,
                          uint32_t key, ::webhook_api::v1::CreateContractReactionPayload value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_CreateContractReactionPayload_get(const google::protobuf::Map<uint32_t, ::webhook_api::v1::CreateContractReactionPayload>* m,
                       uint32_t key, const ::webhook_api::v1::CreateContractReactionPayload** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::CreateContractReactionPayload* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_CreateContractReactionPayload_remove(google::protobuf::Map<uint32_t, ::webhook_api::v1::CreateContractReactionPayload> * m,
                          uint32_t key, ::webhook_api::v1::CreateContractReactionPayload * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_webhook_api_v1_CreateContractReactionPayload_iter(
      const google::protobuf::Map<uint32_t, ::webhook_api::v1::CreateContractReactionPayload>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_CreateContractReactionPayload_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::webhook_api::v1::CreateContractReactionPayload** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::webhook_api::v1::CreateContractReactionPayload>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::webhook_api::v1::CreateContractReactionPayload>* __rust_proto_thunk__Map_i64_webhook_api_v1_CreateContractReactionPayload_new() {
    return new google::protobuf::Map<int64_t, ::webhook_api::v1::CreateContractReactionPayload>();
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_CreateContractReactionPayload_free(const google::protobuf::Map<int64_t, ::webhook_api::v1::CreateContractReactionPayload>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_CreateContractReactionPayload_clear(google::protobuf::Map<int64_t, ::webhook_api::v1::CreateContractReactionPayload> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_webhook_api_v1_CreateContractReactionPayload_size(const google::protobuf::Map<int64_t, ::webhook_api::v1::CreateContractReactionPayload>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_CreateContractReactionPayload_insert(google::protobuf::Map<int64_t, ::webhook_api::v1::CreateContractReactionPayload> * m,
                          int64_t key, ::webhook_api::v1::CreateContractReactionPayload value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_CreateContractReactionPayload_get(const google::protobuf::Map<int64_t, ::webhook_api::v1::CreateContractReactionPayload>* m,
                       int64_t key, const ::webhook_api::v1::CreateContractReactionPayload** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::CreateContractReactionPayload* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_CreateContractReactionPayload_remove(google::protobuf::Map<int64_t, ::webhook_api::v1::CreateContractReactionPayload> * m,
                          int64_t key, ::webhook_api::v1::CreateContractReactionPayload * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_webhook_api_v1_CreateContractReactionPayload_iter(
      const google::protobuf::Map<int64_t, ::webhook_api::v1::CreateContractReactionPayload>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_CreateContractReactionPayload_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::webhook_api::v1::CreateContractReactionPayload** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::webhook_api::v1::CreateContractReactionPayload>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::webhook_api::v1::CreateContractReactionPayload>* __rust_proto_thunk__Map_u64_webhook_api_v1_CreateContractReactionPayload_new() {
    return new google::protobuf::Map<uint64_t, ::webhook_api::v1::CreateContractReactionPayload>();
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_CreateContractReactionPayload_free(const google::protobuf::Map<uint64_t, ::webhook_api::v1::CreateContractReactionPayload>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_CreateContractReactionPayload_clear(google::protobuf::Map<uint64_t, ::webhook_api::v1::CreateContractReactionPayload> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_webhook_api_v1_CreateContractReactionPayload_size(const google::protobuf::Map<uint64_t, ::webhook_api::v1::CreateContractReactionPayload>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_CreateContractReactionPayload_insert(google::protobuf::Map<uint64_t, ::webhook_api::v1::CreateContractReactionPayload> * m,
                          uint64_t key, ::webhook_api::v1::CreateContractReactionPayload value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_CreateContractReactionPayload_get(const google::protobuf::Map<uint64_t, ::webhook_api::v1::CreateContractReactionPayload>* m,
                       uint64_t key, const ::webhook_api::v1::CreateContractReactionPayload** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::CreateContractReactionPayload* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_CreateContractReactionPayload_remove(google::protobuf::Map<uint64_t, ::webhook_api::v1::CreateContractReactionPayload> * m,
                          uint64_t key, ::webhook_api::v1::CreateContractReactionPayload * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_webhook_api_v1_CreateContractReactionPayload_iter(
      const google::protobuf::Map<uint64_t, ::webhook_api::v1::CreateContractReactionPayload>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_CreateContractReactionPayload_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::webhook_api::v1::CreateContractReactionPayload** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::webhook_api::v1::CreateContractReactionPayload>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::webhook_api::v1::CreateContractReactionPayload>* __rust_proto_thunk__Map_bool_webhook_api_v1_CreateContractReactionPayload_new() {
    return new google::protobuf::Map<bool, ::webhook_api::v1::CreateContractReactionPayload>();
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_CreateContractReactionPayload_free(const google::protobuf::Map<bool, ::webhook_api::v1::CreateContractReactionPayload>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_CreateContractReactionPayload_clear(google::protobuf::Map<bool, ::webhook_api::v1::CreateContractReactionPayload> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_webhook_api_v1_CreateContractReactionPayload_size(const google::protobuf::Map<bool, ::webhook_api::v1::CreateContractReactionPayload>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_CreateContractReactionPayload_insert(google::protobuf::Map<bool, ::webhook_api::v1::CreateContractReactionPayload> * m,
                          bool key, ::webhook_api::v1::CreateContractReactionPayload value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_CreateContractReactionPayload_get(const google::protobuf::Map<bool, ::webhook_api::v1::CreateContractReactionPayload>* m,
                       bool key, const ::webhook_api::v1::CreateContractReactionPayload** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::CreateContractReactionPayload* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_CreateContractReactionPayload_remove(google::protobuf::Map<bool, ::webhook_api::v1::CreateContractReactionPayload> * m,
                          bool key, ::webhook_api::v1::CreateContractReactionPayload * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_webhook_api_v1_CreateContractReactionPayload_iter(
      const google::protobuf::Map<bool, ::webhook_api::v1::CreateContractReactionPayload>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_CreateContractReactionPayload_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::webhook_api::v1::CreateContractReactionPayload** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::webhook_api::v1::CreateContractReactionPayload>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::webhook_api::v1::CreateContractReactionPayload>* __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_CreateContractReactionPayload_new() {
    return new google::protobuf::Map<std::string, ::webhook_api::v1::CreateContractReactionPayload>();
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_CreateContractReactionPayload_free(const google::protobuf::Map<std::string, ::webhook_api::v1::CreateContractReactionPayload>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_CreateContractReactionPayload_clear(google::protobuf::Map<std::string, ::webhook_api::v1::CreateContractReactionPayload> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_CreateContractReactionPayload_size(const google::protobuf::Map<std::string, ::webhook_api::v1::CreateContractReactionPayload>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_CreateContractReactionPayload_insert(google::protobuf::Map<std::string, ::webhook_api::v1::CreateContractReactionPayload> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::CreateContractReactionPayload value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_CreateContractReactionPayload_get(const google::protobuf::Map<std::string, ::webhook_api::v1::CreateContractReactionPayload>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::webhook_api::v1::CreateContractReactionPayload** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::CreateContractReactionPayload* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_CreateContractReactionPayload_remove(google::protobuf::Map<std::string, ::webhook_api::v1::CreateContractReactionPayload> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::CreateContractReactionPayload * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_CreateContractReactionPayload_iter(
      const google::protobuf::Map<std::string, ::webhook_api::v1::CreateContractReactionPayload>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_CreateContractReactionPayload_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::webhook_api::v1::CreateContractReactionPayload** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::webhook_api::v1::CreateContractReactionPayload>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// webhook_api.v1.CreateScheduledReactionRequest
        // clang-format off
extern "C" {
void* __rust_proto_thunk__webhook_api_v1_CreateScheduledReactionRequest_new() { return new ::webhook_api::v1::CreateScheduledReactionRequest(); }
void __rust_proto_thunk__webhook_api_v1_CreateScheduledReactionRequest_delete(void* ptr) { delete static_cast<::webhook_api::v1::CreateScheduledReactionRequest*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__webhook_api_v1_CreateScheduledReactionRequest_serialize(::webhook_api::v1::CreateScheduledReactionRequest* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__webhook_api_v1_CreateScheduledReactionRequest_parse(::webhook_api::v1::CreateScheduledReactionRequest* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__webhook_api_v1_CreateScheduledReactionRequest_copy_from(::webhook_api::v1::CreateScheduledReactionRequest* dst, const ::webhook_api::v1::CreateScheduledReactionRequest* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__webhook_api_v1_CreateScheduledReactionRequest_repeated_len(google::protobuf::RepeatedPtrField<::webhook_api::v1::CreateScheduledReactionRequest>* field) {
  return field->size();
}
const ::webhook_api::v1::CreateScheduledReactionRequest& __rust_proto_thunk__webhook_api_v1_CreateScheduledReactionRequest_repeated_get(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::CreateScheduledReactionRequest>* field,
  size_t index) {
  return field->Get(index);
}
::webhook_api::v1::CreateScheduledReactionRequest* __rust_proto_thunk__webhook_api_v1_CreateScheduledReactionRequest_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::CreateScheduledReactionRequest>* field,
  size_t index) {
  return field->Mutable(index);
}
::webhook_api::v1::CreateScheduledReactionRequest* __rust_proto_thunk__webhook_api_v1_CreateScheduledReactionRequest_repeated_add(google::protobuf::RepeatedPtrField<::webhook_api::v1::CreateScheduledReactionRequest>* field) {
  return field->Add();
}
void __rust_proto_thunk__webhook_api_v1_CreateScheduledReactionRequest_repeated_clear(google::protobuf::RepeatedPtrField<::webhook_api::v1::CreateScheduledReactionRequest>* field) {
  field->Clear();
}
void __rust_proto_thunk__webhook_api_v1_CreateScheduledReactionRequest_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::CreateScheduledReactionRequest>& dst,
  const google::protobuf::RepeatedPtrField<::webhook_api::v1::CreateScheduledReactionRequest>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__webhook_api_v1_CreateScheduledReactionRequest_get_api_key(::webhook_api::v1::CreateScheduledReactionRequest* msg) {
  absl::string_view val = msg->api_key();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__webhook_api_v1_CreateScheduledReactionRequest_set_api_key(::webhook_api::v1::CreateScheduledReactionRequest* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_api_key(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__webhook_api_v1_CreateScheduledReactionRequest_get_name(::webhook_api::v1::CreateScheduledReactionRequest* msg) {
  absl::string_view val = msg->name();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__webhook_api_v1_CreateScheduledReactionRequest_set_name(::webhook_api::v1::CreateScheduledReactionRequest* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_name(absl::string_view(s.ptr, s.len));
}

::webhook_api::v1::ScheduleReactionType __rust_proto_thunk__webhook_api_v1_CreateScheduledReactionRequest_get_reaction_type(::webhook_api::v1::CreateScheduledReactionRequest* msg) { return msg->reaction_type(); }
void __rust_proto_thunk__webhook_api_v1_CreateScheduledReactionRequest_set_reaction_type(::webhook_api::v1::CreateScheduledReactionRequest* msg, ::webhook_api::v1::ScheduleReactionType val) {
  msg->set_reaction_type(val);
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__webhook_api_v1_CreateScheduledReactionRequest_get_cron_tab(::webhook_api::v1::CreateScheduledReactionRequest* msg) {
  absl::string_view val = msg->cron_tab();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__webhook_api_v1_CreateScheduledReactionRequest_set_cron_tab(::webhook_api::v1::CreateScheduledReactionRequest* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_cron_tab(absl::string_view(s.ptr, s.len));
}

const void* __rust_proto_thunk__webhook_api_v1_CreateScheduledReactionRequest_get_create_contract_payload(::webhook_api::v1::CreateScheduledReactionRequest* msg) {
  return static_cast<const void*>(&msg->create_contract_payload());
}
void* __rust_proto_thunk__webhook_api_v1_CreateScheduledReactionRequest_get_mut_create_contract_payload(::webhook_api::v1::CreateScheduledReactionRequest* msg) {
  return static_cast<void*>(msg->mutable_create_contract_payload());
}
void __rust_proto_thunk__webhook_api_v1_CreateScheduledReactionRequest_clear_create_contract_payload(::webhook_api::v1::CreateScheduledReactionRequest* msg) { msg->clear_create_contract_payload(); }
bool __rust_proto_thunk__webhook_api_v1_CreateScheduledReactionRequest_has_create_contract_payload(::webhook_api::v1::CreateScheduledReactionRequest* msg) { return msg->has_create_contract_payload(); }


::webhook_api::v1::CreateScheduledReactionRequest::ReactionPayloadCase __rust_proto_thunk__webhook_api_v1_CreateScheduledReactionRequest_case_reaction_payload(::webhook_api::v1::CreateScheduledReactionRequest* msg) {
  return msg->reaction_payload_case();
}
}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::webhook_api::v1::CreateScheduledReactionRequest>* __rust_proto_thunk__Map_i32_webhook_api_v1_CreateScheduledReactionRequest_new() {
    return new google::protobuf::Map<int32_t, ::webhook_api::v1::CreateScheduledReactionRequest>();
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_CreateScheduledReactionRequest_free(const google::protobuf::Map<int32_t, ::webhook_api::v1::CreateScheduledReactionRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_CreateScheduledReactionRequest_clear(google::protobuf::Map<int32_t, ::webhook_api::v1::CreateScheduledReactionRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_webhook_api_v1_CreateScheduledReactionRequest_size(const google::protobuf::Map<int32_t, ::webhook_api::v1::CreateScheduledReactionRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_CreateScheduledReactionRequest_insert(google::protobuf::Map<int32_t, ::webhook_api::v1::CreateScheduledReactionRequest> * m,
                          int32_t key, ::webhook_api::v1::CreateScheduledReactionRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_CreateScheduledReactionRequest_get(const google::protobuf::Map<int32_t, ::webhook_api::v1::CreateScheduledReactionRequest>* m,
                       int32_t key, const ::webhook_api::v1::CreateScheduledReactionRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::CreateScheduledReactionRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_CreateScheduledReactionRequest_remove(google::protobuf::Map<int32_t, ::webhook_api::v1::CreateScheduledReactionRequest> * m,
                          int32_t key, ::webhook_api::v1::CreateScheduledReactionRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_webhook_api_v1_CreateScheduledReactionRequest_iter(
      const google::protobuf::Map<int32_t, ::webhook_api::v1::CreateScheduledReactionRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_CreateScheduledReactionRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::webhook_api::v1::CreateScheduledReactionRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::webhook_api::v1::CreateScheduledReactionRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::webhook_api::v1::CreateScheduledReactionRequest>* __rust_proto_thunk__Map_u32_webhook_api_v1_CreateScheduledReactionRequest_new() {
    return new google::protobuf::Map<uint32_t, ::webhook_api::v1::CreateScheduledReactionRequest>();
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_CreateScheduledReactionRequest_free(const google::protobuf::Map<uint32_t, ::webhook_api::v1::CreateScheduledReactionRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_CreateScheduledReactionRequest_clear(google::protobuf::Map<uint32_t, ::webhook_api::v1::CreateScheduledReactionRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_webhook_api_v1_CreateScheduledReactionRequest_size(const google::protobuf::Map<uint32_t, ::webhook_api::v1::CreateScheduledReactionRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_CreateScheduledReactionRequest_insert(google::protobuf::Map<uint32_t, ::webhook_api::v1::CreateScheduledReactionRequest> * m,
                          uint32_t key, ::webhook_api::v1::CreateScheduledReactionRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_CreateScheduledReactionRequest_get(const google::protobuf::Map<uint32_t, ::webhook_api::v1::CreateScheduledReactionRequest>* m,
                       uint32_t key, const ::webhook_api::v1::CreateScheduledReactionRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::CreateScheduledReactionRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_CreateScheduledReactionRequest_remove(google::protobuf::Map<uint32_t, ::webhook_api::v1::CreateScheduledReactionRequest> * m,
                          uint32_t key, ::webhook_api::v1::CreateScheduledReactionRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_webhook_api_v1_CreateScheduledReactionRequest_iter(
      const google::protobuf::Map<uint32_t, ::webhook_api::v1::CreateScheduledReactionRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_CreateScheduledReactionRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::webhook_api::v1::CreateScheduledReactionRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::webhook_api::v1::CreateScheduledReactionRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::webhook_api::v1::CreateScheduledReactionRequest>* __rust_proto_thunk__Map_i64_webhook_api_v1_CreateScheduledReactionRequest_new() {
    return new google::protobuf::Map<int64_t, ::webhook_api::v1::CreateScheduledReactionRequest>();
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_CreateScheduledReactionRequest_free(const google::protobuf::Map<int64_t, ::webhook_api::v1::CreateScheduledReactionRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_CreateScheduledReactionRequest_clear(google::protobuf::Map<int64_t, ::webhook_api::v1::CreateScheduledReactionRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_webhook_api_v1_CreateScheduledReactionRequest_size(const google::protobuf::Map<int64_t, ::webhook_api::v1::CreateScheduledReactionRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_CreateScheduledReactionRequest_insert(google::protobuf::Map<int64_t, ::webhook_api::v1::CreateScheduledReactionRequest> * m,
                          int64_t key, ::webhook_api::v1::CreateScheduledReactionRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_CreateScheduledReactionRequest_get(const google::protobuf::Map<int64_t, ::webhook_api::v1::CreateScheduledReactionRequest>* m,
                       int64_t key, const ::webhook_api::v1::CreateScheduledReactionRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::CreateScheduledReactionRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_CreateScheduledReactionRequest_remove(google::protobuf::Map<int64_t, ::webhook_api::v1::CreateScheduledReactionRequest> * m,
                          int64_t key, ::webhook_api::v1::CreateScheduledReactionRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_webhook_api_v1_CreateScheduledReactionRequest_iter(
      const google::protobuf::Map<int64_t, ::webhook_api::v1::CreateScheduledReactionRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_CreateScheduledReactionRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::webhook_api::v1::CreateScheduledReactionRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::webhook_api::v1::CreateScheduledReactionRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::webhook_api::v1::CreateScheduledReactionRequest>* __rust_proto_thunk__Map_u64_webhook_api_v1_CreateScheduledReactionRequest_new() {
    return new google::protobuf::Map<uint64_t, ::webhook_api::v1::CreateScheduledReactionRequest>();
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_CreateScheduledReactionRequest_free(const google::protobuf::Map<uint64_t, ::webhook_api::v1::CreateScheduledReactionRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_CreateScheduledReactionRequest_clear(google::protobuf::Map<uint64_t, ::webhook_api::v1::CreateScheduledReactionRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_webhook_api_v1_CreateScheduledReactionRequest_size(const google::protobuf::Map<uint64_t, ::webhook_api::v1::CreateScheduledReactionRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_CreateScheduledReactionRequest_insert(google::protobuf::Map<uint64_t, ::webhook_api::v1::CreateScheduledReactionRequest> * m,
                          uint64_t key, ::webhook_api::v1::CreateScheduledReactionRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_CreateScheduledReactionRequest_get(const google::protobuf::Map<uint64_t, ::webhook_api::v1::CreateScheduledReactionRequest>* m,
                       uint64_t key, const ::webhook_api::v1::CreateScheduledReactionRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::CreateScheduledReactionRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_CreateScheduledReactionRequest_remove(google::protobuf::Map<uint64_t, ::webhook_api::v1::CreateScheduledReactionRequest> * m,
                          uint64_t key, ::webhook_api::v1::CreateScheduledReactionRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_webhook_api_v1_CreateScheduledReactionRequest_iter(
      const google::protobuf::Map<uint64_t, ::webhook_api::v1::CreateScheduledReactionRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_CreateScheduledReactionRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::webhook_api::v1::CreateScheduledReactionRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::webhook_api::v1::CreateScheduledReactionRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::webhook_api::v1::CreateScheduledReactionRequest>* __rust_proto_thunk__Map_bool_webhook_api_v1_CreateScheduledReactionRequest_new() {
    return new google::protobuf::Map<bool, ::webhook_api::v1::CreateScheduledReactionRequest>();
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_CreateScheduledReactionRequest_free(const google::protobuf::Map<bool, ::webhook_api::v1::CreateScheduledReactionRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_CreateScheduledReactionRequest_clear(google::protobuf::Map<bool, ::webhook_api::v1::CreateScheduledReactionRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_webhook_api_v1_CreateScheduledReactionRequest_size(const google::protobuf::Map<bool, ::webhook_api::v1::CreateScheduledReactionRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_CreateScheduledReactionRequest_insert(google::protobuf::Map<bool, ::webhook_api::v1::CreateScheduledReactionRequest> * m,
                          bool key, ::webhook_api::v1::CreateScheduledReactionRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_CreateScheduledReactionRequest_get(const google::protobuf::Map<bool, ::webhook_api::v1::CreateScheduledReactionRequest>* m,
                       bool key, const ::webhook_api::v1::CreateScheduledReactionRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::CreateScheduledReactionRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_CreateScheduledReactionRequest_remove(google::protobuf::Map<bool, ::webhook_api::v1::CreateScheduledReactionRequest> * m,
                          bool key, ::webhook_api::v1::CreateScheduledReactionRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_webhook_api_v1_CreateScheduledReactionRequest_iter(
      const google::protobuf::Map<bool, ::webhook_api::v1::CreateScheduledReactionRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_CreateScheduledReactionRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::webhook_api::v1::CreateScheduledReactionRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::webhook_api::v1::CreateScheduledReactionRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::webhook_api::v1::CreateScheduledReactionRequest>* __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_CreateScheduledReactionRequest_new() {
    return new google::protobuf::Map<std::string, ::webhook_api::v1::CreateScheduledReactionRequest>();
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_CreateScheduledReactionRequest_free(const google::protobuf::Map<std::string, ::webhook_api::v1::CreateScheduledReactionRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_CreateScheduledReactionRequest_clear(google::protobuf::Map<std::string, ::webhook_api::v1::CreateScheduledReactionRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_CreateScheduledReactionRequest_size(const google::protobuf::Map<std::string, ::webhook_api::v1::CreateScheduledReactionRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_CreateScheduledReactionRequest_insert(google::protobuf::Map<std::string, ::webhook_api::v1::CreateScheduledReactionRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::CreateScheduledReactionRequest value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_CreateScheduledReactionRequest_get(const google::protobuf::Map<std::string, ::webhook_api::v1::CreateScheduledReactionRequest>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::webhook_api::v1::CreateScheduledReactionRequest** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::CreateScheduledReactionRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_CreateScheduledReactionRequest_remove(google::protobuf::Map<std::string, ::webhook_api::v1::CreateScheduledReactionRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::CreateScheduledReactionRequest * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_CreateScheduledReactionRequest_iter(
      const google::protobuf::Map<std::string, ::webhook_api::v1::CreateScheduledReactionRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_CreateScheduledReactionRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::webhook_api::v1::CreateScheduledReactionRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::webhook_api::v1::CreateScheduledReactionRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// webhook_api.v1.CreateScheduledReactionResponse
        // clang-format off
extern "C" {
void* __rust_proto_thunk__webhook_api_v1_CreateScheduledReactionResponse_new() { return new ::webhook_api::v1::CreateScheduledReactionResponse(); }
void __rust_proto_thunk__webhook_api_v1_CreateScheduledReactionResponse_delete(void* ptr) { delete static_cast<::webhook_api::v1::CreateScheduledReactionResponse*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__webhook_api_v1_CreateScheduledReactionResponse_serialize(::webhook_api::v1::CreateScheduledReactionResponse* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__webhook_api_v1_CreateScheduledReactionResponse_parse(::webhook_api::v1::CreateScheduledReactionResponse* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__webhook_api_v1_CreateScheduledReactionResponse_copy_from(::webhook_api::v1::CreateScheduledReactionResponse* dst, const ::webhook_api::v1::CreateScheduledReactionResponse* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__webhook_api_v1_CreateScheduledReactionResponse_repeated_len(google::protobuf::RepeatedPtrField<::webhook_api::v1::CreateScheduledReactionResponse>* field) {
  return field->size();
}
const ::webhook_api::v1::CreateScheduledReactionResponse& __rust_proto_thunk__webhook_api_v1_CreateScheduledReactionResponse_repeated_get(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::CreateScheduledReactionResponse>* field,
  size_t index) {
  return field->Get(index);
}
::webhook_api::v1::CreateScheduledReactionResponse* __rust_proto_thunk__webhook_api_v1_CreateScheduledReactionResponse_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::CreateScheduledReactionResponse>* field,
  size_t index) {
  return field->Mutable(index);
}
::webhook_api::v1::CreateScheduledReactionResponse* __rust_proto_thunk__webhook_api_v1_CreateScheduledReactionResponse_repeated_add(google::protobuf::RepeatedPtrField<::webhook_api::v1::CreateScheduledReactionResponse>* field) {
  return field->Add();
}
void __rust_proto_thunk__webhook_api_v1_CreateScheduledReactionResponse_repeated_clear(google::protobuf::RepeatedPtrField<::webhook_api::v1::CreateScheduledReactionResponse>* field) {
  field->Clear();
}
void __rust_proto_thunk__webhook_api_v1_CreateScheduledReactionResponse_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::CreateScheduledReactionResponse>& dst,
  const google::protobuf::RepeatedPtrField<::webhook_api::v1::CreateScheduledReactionResponse>& src) {
  dst = src;
}

const void* __rust_proto_thunk__webhook_api_v1_CreateScheduledReactionResponse_get_scheduled_reaction(::webhook_api::v1::CreateScheduledReactionResponse* msg) {
  return static_cast<const void*>(&msg->scheduled_reaction());
}
void* __rust_proto_thunk__webhook_api_v1_CreateScheduledReactionResponse_get_mut_scheduled_reaction(::webhook_api::v1::CreateScheduledReactionResponse* msg) {
  return static_cast<void*>(msg->mutable_scheduled_reaction());
}
void __rust_proto_thunk__webhook_api_v1_CreateScheduledReactionResponse_clear_scheduled_reaction(::webhook_api::v1::CreateScheduledReactionResponse* msg) { msg->clear_scheduled_reaction(); }
bool __rust_proto_thunk__webhook_api_v1_CreateScheduledReactionResponse_has_scheduled_reaction(::webhook_api::v1::CreateScheduledReactionResponse* msg) { return msg->has_scheduled_reaction(); }


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::webhook_api::v1::CreateScheduledReactionResponse>* __rust_proto_thunk__Map_i32_webhook_api_v1_CreateScheduledReactionResponse_new() {
    return new google::protobuf::Map<int32_t, ::webhook_api::v1::CreateScheduledReactionResponse>();
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_CreateScheduledReactionResponse_free(const google::protobuf::Map<int32_t, ::webhook_api::v1::CreateScheduledReactionResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_CreateScheduledReactionResponse_clear(google::protobuf::Map<int32_t, ::webhook_api::v1::CreateScheduledReactionResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_webhook_api_v1_CreateScheduledReactionResponse_size(const google::protobuf::Map<int32_t, ::webhook_api::v1::CreateScheduledReactionResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_CreateScheduledReactionResponse_insert(google::protobuf::Map<int32_t, ::webhook_api::v1::CreateScheduledReactionResponse> * m,
                          int32_t key, ::webhook_api::v1::CreateScheduledReactionResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_CreateScheduledReactionResponse_get(const google::protobuf::Map<int32_t, ::webhook_api::v1::CreateScheduledReactionResponse>* m,
                       int32_t key, const ::webhook_api::v1::CreateScheduledReactionResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::CreateScheduledReactionResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_CreateScheduledReactionResponse_remove(google::protobuf::Map<int32_t, ::webhook_api::v1::CreateScheduledReactionResponse> * m,
                          int32_t key, ::webhook_api::v1::CreateScheduledReactionResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_webhook_api_v1_CreateScheduledReactionResponse_iter(
      const google::protobuf::Map<int32_t, ::webhook_api::v1::CreateScheduledReactionResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_CreateScheduledReactionResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::webhook_api::v1::CreateScheduledReactionResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::webhook_api::v1::CreateScheduledReactionResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::webhook_api::v1::CreateScheduledReactionResponse>* __rust_proto_thunk__Map_u32_webhook_api_v1_CreateScheduledReactionResponse_new() {
    return new google::protobuf::Map<uint32_t, ::webhook_api::v1::CreateScheduledReactionResponse>();
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_CreateScheduledReactionResponse_free(const google::protobuf::Map<uint32_t, ::webhook_api::v1::CreateScheduledReactionResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_CreateScheduledReactionResponse_clear(google::protobuf::Map<uint32_t, ::webhook_api::v1::CreateScheduledReactionResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_webhook_api_v1_CreateScheduledReactionResponse_size(const google::protobuf::Map<uint32_t, ::webhook_api::v1::CreateScheduledReactionResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_CreateScheduledReactionResponse_insert(google::protobuf::Map<uint32_t, ::webhook_api::v1::CreateScheduledReactionResponse> * m,
                          uint32_t key, ::webhook_api::v1::CreateScheduledReactionResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_CreateScheduledReactionResponse_get(const google::protobuf::Map<uint32_t, ::webhook_api::v1::CreateScheduledReactionResponse>* m,
                       uint32_t key, const ::webhook_api::v1::CreateScheduledReactionResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::CreateScheduledReactionResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_CreateScheduledReactionResponse_remove(google::protobuf::Map<uint32_t, ::webhook_api::v1::CreateScheduledReactionResponse> * m,
                          uint32_t key, ::webhook_api::v1::CreateScheduledReactionResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_webhook_api_v1_CreateScheduledReactionResponse_iter(
      const google::protobuf::Map<uint32_t, ::webhook_api::v1::CreateScheduledReactionResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_CreateScheduledReactionResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::webhook_api::v1::CreateScheduledReactionResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::webhook_api::v1::CreateScheduledReactionResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::webhook_api::v1::CreateScheduledReactionResponse>* __rust_proto_thunk__Map_i64_webhook_api_v1_CreateScheduledReactionResponse_new() {
    return new google::protobuf::Map<int64_t, ::webhook_api::v1::CreateScheduledReactionResponse>();
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_CreateScheduledReactionResponse_free(const google::protobuf::Map<int64_t, ::webhook_api::v1::CreateScheduledReactionResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_CreateScheduledReactionResponse_clear(google::protobuf::Map<int64_t, ::webhook_api::v1::CreateScheduledReactionResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_webhook_api_v1_CreateScheduledReactionResponse_size(const google::protobuf::Map<int64_t, ::webhook_api::v1::CreateScheduledReactionResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_CreateScheduledReactionResponse_insert(google::protobuf::Map<int64_t, ::webhook_api::v1::CreateScheduledReactionResponse> * m,
                          int64_t key, ::webhook_api::v1::CreateScheduledReactionResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_CreateScheduledReactionResponse_get(const google::protobuf::Map<int64_t, ::webhook_api::v1::CreateScheduledReactionResponse>* m,
                       int64_t key, const ::webhook_api::v1::CreateScheduledReactionResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::CreateScheduledReactionResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_CreateScheduledReactionResponse_remove(google::protobuf::Map<int64_t, ::webhook_api::v1::CreateScheduledReactionResponse> * m,
                          int64_t key, ::webhook_api::v1::CreateScheduledReactionResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_webhook_api_v1_CreateScheduledReactionResponse_iter(
      const google::protobuf::Map<int64_t, ::webhook_api::v1::CreateScheduledReactionResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_CreateScheduledReactionResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::webhook_api::v1::CreateScheduledReactionResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::webhook_api::v1::CreateScheduledReactionResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::webhook_api::v1::CreateScheduledReactionResponse>* __rust_proto_thunk__Map_u64_webhook_api_v1_CreateScheduledReactionResponse_new() {
    return new google::protobuf::Map<uint64_t, ::webhook_api::v1::CreateScheduledReactionResponse>();
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_CreateScheduledReactionResponse_free(const google::protobuf::Map<uint64_t, ::webhook_api::v1::CreateScheduledReactionResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_CreateScheduledReactionResponse_clear(google::protobuf::Map<uint64_t, ::webhook_api::v1::CreateScheduledReactionResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_webhook_api_v1_CreateScheduledReactionResponse_size(const google::protobuf::Map<uint64_t, ::webhook_api::v1::CreateScheduledReactionResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_CreateScheduledReactionResponse_insert(google::protobuf::Map<uint64_t, ::webhook_api::v1::CreateScheduledReactionResponse> * m,
                          uint64_t key, ::webhook_api::v1::CreateScheduledReactionResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_CreateScheduledReactionResponse_get(const google::protobuf::Map<uint64_t, ::webhook_api::v1::CreateScheduledReactionResponse>* m,
                       uint64_t key, const ::webhook_api::v1::CreateScheduledReactionResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::CreateScheduledReactionResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_CreateScheduledReactionResponse_remove(google::protobuf::Map<uint64_t, ::webhook_api::v1::CreateScheduledReactionResponse> * m,
                          uint64_t key, ::webhook_api::v1::CreateScheduledReactionResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_webhook_api_v1_CreateScheduledReactionResponse_iter(
      const google::protobuf::Map<uint64_t, ::webhook_api::v1::CreateScheduledReactionResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_CreateScheduledReactionResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::webhook_api::v1::CreateScheduledReactionResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::webhook_api::v1::CreateScheduledReactionResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::webhook_api::v1::CreateScheduledReactionResponse>* __rust_proto_thunk__Map_bool_webhook_api_v1_CreateScheduledReactionResponse_new() {
    return new google::protobuf::Map<bool, ::webhook_api::v1::CreateScheduledReactionResponse>();
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_CreateScheduledReactionResponse_free(const google::protobuf::Map<bool, ::webhook_api::v1::CreateScheduledReactionResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_CreateScheduledReactionResponse_clear(google::protobuf::Map<bool, ::webhook_api::v1::CreateScheduledReactionResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_webhook_api_v1_CreateScheduledReactionResponse_size(const google::protobuf::Map<bool, ::webhook_api::v1::CreateScheduledReactionResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_CreateScheduledReactionResponse_insert(google::protobuf::Map<bool, ::webhook_api::v1::CreateScheduledReactionResponse> * m,
                          bool key, ::webhook_api::v1::CreateScheduledReactionResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_CreateScheduledReactionResponse_get(const google::protobuf::Map<bool, ::webhook_api::v1::CreateScheduledReactionResponse>* m,
                       bool key, const ::webhook_api::v1::CreateScheduledReactionResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::CreateScheduledReactionResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_CreateScheduledReactionResponse_remove(google::protobuf::Map<bool, ::webhook_api::v1::CreateScheduledReactionResponse> * m,
                          bool key, ::webhook_api::v1::CreateScheduledReactionResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_webhook_api_v1_CreateScheduledReactionResponse_iter(
      const google::protobuf::Map<bool, ::webhook_api::v1::CreateScheduledReactionResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_CreateScheduledReactionResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::webhook_api::v1::CreateScheduledReactionResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::webhook_api::v1::CreateScheduledReactionResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::webhook_api::v1::CreateScheduledReactionResponse>* __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_CreateScheduledReactionResponse_new() {
    return new google::protobuf::Map<std::string, ::webhook_api::v1::CreateScheduledReactionResponse>();
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_CreateScheduledReactionResponse_free(const google::protobuf::Map<std::string, ::webhook_api::v1::CreateScheduledReactionResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_CreateScheduledReactionResponse_clear(google::protobuf::Map<std::string, ::webhook_api::v1::CreateScheduledReactionResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_CreateScheduledReactionResponse_size(const google::protobuf::Map<std::string, ::webhook_api::v1::CreateScheduledReactionResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_CreateScheduledReactionResponse_insert(google::protobuf::Map<std::string, ::webhook_api::v1::CreateScheduledReactionResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::CreateScheduledReactionResponse value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_CreateScheduledReactionResponse_get(const google::protobuf::Map<std::string, ::webhook_api::v1::CreateScheduledReactionResponse>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::webhook_api::v1::CreateScheduledReactionResponse** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::CreateScheduledReactionResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_CreateScheduledReactionResponse_remove(google::protobuf::Map<std::string, ::webhook_api::v1::CreateScheduledReactionResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::CreateScheduledReactionResponse * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_CreateScheduledReactionResponse_iter(
      const google::protobuf::Map<std::string, ::webhook_api::v1::CreateScheduledReactionResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_CreateScheduledReactionResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::webhook_api::v1::CreateScheduledReactionResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::webhook_api::v1::CreateScheduledReactionResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// webhook_api.v1.ListScheduledReactionsRequest
        // clang-format off
extern "C" {
void* __rust_proto_thunk__webhook_api_v1_ListScheduledReactionsRequest_new() { return new ::webhook_api::v1::ListScheduledReactionsRequest(); }
void __rust_proto_thunk__webhook_api_v1_ListScheduledReactionsRequest_delete(void* ptr) { delete static_cast<::webhook_api::v1::ListScheduledReactionsRequest*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__webhook_api_v1_ListScheduledReactionsRequest_serialize(::webhook_api::v1::ListScheduledReactionsRequest* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__webhook_api_v1_ListScheduledReactionsRequest_parse(::webhook_api::v1::ListScheduledReactionsRequest* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__webhook_api_v1_ListScheduledReactionsRequest_copy_from(::webhook_api::v1::ListScheduledReactionsRequest* dst, const ::webhook_api::v1::ListScheduledReactionsRequest* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__webhook_api_v1_ListScheduledReactionsRequest_repeated_len(google::protobuf::RepeatedPtrField<::webhook_api::v1::ListScheduledReactionsRequest>* field) {
  return field->size();
}
const ::webhook_api::v1::ListScheduledReactionsRequest& __rust_proto_thunk__webhook_api_v1_ListScheduledReactionsRequest_repeated_get(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::ListScheduledReactionsRequest>* field,
  size_t index) {
  return field->Get(index);
}
::webhook_api::v1::ListScheduledReactionsRequest* __rust_proto_thunk__webhook_api_v1_ListScheduledReactionsRequest_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::ListScheduledReactionsRequest>* field,
  size_t index) {
  return field->Mutable(index);
}
::webhook_api::v1::ListScheduledReactionsRequest* __rust_proto_thunk__webhook_api_v1_ListScheduledReactionsRequest_repeated_add(google::protobuf::RepeatedPtrField<::webhook_api::v1::ListScheduledReactionsRequest>* field) {
  return field->Add();
}
void __rust_proto_thunk__webhook_api_v1_ListScheduledReactionsRequest_repeated_clear(google::protobuf::RepeatedPtrField<::webhook_api::v1::ListScheduledReactionsRequest>* field) {
  field->Clear();
}
void __rust_proto_thunk__webhook_api_v1_ListScheduledReactionsRequest_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::ListScheduledReactionsRequest>& dst,
  const google::protobuf::RepeatedPtrField<::webhook_api::v1::ListScheduledReactionsRequest>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__webhook_api_v1_ListScheduledReactionsRequest_get_api_key(::webhook_api::v1::ListScheduledReactionsRequest* msg) {
  absl::string_view val = msg->api_key();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__webhook_api_v1_ListScheduledReactionsRequest_set_api_key(::webhook_api::v1::ListScheduledReactionsRequest* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_api_key(absl::string_view(s.ptr, s.len));
}

::int32_t __rust_proto_thunk__webhook_api_v1_ListScheduledReactionsRequest_get_page(::webhook_api::v1::ListScheduledReactionsRequest* msg) { return msg->page(); }
void __rust_proto_thunk__webhook_api_v1_ListScheduledReactionsRequest_set_page(::webhook_api::v1::ListScheduledReactionsRequest* msg, ::int32_t val) {
  msg->set_page(val);
}

::int32_t __rust_proto_thunk__webhook_api_v1_ListScheduledReactionsRequest_get_page_size(::webhook_api::v1::ListScheduledReactionsRequest* msg) { return msg->page_size(); }
void __rust_proto_thunk__webhook_api_v1_ListScheduledReactionsRequest_set_page_size(::webhook_api::v1::ListScheduledReactionsRequest* msg, ::int32_t val) {
  msg->set_page_size(val);
}

const void* __rust_proto_thunk__webhook_api_v1_ListScheduledReactionsRequest_get_filters(::webhook_api::v1::ListScheduledReactionsRequest* msg) {
  return static_cast<const void*>(&msg->filters());
}
void* __rust_proto_thunk__webhook_api_v1_ListScheduledReactionsRequest_get_mut_filters(::webhook_api::v1::ListScheduledReactionsRequest* msg) {
  return static_cast<void*>(msg->mutable_filters());
}
void __rust_proto_thunk__webhook_api_v1_ListScheduledReactionsRequest_clear_filters(::webhook_api::v1::ListScheduledReactionsRequest* msg) { msg->clear_filters(); }
bool __rust_proto_thunk__webhook_api_v1_ListScheduledReactionsRequest_has_filters(::webhook_api::v1::ListScheduledReactionsRequest* msg) { return msg->has_filters(); }

const void* __rust_proto_thunk__webhook_api_v1_ListScheduledReactionsRequest_get_sort_by(::webhook_api::v1::ListScheduledReactionsRequest* msg) {
  return static_cast<const void*>(&msg->sort_by());
}
void* __rust_proto_thunk__webhook_api_v1_ListScheduledReactionsRequest_get_mut_sort_by(::webhook_api::v1::ListScheduledReactionsRequest* msg) {
  return static_cast<void*>(msg->mutable_sort_by());
}
void __rust_proto_thunk__webhook_api_v1_ListScheduledReactionsRequest_clear_sort_by(::webhook_api::v1::ListScheduledReactionsRequest* msg) { msg->clear_sort_by(); }
bool __rust_proto_thunk__webhook_api_v1_ListScheduledReactionsRequest_has_sort_by(::webhook_api::v1::ListScheduledReactionsRequest* msg) { return msg->has_sort_by(); }


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::webhook_api::v1::ListScheduledReactionsRequest>* __rust_proto_thunk__Map_i32_webhook_api_v1_ListScheduledReactionsRequest_new() {
    return new google::protobuf::Map<int32_t, ::webhook_api::v1::ListScheduledReactionsRequest>();
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_ListScheduledReactionsRequest_free(const google::protobuf::Map<int32_t, ::webhook_api::v1::ListScheduledReactionsRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_ListScheduledReactionsRequest_clear(google::protobuf::Map<int32_t, ::webhook_api::v1::ListScheduledReactionsRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_webhook_api_v1_ListScheduledReactionsRequest_size(const google::protobuf::Map<int32_t, ::webhook_api::v1::ListScheduledReactionsRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_ListScheduledReactionsRequest_insert(google::protobuf::Map<int32_t, ::webhook_api::v1::ListScheduledReactionsRequest> * m,
                          int32_t key, ::webhook_api::v1::ListScheduledReactionsRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_ListScheduledReactionsRequest_get(const google::protobuf::Map<int32_t, ::webhook_api::v1::ListScheduledReactionsRequest>* m,
                       int32_t key, const ::webhook_api::v1::ListScheduledReactionsRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ListScheduledReactionsRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_ListScheduledReactionsRequest_remove(google::protobuf::Map<int32_t, ::webhook_api::v1::ListScheduledReactionsRequest> * m,
                          int32_t key, ::webhook_api::v1::ListScheduledReactionsRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_webhook_api_v1_ListScheduledReactionsRequest_iter(
      const google::protobuf::Map<int32_t, ::webhook_api::v1::ListScheduledReactionsRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_ListScheduledReactionsRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::webhook_api::v1::ListScheduledReactionsRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::webhook_api::v1::ListScheduledReactionsRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::webhook_api::v1::ListScheduledReactionsRequest>* __rust_proto_thunk__Map_u32_webhook_api_v1_ListScheduledReactionsRequest_new() {
    return new google::protobuf::Map<uint32_t, ::webhook_api::v1::ListScheduledReactionsRequest>();
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_ListScheduledReactionsRequest_free(const google::protobuf::Map<uint32_t, ::webhook_api::v1::ListScheduledReactionsRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_ListScheduledReactionsRequest_clear(google::protobuf::Map<uint32_t, ::webhook_api::v1::ListScheduledReactionsRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_webhook_api_v1_ListScheduledReactionsRequest_size(const google::protobuf::Map<uint32_t, ::webhook_api::v1::ListScheduledReactionsRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_ListScheduledReactionsRequest_insert(google::protobuf::Map<uint32_t, ::webhook_api::v1::ListScheduledReactionsRequest> * m,
                          uint32_t key, ::webhook_api::v1::ListScheduledReactionsRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_ListScheduledReactionsRequest_get(const google::protobuf::Map<uint32_t, ::webhook_api::v1::ListScheduledReactionsRequest>* m,
                       uint32_t key, const ::webhook_api::v1::ListScheduledReactionsRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ListScheduledReactionsRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_ListScheduledReactionsRequest_remove(google::protobuf::Map<uint32_t, ::webhook_api::v1::ListScheduledReactionsRequest> * m,
                          uint32_t key, ::webhook_api::v1::ListScheduledReactionsRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_webhook_api_v1_ListScheduledReactionsRequest_iter(
      const google::protobuf::Map<uint32_t, ::webhook_api::v1::ListScheduledReactionsRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_ListScheduledReactionsRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::webhook_api::v1::ListScheduledReactionsRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::webhook_api::v1::ListScheduledReactionsRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::webhook_api::v1::ListScheduledReactionsRequest>* __rust_proto_thunk__Map_i64_webhook_api_v1_ListScheduledReactionsRequest_new() {
    return new google::protobuf::Map<int64_t, ::webhook_api::v1::ListScheduledReactionsRequest>();
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_ListScheduledReactionsRequest_free(const google::protobuf::Map<int64_t, ::webhook_api::v1::ListScheduledReactionsRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_ListScheduledReactionsRequest_clear(google::protobuf::Map<int64_t, ::webhook_api::v1::ListScheduledReactionsRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_webhook_api_v1_ListScheduledReactionsRequest_size(const google::protobuf::Map<int64_t, ::webhook_api::v1::ListScheduledReactionsRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_ListScheduledReactionsRequest_insert(google::protobuf::Map<int64_t, ::webhook_api::v1::ListScheduledReactionsRequest> * m,
                          int64_t key, ::webhook_api::v1::ListScheduledReactionsRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_ListScheduledReactionsRequest_get(const google::protobuf::Map<int64_t, ::webhook_api::v1::ListScheduledReactionsRequest>* m,
                       int64_t key, const ::webhook_api::v1::ListScheduledReactionsRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ListScheduledReactionsRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_ListScheduledReactionsRequest_remove(google::protobuf::Map<int64_t, ::webhook_api::v1::ListScheduledReactionsRequest> * m,
                          int64_t key, ::webhook_api::v1::ListScheduledReactionsRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_webhook_api_v1_ListScheduledReactionsRequest_iter(
      const google::protobuf::Map<int64_t, ::webhook_api::v1::ListScheduledReactionsRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_ListScheduledReactionsRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::webhook_api::v1::ListScheduledReactionsRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::webhook_api::v1::ListScheduledReactionsRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::webhook_api::v1::ListScheduledReactionsRequest>* __rust_proto_thunk__Map_u64_webhook_api_v1_ListScheduledReactionsRequest_new() {
    return new google::protobuf::Map<uint64_t, ::webhook_api::v1::ListScheduledReactionsRequest>();
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_ListScheduledReactionsRequest_free(const google::protobuf::Map<uint64_t, ::webhook_api::v1::ListScheduledReactionsRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_ListScheduledReactionsRequest_clear(google::protobuf::Map<uint64_t, ::webhook_api::v1::ListScheduledReactionsRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_webhook_api_v1_ListScheduledReactionsRequest_size(const google::protobuf::Map<uint64_t, ::webhook_api::v1::ListScheduledReactionsRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_ListScheduledReactionsRequest_insert(google::protobuf::Map<uint64_t, ::webhook_api::v1::ListScheduledReactionsRequest> * m,
                          uint64_t key, ::webhook_api::v1::ListScheduledReactionsRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_ListScheduledReactionsRequest_get(const google::protobuf::Map<uint64_t, ::webhook_api::v1::ListScheduledReactionsRequest>* m,
                       uint64_t key, const ::webhook_api::v1::ListScheduledReactionsRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ListScheduledReactionsRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_ListScheduledReactionsRequest_remove(google::protobuf::Map<uint64_t, ::webhook_api::v1::ListScheduledReactionsRequest> * m,
                          uint64_t key, ::webhook_api::v1::ListScheduledReactionsRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_webhook_api_v1_ListScheduledReactionsRequest_iter(
      const google::protobuf::Map<uint64_t, ::webhook_api::v1::ListScheduledReactionsRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_ListScheduledReactionsRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::webhook_api::v1::ListScheduledReactionsRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::webhook_api::v1::ListScheduledReactionsRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::webhook_api::v1::ListScheduledReactionsRequest>* __rust_proto_thunk__Map_bool_webhook_api_v1_ListScheduledReactionsRequest_new() {
    return new google::protobuf::Map<bool, ::webhook_api::v1::ListScheduledReactionsRequest>();
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_ListScheduledReactionsRequest_free(const google::protobuf::Map<bool, ::webhook_api::v1::ListScheduledReactionsRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_ListScheduledReactionsRequest_clear(google::protobuf::Map<bool, ::webhook_api::v1::ListScheduledReactionsRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_webhook_api_v1_ListScheduledReactionsRequest_size(const google::protobuf::Map<bool, ::webhook_api::v1::ListScheduledReactionsRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_ListScheduledReactionsRequest_insert(google::protobuf::Map<bool, ::webhook_api::v1::ListScheduledReactionsRequest> * m,
                          bool key, ::webhook_api::v1::ListScheduledReactionsRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_ListScheduledReactionsRequest_get(const google::protobuf::Map<bool, ::webhook_api::v1::ListScheduledReactionsRequest>* m,
                       bool key, const ::webhook_api::v1::ListScheduledReactionsRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ListScheduledReactionsRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_ListScheduledReactionsRequest_remove(google::protobuf::Map<bool, ::webhook_api::v1::ListScheduledReactionsRequest> * m,
                          bool key, ::webhook_api::v1::ListScheduledReactionsRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_webhook_api_v1_ListScheduledReactionsRequest_iter(
      const google::protobuf::Map<bool, ::webhook_api::v1::ListScheduledReactionsRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_ListScheduledReactionsRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::webhook_api::v1::ListScheduledReactionsRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::webhook_api::v1::ListScheduledReactionsRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::webhook_api::v1::ListScheduledReactionsRequest>* __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListScheduledReactionsRequest_new() {
    return new google::protobuf::Map<std::string, ::webhook_api::v1::ListScheduledReactionsRequest>();
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListScheduledReactionsRequest_free(const google::protobuf::Map<std::string, ::webhook_api::v1::ListScheduledReactionsRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListScheduledReactionsRequest_clear(google::protobuf::Map<std::string, ::webhook_api::v1::ListScheduledReactionsRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListScheduledReactionsRequest_size(const google::protobuf::Map<std::string, ::webhook_api::v1::ListScheduledReactionsRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListScheduledReactionsRequest_insert(google::protobuf::Map<std::string, ::webhook_api::v1::ListScheduledReactionsRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::ListScheduledReactionsRequest value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListScheduledReactionsRequest_get(const google::protobuf::Map<std::string, ::webhook_api::v1::ListScheduledReactionsRequest>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::webhook_api::v1::ListScheduledReactionsRequest** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ListScheduledReactionsRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListScheduledReactionsRequest_remove(google::protobuf::Map<std::string, ::webhook_api::v1::ListScheduledReactionsRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::ListScheduledReactionsRequest * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListScheduledReactionsRequest_iter(
      const google::protobuf::Map<std::string, ::webhook_api::v1::ListScheduledReactionsRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListScheduledReactionsRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::webhook_api::v1::ListScheduledReactionsRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::webhook_api::v1::ListScheduledReactionsRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// webhook_api.v1.ListScheduledReactionsResponse
        // clang-format off
extern "C" {
void* __rust_proto_thunk__webhook_api_v1_ListScheduledReactionsResponse_new() { return new ::webhook_api::v1::ListScheduledReactionsResponse(); }
void __rust_proto_thunk__webhook_api_v1_ListScheduledReactionsResponse_delete(void* ptr) { delete static_cast<::webhook_api::v1::ListScheduledReactionsResponse*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__webhook_api_v1_ListScheduledReactionsResponse_serialize(::webhook_api::v1::ListScheduledReactionsResponse* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__webhook_api_v1_ListScheduledReactionsResponse_parse(::webhook_api::v1::ListScheduledReactionsResponse* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__webhook_api_v1_ListScheduledReactionsResponse_copy_from(::webhook_api::v1::ListScheduledReactionsResponse* dst, const ::webhook_api::v1::ListScheduledReactionsResponse* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__webhook_api_v1_ListScheduledReactionsResponse_repeated_len(google::protobuf::RepeatedPtrField<::webhook_api::v1::ListScheduledReactionsResponse>* field) {
  return field->size();
}
const ::webhook_api::v1::ListScheduledReactionsResponse& __rust_proto_thunk__webhook_api_v1_ListScheduledReactionsResponse_repeated_get(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::ListScheduledReactionsResponse>* field,
  size_t index) {
  return field->Get(index);
}
::webhook_api::v1::ListScheduledReactionsResponse* __rust_proto_thunk__webhook_api_v1_ListScheduledReactionsResponse_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::ListScheduledReactionsResponse>* field,
  size_t index) {
  return field->Mutable(index);
}
::webhook_api::v1::ListScheduledReactionsResponse* __rust_proto_thunk__webhook_api_v1_ListScheduledReactionsResponse_repeated_add(google::protobuf::RepeatedPtrField<::webhook_api::v1::ListScheduledReactionsResponse>* field) {
  return field->Add();
}
void __rust_proto_thunk__webhook_api_v1_ListScheduledReactionsResponse_repeated_clear(google::protobuf::RepeatedPtrField<::webhook_api::v1::ListScheduledReactionsResponse>* field) {
  field->Clear();
}
void __rust_proto_thunk__webhook_api_v1_ListScheduledReactionsResponse_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::ListScheduledReactionsResponse>& dst,
  const google::protobuf::RepeatedPtrField<::webhook_api::v1::ListScheduledReactionsResponse>& src) {
  dst = src;
}

void __rust_proto_thunk__webhook_api_v1_ListScheduledReactionsResponse_clear_scheduled_reactions(::webhook_api::v1::ListScheduledReactionsResponse* msg) {
  msg->clear_scheduled_reactions();
}
google::protobuf::RepeatedPtrField<::webhook_api::v1::ScheduledReaction>* __rust_proto_thunk__webhook_api_v1_ListScheduledReactionsResponse_get_mut_scheduled_reactions(::webhook_api::v1::ListScheduledReactionsResponse* msg) {
  return msg->mutable_scheduled_reactions();
}
const google::protobuf::RepeatedPtrField<::webhook_api::v1::ScheduledReaction>* __rust_proto_thunk__webhook_api_v1_ListScheduledReactionsResponse_get_scheduled_reactions(
    const ::webhook_api::v1::ListScheduledReactionsResponse* msg) {
  return &msg->scheduled_reactions();
}

::uint32_t __rust_proto_thunk__webhook_api_v1_ListScheduledReactionsResponse_get_total_owned_scheduled_reactions(::webhook_api::v1::ListScheduledReactionsResponse* msg) { return msg->total_owned_scheduled_reactions(); }
void __rust_proto_thunk__webhook_api_v1_ListScheduledReactionsResponse_set_total_owned_scheduled_reactions(::webhook_api::v1::ListScheduledReactionsResponse* msg, ::uint32_t val) {
  msg->set_total_owned_scheduled_reactions(val);
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::webhook_api::v1::ListScheduledReactionsResponse>* __rust_proto_thunk__Map_i32_webhook_api_v1_ListScheduledReactionsResponse_new() {
    return new google::protobuf::Map<int32_t, ::webhook_api::v1::ListScheduledReactionsResponse>();
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_ListScheduledReactionsResponse_free(const google::protobuf::Map<int32_t, ::webhook_api::v1::ListScheduledReactionsResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_ListScheduledReactionsResponse_clear(google::protobuf::Map<int32_t, ::webhook_api::v1::ListScheduledReactionsResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_webhook_api_v1_ListScheduledReactionsResponse_size(const google::protobuf::Map<int32_t, ::webhook_api::v1::ListScheduledReactionsResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_ListScheduledReactionsResponse_insert(google::protobuf::Map<int32_t, ::webhook_api::v1::ListScheduledReactionsResponse> * m,
                          int32_t key, ::webhook_api::v1::ListScheduledReactionsResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_ListScheduledReactionsResponse_get(const google::protobuf::Map<int32_t, ::webhook_api::v1::ListScheduledReactionsResponse>* m,
                       int32_t key, const ::webhook_api::v1::ListScheduledReactionsResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ListScheduledReactionsResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_ListScheduledReactionsResponse_remove(google::protobuf::Map<int32_t, ::webhook_api::v1::ListScheduledReactionsResponse> * m,
                          int32_t key, ::webhook_api::v1::ListScheduledReactionsResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_webhook_api_v1_ListScheduledReactionsResponse_iter(
      const google::protobuf::Map<int32_t, ::webhook_api::v1::ListScheduledReactionsResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_ListScheduledReactionsResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::webhook_api::v1::ListScheduledReactionsResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::webhook_api::v1::ListScheduledReactionsResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::webhook_api::v1::ListScheduledReactionsResponse>* __rust_proto_thunk__Map_u32_webhook_api_v1_ListScheduledReactionsResponse_new() {
    return new google::protobuf::Map<uint32_t, ::webhook_api::v1::ListScheduledReactionsResponse>();
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_ListScheduledReactionsResponse_free(const google::protobuf::Map<uint32_t, ::webhook_api::v1::ListScheduledReactionsResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_ListScheduledReactionsResponse_clear(google::protobuf::Map<uint32_t, ::webhook_api::v1::ListScheduledReactionsResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_webhook_api_v1_ListScheduledReactionsResponse_size(const google::protobuf::Map<uint32_t, ::webhook_api::v1::ListScheduledReactionsResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_ListScheduledReactionsResponse_insert(google::protobuf::Map<uint32_t, ::webhook_api::v1::ListScheduledReactionsResponse> * m,
                          uint32_t key, ::webhook_api::v1::ListScheduledReactionsResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_ListScheduledReactionsResponse_get(const google::protobuf::Map<uint32_t, ::webhook_api::v1::ListScheduledReactionsResponse>* m,
                       uint32_t key, const ::webhook_api::v1::ListScheduledReactionsResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ListScheduledReactionsResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_ListScheduledReactionsResponse_remove(google::protobuf::Map<uint32_t, ::webhook_api::v1::ListScheduledReactionsResponse> * m,
                          uint32_t key, ::webhook_api::v1::ListScheduledReactionsResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_webhook_api_v1_ListScheduledReactionsResponse_iter(
      const google::protobuf::Map<uint32_t, ::webhook_api::v1::ListScheduledReactionsResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_ListScheduledReactionsResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::webhook_api::v1::ListScheduledReactionsResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::webhook_api::v1::ListScheduledReactionsResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::webhook_api::v1::ListScheduledReactionsResponse>* __rust_proto_thunk__Map_i64_webhook_api_v1_ListScheduledReactionsResponse_new() {
    return new google::protobuf::Map<int64_t, ::webhook_api::v1::ListScheduledReactionsResponse>();
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_ListScheduledReactionsResponse_free(const google::protobuf::Map<int64_t, ::webhook_api::v1::ListScheduledReactionsResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_ListScheduledReactionsResponse_clear(google::protobuf::Map<int64_t, ::webhook_api::v1::ListScheduledReactionsResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_webhook_api_v1_ListScheduledReactionsResponse_size(const google::protobuf::Map<int64_t, ::webhook_api::v1::ListScheduledReactionsResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_ListScheduledReactionsResponse_insert(google::protobuf::Map<int64_t, ::webhook_api::v1::ListScheduledReactionsResponse> * m,
                          int64_t key, ::webhook_api::v1::ListScheduledReactionsResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_ListScheduledReactionsResponse_get(const google::protobuf::Map<int64_t, ::webhook_api::v1::ListScheduledReactionsResponse>* m,
                       int64_t key, const ::webhook_api::v1::ListScheduledReactionsResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ListScheduledReactionsResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_ListScheduledReactionsResponse_remove(google::protobuf::Map<int64_t, ::webhook_api::v1::ListScheduledReactionsResponse> * m,
                          int64_t key, ::webhook_api::v1::ListScheduledReactionsResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_webhook_api_v1_ListScheduledReactionsResponse_iter(
      const google::protobuf::Map<int64_t, ::webhook_api::v1::ListScheduledReactionsResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_ListScheduledReactionsResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::webhook_api::v1::ListScheduledReactionsResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::webhook_api::v1::ListScheduledReactionsResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::webhook_api::v1::ListScheduledReactionsResponse>* __rust_proto_thunk__Map_u64_webhook_api_v1_ListScheduledReactionsResponse_new() {
    return new google::protobuf::Map<uint64_t, ::webhook_api::v1::ListScheduledReactionsResponse>();
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_ListScheduledReactionsResponse_free(const google::protobuf::Map<uint64_t, ::webhook_api::v1::ListScheduledReactionsResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_ListScheduledReactionsResponse_clear(google::protobuf::Map<uint64_t, ::webhook_api::v1::ListScheduledReactionsResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_webhook_api_v1_ListScheduledReactionsResponse_size(const google::protobuf::Map<uint64_t, ::webhook_api::v1::ListScheduledReactionsResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_ListScheduledReactionsResponse_insert(google::protobuf::Map<uint64_t, ::webhook_api::v1::ListScheduledReactionsResponse> * m,
                          uint64_t key, ::webhook_api::v1::ListScheduledReactionsResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_ListScheduledReactionsResponse_get(const google::protobuf::Map<uint64_t, ::webhook_api::v1::ListScheduledReactionsResponse>* m,
                       uint64_t key, const ::webhook_api::v1::ListScheduledReactionsResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ListScheduledReactionsResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_ListScheduledReactionsResponse_remove(google::protobuf::Map<uint64_t, ::webhook_api::v1::ListScheduledReactionsResponse> * m,
                          uint64_t key, ::webhook_api::v1::ListScheduledReactionsResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_webhook_api_v1_ListScheduledReactionsResponse_iter(
      const google::protobuf::Map<uint64_t, ::webhook_api::v1::ListScheduledReactionsResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_ListScheduledReactionsResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::webhook_api::v1::ListScheduledReactionsResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::webhook_api::v1::ListScheduledReactionsResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::webhook_api::v1::ListScheduledReactionsResponse>* __rust_proto_thunk__Map_bool_webhook_api_v1_ListScheduledReactionsResponse_new() {
    return new google::protobuf::Map<bool, ::webhook_api::v1::ListScheduledReactionsResponse>();
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_ListScheduledReactionsResponse_free(const google::protobuf::Map<bool, ::webhook_api::v1::ListScheduledReactionsResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_ListScheduledReactionsResponse_clear(google::protobuf::Map<bool, ::webhook_api::v1::ListScheduledReactionsResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_webhook_api_v1_ListScheduledReactionsResponse_size(const google::protobuf::Map<bool, ::webhook_api::v1::ListScheduledReactionsResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_ListScheduledReactionsResponse_insert(google::protobuf::Map<bool, ::webhook_api::v1::ListScheduledReactionsResponse> * m,
                          bool key, ::webhook_api::v1::ListScheduledReactionsResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_ListScheduledReactionsResponse_get(const google::protobuf::Map<bool, ::webhook_api::v1::ListScheduledReactionsResponse>* m,
                       bool key, const ::webhook_api::v1::ListScheduledReactionsResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ListScheduledReactionsResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_ListScheduledReactionsResponse_remove(google::protobuf::Map<bool, ::webhook_api::v1::ListScheduledReactionsResponse> * m,
                          bool key, ::webhook_api::v1::ListScheduledReactionsResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_webhook_api_v1_ListScheduledReactionsResponse_iter(
      const google::protobuf::Map<bool, ::webhook_api::v1::ListScheduledReactionsResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_ListScheduledReactionsResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::webhook_api::v1::ListScheduledReactionsResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::webhook_api::v1::ListScheduledReactionsResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::webhook_api::v1::ListScheduledReactionsResponse>* __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListScheduledReactionsResponse_new() {
    return new google::protobuf::Map<std::string, ::webhook_api::v1::ListScheduledReactionsResponse>();
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListScheduledReactionsResponse_free(const google::protobuf::Map<std::string, ::webhook_api::v1::ListScheduledReactionsResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListScheduledReactionsResponse_clear(google::protobuf::Map<std::string, ::webhook_api::v1::ListScheduledReactionsResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListScheduledReactionsResponse_size(const google::protobuf::Map<std::string, ::webhook_api::v1::ListScheduledReactionsResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListScheduledReactionsResponse_insert(google::protobuf::Map<std::string, ::webhook_api::v1::ListScheduledReactionsResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::ListScheduledReactionsResponse value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListScheduledReactionsResponse_get(const google::protobuf::Map<std::string, ::webhook_api::v1::ListScheduledReactionsResponse>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::webhook_api::v1::ListScheduledReactionsResponse** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ListScheduledReactionsResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListScheduledReactionsResponse_remove(google::protobuf::Map<std::string, ::webhook_api::v1::ListScheduledReactionsResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::ListScheduledReactionsResponse * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListScheduledReactionsResponse_iter(
      const google::protobuf::Map<std::string, ::webhook_api::v1::ListScheduledReactionsResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListScheduledReactionsResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::webhook_api::v1::ListScheduledReactionsResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::webhook_api::v1::ListScheduledReactionsResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// webhook_api.v1.DeleteScheduledReactionRequest
        // clang-format off
extern "C" {
void* __rust_proto_thunk__webhook_api_v1_DeleteScheduledReactionRequest_new() { return new ::webhook_api::v1::DeleteScheduledReactionRequest(); }
void __rust_proto_thunk__webhook_api_v1_DeleteScheduledReactionRequest_delete(void* ptr) { delete static_cast<::webhook_api::v1::DeleteScheduledReactionRequest*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__webhook_api_v1_DeleteScheduledReactionRequest_serialize(::webhook_api::v1::DeleteScheduledReactionRequest* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__webhook_api_v1_DeleteScheduledReactionRequest_parse(::webhook_api::v1::DeleteScheduledReactionRequest* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__webhook_api_v1_DeleteScheduledReactionRequest_copy_from(::webhook_api::v1::DeleteScheduledReactionRequest* dst, const ::webhook_api::v1::DeleteScheduledReactionRequest* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__webhook_api_v1_DeleteScheduledReactionRequest_repeated_len(google::protobuf::RepeatedPtrField<::webhook_api::v1::DeleteScheduledReactionRequest>* field) {
  return field->size();
}
const ::webhook_api::v1::DeleteScheduledReactionRequest& __rust_proto_thunk__webhook_api_v1_DeleteScheduledReactionRequest_repeated_get(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::DeleteScheduledReactionRequest>* field,
  size_t index) {
  return field->Get(index);
}
::webhook_api::v1::DeleteScheduledReactionRequest* __rust_proto_thunk__webhook_api_v1_DeleteScheduledReactionRequest_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::DeleteScheduledReactionRequest>* field,
  size_t index) {
  return field->Mutable(index);
}
::webhook_api::v1::DeleteScheduledReactionRequest* __rust_proto_thunk__webhook_api_v1_DeleteScheduledReactionRequest_repeated_add(google::protobuf::RepeatedPtrField<::webhook_api::v1::DeleteScheduledReactionRequest>* field) {
  return field->Add();
}
void __rust_proto_thunk__webhook_api_v1_DeleteScheduledReactionRequest_repeated_clear(google::protobuf::RepeatedPtrField<::webhook_api::v1::DeleteScheduledReactionRequest>* field) {
  field->Clear();
}
void __rust_proto_thunk__webhook_api_v1_DeleteScheduledReactionRequest_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::DeleteScheduledReactionRequest>& dst,
  const google::protobuf::RepeatedPtrField<::webhook_api::v1::DeleteScheduledReactionRequest>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__webhook_api_v1_DeleteScheduledReactionRequest_get_api_key(::webhook_api::v1::DeleteScheduledReactionRequest* msg) {
  absl::string_view val = msg->api_key();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__webhook_api_v1_DeleteScheduledReactionRequest_set_api_key(::webhook_api::v1::DeleteScheduledReactionRequest* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_api_key(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__webhook_api_v1_DeleteScheduledReactionRequest_get_scheduled_reaction_id(::webhook_api::v1::DeleteScheduledReactionRequest* msg) {
  absl::string_view val = msg->scheduled_reaction_id();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__webhook_api_v1_DeleteScheduledReactionRequest_set_scheduled_reaction_id(::webhook_api::v1::DeleteScheduledReactionRequest* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_scheduled_reaction_id(absl::string_view(s.ptr, s.len));
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::webhook_api::v1::DeleteScheduledReactionRequest>* __rust_proto_thunk__Map_i32_webhook_api_v1_DeleteScheduledReactionRequest_new() {
    return new google::protobuf::Map<int32_t, ::webhook_api::v1::DeleteScheduledReactionRequest>();
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_DeleteScheduledReactionRequest_free(const google::protobuf::Map<int32_t, ::webhook_api::v1::DeleteScheduledReactionRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_DeleteScheduledReactionRequest_clear(google::protobuf::Map<int32_t, ::webhook_api::v1::DeleteScheduledReactionRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_webhook_api_v1_DeleteScheduledReactionRequest_size(const google::protobuf::Map<int32_t, ::webhook_api::v1::DeleteScheduledReactionRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_DeleteScheduledReactionRequest_insert(google::protobuf::Map<int32_t, ::webhook_api::v1::DeleteScheduledReactionRequest> * m,
                          int32_t key, ::webhook_api::v1::DeleteScheduledReactionRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_DeleteScheduledReactionRequest_get(const google::protobuf::Map<int32_t, ::webhook_api::v1::DeleteScheduledReactionRequest>* m,
                       int32_t key, const ::webhook_api::v1::DeleteScheduledReactionRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::DeleteScheduledReactionRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_DeleteScheduledReactionRequest_remove(google::protobuf::Map<int32_t, ::webhook_api::v1::DeleteScheduledReactionRequest> * m,
                          int32_t key, ::webhook_api::v1::DeleteScheduledReactionRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_webhook_api_v1_DeleteScheduledReactionRequest_iter(
      const google::protobuf::Map<int32_t, ::webhook_api::v1::DeleteScheduledReactionRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_DeleteScheduledReactionRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::webhook_api::v1::DeleteScheduledReactionRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::webhook_api::v1::DeleteScheduledReactionRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::webhook_api::v1::DeleteScheduledReactionRequest>* __rust_proto_thunk__Map_u32_webhook_api_v1_DeleteScheduledReactionRequest_new() {
    return new google::protobuf::Map<uint32_t, ::webhook_api::v1::DeleteScheduledReactionRequest>();
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_DeleteScheduledReactionRequest_free(const google::protobuf::Map<uint32_t, ::webhook_api::v1::DeleteScheduledReactionRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_DeleteScheduledReactionRequest_clear(google::protobuf::Map<uint32_t, ::webhook_api::v1::DeleteScheduledReactionRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_webhook_api_v1_DeleteScheduledReactionRequest_size(const google::protobuf::Map<uint32_t, ::webhook_api::v1::DeleteScheduledReactionRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_DeleteScheduledReactionRequest_insert(google::protobuf::Map<uint32_t, ::webhook_api::v1::DeleteScheduledReactionRequest> * m,
                          uint32_t key, ::webhook_api::v1::DeleteScheduledReactionRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_DeleteScheduledReactionRequest_get(const google::protobuf::Map<uint32_t, ::webhook_api::v1::DeleteScheduledReactionRequest>* m,
                       uint32_t key, const ::webhook_api::v1::DeleteScheduledReactionRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::DeleteScheduledReactionRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_DeleteScheduledReactionRequest_remove(google::protobuf::Map<uint32_t, ::webhook_api::v1::DeleteScheduledReactionRequest> * m,
                          uint32_t key, ::webhook_api::v1::DeleteScheduledReactionRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_webhook_api_v1_DeleteScheduledReactionRequest_iter(
      const google::protobuf::Map<uint32_t, ::webhook_api::v1::DeleteScheduledReactionRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_DeleteScheduledReactionRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::webhook_api::v1::DeleteScheduledReactionRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::webhook_api::v1::DeleteScheduledReactionRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::webhook_api::v1::DeleteScheduledReactionRequest>* __rust_proto_thunk__Map_i64_webhook_api_v1_DeleteScheduledReactionRequest_new() {
    return new google::protobuf::Map<int64_t, ::webhook_api::v1::DeleteScheduledReactionRequest>();
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_DeleteScheduledReactionRequest_free(const google::protobuf::Map<int64_t, ::webhook_api::v1::DeleteScheduledReactionRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_DeleteScheduledReactionRequest_clear(google::protobuf::Map<int64_t, ::webhook_api::v1::DeleteScheduledReactionRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_webhook_api_v1_DeleteScheduledReactionRequest_size(const google::protobuf::Map<int64_t, ::webhook_api::v1::DeleteScheduledReactionRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_DeleteScheduledReactionRequest_insert(google::protobuf::Map<int64_t, ::webhook_api::v1::DeleteScheduledReactionRequest> * m,
                          int64_t key, ::webhook_api::v1::DeleteScheduledReactionRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_DeleteScheduledReactionRequest_get(const google::protobuf::Map<int64_t, ::webhook_api::v1::DeleteScheduledReactionRequest>* m,
                       int64_t key, const ::webhook_api::v1::DeleteScheduledReactionRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::DeleteScheduledReactionRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_DeleteScheduledReactionRequest_remove(google::protobuf::Map<int64_t, ::webhook_api::v1::DeleteScheduledReactionRequest> * m,
                          int64_t key, ::webhook_api::v1::DeleteScheduledReactionRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_webhook_api_v1_DeleteScheduledReactionRequest_iter(
      const google::protobuf::Map<int64_t, ::webhook_api::v1::DeleteScheduledReactionRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_DeleteScheduledReactionRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::webhook_api::v1::DeleteScheduledReactionRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::webhook_api::v1::DeleteScheduledReactionRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::webhook_api::v1::DeleteScheduledReactionRequest>* __rust_proto_thunk__Map_u64_webhook_api_v1_DeleteScheduledReactionRequest_new() {
    return new google::protobuf::Map<uint64_t, ::webhook_api::v1::DeleteScheduledReactionRequest>();
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_DeleteScheduledReactionRequest_free(const google::protobuf::Map<uint64_t, ::webhook_api::v1::DeleteScheduledReactionRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_DeleteScheduledReactionRequest_clear(google::protobuf::Map<uint64_t, ::webhook_api::v1::DeleteScheduledReactionRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_webhook_api_v1_DeleteScheduledReactionRequest_size(const google::protobuf::Map<uint64_t, ::webhook_api::v1::DeleteScheduledReactionRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_DeleteScheduledReactionRequest_insert(google::protobuf::Map<uint64_t, ::webhook_api::v1::DeleteScheduledReactionRequest> * m,
                          uint64_t key, ::webhook_api::v1::DeleteScheduledReactionRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_DeleteScheduledReactionRequest_get(const google::protobuf::Map<uint64_t, ::webhook_api::v1::DeleteScheduledReactionRequest>* m,
                       uint64_t key, const ::webhook_api::v1::DeleteScheduledReactionRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::DeleteScheduledReactionRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_DeleteScheduledReactionRequest_remove(google::protobuf::Map<uint64_t, ::webhook_api::v1::DeleteScheduledReactionRequest> * m,
                          uint64_t key, ::webhook_api::v1::DeleteScheduledReactionRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_webhook_api_v1_DeleteScheduledReactionRequest_iter(
      const google::protobuf::Map<uint64_t, ::webhook_api::v1::DeleteScheduledReactionRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_DeleteScheduledReactionRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::webhook_api::v1::DeleteScheduledReactionRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::webhook_api::v1::DeleteScheduledReactionRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::webhook_api::v1::DeleteScheduledReactionRequest>* __rust_proto_thunk__Map_bool_webhook_api_v1_DeleteScheduledReactionRequest_new() {
    return new google::protobuf::Map<bool, ::webhook_api::v1::DeleteScheduledReactionRequest>();
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_DeleteScheduledReactionRequest_free(const google::protobuf::Map<bool, ::webhook_api::v1::DeleteScheduledReactionRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_DeleteScheduledReactionRequest_clear(google::protobuf::Map<bool, ::webhook_api::v1::DeleteScheduledReactionRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_webhook_api_v1_DeleteScheduledReactionRequest_size(const google::protobuf::Map<bool, ::webhook_api::v1::DeleteScheduledReactionRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_DeleteScheduledReactionRequest_insert(google::protobuf::Map<bool, ::webhook_api::v1::DeleteScheduledReactionRequest> * m,
                          bool key, ::webhook_api::v1::DeleteScheduledReactionRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_DeleteScheduledReactionRequest_get(const google::protobuf::Map<bool, ::webhook_api::v1::DeleteScheduledReactionRequest>* m,
                       bool key, const ::webhook_api::v1::DeleteScheduledReactionRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::DeleteScheduledReactionRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_DeleteScheduledReactionRequest_remove(google::protobuf::Map<bool, ::webhook_api::v1::DeleteScheduledReactionRequest> * m,
                          bool key, ::webhook_api::v1::DeleteScheduledReactionRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_webhook_api_v1_DeleteScheduledReactionRequest_iter(
      const google::protobuf::Map<bool, ::webhook_api::v1::DeleteScheduledReactionRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_DeleteScheduledReactionRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::webhook_api::v1::DeleteScheduledReactionRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::webhook_api::v1::DeleteScheduledReactionRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::webhook_api::v1::DeleteScheduledReactionRequest>* __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_DeleteScheduledReactionRequest_new() {
    return new google::protobuf::Map<std::string, ::webhook_api::v1::DeleteScheduledReactionRequest>();
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_DeleteScheduledReactionRequest_free(const google::protobuf::Map<std::string, ::webhook_api::v1::DeleteScheduledReactionRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_DeleteScheduledReactionRequest_clear(google::protobuf::Map<std::string, ::webhook_api::v1::DeleteScheduledReactionRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_DeleteScheduledReactionRequest_size(const google::protobuf::Map<std::string, ::webhook_api::v1::DeleteScheduledReactionRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_DeleteScheduledReactionRequest_insert(google::protobuf::Map<std::string, ::webhook_api::v1::DeleteScheduledReactionRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::DeleteScheduledReactionRequest value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_DeleteScheduledReactionRequest_get(const google::protobuf::Map<std::string, ::webhook_api::v1::DeleteScheduledReactionRequest>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::webhook_api::v1::DeleteScheduledReactionRequest** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::DeleteScheduledReactionRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_DeleteScheduledReactionRequest_remove(google::protobuf::Map<std::string, ::webhook_api::v1::DeleteScheduledReactionRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::DeleteScheduledReactionRequest * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_DeleteScheduledReactionRequest_iter(
      const google::protobuf::Map<std::string, ::webhook_api::v1::DeleteScheduledReactionRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_DeleteScheduledReactionRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::webhook_api::v1::DeleteScheduledReactionRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::webhook_api::v1::DeleteScheduledReactionRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// webhook_api.v1.DeleteScheduledReactionResponse
        // clang-format off
extern "C" {
void* __rust_proto_thunk__webhook_api_v1_DeleteScheduledReactionResponse_new() { return new ::webhook_api::v1::DeleteScheduledReactionResponse(); }
void __rust_proto_thunk__webhook_api_v1_DeleteScheduledReactionResponse_delete(void* ptr) { delete static_cast<::webhook_api::v1::DeleteScheduledReactionResponse*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__webhook_api_v1_DeleteScheduledReactionResponse_serialize(::webhook_api::v1::DeleteScheduledReactionResponse* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__webhook_api_v1_DeleteScheduledReactionResponse_parse(::webhook_api::v1::DeleteScheduledReactionResponse* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__webhook_api_v1_DeleteScheduledReactionResponse_copy_from(::webhook_api::v1::DeleteScheduledReactionResponse* dst, const ::webhook_api::v1::DeleteScheduledReactionResponse* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__webhook_api_v1_DeleteScheduledReactionResponse_repeated_len(google::protobuf::RepeatedPtrField<::webhook_api::v1::DeleteScheduledReactionResponse>* field) {
  return field->size();
}
const ::webhook_api::v1::DeleteScheduledReactionResponse& __rust_proto_thunk__webhook_api_v1_DeleteScheduledReactionResponse_repeated_get(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::DeleteScheduledReactionResponse>* field,
  size_t index) {
  return field->Get(index);
}
::webhook_api::v1::DeleteScheduledReactionResponse* __rust_proto_thunk__webhook_api_v1_DeleteScheduledReactionResponse_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::DeleteScheduledReactionResponse>* field,
  size_t index) {
  return field->Mutable(index);
}
::webhook_api::v1::DeleteScheduledReactionResponse* __rust_proto_thunk__webhook_api_v1_DeleteScheduledReactionResponse_repeated_add(google::protobuf::RepeatedPtrField<::webhook_api::v1::DeleteScheduledReactionResponse>* field) {
  return field->Add();
}
void __rust_proto_thunk__webhook_api_v1_DeleteScheduledReactionResponse_repeated_clear(google::protobuf::RepeatedPtrField<::webhook_api::v1::DeleteScheduledReactionResponse>* field) {
  field->Clear();
}
void __rust_proto_thunk__webhook_api_v1_DeleteScheduledReactionResponse_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::DeleteScheduledReactionResponse>& dst,
  const google::protobuf::RepeatedPtrField<::webhook_api::v1::DeleteScheduledReactionResponse>& src) {
  dst = src;
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::webhook_api::v1::DeleteScheduledReactionResponse>* __rust_proto_thunk__Map_i32_webhook_api_v1_DeleteScheduledReactionResponse_new() {
    return new google::protobuf::Map<int32_t, ::webhook_api::v1::DeleteScheduledReactionResponse>();
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_DeleteScheduledReactionResponse_free(const google::protobuf::Map<int32_t, ::webhook_api::v1::DeleteScheduledReactionResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_DeleteScheduledReactionResponse_clear(google::protobuf::Map<int32_t, ::webhook_api::v1::DeleteScheduledReactionResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_webhook_api_v1_DeleteScheduledReactionResponse_size(const google::protobuf::Map<int32_t, ::webhook_api::v1::DeleteScheduledReactionResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_DeleteScheduledReactionResponse_insert(google::protobuf::Map<int32_t, ::webhook_api::v1::DeleteScheduledReactionResponse> * m,
                          int32_t key, ::webhook_api::v1::DeleteScheduledReactionResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_DeleteScheduledReactionResponse_get(const google::protobuf::Map<int32_t, ::webhook_api::v1::DeleteScheduledReactionResponse>* m,
                       int32_t key, const ::webhook_api::v1::DeleteScheduledReactionResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::DeleteScheduledReactionResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_DeleteScheduledReactionResponse_remove(google::protobuf::Map<int32_t, ::webhook_api::v1::DeleteScheduledReactionResponse> * m,
                          int32_t key, ::webhook_api::v1::DeleteScheduledReactionResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_webhook_api_v1_DeleteScheduledReactionResponse_iter(
      const google::protobuf::Map<int32_t, ::webhook_api::v1::DeleteScheduledReactionResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_DeleteScheduledReactionResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::webhook_api::v1::DeleteScheduledReactionResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::webhook_api::v1::DeleteScheduledReactionResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::webhook_api::v1::DeleteScheduledReactionResponse>* __rust_proto_thunk__Map_u32_webhook_api_v1_DeleteScheduledReactionResponse_new() {
    return new google::protobuf::Map<uint32_t, ::webhook_api::v1::DeleteScheduledReactionResponse>();
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_DeleteScheduledReactionResponse_free(const google::protobuf::Map<uint32_t, ::webhook_api::v1::DeleteScheduledReactionResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_DeleteScheduledReactionResponse_clear(google::protobuf::Map<uint32_t, ::webhook_api::v1::DeleteScheduledReactionResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_webhook_api_v1_DeleteScheduledReactionResponse_size(const google::protobuf::Map<uint32_t, ::webhook_api::v1::DeleteScheduledReactionResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_DeleteScheduledReactionResponse_insert(google::protobuf::Map<uint32_t, ::webhook_api::v1::DeleteScheduledReactionResponse> * m,
                          uint32_t key, ::webhook_api::v1::DeleteScheduledReactionResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_DeleteScheduledReactionResponse_get(const google::protobuf::Map<uint32_t, ::webhook_api::v1::DeleteScheduledReactionResponse>* m,
                       uint32_t key, const ::webhook_api::v1::DeleteScheduledReactionResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::DeleteScheduledReactionResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_DeleteScheduledReactionResponse_remove(google::protobuf::Map<uint32_t, ::webhook_api::v1::DeleteScheduledReactionResponse> * m,
                          uint32_t key, ::webhook_api::v1::DeleteScheduledReactionResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_webhook_api_v1_DeleteScheduledReactionResponse_iter(
      const google::protobuf::Map<uint32_t, ::webhook_api::v1::DeleteScheduledReactionResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_DeleteScheduledReactionResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::webhook_api::v1::DeleteScheduledReactionResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::webhook_api::v1::DeleteScheduledReactionResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::webhook_api::v1::DeleteScheduledReactionResponse>* __rust_proto_thunk__Map_i64_webhook_api_v1_DeleteScheduledReactionResponse_new() {
    return new google::protobuf::Map<int64_t, ::webhook_api::v1::DeleteScheduledReactionResponse>();
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_DeleteScheduledReactionResponse_free(const google::protobuf::Map<int64_t, ::webhook_api::v1::DeleteScheduledReactionResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_DeleteScheduledReactionResponse_clear(google::protobuf::Map<int64_t, ::webhook_api::v1::DeleteScheduledReactionResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_webhook_api_v1_DeleteScheduledReactionResponse_size(const google::protobuf::Map<int64_t, ::webhook_api::v1::DeleteScheduledReactionResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_DeleteScheduledReactionResponse_insert(google::protobuf::Map<int64_t, ::webhook_api::v1::DeleteScheduledReactionResponse> * m,
                          int64_t key, ::webhook_api::v1::DeleteScheduledReactionResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_DeleteScheduledReactionResponse_get(const google::protobuf::Map<int64_t, ::webhook_api::v1::DeleteScheduledReactionResponse>* m,
                       int64_t key, const ::webhook_api::v1::DeleteScheduledReactionResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::DeleteScheduledReactionResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_DeleteScheduledReactionResponse_remove(google::protobuf::Map<int64_t, ::webhook_api::v1::DeleteScheduledReactionResponse> * m,
                          int64_t key, ::webhook_api::v1::DeleteScheduledReactionResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_webhook_api_v1_DeleteScheduledReactionResponse_iter(
      const google::protobuf::Map<int64_t, ::webhook_api::v1::DeleteScheduledReactionResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_DeleteScheduledReactionResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::webhook_api::v1::DeleteScheduledReactionResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::webhook_api::v1::DeleteScheduledReactionResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::webhook_api::v1::DeleteScheduledReactionResponse>* __rust_proto_thunk__Map_u64_webhook_api_v1_DeleteScheduledReactionResponse_new() {
    return new google::protobuf::Map<uint64_t, ::webhook_api::v1::DeleteScheduledReactionResponse>();
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_DeleteScheduledReactionResponse_free(const google::protobuf::Map<uint64_t, ::webhook_api::v1::DeleteScheduledReactionResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_DeleteScheduledReactionResponse_clear(google::protobuf::Map<uint64_t, ::webhook_api::v1::DeleteScheduledReactionResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_webhook_api_v1_DeleteScheduledReactionResponse_size(const google::protobuf::Map<uint64_t, ::webhook_api::v1::DeleteScheduledReactionResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_DeleteScheduledReactionResponse_insert(google::protobuf::Map<uint64_t, ::webhook_api::v1::DeleteScheduledReactionResponse> * m,
                          uint64_t key, ::webhook_api::v1::DeleteScheduledReactionResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_DeleteScheduledReactionResponse_get(const google::protobuf::Map<uint64_t, ::webhook_api::v1::DeleteScheduledReactionResponse>* m,
                       uint64_t key, const ::webhook_api::v1::DeleteScheduledReactionResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::DeleteScheduledReactionResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_DeleteScheduledReactionResponse_remove(google::protobuf::Map<uint64_t, ::webhook_api::v1::DeleteScheduledReactionResponse> * m,
                          uint64_t key, ::webhook_api::v1::DeleteScheduledReactionResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_webhook_api_v1_DeleteScheduledReactionResponse_iter(
      const google::protobuf::Map<uint64_t, ::webhook_api::v1::DeleteScheduledReactionResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_DeleteScheduledReactionResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::webhook_api::v1::DeleteScheduledReactionResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::webhook_api::v1::DeleteScheduledReactionResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::webhook_api::v1::DeleteScheduledReactionResponse>* __rust_proto_thunk__Map_bool_webhook_api_v1_DeleteScheduledReactionResponse_new() {
    return new google::protobuf::Map<bool, ::webhook_api::v1::DeleteScheduledReactionResponse>();
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_DeleteScheduledReactionResponse_free(const google::protobuf::Map<bool, ::webhook_api::v1::DeleteScheduledReactionResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_DeleteScheduledReactionResponse_clear(google::protobuf::Map<bool, ::webhook_api::v1::DeleteScheduledReactionResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_webhook_api_v1_DeleteScheduledReactionResponse_size(const google::protobuf::Map<bool, ::webhook_api::v1::DeleteScheduledReactionResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_DeleteScheduledReactionResponse_insert(google::protobuf::Map<bool, ::webhook_api::v1::DeleteScheduledReactionResponse> * m,
                          bool key, ::webhook_api::v1::DeleteScheduledReactionResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_DeleteScheduledReactionResponse_get(const google::protobuf::Map<bool, ::webhook_api::v1::DeleteScheduledReactionResponse>* m,
                       bool key, const ::webhook_api::v1::DeleteScheduledReactionResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::DeleteScheduledReactionResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_DeleteScheduledReactionResponse_remove(google::protobuf::Map<bool, ::webhook_api::v1::DeleteScheduledReactionResponse> * m,
                          bool key, ::webhook_api::v1::DeleteScheduledReactionResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_webhook_api_v1_DeleteScheduledReactionResponse_iter(
      const google::protobuf::Map<bool, ::webhook_api::v1::DeleteScheduledReactionResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_DeleteScheduledReactionResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::webhook_api::v1::DeleteScheduledReactionResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::webhook_api::v1::DeleteScheduledReactionResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::webhook_api::v1::DeleteScheduledReactionResponse>* __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_DeleteScheduledReactionResponse_new() {
    return new google::protobuf::Map<std::string, ::webhook_api::v1::DeleteScheduledReactionResponse>();
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_DeleteScheduledReactionResponse_free(const google::protobuf::Map<std::string, ::webhook_api::v1::DeleteScheduledReactionResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_DeleteScheduledReactionResponse_clear(google::protobuf::Map<std::string, ::webhook_api::v1::DeleteScheduledReactionResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_DeleteScheduledReactionResponse_size(const google::protobuf::Map<std::string, ::webhook_api::v1::DeleteScheduledReactionResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_DeleteScheduledReactionResponse_insert(google::protobuf::Map<std::string, ::webhook_api::v1::DeleteScheduledReactionResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::DeleteScheduledReactionResponse value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_DeleteScheduledReactionResponse_get(const google::protobuf::Map<std::string, ::webhook_api::v1::DeleteScheduledReactionResponse>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::webhook_api::v1::DeleteScheduledReactionResponse** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::DeleteScheduledReactionResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_DeleteScheduledReactionResponse_remove(google::protobuf::Map<std::string, ::webhook_api::v1::DeleteScheduledReactionResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::DeleteScheduledReactionResponse * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_DeleteScheduledReactionResponse_iter(
      const google::protobuf::Map<std::string, ::webhook_api::v1::DeleteScheduledReactionResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_DeleteScheduledReactionResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::webhook_api::v1::DeleteScheduledReactionResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::webhook_api::v1::DeleteScheduledReactionResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// webhook_api.v1.Prevalidation
        // clang-format off
extern "C" {
void* __rust_proto_thunk__webhook_api_v1_Prevalidation_new() { return new ::webhook_api::v1::Prevalidation(); }
void __rust_proto_thunk__webhook_api_v1_Prevalidation_delete(void* ptr) { delete static_cast<::webhook_api::v1::Prevalidation*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__webhook_api_v1_Prevalidation_serialize(::webhook_api::v1::Prevalidation* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__webhook_api_v1_Prevalidation_parse(::webhook_api::v1::Prevalidation* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__webhook_api_v1_Prevalidation_copy_from(::webhook_api::v1::Prevalidation* dst, const ::webhook_api::v1::Prevalidation* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__webhook_api_v1_Prevalidation_repeated_len(google::protobuf::RepeatedPtrField<::webhook_api::v1::Prevalidation>* field) {
  return field->size();
}
const ::webhook_api::v1::Prevalidation& __rust_proto_thunk__webhook_api_v1_Prevalidation_repeated_get(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::Prevalidation>* field,
  size_t index) {
  return field->Get(index);
}
::webhook_api::v1::Prevalidation* __rust_proto_thunk__webhook_api_v1_Prevalidation_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::Prevalidation>* field,
  size_t index) {
  return field->Mutable(index);
}
::webhook_api::v1::Prevalidation* __rust_proto_thunk__webhook_api_v1_Prevalidation_repeated_add(google::protobuf::RepeatedPtrField<::webhook_api::v1::Prevalidation>* field) {
  return field->Add();
}
void __rust_proto_thunk__webhook_api_v1_Prevalidation_repeated_clear(google::protobuf::RepeatedPtrField<::webhook_api::v1::Prevalidation>* field) {
  field->Clear();
}
void __rust_proto_thunk__webhook_api_v1_Prevalidation_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::Prevalidation>& dst,
  const google::protobuf::RepeatedPtrField<::webhook_api::v1::Prevalidation>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__webhook_api_v1_Prevalidation_get_id(::webhook_api::v1::Prevalidation* msg) {
  absl::string_view val = msg->id();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__webhook_api_v1_Prevalidation_set_id(::webhook_api::v1::Prevalidation* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_id(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__webhook_api_v1_Prevalidation_get_name(::webhook_api::v1::Prevalidation* msg) {
  absl::string_view val = msg->name();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__webhook_api_v1_Prevalidation_set_name(::webhook_api::v1::Prevalidation* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_name(absl::string_view(s.ptr, s.len));
}

::webhook_api::v1::PrevalidationTriggerType __rust_proto_thunk__webhook_api_v1_Prevalidation_get_trigger_type(::webhook_api::v1::Prevalidation* msg) { return msg->trigger_type(); }
void __rust_proto_thunk__webhook_api_v1_Prevalidation_set_trigger_type(::webhook_api::v1::Prevalidation* msg, ::webhook_api::v1::PrevalidationTriggerType val) {
  msg->set_trigger_type(val);
}

const void* __rust_proto_thunk__webhook_api_v1_Prevalidation_get_considering_contract_prevalidation(::webhook_api::v1::Prevalidation* msg) {
  return static_cast<const void*>(&msg->considering_contract_prevalidation());
}
void* __rust_proto_thunk__webhook_api_v1_Prevalidation_get_mut_considering_contract_prevalidation(::webhook_api::v1::Prevalidation* msg) {
  return static_cast<void*>(msg->mutable_considering_contract_prevalidation());
}
void __rust_proto_thunk__webhook_api_v1_Prevalidation_clear_considering_contract_prevalidation(::webhook_api::v1::Prevalidation* msg) { msg->clear_considering_contract_prevalidation(); }
bool __rust_proto_thunk__webhook_api_v1_Prevalidation_has_considering_contract_prevalidation(::webhook_api::v1::Prevalidation* msg) { return msg->has_considering_contract_prevalidation(); }

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__webhook_api_v1_Prevalidation_get_created(::webhook_api::v1::Prevalidation* msg) {
  absl::string_view val = msg->created();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__webhook_api_v1_Prevalidation_set_created(::webhook_api::v1::Prevalidation* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_created(absl::string_view(s.ptr, s.len));
}


::webhook_api::v1::Prevalidation::PayloadCase __rust_proto_thunk__webhook_api_v1_Prevalidation_case_payload(::webhook_api::v1::Prevalidation* msg) {
  return msg->payload_case();
}
}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::webhook_api::v1::Prevalidation>* __rust_proto_thunk__Map_i32_webhook_api_v1_Prevalidation_new() {
    return new google::protobuf::Map<int32_t, ::webhook_api::v1::Prevalidation>();
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_Prevalidation_free(const google::protobuf::Map<int32_t, ::webhook_api::v1::Prevalidation>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_Prevalidation_clear(google::protobuf::Map<int32_t, ::webhook_api::v1::Prevalidation> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_webhook_api_v1_Prevalidation_size(const google::protobuf::Map<int32_t, ::webhook_api::v1::Prevalidation>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_Prevalidation_insert(google::protobuf::Map<int32_t, ::webhook_api::v1::Prevalidation> * m,
                          int32_t key, ::webhook_api::v1::Prevalidation value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_Prevalidation_get(const google::protobuf::Map<int32_t, ::webhook_api::v1::Prevalidation>* m,
                       int32_t key, const ::webhook_api::v1::Prevalidation** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::Prevalidation* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_Prevalidation_remove(google::protobuf::Map<int32_t, ::webhook_api::v1::Prevalidation> * m,
                          int32_t key, ::webhook_api::v1::Prevalidation * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_webhook_api_v1_Prevalidation_iter(
      const google::protobuf::Map<int32_t, ::webhook_api::v1::Prevalidation>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_Prevalidation_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::webhook_api::v1::Prevalidation** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::webhook_api::v1::Prevalidation>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::webhook_api::v1::Prevalidation>* __rust_proto_thunk__Map_u32_webhook_api_v1_Prevalidation_new() {
    return new google::protobuf::Map<uint32_t, ::webhook_api::v1::Prevalidation>();
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_Prevalidation_free(const google::protobuf::Map<uint32_t, ::webhook_api::v1::Prevalidation>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_Prevalidation_clear(google::protobuf::Map<uint32_t, ::webhook_api::v1::Prevalidation> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_webhook_api_v1_Prevalidation_size(const google::protobuf::Map<uint32_t, ::webhook_api::v1::Prevalidation>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_Prevalidation_insert(google::protobuf::Map<uint32_t, ::webhook_api::v1::Prevalidation> * m,
                          uint32_t key, ::webhook_api::v1::Prevalidation value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_Prevalidation_get(const google::protobuf::Map<uint32_t, ::webhook_api::v1::Prevalidation>* m,
                       uint32_t key, const ::webhook_api::v1::Prevalidation** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::Prevalidation* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_Prevalidation_remove(google::protobuf::Map<uint32_t, ::webhook_api::v1::Prevalidation> * m,
                          uint32_t key, ::webhook_api::v1::Prevalidation * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_webhook_api_v1_Prevalidation_iter(
      const google::protobuf::Map<uint32_t, ::webhook_api::v1::Prevalidation>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_Prevalidation_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::webhook_api::v1::Prevalidation** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::webhook_api::v1::Prevalidation>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::webhook_api::v1::Prevalidation>* __rust_proto_thunk__Map_i64_webhook_api_v1_Prevalidation_new() {
    return new google::protobuf::Map<int64_t, ::webhook_api::v1::Prevalidation>();
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_Prevalidation_free(const google::protobuf::Map<int64_t, ::webhook_api::v1::Prevalidation>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_Prevalidation_clear(google::protobuf::Map<int64_t, ::webhook_api::v1::Prevalidation> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_webhook_api_v1_Prevalidation_size(const google::protobuf::Map<int64_t, ::webhook_api::v1::Prevalidation>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_Prevalidation_insert(google::protobuf::Map<int64_t, ::webhook_api::v1::Prevalidation> * m,
                          int64_t key, ::webhook_api::v1::Prevalidation value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_Prevalidation_get(const google::protobuf::Map<int64_t, ::webhook_api::v1::Prevalidation>* m,
                       int64_t key, const ::webhook_api::v1::Prevalidation** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::Prevalidation* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_Prevalidation_remove(google::protobuf::Map<int64_t, ::webhook_api::v1::Prevalidation> * m,
                          int64_t key, ::webhook_api::v1::Prevalidation * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_webhook_api_v1_Prevalidation_iter(
      const google::protobuf::Map<int64_t, ::webhook_api::v1::Prevalidation>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_Prevalidation_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::webhook_api::v1::Prevalidation** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::webhook_api::v1::Prevalidation>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::webhook_api::v1::Prevalidation>* __rust_proto_thunk__Map_u64_webhook_api_v1_Prevalidation_new() {
    return new google::protobuf::Map<uint64_t, ::webhook_api::v1::Prevalidation>();
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_Prevalidation_free(const google::protobuf::Map<uint64_t, ::webhook_api::v1::Prevalidation>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_Prevalidation_clear(google::protobuf::Map<uint64_t, ::webhook_api::v1::Prevalidation> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_webhook_api_v1_Prevalidation_size(const google::protobuf::Map<uint64_t, ::webhook_api::v1::Prevalidation>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_Prevalidation_insert(google::protobuf::Map<uint64_t, ::webhook_api::v1::Prevalidation> * m,
                          uint64_t key, ::webhook_api::v1::Prevalidation value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_Prevalidation_get(const google::protobuf::Map<uint64_t, ::webhook_api::v1::Prevalidation>* m,
                       uint64_t key, const ::webhook_api::v1::Prevalidation** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::Prevalidation* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_Prevalidation_remove(google::protobuf::Map<uint64_t, ::webhook_api::v1::Prevalidation> * m,
                          uint64_t key, ::webhook_api::v1::Prevalidation * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_webhook_api_v1_Prevalidation_iter(
      const google::protobuf::Map<uint64_t, ::webhook_api::v1::Prevalidation>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_Prevalidation_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::webhook_api::v1::Prevalidation** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::webhook_api::v1::Prevalidation>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::webhook_api::v1::Prevalidation>* __rust_proto_thunk__Map_bool_webhook_api_v1_Prevalidation_new() {
    return new google::protobuf::Map<bool, ::webhook_api::v1::Prevalidation>();
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_Prevalidation_free(const google::protobuf::Map<bool, ::webhook_api::v1::Prevalidation>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_Prevalidation_clear(google::protobuf::Map<bool, ::webhook_api::v1::Prevalidation> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_webhook_api_v1_Prevalidation_size(const google::protobuf::Map<bool, ::webhook_api::v1::Prevalidation>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_Prevalidation_insert(google::protobuf::Map<bool, ::webhook_api::v1::Prevalidation> * m,
                          bool key, ::webhook_api::v1::Prevalidation value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_Prevalidation_get(const google::protobuf::Map<bool, ::webhook_api::v1::Prevalidation>* m,
                       bool key, const ::webhook_api::v1::Prevalidation** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::Prevalidation* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_Prevalidation_remove(google::protobuf::Map<bool, ::webhook_api::v1::Prevalidation> * m,
                          bool key, ::webhook_api::v1::Prevalidation * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_webhook_api_v1_Prevalidation_iter(
      const google::protobuf::Map<bool, ::webhook_api::v1::Prevalidation>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_Prevalidation_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::webhook_api::v1::Prevalidation** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::webhook_api::v1::Prevalidation>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::webhook_api::v1::Prevalidation>* __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_Prevalidation_new() {
    return new google::protobuf::Map<std::string, ::webhook_api::v1::Prevalidation>();
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_Prevalidation_free(const google::protobuf::Map<std::string, ::webhook_api::v1::Prevalidation>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_Prevalidation_clear(google::protobuf::Map<std::string, ::webhook_api::v1::Prevalidation> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_Prevalidation_size(const google::protobuf::Map<std::string, ::webhook_api::v1::Prevalidation>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_Prevalidation_insert(google::protobuf::Map<std::string, ::webhook_api::v1::Prevalidation> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::Prevalidation value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_Prevalidation_get(const google::protobuf::Map<std::string, ::webhook_api::v1::Prevalidation>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::webhook_api::v1::Prevalidation** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::Prevalidation* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_Prevalidation_remove(google::protobuf::Map<std::string, ::webhook_api::v1::Prevalidation> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::Prevalidation * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_Prevalidation_iter(
      const google::protobuf::Map<std::string, ::webhook_api::v1::Prevalidation>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_Prevalidation_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::webhook_api::v1::Prevalidation** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::webhook_api::v1::Prevalidation>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// webhook_api.v1.ValidationCondition
        // clang-format off
extern "C" {
void* __rust_proto_thunk__webhook_api_v1_ValidationCondition_new() { return new ::webhook_api::v1::ValidationCondition(); }
void __rust_proto_thunk__webhook_api_v1_ValidationCondition_delete(void* ptr) { delete static_cast<::webhook_api::v1::ValidationCondition*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__webhook_api_v1_ValidationCondition_serialize(::webhook_api::v1::ValidationCondition* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__webhook_api_v1_ValidationCondition_parse(::webhook_api::v1::ValidationCondition* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__webhook_api_v1_ValidationCondition_copy_from(::webhook_api::v1::ValidationCondition* dst, const ::webhook_api::v1::ValidationCondition* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__webhook_api_v1_ValidationCondition_repeated_len(google::protobuf::RepeatedPtrField<::webhook_api::v1::ValidationCondition>* field) {
  return field->size();
}
const ::webhook_api::v1::ValidationCondition& __rust_proto_thunk__webhook_api_v1_ValidationCondition_repeated_get(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::ValidationCondition>* field,
  size_t index) {
  return field->Get(index);
}
::webhook_api::v1::ValidationCondition* __rust_proto_thunk__webhook_api_v1_ValidationCondition_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::ValidationCondition>* field,
  size_t index) {
  return field->Mutable(index);
}
::webhook_api::v1::ValidationCondition* __rust_proto_thunk__webhook_api_v1_ValidationCondition_repeated_add(google::protobuf::RepeatedPtrField<::webhook_api::v1::ValidationCondition>* field) {
  return field->Add();
}
void __rust_proto_thunk__webhook_api_v1_ValidationCondition_repeated_clear(google::protobuf::RepeatedPtrField<::webhook_api::v1::ValidationCondition>* field) {
  field->Clear();
}
void __rust_proto_thunk__webhook_api_v1_ValidationCondition_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::ValidationCondition>& dst,
  const google::protobuf::RepeatedPtrField<::webhook_api::v1::ValidationCondition>& src) {
  dst = src;
}

::webhook_api::v1::ValidationTarget __rust_proto_thunk__webhook_api_v1_ValidationCondition_get_target(::webhook_api::v1::ValidationCondition* msg) { return msg->target(); }
void __rust_proto_thunk__webhook_api_v1_ValidationCondition_set_target(::webhook_api::v1::ValidationCondition* msg, ::webhook_api::v1::ValidationTarget val) {
  msg->set_target(val);
}

::webhook_api::v1::ValidationOperator __rust_proto_thunk__webhook_api_v1_ValidationCondition_get_operator(::webhook_api::v1::ValidationCondition* msg) { return msg->operator_(); }
void __rust_proto_thunk__webhook_api_v1_ValidationCondition_set_operator(::webhook_api::v1::ValidationCondition* msg, ::webhook_api::v1::ValidationOperator val) {
  msg->set_operator_(val);
}

const void* __rust_proto_thunk__webhook_api_v1_ValidationCondition_get_amount_value(::webhook_api::v1::ValidationCondition* msg) {
  return static_cast<const void*>(&msg->amount_value());
}
void* __rust_proto_thunk__webhook_api_v1_ValidationCondition_get_mut_amount_value(::webhook_api::v1::ValidationCondition* msg) {
  return static_cast<void*>(msg->mutable_amount_value());
}
void __rust_proto_thunk__webhook_api_v1_ValidationCondition_clear_amount_value(::webhook_api::v1::ValidationCondition* msg) { msg->clear_amount_value(); }
bool __rust_proto_thunk__webhook_api_v1_ValidationCondition_has_amount_value(::webhook_api::v1::ValidationCondition* msg) { return msg->has_amount_value(); }

bool __rust_proto_thunk__webhook_api_v1_ValidationCondition_has_string_value(::webhook_api::v1::ValidationCondition* msg) {
  return msg->has_string_value();
}
void __rust_proto_thunk__webhook_api_v1_ValidationCondition_clear_string_value(::webhook_api::v1::ValidationCondition* msg) { msg->clear_string_value(); }
::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__webhook_api_v1_ValidationCondition_get_string_value(::webhook_api::v1::ValidationCondition* msg) {
  absl::string_view val = msg->string_value();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__webhook_api_v1_ValidationCondition_set_string_value(::webhook_api::v1::ValidationCondition* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_string_value(absl::string_view(s.ptr, s.len));
}


::webhook_api::v1::ValidationCondition::ValueCase __rust_proto_thunk__webhook_api_v1_ValidationCondition_case_value(::webhook_api::v1::ValidationCondition* msg) {
  return msg->value_case();
}
}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::webhook_api::v1::ValidationCondition>* __rust_proto_thunk__Map_i32_webhook_api_v1_ValidationCondition_new() {
    return new google::protobuf::Map<int32_t, ::webhook_api::v1::ValidationCondition>();
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_ValidationCondition_free(const google::protobuf::Map<int32_t, ::webhook_api::v1::ValidationCondition>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_ValidationCondition_clear(google::protobuf::Map<int32_t, ::webhook_api::v1::ValidationCondition> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_webhook_api_v1_ValidationCondition_size(const google::protobuf::Map<int32_t, ::webhook_api::v1::ValidationCondition>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_ValidationCondition_insert(google::protobuf::Map<int32_t, ::webhook_api::v1::ValidationCondition> * m,
                          int32_t key, ::webhook_api::v1::ValidationCondition value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_ValidationCondition_get(const google::protobuf::Map<int32_t, ::webhook_api::v1::ValidationCondition>* m,
                       int32_t key, const ::webhook_api::v1::ValidationCondition** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ValidationCondition* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_ValidationCondition_remove(google::protobuf::Map<int32_t, ::webhook_api::v1::ValidationCondition> * m,
                          int32_t key, ::webhook_api::v1::ValidationCondition * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_webhook_api_v1_ValidationCondition_iter(
      const google::protobuf::Map<int32_t, ::webhook_api::v1::ValidationCondition>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_ValidationCondition_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::webhook_api::v1::ValidationCondition** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::webhook_api::v1::ValidationCondition>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::webhook_api::v1::ValidationCondition>* __rust_proto_thunk__Map_u32_webhook_api_v1_ValidationCondition_new() {
    return new google::protobuf::Map<uint32_t, ::webhook_api::v1::ValidationCondition>();
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_ValidationCondition_free(const google::protobuf::Map<uint32_t, ::webhook_api::v1::ValidationCondition>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_ValidationCondition_clear(google::protobuf::Map<uint32_t, ::webhook_api::v1::ValidationCondition> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_webhook_api_v1_ValidationCondition_size(const google::protobuf::Map<uint32_t, ::webhook_api::v1::ValidationCondition>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_ValidationCondition_insert(google::protobuf::Map<uint32_t, ::webhook_api::v1::ValidationCondition> * m,
                          uint32_t key, ::webhook_api::v1::ValidationCondition value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_ValidationCondition_get(const google::protobuf::Map<uint32_t, ::webhook_api::v1::ValidationCondition>* m,
                       uint32_t key, const ::webhook_api::v1::ValidationCondition** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ValidationCondition* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_ValidationCondition_remove(google::protobuf::Map<uint32_t, ::webhook_api::v1::ValidationCondition> * m,
                          uint32_t key, ::webhook_api::v1::ValidationCondition * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_webhook_api_v1_ValidationCondition_iter(
      const google::protobuf::Map<uint32_t, ::webhook_api::v1::ValidationCondition>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_ValidationCondition_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::webhook_api::v1::ValidationCondition** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::webhook_api::v1::ValidationCondition>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::webhook_api::v1::ValidationCondition>* __rust_proto_thunk__Map_i64_webhook_api_v1_ValidationCondition_new() {
    return new google::protobuf::Map<int64_t, ::webhook_api::v1::ValidationCondition>();
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_ValidationCondition_free(const google::protobuf::Map<int64_t, ::webhook_api::v1::ValidationCondition>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_ValidationCondition_clear(google::protobuf::Map<int64_t, ::webhook_api::v1::ValidationCondition> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_webhook_api_v1_ValidationCondition_size(const google::protobuf::Map<int64_t, ::webhook_api::v1::ValidationCondition>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_ValidationCondition_insert(google::protobuf::Map<int64_t, ::webhook_api::v1::ValidationCondition> * m,
                          int64_t key, ::webhook_api::v1::ValidationCondition value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_ValidationCondition_get(const google::protobuf::Map<int64_t, ::webhook_api::v1::ValidationCondition>* m,
                       int64_t key, const ::webhook_api::v1::ValidationCondition** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ValidationCondition* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_ValidationCondition_remove(google::protobuf::Map<int64_t, ::webhook_api::v1::ValidationCondition> * m,
                          int64_t key, ::webhook_api::v1::ValidationCondition * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_webhook_api_v1_ValidationCondition_iter(
      const google::protobuf::Map<int64_t, ::webhook_api::v1::ValidationCondition>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_ValidationCondition_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::webhook_api::v1::ValidationCondition** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::webhook_api::v1::ValidationCondition>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::webhook_api::v1::ValidationCondition>* __rust_proto_thunk__Map_u64_webhook_api_v1_ValidationCondition_new() {
    return new google::protobuf::Map<uint64_t, ::webhook_api::v1::ValidationCondition>();
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_ValidationCondition_free(const google::protobuf::Map<uint64_t, ::webhook_api::v1::ValidationCondition>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_ValidationCondition_clear(google::protobuf::Map<uint64_t, ::webhook_api::v1::ValidationCondition> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_webhook_api_v1_ValidationCondition_size(const google::protobuf::Map<uint64_t, ::webhook_api::v1::ValidationCondition>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_ValidationCondition_insert(google::protobuf::Map<uint64_t, ::webhook_api::v1::ValidationCondition> * m,
                          uint64_t key, ::webhook_api::v1::ValidationCondition value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_ValidationCondition_get(const google::protobuf::Map<uint64_t, ::webhook_api::v1::ValidationCondition>* m,
                       uint64_t key, const ::webhook_api::v1::ValidationCondition** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ValidationCondition* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_ValidationCondition_remove(google::protobuf::Map<uint64_t, ::webhook_api::v1::ValidationCondition> * m,
                          uint64_t key, ::webhook_api::v1::ValidationCondition * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_webhook_api_v1_ValidationCondition_iter(
      const google::protobuf::Map<uint64_t, ::webhook_api::v1::ValidationCondition>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_ValidationCondition_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::webhook_api::v1::ValidationCondition** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::webhook_api::v1::ValidationCondition>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::webhook_api::v1::ValidationCondition>* __rust_proto_thunk__Map_bool_webhook_api_v1_ValidationCondition_new() {
    return new google::protobuf::Map<bool, ::webhook_api::v1::ValidationCondition>();
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_ValidationCondition_free(const google::protobuf::Map<bool, ::webhook_api::v1::ValidationCondition>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_ValidationCondition_clear(google::protobuf::Map<bool, ::webhook_api::v1::ValidationCondition> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_webhook_api_v1_ValidationCondition_size(const google::protobuf::Map<bool, ::webhook_api::v1::ValidationCondition>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_ValidationCondition_insert(google::protobuf::Map<bool, ::webhook_api::v1::ValidationCondition> * m,
                          bool key, ::webhook_api::v1::ValidationCondition value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_ValidationCondition_get(const google::protobuf::Map<bool, ::webhook_api::v1::ValidationCondition>* m,
                       bool key, const ::webhook_api::v1::ValidationCondition** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ValidationCondition* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_ValidationCondition_remove(google::protobuf::Map<bool, ::webhook_api::v1::ValidationCondition> * m,
                          bool key, ::webhook_api::v1::ValidationCondition * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_webhook_api_v1_ValidationCondition_iter(
      const google::protobuf::Map<bool, ::webhook_api::v1::ValidationCondition>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_ValidationCondition_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::webhook_api::v1::ValidationCondition** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::webhook_api::v1::ValidationCondition>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::webhook_api::v1::ValidationCondition>* __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ValidationCondition_new() {
    return new google::protobuf::Map<std::string, ::webhook_api::v1::ValidationCondition>();
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ValidationCondition_free(const google::protobuf::Map<std::string, ::webhook_api::v1::ValidationCondition>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ValidationCondition_clear(google::protobuf::Map<std::string, ::webhook_api::v1::ValidationCondition> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ValidationCondition_size(const google::protobuf::Map<std::string, ::webhook_api::v1::ValidationCondition>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ValidationCondition_insert(google::protobuf::Map<std::string, ::webhook_api::v1::ValidationCondition> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::ValidationCondition value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ValidationCondition_get(const google::protobuf::Map<std::string, ::webhook_api::v1::ValidationCondition>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::webhook_api::v1::ValidationCondition** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ValidationCondition* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ValidationCondition_remove(google::protobuf::Map<std::string, ::webhook_api::v1::ValidationCondition> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::ValidationCondition * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ValidationCondition_iter(
      const google::protobuf::Map<std::string, ::webhook_api::v1::ValidationCondition>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ValidationCondition_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::webhook_api::v1::ValidationCondition** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::webhook_api::v1::ValidationCondition>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// webhook_api.v1.ConsideringContractPrevalidation
        // clang-format off
extern "C" {
void* __rust_proto_thunk__webhook_api_v1_ConsideringContractPrevalidation_new() { return new ::webhook_api::v1::ConsideringContractPrevalidation(); }
void __rust_proto_thunk__webhook_api_v1_ConsideringContractPrevalidation_delete(void* ptr) { delete static_cast<::webhook_api::v1::ConsideringContractPrevalidation*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__webhook_api_v1_ConsideringContractPrevalidation_serialize(::webhook_api::v1::ConsideringContractPrevalidation* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__webhook_api_v1_ConsideringContractPrevalidation_parse(::webhook_api::v1::ConsideringContractPrevalidation* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__webhook_api_v1_ConsideringContractPrevalidation_copy_from(::webhook_api::v1::ConsideringContractPrevalidation* dst, const ::webhook_api::v1::ConsideringContractPrevalidation* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__webhook_api_v1_ConsideringContractPrevalidation_repeated_len(google::protobuf::RepeatedPtrField<::webhook_api::v1::ConsideringContractPrevalidation>* field) {
  return field->size();
}
const ::webhook_api::v1::ConsideringContractPrevalidation& __rust_proto_thunk__webhook_api_v1_ConsideringContractPrevalidation_repeated_get(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::ConsideringContractPrevalidation>* field,
  size_t index) {
  return field->Get(index);
}
::webhook_api::v1::ConsideringContractPrevalidation* __rust_proto_thunk__webhook_api_v1_ConsideringContractPrevalidation_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::ConsideringContractPrevalidation>* field,
  size_t index) {
  return field->Mutable(index);
}
::webhook_api::v1::ConsideringContractPrevalidation* __rust_proto_thunk__webhook_api_v1_ConsideringContractPrevalidation_repeated_add(google::protobuf::RepeatedPtrField<::webhook_api::v1::ConsideringContractPrevalidation>* field) {
  return field->Add();
}
void __rust_proto_thunk__webhook_api_v1_ConsideringContractPrevalidation_repeated_clear(google::protobuf::RepeatedPtrField<::webhook_api::v1::ConsideringContractPrevalidation>* field) {
  field->Clear();
}
void __rust_proto_thunk__webhook_api_v1_ConsideringContractPrevalidation_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::ConsideringContractPrevalidation>& dst,
  const google::protobuf::RepeatedPtrField<::webhook_api::v1::ConsideringContractPrevalidation>& src) {
  dst = src;
}

void __rust_proto_thunk__webhook_api_v1_ConsideringContractPrevalidation_clear_conditions(::webhook_api::v1::ConsideringContractPrevalidation* msg) {
  msg->clear_conditions();
}
google::protobuf::RepeatedPtrField<::webhook_api::v1::ValidationCondition>* __rust_proto_thunk__webhook_api_v1_ConsideringContractPrevalidation_get_mut_conditions(::webhook_api::v1::ConsideringContractPrevalidation* msg) {
  return msg->mutable_conditions();
}
const google::protobuf::RepeatedPtrField<::webhook_api::v1::ValidationCondition>* __rust_proto_thunk__webhook_api_v1_ConsideringContractPrevalidation_get_conditions(
    const ::webhook_api::v1::ConsideringContractPrevalidation* msg) {
  return &msg->conditions();
}

::webhook_api::v1::ConsideringContractPrevalidation_ValidationOutcome __rust_proto_thunk__webhook_api_v1_ConsideringContractPrevalidation_get_outcome(::webhook_api::v1::ConsideringContractPrevalidation* msg) { return msg->outcome(); }
void __rust_proto_thunk__webhook_api_v1_ConsideringContractPrevalidation_set_outcome(::webhook_api::v1::ConsideringContractPrevalidation* msg, ::webhook_api::v1::ConsideringContractPrevalidation_ValidationOutcome val) {
  msg->set_outcome(val);
}

::common::LogicalOperator __rust_proto_thunk__webhook_api_v1_ConsideringContractPrevalidation_get_logical_base(::webhook_api::v1::ConsideringContractPrevalidation* msg) { return msg->logical_base(); }
void __rust_proto_thunk__webhook_api_v1_ConsideringContractPrevalidation_set_logical_base(::webhook_api::v1::ConsideringContractPrevalidation* msg, ::common::LogicalOperator val) {
  msg->set_logical_base(val);
}


}  // extern "C"
// clang-format on

extern "C" {
  __PB_RUST_EXPOSE_SCALAR_MAP_METHODS_FOR_VALUE_TYPE(
      ::webhook_api::v1::ConsideringContractPrevalidation_ValidationOutcome, webhook_api_v1_ConsideringContractPrevalidation_ValidationOutcome, ::webhook_api::v1::ConsideringContractPrevalidation_ValidationOutcome, value, cpp_value)
}
extern "C" {
  const google::protobuf::Map<int32_t, ::webhook_api::v1::ConsideringContractPrevalidation>* __rust_proto_thunk__Map_i32_webhook_api_v1_ConsideringContractPrevalidation_new() {
    return new google::protobuf::Map<int32_t, ::webhook_api::v1::ConsideringContractPrevalidation>();
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_ConsideringContractPrevalidation_free(const google::protobuf::Map<int32_t, ::webhook_api::v1::ConsideringContractPrevalidation>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_ConsideringContractPrevalidation_clear(google::protobuf::Map<int32_t, ::webhook_api::v1::ConsideringContractPrevalidation> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_webhook_api_v1_ConsideringContractPrevalidation_size(const google::protobuf::Map<int32_t, ::webhook_api::v1::ConsideringContractPrevalidation>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_ConsideringContractPrevalidation_insert(google::protobuf::Map<int32_t, ::webhook_api::v1::ConsideringContractPrevalidation> * m,
                          int32_t key, ::webhook_api::v1::ConsideringContractPrevalidation value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_ConsideringContractPrevalidation_get(const google::protobuf::Map<int32_t, ::webhook_api::v1::ConsideringContractPrevalidation>* m,
                       int32_t key, const ::webhook_api::v1::ConsideringContractPrevalidation** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ConsideringContractPrevalidation* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_ConsideringContractPrevalidation_remove(google::protobuf::Map<int32_t, ::webhook_api::v1::ConsideringContractPrevalidation> * m,
                          int32_t key, ::webhook_api::v1::ConsideringContractPrevalidation * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_webhook_api_v1_ConsideringContractPrevalidation_iter(
      const google::protobuf::Map<int32_t, ::webhook_api::v1::ConsideringContractPrevalidation>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_ConsideringContractPrevalidation_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::webhook_api::v1::ConsideringContractPrevalidation** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::webhook_api::v1::ConsideringContractPrevalidation>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::webhook_api::v1::ConsideringContractPrevalidation>* __rust_proto_thunk__Map_u32_webhook_api_v1_ConsideringContractPrevalidation_new() {
    return new google::protobuf::Map<uint32_t, ::webhook_api::v1::ConsideringContractPrevalidation>();
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_ConsideringContractPrevalidation_free(const google::protobuf::Map<uint32_t, ::webhook_api::v1::ConsideringContractPrevalidation>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_ConsideringContractPrevalidation_clear(google::protobuf::Map<uint32_t, ::webhook_api::v1::ConsideringContractPrevalidation> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_webhook_api_v1_ConsideringContractPrevalidation_size(const google::protobuf::Map<uint32_t, ::webhook_api::v1::ConsideringContractPrevalidation>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_ConsideringContractPrevalidation_insert(google::protobuf::Map<uint32_t, ::webhook_api::v1::ConsideringContractPrevalidation> * m,
                          uint32_t key, ::webhook_api::v1::ConsideringContractPrevalidation value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_ConsideringContractPrevalidation_get(const google::protobuf::Map<uint32_t, ::webhook_api::v1::ConsideringContractPrevalidation>* m,
                       uint32_t key, const ::webhook_api::v1::ConsideringContractPrevalidation** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ConsideringContractPrevalidation* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_ConsideringContractPrevalidation_remove(google::protobuf::Map<uint32_t, ::webhook_api::v1::ConsideringContractPrevalidation> * m,
                          uint32_t key, ::webhook_api::v1::ConsideringContractPrevalidation * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_webhook_api_v1_ConsideringContractPrevalidation_iter(
      const google::protobuf::Map<uint32_t, ::webhook_api::v1::ConsideringContractPrevalidation>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_ConsideringContractPrevalidation_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::webhook_api::v1::ConsideringContractPrevalidation** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::webhook_api::v1::ConsideringContractPrevalidation>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::webhook_api::v1::ConsideringContractPrevalidation>* __rust_proto_thunk__Map_i64_webhook_api_v1_ConsideringContractPrevalidation_new() {
    return new google::protobuf::Map<int64_t, ::webhook_api::v1::ConsideringContractPrevalidation>();
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_ConsideringContractPrevalidation_free(const google::protobuf::Map<int64_t, ::webhook_api::v1::ConsideringContractPrevalidation>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_ConsideringContractPrevalidation_clear(google::protobuf::Map<int64_t, ::webhook_api::v1::ConsideringContractPrevalidation> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_webhook_api_v1_ConsideringContractPrevalidation_size(const google::protobuf::Map<int64_t, ::webhook_api::v1::ConsideringContractPrevalidation>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_ConsideringContractPrevalidation_insert(google::protobuf::Map<int64_t, ::webhook_api::v1::ConsideringContractPrevalidation> * m,
                          int64_t key, ::webhook_api::v1::ConsideringContractPrevalidation value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_ConsideringContractPrevalidation_get(const google::protobuf::Map<int64_t, ::webhook_api::v1::ConsideringContractPrevalidation>* m,
                       int64_t key, const ::webhook_api::v1::ConsideringContractPrevalidation** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ConsideringContractPrevalidation* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_ConsideringContractPrevalidation_remove(google::protobuf::Map<int64_t, ::webhook_api::v1::ConsideringContractPrevalidation> * m,
                          int64_t key, ::webhook_api::v1::ConsideringContractPrevalidation * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_webhook_api_v1_ConsideringContractPrevalidation_iter(
      const google::protobuf::Map<int64_t, ::webhook_api::v1::ConsideringContractPrevalidation>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_ConsideringContractPrevalidation_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::webhook_api::v1::ConsideringContractPrevalidation** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::webhook_api::v1::ConsideringContractPrevalidation>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::webhook_api::v1::ConsideringContractPrevalidation>* __rust_proto_thunk__Map_u64_webhook_api_v1_ConsideringContractPrevalidation_new() {
    return new google::protobuf::Map<uint64_t, ::webhook_api::v1::ConsideringContractPrevalidation>();
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_ConsideringContractPrevalidation_free(const google::protobuf::Map<uint64_t, ::webhook_api::v1::ConsideringContractPrevalidation>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_ConsideringContractPrevalidation_clear(google::protobuf::Map<uint64_t, ::webhook_api::v1::ConsideringContractPrevalidation> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_webhook_api_v1_ConsideringContractPrevalidation_size(const google::protobuf::Map<uint64_t, ::webhook_api::v1::ConsideringContractPrevalidation>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_ConsideringContractPrevalidation_insert(google::protobuf::Map<uint64_t, ::webhook_api::v1::ConsideringContractPrevalidation> * m,
                          uint64_t key, ::webhook_api::v1::ConsideringContractPrevalidation value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_ConsideringContractPrevalidation_get(const google::protobuf::Map<uint64_t, ::webhook_api::v1::ConsideringContractPrevalidation>* m,
                       uint64_t key, const ::webhook_api::v1::ConsideringContractPrevalidation** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ConsideringContractPrevalidation* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_ConsideringContractPrevalidation_remove(google::protobuf::Map<uint64_t, ::webhook_api::v1::ConsideringContractPrevalidation> * m,
                          uint64_t key, ::webhook_api::v1::ConsideringContractPrevalidation * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_webhook_api_v1_ConsideringContractPrevalidation_iter(
      const google::protobuf::Map<uint64_t, ::webhook_api::v1::ConsideringContractPrevalidation>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_ConsideringContractPrevalidation_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::webhook_api::v1::ConsideringContractPrevalidation** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::webhook_api::v1::ConsideringContractPrevalidation>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::webhook_api::v1::ConsideringContractPrevalidation>* __rust_proto_thunk__Map_bool_webhook_api_v1_ConsideringContractPrevalidation_new() {
    return new google::protobuf::Map<bool, ::webhook_api::v1::ConsideringContractPrevalidation>();
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_ConsideringContractPrevalidation_free(const google::protobuf::Map<bool, ::webhook_api::v1::ConsideringContractPrevalidation>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_ConsideringContractPrevalidation_clear(google::protobuf::Map<bool, ::webhook_api::v1::ConsideringContractPrevalidation> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_webhook_api_v1_ConsideringContractPrevalidation_size(const google::protobuf::Map<bool, ::webhook_api::v1::ConsideringContractPrevalidation>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_ConsideringContractPrevalidation_insert(google::protobuf::Map<bool, ::webhook_api::v1::ConsideringContractPrevalidation> * m,
                          bool key, ::webhook_api::v1::ConsideringContractPrevalidation value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_ConsideringContractPrevalidation_get(const google::protobuf::Map<bool, ::webhook_api::v1::ConsideringContractPrevalidation>* m,
                       bool key, const ::webhook_api::v1::ConsideringContractPrevalidation** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ConsideringContractPrevalidation* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_ConsideringContractPrevalidation_remove(google::protobuf::Map<bool, ::webhook_api::v1::ConsideringContractPrevalidation> * m,
                          bool key, ::webhook_api::v1::ConsideringContractPrevalidation * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_webhook_api_v1_ConsideringContractPrevalidation_iter(
      const google::protobuf::Map<bool, ::webhook_api::v1::ConsideringContractPrevalidation>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_ConsideringContractPrevalidation_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::webhook_api::v1::ConsideringContractPrevalidation** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::webhook_api::v1::ConsideringContractPrevalidation>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::webhook_api::v1::ConsideringContractPrevalidation>* __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ConsideringContractPrevalidation_new() {
    return new google::protobuf::Map<std::string, ::webhook_api::v1::ConsideringContractPrevalidation>();
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ConsideringContractPrevalidation_free(const google::protobuf::Map<std::string, ::webhook_api::v1::ConsideringContractPrevalidation>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ConsideringContractPrevalidation_clear(google::protobuf::Map<std::string, ::webhook_api::v1::ConsideringContractPrevalidation> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ConsideringContractPrevalidation_size(const google::protobuf::Map<std::string, ::webhook_api::v1::ConsideringContractPrevalidation>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ConsideringContractPrevalidation_insert(google::protobuf::Map<std::string, ::webhook_api::v1::ConsideringContractPrevalidation> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::ConsideringContractPrevalidation value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ConsideringContractPrevalidation_get(const google::protobuf::Map<std::string, ::webhook_api::v1::ConsideringContractPrevalidation>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::webhook_api::v1::ConsideringContractPrevalidation** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ConsideringContractPrevalidation* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ConsideringContractPrevalidation_remove(google::protobuf::Map<std::string, ::webhook_api::v1::ConsideringContractPrevalidation> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::ConsideringContractPrevalidation * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ConsideringContractPrevalidation_iter(
      const google::protobuf::Map<std::string, ::webhook_api::v1::ConsideringContractPrevalidation>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ConsideringContractPrevalidation_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::webhook_api::v1::ConsideringContractPrevalidation** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::webhook_api::v1::ConsideringContractPrevalidation>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// webhook_api.v1.CreatePrevalidationRequest
        // clang-format off
extern "C" {
void* __rust_proto_thunk__webhook_api_v1_CreatePrevalidationRequest_new() { return new ::webhook_api::v1::CreatePrevalidationRequest(); }
void __rust_proto_thunk__webhook_api_v1_CreatePrevalidationRequest_delete(void* ptr) { delete static_cast<::webhook_api::v1::CreatePrevalidationRequest*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__webhook_api_v1_CreatePrevalidationRequest_serialize(::webhook_api::v1::CreatePrevalidationRequest* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__webhook_api_v1_CreatePrevalidationRequest_parse(::webhook_api::v1::CreatePrevalidationRequest* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__webhook_api_v1_CreatePrevalidationRequest_copy_from(::webhook_api::v1::CreatePrevalidationRequest* dst, const ::webhook_api::v1::CreatePrevalidationRequest* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__webhook_api_v1_CreatePrevalidationRequest_repeated_len(google::protobuf::RepeatedPtrField<::webhook_api::v1::CreatePrevalidationRequest>* field) {
  return field->size();
}
const ::webhook_api::v1::CreatePrevalidationRequest& __rust_proto_thunk__webhook_api_v1_CreatePrevalidationRequest_repeated_get(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::CreatePrevalidationRequest>* field,
  size_t index) {
  return field->Get(index);
}
::webhook_api::v1::CreatePrevalidationRequest* __rust_proto_thunk__webhook_api_v1_CreatePrevalidationRequest_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::CreatePrevalidationRequest>* field,
  size_t index) {
  return field->Mutable(index);
}
::webhook_api::v1::CreatePrevalidationRequest* __rust_proto_thunk__webhook_api_v1_CreatePrevalidationRequest_repeated_add(google::protobuf::RepeatedPtrField<::webhook_api::v1::CreatePrevalidationRequest>* field) {
  return field->Add();
}
void __rust_proto_thunk__webhook_api_v1_CreatePrevalidationRequest_repeated_clear(google::protobuf::RepeatedPtrField<::webhook_api::v1::CreatePrevalidationRequest>* field) {
  field->Clear();
}
void __rust_proto_thunk__webhook_api_v1_CreatePrevalidationRequest_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::CreatePrevalidationRequest>& dst,
  const google::protobuf::RepeatedPtrField<::webhook_api::v1::CreatePrevalidationRequest>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__webhook_api_v1_CreatePrevalidationRequest_get_api_key(::webhook_api::v1::CreatePrevalidationRequest* msg) {
  absl::string_view val = msg->api_key();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__webhook_api_v1_CreatePrevalidationRequest_set_api_key(::webhook_api::v1::CreatePrevalidationRequest* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_api_key(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__webhook_api_v1_CreatePrevalidationRequest_get_name(::webhook_api::v1::CreatePrevalidationRequest* msg) {
  absl::string_view val = msg->name();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__webhook_api_v1_CreatePrevalidationRequest_set_name(::webhook_api::v1::CreatePrevalidationRequest* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_name(absl::string_view(s.ptr, s.len));
}

::webhook_api::v1::PrevalidationTriggerType __rust_proto_thunk__webhook_api_v1_CreatePrevalidationRequest_get_trigger_type(::webhook_api::v1::CreatePrevalidationRequest* msg) { return msg->trigger_type(); }
void __rust_proto_thunk__webhook_api_v1_CreatePrevalidationRequest_set_trigger_type(::webhook_api::v1::CreatePrevalidationRequest* msg, ::webhook_api::v1::PrevalidationTriggerType val) {
  msg->set_trigger_type(val);
}

const void* __rust_proto_thunk__webhook_api_v1_CreatePrevalidationRequest_get_considering_contract_prevalidation(::webhook_api::v1::CreatePrevalidationRequest* msg) {
  return static_cast<const void*>(&msg->considering_contract_prevalidation());
}
void* __rust_proto_thunk__webhook_api_v1_CreatePrevalidationRequest_get_mut_considering_contract_prevalidation(::webhook_api::v1::CreatePrevalidationRequest* msg) {
  return static_cast<void*>(msg->mutable_considering_contract_prevalidation());
}
void __rust_proto_thunk__webhook_api_v1_CreatePrevalidationRequest_clear_considering_contract_prevalidation(::webhook_api::v1::CreatePrevalidationRequest* msg) { msg->clear_considering_contract_prevalidation(); }
bool __rust_proto_thunk__webhook_api_v1_CreatePrevalidationRequest_has_considering_contract_prevalidation(::webhook_api::v1::CreatePrevalidationRequest* msg) { return msg->has_considering_contract_prevalidation(); }


::webhook_api::v1::CreatePrevalidationRequest::TriggerCase __rust_proto_thunk__webhook_api_v1_CreatePrevalidationRequest_case_trigger(::webhook_api::v1::CreatePrevalidationRequest* msg) {
  return msg->trigger_case();
}
}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::webhook_api::v1::CreatePrevalidationRequest>* __rust_proto_thunk__Map_i32_webhook_api_v1_CreatePrevalidationRequest_new() {
    return new google::protobuf::Map<int32_t, ::webhook_api::v1::CreatePrevalidationRequest>();
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_CreatePrevalidationRequest_free(const google::protobuf::Map<int32_t, ::webhook_api::v1::CreatePrevalidationRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_CreatePrevalidationRequest_clear(google::protobuf::Map<int32_t, ::webhook_api::v1::CreatePrevalidationRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_webhook_api_v1_CreatePrevalidationRequest_size(const google::protobuf::Map<int32_t, ::webhook_api::v1::CreatePrevalidationRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_CreatePrevalidationRequest_insert(google::protobuf::Map<int32_t, ::webhook_api::v1::CreatePrevalidationRequest> * m,
                          int32_t key, ::webhook_api::v1::CreatePrevalidationRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_CreatePrevalidationRequest_get(const google::protobuf::Map<int32_t, ::webhook_api::v1::CreatePrevalidationRequest>* m,
                       int32_t key, const ::webhook_api::v1::CreatePrevalidationRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::CreatePrevalidationRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_CreatePrevalidationRequest_remove(google::protobuf::Map<int32_t, ::webhook_api::v1::CreatePrevalidationRequest> * m,
                          int32_t key, ::webhook_api::v1::CreatePrevalidationRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_webhook_api_v1_CreatePrevalidationRequest_iter(
      const google::protobuf::Map<int32_t, ::webhook_api::v1::CreatePrevalidationRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_CreatePrevalidationRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::webhook_api::v1::CreatePrevalidationRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::webhook_api::v1::CreatePrevalidationRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::webhook_api::v1::CreatePrevalidationRequest>* __rust_proto_thunk__Map_u32_webhook_api_v1_CreatePrevalidationRequest_new() {
    return new google::protobuf::Map<uint32_t, ::webhook_api::v1::CreatePrevalidationRequest>();
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_CreatePrevalidationRequest_free(const google::protobuf::Map<uint32_t, ::webhook_api::v1::CreatePrevalidationRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_CreatePrevalidationRequest_clear(google::protobuf::Map<uint32_t, ::webhook_api::v1::CreatePrevalidationRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_webhook_api_v1_CreatePrevalidationRequest_size(const google::protobuf::Map<uint32_t, ::webhook_api::v1::CreatePrevalidationRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_CreatePrevalidationRequest_insert(google::protobuf::Map<uint32_t, ::webhook_api::v1::CreatePrevalidationRequest> * m,
                          uint32_t key, ::webhook_api::v1::CreatePrevalidationRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_CreatePrevalidationRequest_get(const google::protobuf::Map<uint32_t, ::webhook_api::v1::CreatePrevalidationRequest>* m,
                       uint32_t key, const ::webhook_api::v1::CreatePrevalidationRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::CreatePrevalidationRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_CreatePrevalidationRequest_remove(google::protobuf::Map<uint32_t, ::webhook_api::v1::CreatePrevalidationRequest> * m,
                          uint32_t key, ::webhook_api::v1::CreatePrevalidationRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_webhook_api_v1_CreatePrevalidationRequest_iter(
      const google::protobuf::Map<uint32_t, ::webhook_api::v1::CreatePrevalidationRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_CreatePrevalidationRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::webhook_api::v1::CreatePrevalidationRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::webhook_api::v1::CreatePrevalidationRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::webhook_api::v1::CreatePrevalidationRequest>* __rust_proto_thunk__Map_i64_webhook_api_v1_CreatePrevalidationRequest_new() {
    return new google::protobuf::Map<int64_t, ::webhook_api::v1::CreatePrevalidationRequest>();
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_CreatePrevalidationRequest_free(const google::protobuf::Map<int64_t, ::webhook_api::v1::CreatePrevalidationRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_CreatePrevalidationRequest_clear(google::protobuf::Map<int64_t, ::webhook_api::v1::CreatePrevalidationRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_webhook_api_v1_CreatePrevalidationRequest_size(const google::protobuf::Map<int64_t, ::webhook_api::v1::CreatePrevalidationRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_CreatePrevalidationRequest_insert(google::protobuf::Map<int64_t, ::webhook_api::v1::CreatePrevalidationRequest> * m,
                          int64_t key, ::webhook_api::v1::CreatePrevalidationRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_CreatePrevalidationRequest_get(const google::protobuf::Map<int64_t, ::webhook_api::v1::CreatePrevalidationRequest>* m,
                       int64_t key, const ::webhook_api::v1::CreatePrevalidationRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::CreatePrevalidationRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_CreatePrevalidationRequest_remove(google::protobuf::Map<int64_t, ::webhook_api::v1::CreatePrevalidationRequest> * m,
                          int64_t key, ::webhook_api::v1::CreatePrevalidationRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_webhook_api_v1_CreatePrevalidationRequest_iter(
      const google::protobuf::Map<int64_t, ::webhook_api::v1::CreatePrevalidationRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_CreatePrevalidationRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::webhook_api::v1::CreatePrevalidationRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::webhook_api::v1::CreatePrevalidationRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::webhook_api::v1::CreatePrevalidationRequest>* __rust_proto_thunk__Map_u64_webhook_api_v1_CreatePrevalidationRequest_new() {
    return new google::protobuf::Map<uint64_t, ::webhook_api::v1::CreatePrevalidationRequest>();
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_CreatePrevalidationRequest_free(const google::protobuf::Map<uint64_t, ::webhook_api::v1::CreatePrevalidationRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_CreatePrevalidationRequest_clear(google::protobuf::Map<uint64_t, ::webhook_api::v1::CreatePrevalidationRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_webhook_api_v1_CreatePrevalidationRequest_size(const google::protobuf::Map<uint64_t, ::webhook_api::v1::CreatePrevalidationRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_CreatePrevalidationRequest_insert(google::protobuf::Map<uint64_t, ::webhook_api::v1::CreatePrevalidationRequest> * m,
                          uint64_t key, ::webhook_api::v1::CreatePrevalidationRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_CreatePrevalidationRequest_get(const google::protobuf::Map<uint64_t, ::webhook_api::v1::CreatePrevalidationRequest>* m,
                       uint64_t key, const ::webhook_api::v1::CreatePrevalidationRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::CreatePrevalidationRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_CreatePrevalidationRequest_remove(google::protobuf::Map<uint64_t, ::webhook_api::v1::CreatePrevalidationRequest> * m,
                          uint64_t key, ::webhook_api::v1::CreatePrevalidationRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_webhook_api_v1_CreatePrevalidationRequest_iter(
      const google::protobuf::Map<uint64_t, ::webhook_api::v1::CreatePrevalidationRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_CreatePrevalidationRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::webhook_api::v1::CreatePrevalidationRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::webhook_api::v1::CreatePrevalidationRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::webhook_api::v1::CreatePrevalidationRequest>* __rust_proto_thunk__Map_bool_webhook_api_v1_CreatePrevalidationRequest_new() {
    return new google::protobuf::Map<bool, ::webhook_api::v1::CreatePrevalidationRequest>();
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_CreatePrevalidationRequest_free(const google::protobuf::Map<bool, ::webhook_api::v1::CreatePrevalidationRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_CreatePrevalidationRequest_clear(google::protobuf::Map<bool, ::webhook_api::v1::CreatePrevalidationRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_webhook_api_v1_CreatePrevalidationRequest_size(const google::protobuf::Map<bool, ::webhook_api::v1::CreatePrevalidationRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_CreatePrevalidationRequest_insert(google::protobuf::Map<bool, ::webhook_api::v1::CreatePrevalidationRequest> * m,
                          bool key, ::webhook_api::v1::CreatePrevalidationRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_CreatePrevalidationRequest_get(const google::protobuf::Map<bool, ::webhook_api::v1::CreatePrevalidationRequest>* m,
                       bool key, const ::webhook_api::v1::CreatePrevalidationRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::CreatePrevalidationRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_CreatePrevalidationRequest_remove(google::protobuf::Map<bool, ::webhook_api::v1::CreatePrevalidationRequest> * m,
                          bool key, ::webhook_api::v1::CreatePrevalidationRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_webhook_api_v1_CreatePrevalidationRequest_iter(
      const google::protobuf::Map<bool, ::webhook_api::v1::CreatePrevalidationRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_CreatePrevalidationRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::webhook_api::v1::CreatePrevalidationRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::webhook_api::v1::CreatePrevalidationRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::webhook_api::v1::CreatePrevalidationRequest>* __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_CreatePrevalidationRequest_new() {
    return new google::protobuf::Map<std::string, ::webhook_api::v1::CreatePrevalidationRequest>();
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_CreatePrevalidationRequest_free(const google::protobuf::Map<std::string, ::webhook_api::v1::CreatePrevalidationRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_CreatePrevalidationRequest_clear(google::protobuf::Map<std::string, ::webhook_api::v1::CreatePrevalidationRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_CreatePrevalidationRequest_size(const google::protobuf::Map<std::string, ::webhook_api::v1::CreatePrevalidationRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_CreatePrevalidationRequest_insert(google::protobuf::Map<std::string, ::webhook_api::v1::CreatePrevalidationRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::CreatePrevalidationRequest value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_CreatePrevalidationRequest_get(const google::protobuf::Map<std::string, ::webhook_api::v1::CreatePrevalidationRequest>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::webhook_api::v1::CreatePrevalidationRequest** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::CreatePrevalidationRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_CreatePrevalidationRequest_remove(google::protobuf::Map<std::string, ::webhook_api::v1::CreatePrevalidationRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::CreatePrevalidationRequest * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_CreatePrevalidationRequest_iter(
      const google::protobuf::Map<std::string, ::webhook_api::v1::CreatePrevalidationRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_CreatePrevalidationRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::webhook_api::v1::CreatePrevalidationRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::webhook_api::v1::CreatePrevalidationRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// webhook_api.v1.CreatePrevalidationResponse
        // clang-format off
extern "C" {
void* __rust_proto_thunk__webhook_api_v1_CreatePrevalidationResponse_new() { return new ::webhook_api::v1::CreatePrevalidationResponse(); }
void __rust_proto_thunk__webhook_api_v1_CreatePrevalidationResponse_delete(void* ptr) { delete static_cast<::webhook_api::v1::CreatePrevalidationResponse*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__webhook_api_v1_CreatePrevalidationResponse_serialize(::webhook_api::v1::CreatePrevalidationResponse* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__webhook_api_v1_CreatePrevalidationResponse_parse(::webhook_api::v1::CreatePrevalidationResponse* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__webhook_api_v1_CreatePrevalidationResponse_copy_from(::webhook_api::v1::CreatePrevalidationResponse* dst, const ::webhook_api::v1::CreatePrevalidationResponse* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__webhook_api_v1_CreatePrevalidationResponse_repeated_len(google::protobuf::RepeatedPtrField<::webhook_api::v1::CreatePrevalidationResponse>* field) {
  return field->size();
}
const ::webhook_api::v1::CreatePrevalidationResponse& __rust_proto_thunk__webhook_api_v1_CreatePrevalidationResponse_repeated_get(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::CreatePrevalidationResponse>* field,
  size_t index) {
  return field->Get(index);
}
::webhook_api::v1::CreatePrevalidationResponse* __rust_proto_thunk__webhook_api_v1_CreatePrevalidationResponse_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::CreatePrevalidationResponse>* field,
  size_t index) {
  return field->Mutable(index);
}
::webhook_api::v1::CreatePrevalidationResponse* __rust_proto_thunk__webhook_api_v1_CreatePrevalidationResponse_repeated_add(google::protobuf::RepeatedPtrField<::webhook_api::v1::CreatePrevalidationResponse>* field) {
  return field->Add();
}
void __rust_proto_thunk__webhook_api_v1_CreatePrevalidationResponse_repeated_clear(google::protobuf::RepeatedPtrField<::webhook_api::v1::CreatePrevalidationResponse>* field) {
  field->Clear();
}
void __rust_proto_thunk__webhook_api_v1_CreatePrevalidationResponse_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::CreatePrevalidationResponse>& dst,
  const google::protobuf::RepeatedPtrField<::webhook_api::v1::CreatePrevalidationResponse>& src) {
  dst = src;
}

const void* __rust_proto_thunk__webhook_api_v1_CreatePrevalidationResponse_get_prevalidation(::webhook_api::v1::CreatePrevalidationResponse* msg) {
  return static_cast<const void*>(&msg->prevalidation());
}
void* __rust_proto_thunk__webhook_api_v1_CreatePrevalidationResponse_get_mut_prevalidation(::webhook_api::v1::CreatePrevalidationResponse* msg) {
  return static_cast<void*>(msg->mutable_prevalidation());
}
void __rust_proto_thunk__webhook_api_v1_CreatePrevalidationResponse_clear_prevalidation(::webhook_api::v1::CreatePrevalidationResponse* msg) { msg->clear_prevalidation(); }
bool __rust_proto_thunk__webhook_api_v1_CreatePrevalidationResponse_has_prevalidation(::webhook_api::v1::CreatePrevalidationResponse* msg) { return msg->has_prevalidation(); }


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::webhook_api::v1::CreatePrevalidationResponse>* __rust_proto_thunk__Map_i32_webhook_api_v1_CreatePrevalidationResponse_new() {
    return new google::protobuf::Map<int32_t, ::webhook_api::v1::CreatePrevalidationResponse>();
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_CreatePrevalidationResponse_free(const google::protobuf::Map<int32_t, ::webhook_api::v1::CreatePrevalidationResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_CreatePrevalidationResponse_clear(google::protobuf::Map<int32_t, ::webhook_api::v1::CreatePrevalidationResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_webhook_api_v1_CreatePrevalidationResponse_size(const google::protobuf::Map<int32_t, ::webhook_api::v1::CreatePrevalidationResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_CreatePrevalidationResponse_insert(google::protobuf::Map<int32_t, ::webhook_api::v1::CreatePrevalidationResponse> * m,
                          int32_t key, ::webhook_api::v1::CreatePrevalidationResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_CreatePrevalidationResponse_get(const google::protobuf::Map<int32_t, ::webhook_api::v1::CreatePrevalidationResponse>* m,
                       int32_t key, const ::webhook_api::v1::CreatePrevalidationResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::CreatePrevalidationResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_CreatePrevalidationResponse_remove(google::protobuf::Map<int32_t, ::webhook_api::v1::CreatePrevalidationResponse> * m,
                          int32_t key, ::webhook_api::v1::CreatePrevalidationResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_webhook_api_v1_CreatePrevalidationResponse_iter(
      const google::protobuf::Map<int32_t, ::webhook_api::v1::CreatePrevalidationResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_CreatePrevalidationResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::webhook_api::v1::CreatePrevalidationResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::webhook_api::v1::CreatePrevalidationResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::webhook_api::v1::CreatePrevalidationResponse>* __rust_proto_thunk__Map_u32_webhook_api_v1_CreatePrevalidationResponse_new() {
    return new google::protobuf::Map<uint32_t, ::webhook_api::v1::CreatePrevalidationResponse>();
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_CreatePrevalidationResponse_free(const google::protobuf::Map<uint32_t, ::webhook_api::v1::CreatePrevalidationResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_CreatePrevalidationResponse_clear(google::protobuf::Map<uint32_t, ::webhook_api::v1::CreatePrevalidationResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_webhook_api_v1_CreatePrevalidationResponse_size(const google::protobuf::Map<uint32_t, ::webhook_api::v1::CreatePrevalidationResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_CreatePrevalidationResponse_insert(google::protobuf::Map<uint32_t, ::webhook_api::v1::CreatePrevalidationResponse> * m,
                          uint32_t key, ::webhook_api::v1::CreatePrevalidationResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_CreatePrevalidationResponse_get(const google::protobuf::Map<uint32_t, ::webhook_api::v1::CreatePrevalidationResponse>* m,
                       uint32_t key, const ::webhook_api::v1::CreatePrevalidationResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::CreatePrevalidationResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_CreatePrevalidationResponse_remove(google::protobuf::Map<uint32_t, ::webhook_api::v1::CreatePrevalidationResponse> * m,
                          uint32_t key, ::webhook_api::v1::CreatePrevalidationResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_webhook_api_v1_CreatePrevalidationResponse_iter(
      const google::protobuf::Map<uint32_t, ::webhook_api::v1::CreatePrevalidationResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_CreatePrevalidationResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::webhook_api::v1::CreatePrevalidationResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::webhook_api::v1::CreatePrevalidationResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::webhook_api::v1::CreatePrevalidationResponse>* __rust_proto_thunk__Map_i64_webhook_api_v1_CreatePrevalidationResponse_new() {
    return new google::protobuf::Map<int64_t, ::webhook_api::v1::CreatePrevalidationResponse>();
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_CreatePrevalidationResponse_free(const google::protobuf::Map<int64_t, ::webhook_api::v1::CreatePrevalidationResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_CreatePrevalidationResponse_clear(google::protobuf::Map<int64_t, ::webhook_api::v1::CreatePrevalidationResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_webhook_api_v1_CreatePrevalidationResponse_size(const google::protobuf::Map<int64_t, ::webhook_api::v1::CreatePrevalidationResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_CreatePrevalidationResponse_insert(google::protobuf::Map<int64_t, ::webhook_api::v1::CreatePrevalidationResponse> * m,
                          int64_t key, ::webhook_api::v1::CreatePrevalidationResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_CreatePrevalidationResponse_get(const google::protobuf::Map<int64_t, ::webhook_api::v1::CreatePrevalidationResponse>* m,
                       int64_t key, const ::webhook_api::v1::CreatePrevalidationResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::CreatePrevalidationResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_CreatePrevalidationResponse_remove(google::protobuf::Map<int64_t, ::webhook_api::v1::CreatePrevalidationResponse> * m,
                          int64_t key, ::webhook_api::v1::CreatePrevalidationResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_webhook_api_v1_CreatePrevalidationResponse_iter(
      const google::protobuf::Map<int64_t, ::webhook_api::v1::CreatePrevalidationResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_CreatePrevalidationResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::webhook_api::v1::CreatePrevalidationResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::webhook_api::v1::CreatePrevalidationResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::webhook_api::v1::CreatePrevalidationResponse>* __rust_proto_thunk__Map_u64_webhook_api_v1_CreatePrevalidationResponse_new() {
    return new google::protobuf::Map<uint64_t, ::webhook_api::v1::CreatePrevalidationResponse>();
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_CreatePrevalidationResponse_free(const google::protobuf::Map<uint64_t, ::webhook_api::v1::CreatePrevalidationResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_CreatePrevalidationResponse_clear(google::protobuf::Map<uint64_t, ::webhook_api::v1::CreatePrevalidationResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_webhook_api_v1_CreatePrevalidationResponse_size(const google::protobuf::Map<uint64_t, ::webhook_api::v1::CreatePrevalidationResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_CreatePrevalidationResponse_insert(google::protobuf::Map<uint64_t, ::webhook_api::v1::CreatePrevalidationResponse> * m,
                          uint64_t key, ::webhook_api::v1::CreatePrevalidationResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_CreatePrevalidationResponse_get(const google::protobuf::Map<uint64_t, ::webhook_api::v1::CreatePrevalidationResponse>* m,
                       uint64_t key, const ::webhook_api::v1::CreatePrevalidationResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::CreatePrevalidationResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_CreatePrevalidationResponse_remove(google::protobuf::Map<uint64_t, ::webhook_api::v1::CreatePrevalidationResponse> * m,
                          uint64_t key, ::webhook_api::v1::CreatePrevalidationResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_webhook_api_v1_CreatePrevalidationResponse_iter(
      const google::protobuf::Map<uint64_t, ::webhook_api::v1::CreatePrevalidationResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_CreatePrevalidationResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::webhook_api::v1::CreatePrevalidationResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::webhook_api::v1::CreatePrevalidationResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::webhook_api::v1::CreatePrevalidationResponse>* __rust_proto_thunk__Map_bool_webhook_api_v1_CreatePrevalidationResponse_new() {
    return new google::protobuf::Map<bool, ::webhook_api::v1::CreatePrevalidationResponse>();
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_CreatePrevalidationResponse_free(const google::protobuf::Map<bool, ::webhook_api::v1::CreatePrevalidationResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_CreatePrevalidationResponse_clear(google::protobuf::Map<bool, ::webhook_api::v1::CreatePrevalidationResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_webhook_api_v1_CreatePrevalidationResponse_size(const google::protobuf::Map<bool, ::webhook_api::v1::CreatePrevalidationResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_CreatePrevalidationResponse_insert(google::protobuf::Map<bool, ::webhook_api::v1::CreatePrevalidationResponse> * m,
                          bool key, ::webhook_api::v1::CreatePrevalidationResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_CreatePrevalidationResponse_get(const google::protobuf::Map<bool, ::webhook_api::v1::CreatePrevalidationResponse>* m,
                       bool key, const ::webhook_api::v1::CreatePrevalidationResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::CreatePrevalidationResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_CreatePrevalidationResponse_remove(google::protobuf::Map<bool, ::webhook_api::v1::CreatePrevalidationResponse> * m,
                          bool key, ::webhook_api::v1::CreatePrevalidationResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_webhook_api_v1_CreatePrevalidationResponse_iter(
      const google::protobuf::Map<bool, ::webhook_api::v1::CreatePrevalidationResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_CreatePrevalidationResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::webhook_api::v1::CreatePrevalidationResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::webhook_api::v1::CreatePrevalidationResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::webhook_api::v1::CreatePrevalidationResponse>* __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_CreatePrevalidationResponse_new() {
    return new google::protobuf::Map<std::string, ::webhook_api::v1::CreatePrevalidationResponse>();
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_CreatePrevalidationResponse_free(const google::protobuf::Map<std::string, ::webhook_api::v1::CreatePrevalidationResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_CreatePrevalidationResponse_clear(google::protobuf::Map<std::string, ::webhook_api::v1::CreatePrevalidationResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_CreatePrevalidationResponse_size(const google::protobuf::Map<std::string, ::webhook_api::v1::CreatePrevalidationResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_CreatePrevalidationResponse_insert(google::protobuf::Map<std::string, ::webhook_api::v1::CreatePrevalidationResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::CreatePrevalidationResponse value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_CreatePrevalidationResponse_get(const google::protobuf::Map<std::string, ::webhook_api::v1::CreatePrevalidationResponse>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::webhook_api::v1::CreatePrevalidationResponse** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::CreatePrevalidationResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_CreatePrevalidationResponse_remove(google::protobuf::Map<std::string, ::webhook_api::v1::CreatePrevalidationResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::CreatePrevalidationResponse * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_CreatePrevalidationResponse_iter(
      const google::protobuf::Map<std::string, ::webhook_api::v1::CreatePrevalidationResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_CreatePrevalidationResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::webhook_api::v1::CreatePrevalidationResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::webhook_api::v1::CreatePrevalidationResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// webhook_api.v1.ListPrevalidationsRequest
        // clang-format off
extern "C" {
void* __rust_proto_thunk__webhook_api_v1_ListPrevalidationsRequest_new() { return new ::webhook_api::v1::ListPrevalidationsRequest(); }
void __rust_proto_thunk__webhook_api_v1_ListPrevalidationsRequest_delete(void* ptr) { delete static_cast<::webhook_api::v1::ListPrevalidationsRequest*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__webhook_api_v1_ListPrevalidationsRequest_serialize(::webhook_api::v1::ListPrevalidationsRequest* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__webhook_api_v1_ListPrevalidationsRequest_parse(::webhook_api::v1::ListPrevalidationsRequest* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__webhook_api_v1_ListPrevalidationsRequest_copy_from(::webhook_api::v1::ListPrevalidationsRequest* dst, const ::webhook_api::v1::ListPrevalidationsRequest* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__webhook_api_v1_ListPrevalidationsRequest_repeated_len(google::protobuf::RepeatedPtrField<::webhook_api::v1::ListPrevalidationsRequest>* field) {
  return field->size();
}
const ::webhook_api::v1::ListPrevalidationsRequest& __rust_proto_thunk__webhook_api_v1_ListPrevalidationsRequest_repeated_get(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::ListPrevalidationsRequest>* field,
  size_t index) {
  return field->Get(index);
}
::webhook_api::v1::ListPrevalidationsRequest* __rust_proto_thunk__webhook_api_v1_ListPrevalidationsRequest_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::ListPrevalidationsRequest>* field,
  size_t index) {
  return field->Mutable(index);
}
::webhook_api::v1::ListPrevalidationsRequest* __rust_proto_thunk__webhook_api_v1_ListPrevalidationsRequest_repeated_add(google::protobuf::RepeatedPtrField<::webhook_api::v1::ListPrevalidationsRequest>* field) {
  return field->Add();
}
void __rust_proto_thunk__webhook_api_v1_ListPrevalidationsRequest_repeated_clear(google::protobuf::RepeatedPtrField<::webhook_api::v1::ListPrevalidationsRequest>* field) {
  field->Clear();
}
void __rust_proto_thunk__webhook_api_v1_ListPrevalidationsRequest_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::ListPrevalidationsRequest>& dst,
  const google::protobuf::RepeatedPtrField<::webhook_api::v1::ListPrevalidationsRequest>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__webhook_api_v1_ListPrevalidationsRequest_get_api_key(::webhook_api::v1::ListPrevalidationsRequest* msg) {
  absl::string_view val = msg->api_key();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__webhook_api_v1_ListPrevalidationsRequest_set_api_key(::webhook_api::v1::ListPrevalidationsRequest* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_api_key(absl::string_view(s.ptr, s.len));
}

::int32_t __rust_proto_thunk__webhook_api_v1_ListPrevalidationsRequest_get_page(::webhook_api::v1::ListPrevalidationsRequest* msg) { return msg->page(); }
void __rust_proto_thunk__webhook_api_v1_ListPrevalidationsRequest_set_page(::webhook_api::v1::ListPrevalidationsRequest* msg, ::int32_t val) {
  msg->set_page(val);
}

::int32_t __rust_proto_thunk__webhook_api_v1_ListPrevalidationsRequest_get_page_size(::webhook_api::v1::ListPrevalidationsRequest* msg) { return msg->page_size(); }
void __rust_proto_thunk__webhook_api_v1_ListPrevalidationsRequest_set_page_size(::webhook_api::v1::ListPrevalidationsRequest* msg, ::int32_t val) {
  msg->set_page_size(val);
}

const void* __rust_proto_thunk__webhook_api_v1_ListPrevalidationsRequest_get_filters(::webhook_api::v1::ListPrevalidationsRequest* msg) {
  return static_cast<const void*>(&msg->filters());
}
void* __rust_proto_thunk__webhook_api_v1_ListPrevalidationsRequest_get_mut_filters(::webhook_api::v1::ListPrevalidationsRequest* msg) {
  return static_cast<void*>(msg->mutable_filters());
}
void __rust_proto_thunk__webhook_api_v1_ListPrevalidationsRequest_clear_filters(::webhook_api::v1::ListPrevalidationsRequest* msg) { msg->clear_filters(); }
bool __rust_proto_thunk__webhook_api_v1_ListPrevalidationsRequest_has_filters(::webhook_api::v1::ListPrevalidationsRequest* msg) { return msg->has_filters(); }

const void* __rust_proto_thunk__webhook_api_v1_ListPrevalidationsRequest_get_sort_by(::webhook_api::v1::ListPrevalidationsRequest* msg) {
  return static_cast<const void*>(&msg->sort_by());
}
void* __rust_proto_thunk__webhook_api_v1_ListPrevalidationsRequest_get_mut_sort_by(::webhook_api::v1::ListPrevalidationsRequest* msg) {
  return static_cast<void*>(msg->mutable_sort_by());
}
void __rust_proto_thunk__webhook_api_v1_ListPrevalidationsRequest_clear_sort_by(::webhook_api::v1::ListPrevalidationsRequest* msg) { msg->clear_sort_by(); }
bool __rust_proto_thunk__webhook_api_v1_ListPrevalidationsRequest_has_sort_by(::webhook_api::v1::ListPrevalidationsRequest* msg) { return msg->has_sort_by(); }


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::webhook_api::v1::ListPrevalidationsRequest>* __rust_proto_thunk__Map_i32_webhook_api_v1_ListPrevalidationsRequest_new() {
    return new google::protobuf::Map<int32_t, ::webhook_api::v1::ListPrevalidationsRequest>();
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_ListPrevalidationsRequest_free(const google::protobuf::Map<int32_t, ::webhook_api::v1::ListPrevalidationsRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_ListPrevalidationsRequest_clear(google::protobuf::Map<int32_t, ::webhook_api::v1::ListPrevalidationsRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_webhook_api_v1_ListPrevalidationsRequest_size(const google::protobuf::Map<int32_t, ::webhook_api::v1::ListPrevalidationsRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_ListPrevalidationsRequest_insert(google::protobuf::Map<int32_t, ::webhook_api::v1::ListPrevalidationsRequest> * m,
                          int32_t key, ::webhook_api::v1::ListPrevalidationsRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_ListPrevalidationsRequest_get(const google::protobuf::Map<int32_t, ::webhook_api::v1::ListPrevalidationsRequest>* m,
                       int32_t key, const ::webhook_api::v1::ListPrevalidationsRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ListPrevalidationsRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_ListPrevalidationsRequest_remove(google::protobuf::Map<int32_t, ::webhook_api::v1::ListPrevalidationsRequest> * m,
                          int32_t key, ::webhook_api::v1::ListPrevalidationsRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_webhook_api_v1_ListPrevalidationsRequest_iter(
      const google::protobuf::Map<int32_t, ::webhook_api::v1::ListPrevalidationsRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_ListPrevalidationsRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::webhook_api::v1::ListPrevalidationsRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::webhook_api::v1::ListPrevalidationsRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::webhook_api::v1::ListPrevalidationsRequest>* __rust_proto_thunk__Map_u32_webhook_api_v1_ListPrevalidationsRequest_new() {
    return new google::protobuf::Map<uint32_t, ::webhook_api::v1::ListPrevalidationsRequest>();
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_ListPrevalidationsRequest_free(const google::protobuf::Map<uint32_t, ::webhook_api::v1::ListPrevalidationsRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_ListPrevalidationsRequest_clear(google::protobuf::Map<uint32_t, ::webhook_api::v1::ListPrevalidationsRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_webhook_api_v1_ListPrevalidationsRequest_size(const google::protobuf::Map<uint32_t, ::webhook_api::v1::ListPrevalidationsRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_ListPrevalidationsRequest_insert(google::protobuf::Map<uint32_t, ::webhook_api::v1::ListPrevalidationsRequest> * m,
                          uint32_t key, ::webhook_api::v1::ListPrevalidationsRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_ListPrevalidationsRequest_get(const google::protobuf::Map<uint32_t, ::webhook_api::v1::ListPrevalidationsRequest>* m,
                       uint32_t key, const ::webhook_api::v1::ListPrevalidationsRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ListPrevalidationsRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_ListPrevalidationsRequest_remove(google::protobuf::Map<uint32_t, ::webhook_api::v1::ListPrevalidationsRequest> * m,
                          uint32_t key, ::webhook_api::v1::ListPrevalidationsRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_webhook_api_v1_ListPrevalidationsRequest_iter(
      const google::protobuf::Map<uint32_t, ::webhook_api::v1::ListPrevalidationsRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_ListPrevalidationsRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::webhook_api::v1::ListPrevalidationsRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::webhook_api::v1::ListPrevalidationsRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::webhook_api::v1::ListPrevalidationsRequest>* __rust_proto_thunk__Map_i64_webhook_api_v1_ListPrevalidationsRequest_new() {
    return new google::protobuf::Map<int64_t, ::webhook_api::v1::ListPrevalidationsRequest>();
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_ListPrevalidationsRequest_free(const google::protobuf::Map<int64_t, ::webhook_api::v1::ListPrevalidationsRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_ListPrevalidationsRequest_clear(google::protobuf::Map<int64_t, ::webhook_api::v1::ListPrevalidationsRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_webhook_api_v1_ListPrevalidationsRequest_size(const google::protobuf::Map<int64_t, ::webhook_api::v1::ListPrevalidationsRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_ListPrevalidationsRequest_insert(google::protobuf::Map<int64_t, ::webhook_api::v1::ListPrevalidationsRequest> * m,
                          int64_t key, ::webhook_api::v1::ListPrevalidationsRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_ListPrevalidationsRequest_get(const google::protobuf::Map<int64_t, ::webhook_api::v1::ListPrevalidationsRequest>* m,
                       int64_t key, const ::webhook_api::v1::ListPrevalidationsRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ListPrevalidationsRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_ListPrevalidationsRequest_remove(google::protobuf::Map<int64_t, ::webhook_api::v1::ListPrevalidationsRequest> * m,
                          int64_t key, ::webhook_api::v1::ListPrevalidationsRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_webhook_api_v1_ListPrevalidationsRequest_iter(
      const google::protobuf::Map<int64_t, ::webhook_api::v1::ListPrevalidationsRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_ListPrevalidationsRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::webhook_api::v1::ListPrevalidationsRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::webhook_api::v1::ListPrevalidationsRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::webhook_api::v1::ListPrevalidationsRequest>* __rust_proto_thunk__Map_u64_webhook_api_v1_ListPrevalidationsRequest_new() {
    return new google::protobuf::Map<uint64_t, ::webhook_api::v1::ListPrevalidationsRequest>();
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_ListPrevalidationsRequest_free(const google::protobuf::Map<uint64_t, ::webhook_api::v1::ListPrevalidationsRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_ListPrevalidationsRequest_clear(google::protobuf::Map<uint64_t, ::webhook_api::v1::ListPrevalidationsRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_webhook_api_v1_ListPrevalidationsRequest_size(const google::protobuf::Map<uint64_t, ::webhook_api::v1::ListPrevalidationsRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_ListPrevalidationsRequest_insert(google::protobuf::Map<uint64_t, ::webhook_api::v1::ListPrevalidationsRequest> * m,
                          uint64_t key, ::webhook_api::v1::ListPrevalidationsRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_ListPrevalidationsRequest_get(const google::protobuf::Map<uint64_t, ::webhook_api::v1::ListPrevalidationsRequest>* m,
                       uint64_t key, const ::webhook_api::v1::ListPrevalidationsRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ListPrevalidationsRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_ListPrevalidationsRequest_remove(google::protobuf::Map<uint64_t, ::webhook_api::v1::ListPrevalidationsRequest> * m,
                          uint64_t key, ::webhook_api::v1::ListPrevalidationsRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_webhook_api_v1_ListPrevalidationsRequest_iter(
      const google::protobuf::Map<uint64_t, ::webhook_api::v1::ListPrevalidationsRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_ListPrevalidationsRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::webhook_api::v1::ListPrevalidationsRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::webhook_api::v1::ListPrevalidationsRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::webhook_api::v1::ListPrevalidationsRequest>* __rust_proto_thunk__Map_bool_webhook_api_v1_ListPrevalidationsRequest_new() {
    return new google::protobuf::Map<bool, ::webhook_api::v1::ListPrevalidationsRequest>();
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_ListPrevalidationsRequest_free(const google::protobuf::Map<bool, ::webhook_api::v1::ListPrevalidationsRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_ListPrevalidationsRequest_clear(google::protobuf::Map<bool, ::webhook_api::v1::ListPrevalidationsRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_webhook_api_v1_ListPrevalidationsRequest_size(const google::protobuf::Map<bool, ::webhook_api::v1::ListPrevalidationsRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_ListPrevalidationsRequest_insert(google::protobuf::Map<bool, ::webhook_api::v1::ListPrevalidationsRequest> * m,
                          bool key, ::webhook_api::v1::ListPrevalidationsRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_ListPrevalidationsRequest_get(const google::protobuf::Map<bool, ::webhook_api::v1::ListPrevalidationsRequest>* m,
                       bool key, const ::webhook_api::v1::ListPrevalidationsRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ListPrevalidationsRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_ListPrevalidationsRequest_remove(google::protobuf::Map<bool, ::webhook_api::v1::ListPrevalidationsRequest> * m,
                          bool key, ::webhook_api::v1::ListPrevalidationsRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_webhook_api_v1_ListPrevalidationsRequest_iter(
      const google::protobuf::Map<bool, ::webhook_api::v1::ListPrevalidationsRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_ListPrevalidationsRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::webhook_api::v1::ListPrevalidationsRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::webhook_api::v1::ListPrevalidationsRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::webhook_api::v1::ListPrevalidationsRequest>* __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListPrevalidationsRequest_new() {
    return new google::protobuf::Map<std::string, ::webhook_api::v1::ListPrevalidationsRequest>();
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListPrevalidationsRequest_free(const google::protobuf::Map<std::string, ::webhook_api::v1::ListPrevalidationsRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListPrevalidationsRequest_clear(google::protobuf::Map<std::string, ::webhook_api::v1::ListPrevalidationsRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListPrevalidationsRequest_size(const google::protobuf::Map<std::string, ::webhook_api::v1::ListPrevalidationsRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListPrevalidationsRequest_insert(google::protobuf::Map<std::string, ::webhook_api::v1::ListPrevalidationsRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::ListPrevalidationsRequest value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListPrevalidationsRequest_get(const google::protobuf::Map<std::string, ::webhook_api::v1::ListPrevalidationsRequest>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::webhook_api::v1::ListPrevalidationsRequest** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ListPrevalidationsRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListPrevalidationsRequest_remove(google::protobuf::Map<std::string, ::webhook_api::v1::ListPrevalidationsRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::ListPrevalidationsRequest * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListPrevalidationsRequest_iter(
      const google::protobuf::Map<std::string, ::webhook_api::v1::ListPrevalidationsRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListPrevalidationsRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::webhook_api::v1::ListPrevalidationsRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::webhook_api::v1::ListPrevalidationsRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// webhook_api.v1.ListPrevalidationsResponse
        // clang-format off
extern "C" {
void* __rust_proto_thunk__webhook_api_v1_ListPrevalidationsResponse_new() { return new ::webhook_api::v1::ListPrevalidationsResponse(); }
void __rust_proto_thunk__webhook_api_v1_ListPrevalidationsResponse_delete(void* ptr) { delete static_cast<::webhook_api::v1::ListPrevalidationsResponse*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__webhook_api_v1_ListPrevalidationsResponse_serialize(::webhook_api::v1::ListPrevalidationsResponse* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__webhook_api_v1_ListPrevalidationsResponse_parse(::webhook_api::v1::ListPrevalidationsResponse* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__webhook_api_v1_ListPrevalidationsResponse_copy_from(::webhook_api::v1::ListPrevalidationsResponse* dst, const ::webhook_api::v1::ListPrevalidationsResponse* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__webhook_api_v1_ListPrevalidationsResponse_repeated_len(google::protobuf::RepeatedPtrField<::webhook_api::v1::ListPrevalidationsResponse>* field) {
  return field->size();
}
const ::webhook_api::v1::ListPrevalidationsResponse& __rust_proto_thunk__webhook_api_v1_ListPrevalidationsResponse_repeated_get(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::ListPrevalidationsResponse>* field,
  size_t index) {
  return field->Get(index);
}
::webhook_api::v1::ListPrevalidationsResponse* __rust_proto_thunk__webhook_api_v1_ListPrevalidationsResponse_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::ListPrevalidationsResponse>* field,
  size_t index) {
  return field->Mutable(index);
}
::webhook_api::v1::ListPrevalidationsResponse* __rust_proto_thunk__webhook_api_v1_ListPrevalidationsResponse_repeated_add(google::protobuf::RepeatedPtrField<::webhook_api::v1::ListPrevalidationsResponse>* field) {
  return field->Add();
}
void __rust_proto_thunk__webhook_api_v1_ListPrevalidationsResponse_repeated_clear(google::protobuf::RepeatedPtrField<::webhook_api::v1::ListPrevalidationsResponse>* field) {
  field->Clear();
}
void __rust_proto_thunk__webhook_api_v1_ListPrevalidationsResponse_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::ListPrevalidationsResponse>& dst,
  const google::protobuf::RepeatedPtrField<::webhook_api::v1::ListPrevalidationsResponse>& src) {
  dst = src;
}

void __rust_proto_thunk__webhook_api_v1_ListPrevalidationsResponse_clear_prevalidations(::webhook_api::v1::ListPrevalidationsResponse* msg) {
  msg->clear_prevalidations();
}
google::protobuf::RepeatedPtrField<::webhook_api::v1::Prevalidation>* __rust_proto_thunk__webhook_api_v1_ListPrevalidationsResponse_get_mut_prevalidations(::webhook_api::v1::ListPrevalidationsResponse* msg) {
  return msg->mutable_prevalidations();
}
const google::protobuf::RepeatedPtrField<::webhook_api::v1::Prevalidation>* __rust_proto_thunk__webhook_api_v1_ListPrevalidationsResponse_get_prevalidations(
    const ::webhook_api::v1::ListPrevalidationsResponse* msg) {
  return &msg->prevalidations();
}

::uint32_t __rust_proto_thunk__webhook_api_v1_ListPrevalidationsResponse_get_total_owned_prevalidations(::webhook_api::v1::ListPrevalidationsResponse* msg) { return msg->total_owned_prevalidations(); }
void __rust_proto_thunk__webhook_api_v1_ListPrevalidationsResponse_set_total_owned_prevalidations(::webhook_api::v1::ListPrevalidationsResponse* msg, ::uint32_t val) {
  msg->set_total_owned_prevalidations(val);
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::webhook_api::v1::ListPrevalidationsResponse>* __rust_proto_thunk__Map_i32_webhook_api_v1_ListPrevalidationsResponse_new() {
    return new google::protobuf::Map<int32_t, ::webhook_api::v1::ListPrevalidationsResponse>();
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_ListPrevalidationsResponse_free(const google::protobuf::Map<int32_t, ::webhook_api::v1::ListPrevalidationsResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_ListPrevalidationsResponse_clear(google::protobuf::Map<int32_t, ::webhook_api::v1::ListPrevalidationsResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_webhook_api_v1_ListPrevalidationsResponse_size(const google::protobuf::Map<int32_t, ::webhook_api::v1::ListPrevalidationsResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_ListPrevalidationsResponse_insert(google::protobuf::Map<int32_t, ::webhook_api::v1::ListPrevalidationsResponse> * m,
                          int32_t key, ::webhook_api::v1::ListPrevalidationsResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_ListPrevalidationsResponse_get(const google::protobuf::Map<int32_t, ::webhook_api::v1::ListPrevalidationsResponse>* m,
                       int32_t key, const ::webhook_api::v1::ListPrevalidationsResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ListPrevalidationsResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_ListPrevalidationsResponse_remove(google::protobuf::Map<int32_t, ::webhook_api::v1::ListPrevalidationsResponse> * m,
                          int32_t key, ::webhook_api::v1::ListPrevalidationsResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_webhook_api_v1_ListPrevalidationsResponse_iter(
      const google::protobuf::Map<int32_t, ::webhook_api::v1::ListPrevalidationsResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_ListPrevalidationsResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::webhook_api::v1::ListPrevalidationsResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::webhook_api::v1::ListPrevalidationsResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::webhook_api::v1::ListPrevalidationsResponse>* __rust_proto_thunk__Map_u32_webhook_api_v1_ListPrevalidationsResponse_new() {
    return new google::protobuf::Map<uint32_t, ::webhook_api::v1::ListPrevalidationsResponse>();
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_ListPrevalidationsResponse_free(const google::protobuf::Map<uint32_t, ::webhook_api::v1::ListPrevalidationsResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_ListPrevalidationsResponse_clear(google::protobuf::Map<uint32_t, ::webhook_api::v1::ListPrevalidationsResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_webhook_api_v1_ListPrevalidationsResponse_size(const google::protobuf::Map<uint32_t, ::webhook_api::v1::ListPrevalidationsResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_ListPrevalidationsResponse_insert(google::protobuf::Map<uint32_t, ::webhook_api::v1::ListPrevalidationsResponse> * m,
                          uint32_t key, ::webhook_api::v1::ListPrevalidationsResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_ListPrevalidationsResponse_get(const google::protobuf::Map<uint32_t, ::webhook_api::v1::ListPrevalidationsResponse>* m,
                       uint32_t key, const ::webhook_api::v1::ListPrevalidationsResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ListPrevalidationsResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_ListPrevalidationsResponse_remove(google::protobuf::Map<uint32_t, ::webhook_api::v1::ListPrevalidationsResponse> * m,
                          uint32_t key, ::webhook_api::v1::ListPrevalidationsResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_webhook_api_v1_ListPrevalidationsResponse_iter(
      const google::protobuf::Map<uint32_t, ::webhook_api::v1::ListPrevalidationsResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_ListPrevalidationsResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::webhook_api::v1::ListPrevalidationsResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::webhook_api::v1::ListPrevalidationsResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::webhook_api::v1::ListPrevalidationsResponse>* __rust_proto_thunk__Map_i64_webhook_api_v1_ListPrevalidationsResponse_new() {
    return new google::protobuf::Map<int64_t, ::webhook_api::v1::ListPrevalidationsResponse>();
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_ListPrevalidationsResponse_free(const google::protobuf::Map<int64_t, ::webhook_api::v1::ListPrevalidationsResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_ListPrevalidationsResponse_clear(google::protobuf::Map<int64_t, ::webhook_api::v1::ListPrevalidationsResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_webhook_api_v1_ListPrevalidationsResponse_size(const google::protobuf::Map<int64_t, ::webhook_api::v1::ListPrevalidationsResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_ListPrevalidationsResponse_insert(google::protobuf::Map<int64_t, ::webhook_api::v1::ListPrevalidationsResponse> * m,
                          int64_t key, ::webhook_api::v1::ListPrevalidationsResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_ListPrevalidationsResponse_get(const google::protobuf::Map<int64_t, ::webhook_api::v1::ListPrevalidationsResponse>* m,
                       int64_t key, const ::webhook_api::v1::ListPrevalidationsResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ListPrevalidationsResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_ListPrevalidationsResponse_remove(google::protobuf::Map<int64_t, ::webhook_api::v1::ListPrevalidationsResponse> * m,
                          int64_t key, ::webhook_api::v1::ListPrevalidationsResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_webhook_api_v1_ListPrevalidationsResponse_iter(
      const google::protobuf::Map<int64_t, ::webhook_api::v1::ListPrevalidationsResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_ListPrevalidationsResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::webhook_api::v1::ListPrevalidationsResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::webhook_api::v1::ListPrevalidationsResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::webhook_api::v1::ListPrevalidationsResponse>* __rust_proto_thunk__Map_u64_webhook_api_v1_ListPrevalidationsResponse_new() {
    return new google::protobuf::Map<uint64_t, ::webhook_api::v1::ListPrevalidationsResponse>();
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_ListPrevalidationsResponse_free(const google::protobuf::Map<uint64_t, ::webhook_api::v1::ListPrevalidationsResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_ListPrevalidationsResponse_clear(google::protobuf::Map<uint64_t, ::webhook_api::v1::ListPrevalidationsResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_webhook_api_v1_ListPrevalidationsResponse_size(const google::protobuf::Map<uint64_t, ::webhook_api::v1::ListPrevalidationsResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_ListPrevalidationsResponse_insert(google::protobuf::Map<uint64_t, ::webhook_api::v1::ListPrevalidationsResponse> * m,
                          uint64_t key, ::webhook_api::v1::ListPrevalidationsResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_ListPrevalidationsResponse_get(const google::protobuf::Map<uint64_t, ::webhook_api::v1::ListPrevalidationsResponse>* m,
                       uint64_t key, const ::webhook_api::v1::ListPrevalidationsResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ListPrevalidationsResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_ListPrevalidationsResponse_remove(google::protobuf::Map<uint64_t, ::webhook_api::v1::ListPrevalidationsResponse> * m,
                          uint64_t key, ::webhook_api::v1::ListPrevalidationsResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_webhook_api_v1_ListPrevalidationsResponse_iter(
      const google::protobuf::Map<uint64_t, ::webhook_api::v1::ListPrevalidationsResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_ListPrevalidationsResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::webhook_api::v1::ListPrevalidationsResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::webhook_api::v1::ListPrevalidationsResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::webhook_api::v1::ListPrevalidationsResponse>* __rust_proto_thunk__Map_bool_webhook_api_v1_ListPrevalidationsResponse_new() {
    return new google::protobuf::Map<bool, ::webhook_api::v1::ListPrevalidationsResponse>();
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_ListPrevalidationsResponse_free(const google::protobuf::Map<bool, ::webhook_api::v1::ListPrevalidationsResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_ListPrevalidationsResponse_clear(google::protobuf::Map<bool, ::webhook_api::v1::ListPrevalidationsResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_webhook_api_v1_ListPrevalidationsResponse_size(const google::protobuf::Map<bool, ::webhook_api::v1::ListPrevalidationsResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_ListPrevalidationsResponse_insert(google::protobuf::Map<bool, ::webhook_api::v1::ListPrevalidationsResponse> * m,
                          bool key, ::webhook_api::v1::ListPrevalidationsResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_ListPrevalidationsResponse_get(const google::protobuf::Map<bool, ::webhook_api::v1::ListPrevalidationsResponse>* m,
                       bool key, const ::webhook_api::v1::ListPrevalidationsResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ListPrevalidationsResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_ListPrevalidationsResponse_remove(google::protobuf::Map<bool, ::webhook_api::v1::ListPrevalidationsResponse> * m,
                          bool key, ::webhook_api::v1::ListPrevalidationsResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_webhook_api_v1_ListPrevalidationsResponse_iter(
      const google::protobuf::Map<bool, ::webhook_api::v1::ListPrevalidationsResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_ListPrevalidationsResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::webhook_api::v1::ListPrevalidationsResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::webhook_api::v1::ListPrevalidationsResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::webhook_api::v1::ListPrevalidationsResponse>* __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListPrevalidationsResponse_new() {
    return new google::protobuf::Map<std::string, ::webhook_api::v1::ListPrevalidationsResponse>();
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListPrevalidationsResponse_free(const google::protobuf::Map<std::string, ::webhook_api::v1::ListPrevalidationsResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListPrevalidationsResponse_clear(google::protobuf::Map<std::string, ::webhook_api::v1::ListPrevalidationsResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListPrevalidationsResponse_size(const google::protobuf::Map<std::string, ::webhook_api::v1::ListPrevalidationsResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListPrevalidationsResponse_insert(google::protobuf::Map<std::string, ::webhook_api::v1::ListPrevalidationsResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::ListPrevalidationsResponse value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListPrevalidationsResponse_get(const google::protobuf::Map<std::string, ::webhook_api::v1::ListPrevalidationsResponse>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::webhook_api::v1::ListPrevalidationsResponse** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::ListPrevalidationsResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListPrevalidationsResponse_remove(google::protobuf::Map<std::string, ::webhook_api::v1::ListPrevalidationsResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::ListPrevalidationsResponse * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListPrevalidationsResponse_iter(
      const google::protobuf::Map<std::string, ::webhook_api::v1::ListPrevalidationsResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_ListPrevalidationsResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::webhook_api::v1::ListPrevalidationsResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::webhook_api::v1::ListPrevalidationsResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// webhook_api.v1.DeletePrevalidationRequest
        // clang-format off
extern "C" {
void* __rust_proto_thunk__webhook_api_v1_DeletePrevalidationRequest_new() { return new ::webhook_api::v1::DeletePrevalidationRequest(); }
void __rust_proto_thunk__webhook_api_v1_DeletePrevalidationRequest_delete(void* ptr) { delete static_cast<::webhook_api::v1::DeletePrevalidationRequest*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__webhook_api_v1_DeletePrevalidationRequest_serialize(::webhook_api::v1::DeletePrevalidationRequest* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__webhook_api_v1_DeletePrevalidationRequest_parse(::webhook_api::v1::DeletePrevalidationRequest* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__webhook_api_v1_DeletePrevalidationRequest_copy_from(::webhook_api::v1::DeletePrevalidationRequest* dst, const ::webhook_api::v1::DeletePrevalidationRequest* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__webhook_api_v1_DeletePrevalidationRequest_repeated_len(google::protobuf::RepeatedPtrField<::webhook_api::v1::DeletePrevalidationRequest>* field) {
  return field->size();
}
const ::webhook_api::v1::DeletePrevalidationRequest& __rust_proto_thunk__webhook_api_v1_DeletePrevalidationRequest_repeated_get(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::DeletePrevalidationRequest>* field,
  size_t index) {
  return field->Get(index);
}
::webhook_api::v1::DeletePrevalidationRequest* __rust_proto_thunk__webhook_api_v1_DeletePrevalidationRequest_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::DeletePrevalidationRequest>* field,
  size_t index) {
  return field->Mutable(index);
}
::webhook_api::v1::DeletePrevalidationRequest* __rust_proto_thunk__webhook_api_v1_DeletePrevalidationRequest_repeated_add(google::protobuf::RepeatedPtrField<::webhook_api::v1::DeletePrevalidationRequest>* field) {
  return field->Add();
}
void __rust_proto_thunk__webhook_api_v1_DeletePrevalidationRequest_repeated_clear(google::protobuf::RepeatedPtrField<::webhook_api::v1::DeletePrevalidationRequest>* field) {
  field->Clear();
}
void __rust_proto_thunk__webhook_api_v1_DeletePrevalidationRequest_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::DeletePrevalidationRequest>& dst,
  const google::protobuf::RepeatedPtrField<::webhook_api::v1::DeletePrevalidationRequest>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__webhook_api_v1_DeletePrevalidationRequest_get_api_key(::webhook_api::v1::DeletePrevalidationRequest* msg) {
  absl::string_view val = msg->api_key();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__webhook_api_v1_DeletePrevalidationRequest_set_api_key(::webhook_api::v1::DeletePrevalidationRequest* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_api_key(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__webhook_api_v1_DeletePrevalidationRequest_get_prevalidation_id(::webhook_api::v1::DeletePrevalidationRequest* msg) {
  absl::string_view val = msg->prevalidation_id();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__webhook_api_v1_DeletePrevalidationRequest_set_prevalidation_id(::webhook_api::v1::DeletePrevalidationRequest* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_prevalidation_id(absl::string_view(s.ptr, s.len));
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::webhook_api::v1::DeletePrevalidationRequest>* __rust_proto_thunk__Map_i32_webhook_api_v1_DeletePrevalidationRequest_new() {
    return new google::protobuf::Map<int32_t, ::webhook_api::v1::DeletePrevalidationRequest>();
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_DeletePrevalidationRequest_free(const google::protobuf::Map<int32_t, ::webhook_api::v1::DeletePrevalidationRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_DeletePrevalidationRequest_clear(google::protobuf::Map<int32_t, ::webhook_api::v1::DeletePrevalidationRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_webhook_api_v1_DeletePrevalidationRequest_size(const google::protobuf::Map<int32_t, ::webhook_api::v1::DeletePrevalidationRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_DeletePrevalidationRequest_insert(google::protobuf::Map<int32_t, ::webhook_api::v1::DeletePrevalidationRequest> * m,
                          int32_t key, ::webhook_api::v1::DeletePrevalidationRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_DeletePrevalidationRequest_get(const google::protobuf::Map<int32_t, ::webhook_api::v1::DeletePrevalidationRequest>* m,
                       int32_t key, const ::webhook_api::v1::DeletePrevalidationRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::DeletePrevalidationRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_DeletePrevalidationRequest_remove(google::protobuf::Map<int32_t, ::webhook_api::v1::DeletePrevalidationRequest> * m,
                          int32_t key, ::webhook_api::v1::DeletePrevalidationRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_webhook_api_v1_DeletePrevalidationRequest_iter(
      const google::protobuf::Map<int32_t, ::webhook_api::v1::DeletePrevalidationRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_DeletePrevalidationRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::webhook_api::v1::DeletePrevalidationRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::webhook_api::v1::DeletePrevalidationRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::webhook_api::v1::DeletePrevalidationRequest>* __rust_proto_thunk__Map_u32_webhook_api_v1_DeletePrevalidationRequest_new() {
    return new google::protobuf::Map<uint32_t, ::webhook_api::v1::DeletePrevalidationRequest>();
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_DeletePrevalidationRequest_free(const google::protobuf::Map<uint32_t, ::webhook_api::v1::DeletePrevalidationRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_DeletePrevalidationRequest_clear(google::protobuf::Map<uint32_t, ::webhook_api::v1::DeletePrevalidationRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_webhook_api_v1_DeletePrevalidationRequest_size(const google::protobuf::Map<uint32_t, ::webhook_api::v1::DeletePrevalidationRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_DeletePrevalidationRequest_insert(google::protobuf::Map<uint32_t, ::webhook_api::v1::DeletePrevalidationRequest> * m,
                          uint32_t key, ::webhook_api::v1::DeletePrevalidationRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_DeletePrevalidationRequest_get(const google::protobuf::Map<uint32_t, ::webhook_api::v1::DeletePrevalidationRequest>* m,
                       uint32_t key, const ::webhook_api::v1::DeletePrevalidationRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::DeletePrevalidationRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_DeletePrevalidationRequest_remove(google::protobuf::Map<uint32_t, ::webhook_api::v1::DeletePrevalidationRequest> * m,
                          uint32_t key, ::webhook_api::v1::DeletePrevalidationRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_webhook_api_v1_DeletePrevalidationRequest_iter(
      const google::protobuf::Map<uint32_t, ::webhook_api::v1::DeletePrevalidationRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_DeletePrevalidationRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::webhook_api::v1::DeletePrevalidationRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::webhook_api::v1::DeletePrevalidationRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::webhook_api::v1::DeletePrevalidationRequest>* __rust_proto_thunk__Map_i64_webhook_api_v1_DeletePrevalidationRequest_new() {
    return new google::protobuf::Map<int64_t, ::webhook_api::v1::DeletePrevalidationRequest>();
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_DeletePrevalidationRequest_free(const google::protobuf::Map<int64_t, ::webhook_api::v1::DeletePrevalidationRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_DeletePrevalidationRequest_clear(google::protobuf::Map<int64_t, ::webhook_api::v1::DeletePrevalidationRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_webhook_api_v1_DeletePrevalidationRequest_size(const google::protobuf::Map<int64_t, ::webhook_api::v1::DeletePrevalidationRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_DeletePrevalidationRequest_insert(google::protobuf::Map<int64_t, ::webhook_api::v1::DeletePrevalidationRequest> * m,
                          int64_t key, ::webhook_api::v1::DeletePrevalidationRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_DeletePrevalidationRequest_get(const google::protobuf::Map<int64_t, ::webhook_api::v1::DeletePrevalidationRequest>* m,
                       int64_t key, const ::webhook_api::v1::DeletePrevalidationRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::DeletePrevalidationRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_DeletePrevalidationRequest_remove(google::protobuf::Map<int64_t, ::webhook_api::v1::DeletePrevalidationRequest> * m,
                          int64_t key, ::webhook_api::v1::DeletePrevalidationRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_webhook_api_v1_DeletePrevalidationRequest_iter(
      const google::protobuf::Map<int64_t, ::webhook_api::v1::DeletePrevalidationRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_DeletePrevalidationRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::webhook_api::v1::DeletePrevalidationRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::webhook_api::v1::DeletePrevalidationRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::webhook_api::v1::DeletePrevalidationRequest>* __rust_proto_thunk__Map_u64_webhook_api_v1_DeletePrevalidationRequest_new() {
    return new google::protobuf::Map<uint64_t, ::webhook_api::v1::DeletePrevalidationRequest>();
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_DeletePrevalidationRequest_free(const google::protobuf::Map<uint64_t, ::webhook_api::v1::DeletePrevalidationRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_DeletePrevalidationRequest_clear(google::protobuf::Map<uint64_t, ::webhook_api::v1::DeletePrevalidationRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_webhook_api_v1_DeletePrevalidationRequest_size(const google::protobuf::Map<uint64_t, ::webhook_api::v1::DeletePrevalidationRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_DeletePrevalidationRequest_insert(google::protobuf::Map<uint64_t, ::webhook_api::v1::DeletePrevalidationRequest> * m,
                          uint64_t key, ::webhook_api::v1::DeletePrevalidationRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_DeletePrevalidationRequest_get(const google::protobuf::Map<uint64_t, ::webhook_api::v1::DeletePrevalidationRequest>* m,
                       uint64_t key, const ::webhook_api::v1::DeletePrevalidationRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::DeletePrevalidationRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_DeletePrevalidationRequest_remove(google::protobuf::Map<uint64_t, ::webhook_api::v1::DeletePrevalidationRequest> * m,
                          uint64_t key, ::webhook_api::v1::DeletePrevalidationRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_webhook_api_v1_DeletePrevalidationRequest_iter(
      const google::protobuf::Map<uint64_t, ::webhook_api::v1::DeletePrevalidationRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_DeletePrevalidationRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::webhook_api::v1::DeletePrevalidationRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::webhook_api::v1::DeletePrevalidationRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::webhook_api::v1::DeletePrevalidationRequest>* __rust_proto_thunk__Map_bool_webhook_api_v1_DeletePrevalidationRequest_new() {
    return new google::protobuf::Map<bool, ::webhook_api::v1::DeletePrevalidationRequest>();
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_DeletePrevalidationRequest_free(const google::protobuf::Map<bool, ::webhook_api::v1::DeletePrevalidationRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_DeletePrevalidationRequest_clear(google::protobuf::Map<bool, ::webhook_api::v1::DeletePrevalidationRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_webhook_api_v1_DeletePrevalidationRequest_size(const google::protobuf::Map<bool, ::webhook_api::v1::DeletePrevalidationRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_DeletePrevalidationRequest_insert(google::protobuf::Map<bool, ::webhook_api::v1::DeletePrevalidationRequest> * m,
                          bool key, ::webhook_api::v1::DeletePrevalidationRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_DeletePrevalidationRequest_get(const google::protobuf::Map<bool, ::webhook_api::v1::DeletePrevalidationRequest>* m,
                       bool key, const ::webhook_api::v1::DeletePrevalidationRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::DeletePrevalidationRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_DeletePrevalidationRequest_remove(google::protobuf::Map<bool, ::webhook_api::v1::DeletePrevalidationRequest> * m,
                          bool key, ::webhook_api::v1::DeletePrevalidationRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_webhook_api_v1_DeletePrevalidationRequest_iter(
      const google::protobuf::Map<bool, ::webhook_api::v1::DeletePrevalidationRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_DeletePrevalidationRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::webhook_api::v1::DeletePrevalidationRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::webhook_api::v1::DeletePrevalidationRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::webhook_api::v1::DeletePrevalidationRequest>* __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_DeletePrevalidationRequest_new() {
    return new google::protobuf::Map<std::string, ::webhook_api::v1::DeletePrevalidationRequest>();
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_DeletePrevalidationRequest_free(const google::protobuf::Map<std::string, ::webhook_api::v1::DeletePrevalidationRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_DeletePrevalidationRequest_clear(google::protobuf::Map<std::string, ::webhook_api::v1::DeletePrevalidationRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_DeletePrevalidationRequest_size(const google::protobuf::Map<std::string, ::webhook_api::v1::DeletePrevalidationRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_DeletePrevalidationRequest_insert(google::protobuf::Map<std::string, ::webhook_api::v1::DeletePrevalidationRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::DeletePrevalidationRequest value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_DeletePrevalidationRequest_get(const google::protobuf::Map<std::string, ::webhook_api::v1::DeletePrevalidationRequest>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::webhook_api::v1::DeletePrevalidationRequest** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::DeletePrevalidationRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_DeletePrevalidationRequest_remove(google::protobuf::Map<std::string, ::webhook_api::v1::DeletePrevalidationRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::DeletePrevalidationRequest * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_DeletePrevalidationRequest_iter(
      const google::protobuf::Map<std::string, ::webhook_api::v1::DeletePrevalidationRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_DeletePrevalidationRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::webhook_api::v1::DeletePrevalidationRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::webhook_api::v1::DeletePrevalidationRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// webhook_api.v1.DeletePrevalidationResponse
        // clang-format off
extern "C" {
void* __rust_proto_thunk__webhook_api_v1_DeletePrevalidationResponse_new() { return new ::webhook_api::v1::DeletePrevalidationResponse(); }
void __rust_proto_thunk__webhook_api_v1_DeletePrevalidationResponse_delete(void* ptr) { delete static_cast<::webhook_api::v1::DeletePrevalidationResponse*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__webhook_api_v1_DeletePrevalidationResponse_serialize(::webhook_api::v1::DeletePrevalidationResponse* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__webhook_api_v1_DeletePrevalidationResponse_parse(::webhook_api::v1::DeletePrevalidationResponse* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__webhook_api_v1_DeletePrevalidationResponse_copy_from(::webhook_api::v1::DeletePrevalidationResponse* dst, const ::webhook_api::v1::DeletePrevalidationResponse* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__webhook_api_v1_DeletePrevalidationResponse_repeated_len(google::protobuf::RepeatedPtrField<::webhook_api::v1::DeletePrevalidationResponse>* field) {
  return field->size();
}
const ::webhook_api::v1::DeletePrevalidationResponse& __rust_proto_thunk__webhook_api_v1_DeletePrevalidationResponse_repeated_get(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::DeletePrevalidationResponse>* field,
  size_t index) {
  return field->Get(index);
}
::webhook_api::v1::DeletePrevalidationResponse* __rust_proto_thunk__webhook_api_v1_DeletePrevalidationResponse_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::DeletePrevalidationResponse>* field,
  size_t index) {
  return field->Mutable(index);
}
::webhook_api::v1::DeletePrevalidationResponse* __rust_proto_thunk__webhook_api_v1_DeletePrevalidationResponse_repeated_add(google::protobuf::RepeatedPtrField<::webhook_api::v1::DeletePrevalidationResponse>* field) {
  return field->Add();
}
void __rust_proto_thunk__webhook_api_v1_DeletePrevalidationResponse_repeated_clear(google::protobuf::RepeatedPtrField<::webhook_api::v1::DeletePrevalidationResponse>* field) {
  field->Clear();
}
void __rust_proto_thunk__webhook_api_v1_DeletePrevalidationResponse_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::webhook_api::v1::DeletePrevalidationResponse>& dst,
  const google::protobuf::RepeatedPtrField<::webhook_api::v1::DeletePrevalidationResponse>& src) {
  dst = src;
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::webhook_api::v1::DeletePrevalidationResponse>* __rust_proto_thunk__Map_i32_webhook_api_v1_DeletePrevalidationResponse_new() {
    return new google::protobuf::Map<int32_t, ::webhook_api::v1::DeletePrevalidationResponse>();
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_DeletePrevalidationResponse_free(const google::protobuf::Map<int32_t, ::webhook_api::v1::DeletePrevalidationResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_DeletePrevalidationResponse_clear(google::protobuf::Map<int32_t, ::webhook_api::v1::DeletePrevalidationResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_webhook_api_v1_DeletePrevalidationResponse_size(const google::protobuf::Map<int32_t, ::webhook_api::v1::DeletePrevalidationResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_DeletePrevalidationResponse_insert(google::protobuf::Map<int32_t, ::webhook_api::v1::DeletePrevalidationResponse> * m,
                          int32_t key, ::webhook_api::v1::DeletePrevalidationResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_DeletePrevalidationResponse_get(const google::protobuf::Map<int32_t, ::webhook_api::v1::DeletePrevalidationResponse>* m,
                       int32_t key, const ::webhook_api::v1::DeletePrevalidationResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::DeletePrevalidationResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_webhook_api_v1_DeletePrevalidationResponse_remove(google::protobuf::Map<int32_t, ::webhook_api::v1::DeletePrevalidationResponse> * m,
                          int32_t key, ::webhook_api::v1::DeletePrevalidationResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_webhook_api_v1_DeletePrevalidationResponse_iter(
      const google::protobuf::Map<int32_t, ::webhook_api::v1::DeletePrevalidationResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_webhook_api_v1_DeletePrevalidationResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::webhook_api::v1::DeletePrevalidationResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::webhook_api::v1::DeletePrevalidationResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::webhook_api::v1::DeletePrevalidationResponse>* __rust_proto_thunk__Map_u32_webhook_api_v1_DeletePrevalidationResponse_new() {
    return new google::protobuf::Map<uint32_t, ::webhook_api::v1::DeletePrevalidationResponse>();
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_DeletePrevalidationResponse_free(const google::protobuf::Map<uint32_t, ::webhook_api::v1::DeletePrevalidationResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_DeletePrevalidationResponse_clear(google::protobuf::Map<uint32_t, ::webhook_api::v1::DeletePrevalidationResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_webhook_api_v1_DeletePrevalidationResponse_size(const google::protobuf::Map<uint32_t, ::webhook_api::v1::DeletePrevalidationResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_DeletePrevalidationResponse_insert(google::protobuf::Map<uint32_t, ::webhook_api::v1::DeletePrevalidationResponse> * m,
                          uint32_t key, ::webhook_api::v1::DeletePrevalidationResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_DeletePrevalidationResponse_get(const google::protobuf::Map<uint32_t, ::webhook_api::v1::DeletePrevalidationResponse>* m,
                       uint32_t key, const ::webhook_api::v1::DeletePrevalidationResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::DeletePrevalidationResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_webhook_api_v1_DeletePrevalidationResponse_remove(google::protobuf::Map<uint32_t, ::webhook_api::v1::DeletePrevalidationResponse> * m,
                          uint32_t key, ::webhook_api::v1::DeletePrevalidationResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_webhook_api_v1_DeletePrevalidationResponse_iter(
      const google::protobuf::Map<uint32_t, ::webhook_api::v1::DeletePrevalidationResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_webhook_api_v1_DeletePrevalidationResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::webhook_api::v1::DeletePrevalidationResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::webhook_api::v1::DeletePrevalidationResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::webhook_api::v1::DeletePrevalidationResponse>* __rust_proto_thunk__Map_i64_webhook_api_v1_DeletePrevalidationResponse_new() {
    return new google::protobuf::Map<int64_t, ::webhook_api::v1::DeletePrevalidationResponse>();
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_DeletePrevalidationResponse_free(const google::protobuf::Map<int64_t, ::webhook_api::v1::DeletePrevalidationResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_DeletePrevalidationResponse_clear(google::protobuf::Map<int64_t, ::webhook_api::v1::DeletePrevalidationResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_webhook_api_v1_DeletePrevalidationResponse_size(const google::protobuf::Map<int64_t, ::webhook_api::v1::DeletePrevalidationResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_DeletePrevalidationResponse_insert(google::protobuf::Map<int64_t, ::webhook_api::v1::DeletePrevalidationResponse> * m,
                          int64_t key, ::webhook_api::v1::DeletePrevalidationResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_DeletePrevalidationResponse_get(const google::protobuf::Map<int64_t, ::webhook_api::v1::DeletePrevalidationResponse>* m,
                       int64_t key, const ::webhook_api::v1::DeletePrevalidationResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::DeletePrevalidationResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_webhook_api_v1_DeletePrevalidationResponse_remove(google::protobuf::Map<int64_t, ::webhook_api::v1::DeletePrevalidationResponse> * m,
                          int64_t key, ::webhook_api::v1::DeletePrevalidationResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_webhook_api_v1_DeletePrevalidationResponse_iter(
      const google::protobuf::Map<int64_t, ::webhook_api::v1::DeletePrevalidationResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_webhook_api_v1_DeletePrevalidationResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::webhook_api::v1::DeletePrevalidationResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::webhook_api::v1::DeletePrevalidationResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::webhook_api::v1::DeletePrevalidationResponse>* __rust_proto_thunk__Map_u64_webhook_api_v1_DeletePrevalidationResponse_new() {
    return new google::protobuf::Map<uint64_t, ::webhook_api::v1::DeletePrevalidationResponse>();
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_DeletePrevalidationResponse_free(const google::protobuf::Map<uint64_t, ::webhook_api::v1::DeletePrevalidationResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_DeletePrevalidationResponse_clear(google::protobuf::Map<uint64_t, ::webhook_api::v1::DeletePrevalidationResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_webhook_api_v1_DeletePrevalidationResponse_size(const google::protobuf::Map<uint64_t, ::webhook_api::v1::DeletePrevalidationResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_DeletePrevalidationResponse_insert(google::protobuf::Map<uint64_t, ::webhook_api::v1::DeletePrevalidationResponse> * m,
                          uint64_t key, ::webhook_api::v1::DeletePrevalidationResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_DeletePrevalidationResponse_get(const google::protobuf::Map<uint64_t, ::webhook_api::v1::DeletePrevalidationResponse>* m,
                       uint64_t key, const ::webhook_api::v1::DeletePrevalidationResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::DeletePrevalidationResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_webhook_api_v1_DeletePrevalidationResponse_remove(google::protobuf::Map<uint64_t, ::webhook_api::v1::DeletePrevalidationResponse> * m,
                          uint64_t key, ::webhook_api::v1::DeletePrevalidationResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_webhook_api_v1_DeletePrevalidationResponse_iter(
      const google::protobuf::Map<uint64_t, ::webhook_api::v1::DeletePrevalidationResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_webhook_api_v1_DeletePrevalidationResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::webhook_api::v1::DeletePrevalidationResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::webhook_api::v1::DeletePrevalidationResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::webhook_api::v1::DeletePrevalidationResponse>* __rust_proto_thunk__Map_bool_webhook_api_v1_DeletePrevalidationResponse_new() {
    return new google::protobuf::Map<bool, ::webhook_api::v1::DeletePrevalidationResponse>();
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_DeletePrevalidationResponse_free(const google::protobuf::Map<bool, ::webhook_api::v1::DeletePrevalidationResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_DeletePrevalidationResponse_clear(google::protobuf::Map<bool, ::webhook_api::v1::DeletePrevalidationResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_webhook_api_v1_DeletePrevalidationResponse_size(const google::protobuf::Map<bool, ::webhook_api::v1::DeletePrevalidationResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_DeletePrevalidationResponse_insert(google::protobuf::Map<bool, ::webhook_api::v1::DeletePrevalidationResponse> * m,
                          bool key, ::webhook_api::v1::DeletePrevalidationResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_DeletePrevalidationResponse_get(const google::protobuf::Map<bool, ::webhook_api::v1::DeletePrevalidationResponse>* m,
                       bool key, const ::webhook_api::v1::DeletePrevalidationResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::DeletePrevalidationResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_webhook_api_v1_DeletePrevalidationResponse_remove(google::protobuf::Map<bool, ::webhook_api::v1::DeletePrevalidationResponse> * m,
                          bool key, ::webhook_api::v1::DeletePrevalidationResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_webhook_api_v1_DeletePrevalidationResponse_iter(
      const google::protobuf::Map<bool, ::webhook_api::v1::DeletePrevalidationResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_webhook_api_v1_DeletePrevalidationResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::webhook_api::v1::DeletePrevalidationResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::webhook_api::v1::DeletePrevalidationResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::webhook_api::v1::DeletePrevalidationResponse>* __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_DeletePrevalidationResponse_new() {
    return new google::protobuf::Map<std::string, ::webhook_api::v1::DeletePrevalidationResponse>();
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_DeletePrevalidationResponse_free(const google::protobuf::Map<std::string, ::webhook_api::v1::DeletePrevalidationResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_DeletePrevalidationResponse_clear(google::protobuf::Map<std::string, ::webhook_api::v1::DeletePrevalidationResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_DeletePrevalidationResponse_size(const google::protobuf::Map<std::string, ::webhook_api::v1::DeletePrevalidationResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_DeletePrevalidationResponse_insert(google::protobuf::Map<std::string, ::webhook_api::v1::DeletePrevalidationResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::DeletePrevalidationResponse value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_DeletePrevalidationResponse_get(const google::protobuf::Map<std::string, ::webhook_api::v1::DeletePrevalidationResponse>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::webhook_api::v1::DeletePrevalidationResponse** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::webhook_api::v1::DeletePrevalidationResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_DeletePrevalidationResponse_remove(google::protobuf::Map<std::string, ::webhook_api::v1::DeletePrevalidationResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::webhook_api::v1::DeletePrevalidationResponse * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_DeletePrevalidationResponse_iter(
      const google::protobuf::Map<std::string, ::webhook_api::v1::DeletePrevalidationResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_webhook_api_v1_DeletePrevalidationResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::webhook_api::v1::DeletePrevalidationResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::webhook_api::v1::DeletePrevalidationResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// webhook_api.v1.DeliveryStage
extern "C" {
  __PB_RUST_EXPOSE_SCALAR_MAP_METHODS_FOR_VALUE_TYPE(
      ::webhook_api::v1::DeliveryStage, webhook_api_v1_DeliveryStage, ::webhook_api::v1::DeliveryStage, value, cpp_value)
}

// webhook_api.v1.EventType
extern "C" {
  __PB_RUST_EXPOSE_SCALAR_MAP_METHODS_FOR_VALUE_TYPE(
      ::webhook_api::v1::EventType, webhook_api_v1_EventType, ::webhook_api::v1::EventType, value, cpp_value)
}

// webhook_api.v1.DeliveryMethodType
extern "C" {
  __PB_RUST_EXPOSE_SCALAR_MAP_METHODS_FOR_VALUE_TYPE(
      ::webhook_api::v1::DeliveryMethodType, webhook_api_v1_DeliveryMethodType, ::webhook_api::v1::DeliveryMethodType, value, cpp_value)
}

// webhook_api.v1.ScheduleReactionType
extern "C" {
  __PB_RUST_EXPOSE_SCALAR_MAP_METHODS_FOR_VALUE_TYPE(
      ::webhook_api::v1::ScheduleReactionType, webhook_api_v1_ScheduleReactionType, ::webhook_api::v1::ScheduleReactionType, value, cpp_value)
}

// webhook_api.v1.PrevalidationTriggerType
extern "C" {
  __PB_RUST_EXPOSE_SCALAR_MAP_METHODS_FOR_VALUE_TYPE(
      ::webhook_api::v1::PrevalidationTriggerType, webhook_api_v1_PrevalidationTriggerType, ::webhook_api::v1::PrevalidationTriggerType, value, cpp_value)
}

// webhook_api.v1.ValidationOperator
extern "C" {
  __PB_RUST_EXPOSE_SCALAR_MAP_METHODS_FOR_VALUE_TYPE(
      ::webhook_api::v1::ValidationOperator, webhook_api_v1_ValidationOperator, ::webhook_api::v1::ValidationOperator, value, cpp_value)
}

// webhook_api.v1.ValidationTarget
extern "C" {
  __PB_RUST_EXPOSE_SCALAR_MAP_METHODS_FOR_VALUE_TYPE(
      ::webhook_api::v1::ValidationTarget, webhook_api_v1_ValidationTarget, ::webhook_api::v1::ValidationTarget, value, cpp_value)
}

