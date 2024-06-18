#include "common/v1/packet.proto.h"
#include "google/protobuf/rust/cpp_kernel/cpp_api.h"
// common.Packet
        // clang-format off
extern "C" {
void* __rust_proto_thunk__common_Packet_new() { return new ::common::Packet(); }
void __rust_proto_thunk__common_Packet_delete(void* ptr) { delete static_cast<::common::Packet*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__common_Packet_serialize(::common::Packet* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__common_Packet_parse(::common::Packet* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__common_Packet_copy_from(::common::Packet* dst, const ::common::Packet* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__common_Packet_repeated_len(google::protobuf::RepeatedPtrField<::common::Packet>* field) {
  return field->size();
}
const ::common::Packet& __rust_proto_thunk__common_Packet_repeated_get(
  google::protobuf::RepeatedPtrField<::common::Packet>* field,
  size_t index) {
  return field->Get(index);
}
::common::Packet* __rust_proto_thunk__common_Packet_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::common::Packet>* field,
  size_t index) {
  return field->Mutable(index);
}
::common::Packet* __rust_proto_thunk__common_Packet_repeated_add(google::protobuf::RepeatedPtrField<::common::Packet>* field) {
  return field->Add();
}
void __rust_proto_thunk__common_Packet_repeated_clear(google::protobuf::RepeatedPtrField<::common::Packet>* field) {
  field->Clear();
}
void __rust_proto_thunk__common_Packet_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::common::Packet>& dst,
  const google::protobuf::RepeatedPtrField<::common::Packet>& src) {
  dst = src;
}

::common::PacketType __rust_proto_thunk__common_Packet_get_packet_type(::common::Packet* msg) { return msg->packet_type(); }
void __rust_proto_thunk__common_Packet_set_packet_type(::common::Packet* msg, ::common::PacketType val) {
  msg->set_packet_type(val);
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__common_Packet_get_data(::common::Packet* msg) {
  absl::string_view val = msg->data();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__common_Packet_set_data(::common::Packet* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_data(absl::string_view(s.ptr, s.len));
}

const void* __rust_proto_thunk__common_Packet_get_sender(::common::Packet* msg) {
  return static_cast<const void*>(&msg->sender());
}
void* __rust_proto_thunk__common_Packet_get_mut_sender(::common::Packet* msg) {
  return static_cast<void*>(msg->mutable_sender());
}
void __rust_proto_thunk__common_Packet_clear_sender(::common::Packet* msg) { msg->clear_sender(); }
bool __rust_proto_thunk__common_Packet_has_sender(::common::Packet* msg) { return msg->has_sender(); }

const void* __rust_proto_thunk__common_Packet_get_recipient(::common::Packet* msg) {
  return static_cast<const void*>(&msg->recipient());
}
void* __rust_proto_thunk__common_Packet_get_mut_recipient(::common::Packet* msg) {
  return static_cast<void*>(msg->mutable_recipient());
}
void __rust_proto_thunk__common_Packet_clear_recipient(::common::Packet* msg) { msg->clear_recipient(); }
bool __rust_proto_thunk__common_Packet_has_recipient(::common::Packet* msg) { return msg->has_recipient(); }

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__common_Packet_get_nonce(::common::Packet* msg) {
  absl::string_view val = msg->nonce();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__common_Packet_set_nonce(::common::Packet* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_nonce(absl::string_view(s.ptr, s.len));
}

const void* __rust_proto_thunk__common_Packet_get_transaction_header(::common::Packet* msg) {
  return static_cast<const void*>(&msg->transaction_header());
}
void* __rust_proto_thunk__common_Packet_get_mut_transaction_header(::common::Packet* msg) {
  return static_cast<void*>(msg->mutable_transaction_header());
}
void __rust_proto_thunk__common_Packet_clear_transaction_header(::common::Packet* msg) { msg->clear_transaction_header(); }
bool __rust_proto_thunk__common_Packet_has_transaction_header(::common::Packet* msg) { return msg->has_transaction_header(); }


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::common::Packet>* __rust_proto_thunk__Map_i32_common_Packet_new() {
    return new google::protobuf::Map<int32_t, ::common::Packet>();
  }
  void __rust_proto_thunk__Map_i32_common_Packet_free(const google::protobuf::Map<int32_t, ::common::Packet>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_common_Packet_clear(google::protobuf::Map<int32_t, ::common::Packet> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_common_Packet_size(const google::protobuf::Map<int32_t, ::common::Packet>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_common_Packet_insert(google::protobuf::Map<int32_t, ::common::Packet> * m,
                          int32_t key, ::common::Packet value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_Packet_get(const google::protobuf::Map<int32_t, ::common::Packet>* m,
                       int32_t key, const ::common::Packet** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::Packet* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_Packet_remove(google::protobuf::Map<int32_t, ::common::Packet> * m,
                          int32_t key, ::common::Packet * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_common_Packet_iter(
      const google::protobuf::Map<int32_t, ::common::Packet>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_common_Packet_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::common::Packet** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::common::Packet>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::common::Packet>* __rust_proto_thunk__Map_u32_common_Packet_new() {
    return new google::protobuf::Map<uint32_t, ::common::Packet>();
  }
  void __rust_proto_thunk__Map_u32_common_Packet_free(const google::protobuf::Map<uint32_t, ::common::Packet>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_common_Packet_clear(google::protobuf::Map<uint32_t, ::common::Packet> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_common_Packet_size(const google::protobuf::Map<uint32_t, ::common::Packet>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_common_Packet_insert(google::protobuf::Map<uint32_t, ::common::Packet> * m,
                          uint32_t key, ::common::Packet value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_Packet_get(const google::protobuf::Map<uint32_t, ::common::Packet>* m,
                       uint32_t key, const ::common::Packet** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::Packet* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_Packet_remove(google::protobuf::Map<uint32_t, ::common::Packet> * m,
                          uint32_t key, ::common::Packet * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_common_Packet_iter(
      const google::protobuf::Map<uint32_t, ::common::Packet>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_common_Packet_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::common::Packet** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::common::Packet>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::common::Packet>* __rust_proto_thunk__Map_i64_common_Packet_new() {
    return new google::protobuf::Map<int64_t, ::common::Packet>();
  }
  void __rust_proto_thunk__Map_i64_common_Packet_free(const google::protobuf::Map<int64_t, ::common::Packet>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_common_Packet_clear(google::protobuf::Map<int64_t, ::common::Packet> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_common_Packet_size(const google::protobuf::Map<int64_t, ::common::Packet>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_common_Packet_insert(google::protobuf::Map<int64_t, ::common::Packet> * m,
                          int64_t key, ::common::Packet value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_Packet_get(const google::protobuf::Map<int64_t, ::common::Packet>* m,
                       int64_t key, const ::common::Packet** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::Packet* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_Packet_remove(google::protobuf::Map<int64_t, ::common::Packet> * m,
                          int64_t key, ::common::Packet * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_common_Packet_iter(
      const google::protobuf::Map<int64_t, ::common::Packet>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_common_Packet_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::common::Packet** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::common::Packet>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::common::Packet>* __rust_proto_thunk__Map_u64_common_Packet_new() {
    return new google::protobuf::Map<uint64_t, ::common::Packet>();
  }
  void __rust_proto_thunk__Map_u64_common_Packet_free(const google::protobuf::Map<uint64_t, ::common::Packet>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_common_Packet_clear(google::protobuf::Map<uint64_t, ::common::Packet> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_common_Packet_size(const google::protobuf::Map<uint64_t, ::common::Packet>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_common_Packet_insert(google::protobuf::Map<uint64_t, ::common::Packet> * m,
                          uint64_t key, ::common::Packet value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_Packet_get(const google::protobuf::Map<uint64_t, ::common::Packet>* m,
                       uint64_t key, const ::common::Packet** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::Packet* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_Packet_remove(google::protobuf::Map<uint64_t, ::common::Packet> * m,
                          uint64_t key, ::common::Packet * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_common_Packet_iter(
      const google::protobuf::Map<uint64_t, ::common::Packet>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_common_Packet_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::common::Packet** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::common::Packet>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::common::Packet>* __rust_proto_thunk__Map_bool_common_Packet_new() {
    return new google::protobuf::Map<bool, ::common::Packet>();
  }
  void __rust_proto_thunk__Map_bool_common_Packet_free(const google::protobuf::Map<bool, ::common::Packet>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_common_Packet_clear(google::protobuf::Map<bool, ::common::Packet> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_common_Packet_size(const google::protobuf::Map<bool, ::common::Packet>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_common_Packet_insert(google::protobuf::Map<bool, ::common::Packet> * m,
                          bool key, ::common::Packet value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_Packet_get(const google::protobuf::Map<bool, ::common::Packet>* m,
                       bool key, const ::common::Packet** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::Packet* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_Packet_remove(google::protobuf::Map<bool, ::common::Packet> * m,
                          bool key, ::common::Packet * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_common_Packet_iter(
      const google::protobuf::Map<bool, ::common::Packet>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_common_Packet_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::common::Packet** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::common::Packet>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::common::Packet>* __rust_proto_thunk__Map_ProtoStr_common_Packet_new() {
    return new google::protobuf::Map<std::string, ::common::Packet>();
  }
  void __rust_proto_thunk__Map_ProtoStr_common_Packet_free(const google::protobuf::Map<std::string, ::common::Packet>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_common_Packet_clear(google::protobuf::Map<std::string, ::common::Packet> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_common_Packet_size(const google::protobuf::Map<std::string, ::common::Packet>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_Packet_insert(google::protobuf::Map<std::string, ::common::Packet> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::Packet value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_Packet_get(const google::protobuf::Map<std::string, ::common::Packet>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::common::Packet** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::common::Packet* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_Packet_remove(google::protobuf::Map<std::string, ::common::Packet> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::Packet * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_common_Packet_iter(
      const google::protobuf::Map<std::string, ::common::Packet>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_common_Packet_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::common::Packet** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::common::Packet>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// common.TransactionHeader
        // clang-format off
extern "C" {
void* __rust_proto_thunk__common_TransactionHeader_new() { return new ::common::TransactionHeader(); }
void __rust_proto_thunk__common_TransactionHeader_delete(void* ptr) { delete static_cast<::common::TransactionHeader*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__common_TransactionHeader_serialize(::common::TransactionHeader* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__common_TransactionHeader_parse(::common::TransactionHeader* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__common_TransactionHeader_copy_from(::common::TransactionHeader* dst, const ::common::TransactionHeader* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__common_TransactionHeader_repeated_len(google::protobuf::RepeatedPtrField<::common::TransactionHeader>* field) {
  return field->size();
}
const ::common::TransactionHeader& __rust_proto_thunk__common_TransactionHeader_repeated_get(
  google::protobuf::RepeatedPtrField<::common::TransactionHeader>* field,
  size_t index) {
  return field->Get(index);
}
::common::TransactionHeader* __rust_proto_thunk__common_TransactionHeader_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::common::TransactionHeader>* field,
  size_t index) {
  return field->Mutable(index);
}
::common::TransactionHeader* __rust_proto_thunk__common_TransactionHeader_repeated_add(google::protobuf::RepeatedPtrField<::common::TransactionHeader>* field) {
  return field->Add();
}
void __rust_proto_thunk__common_TransactionHeader_repeated_clear(google::protobuf::RepeatedPtrField<::common::TransactionHeader>* field) {
  field->Clear();
}
void __rust_proto_thunk__common_TransactionHeader_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::common::TransactionHeader>& dst,
  const google::protobuf::RepeatedPtrField<::common::TransactionHeader>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__common_TransactionHeader_get_uetr(::common::TransactionHeader* msg) {
  absl::string_view val = msg->uetr();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__common_TransactionHeader_set_uetr(::common::TransactionHeader* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_uetr(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__common_TransactionHeader_get_urn(::common::TransactionHeader* msg) {
  absl::string_view val = msg->urn();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__common_TransactionHeader_set_urn(::common::TransactionHeader* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_urn(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__common_TransactionHeader_get_memo(::common::TransactionHeader* msg) {
  absl::string_view val = msg->memo();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__common_TransactionHeader_set_memo(::common::TransactionHeader* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_memo(absl::string_view(s.ptr, s.len));
}

::uint64_t __rust_proto_thunk__common_TransactionHeader_get_timestamp(::common::TransactionHeader* msg) { return msg->timestamp(); }
void __rust_proto_thunk__common_TransactionHeader_set_timestamp(::common::TransactionHeader* msg, ::uint64_t val) {
  msg->set_timestamp(val);
}

void __rust_proto_thunk__common_TransactionHeader_clear_related_uetrs(::common::TransactionHeader* msg) {
  msg->clear_related_uetrs();
}
google::protobuf::RepeatedPtrField<std::string>* __rust_proto_thunk__common_TransactionHeader_get_mut_related_uetrs(::common::TransactionHeader* msg) {
  return msg->mutable_related_uetrs();
}
const google::protobuf::RepeatedPtrField<std::string>* __rust_proto_thunk__common_TransactionHeader_get_related_uetrs(
    const ::common::TransactionHeader* msg) {
  return &msg->related_uetrs();
}

const void* __rust_proto_thunk__common_TransactionHeader_get_metadata(const ::common::TransactionHeader* msg) {
  return &msg->metadata();
}
void* __rust_proto_thunk__common_TransactionHeader_get_mut_metadata(::common::TransactionHeader* msg) { return msg->mutable_metadata(); }


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::common::TransactionHeader>* __rust_proto_thunk__Map_i32_common_TransactionHeader_new() {
    return new google::protobuf::Map<int32_t, ::common::TransactionHeader>();
  }
  void __rust_proto_thunk__Map_i32_common_TransactionHeader_free(const google::protobuf::Map<int32_t, ::common::TransactionHeader>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_common_TransactionHeader_clear(google::protobuf::Map<int32_t, ::common::TransactionHeader> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_common_TransactionHeader_size(const google::protobuf::Map<int32_t, ::common::TransactionHeader>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_common_TransactionHeader_insert(google::protobuf::Map<int32_t, ::common::TransactionHeader> * m,
                          int32_t key, ::common::TransactionHeader value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_TransactionHeader_get(const google::protobuf::Map<int32_t, ::common::TransactionHeader>* m,
                       int32_t key, const ::common::TransactionHeader** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::TransactionHeader* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_TransactionHeader_remove(google::protobuf::Map<int32_t, ::common::TransactionHeader> * m,
                          int32_t key, ::common::TransactionHeader * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_common_TransactionHeader_iter(
      const google::protobuf::Map<int32_t, ::common::TransactionHeader>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_common_TransactionHeader_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::common::TransactionHeader** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::common::TransactionHeader>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::common::TransactionHeader>* __rust_proto_thunk__Map_u32_common_TransactionHeader_new() {
    return new google::protobuf::Map<uint32_t, ::common::TransactionHeader>();
  }
  void __rust_proto_thunk__Map_u32_common_TransactionHeader_free(const google::protobuf::Map<uint32_t, ::common::TransactionHeader>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_common_TransactionHeader_clear(google::protobuf::Map<uint32_t, ::common::TransactionHeader> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_common_TransactionHeader_size(const google::protobuf::Map<uint32_t, ::common::TransactionHeader>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_common_TransactionHeader_insert(google::protobuf::Map<uint32_t, ::common::TransactionHeader> * m,
                          uint32_t key, ::common::TransactionHeader value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_TransactionHeader_get(const google::protobuf::Map<uint32_t, ::common::TransactionHeader>* m,
                       uint32_t key, const ::common::TransactionHeader** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::TransactionHeader* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_TransactionHeader_remove(google::protobuf::Map<uint32_t, ::common::TransactionHeader> * m,
                          uint32_t key, ::common::TransactionHeader * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_common_TransactionHeader_iter(
      const google::protobuf::Map<uint32_t, ::common::TransactionHeader>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_common_TransactionHeader_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::common::TransactionHeader** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::common::TransactionHeader>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::common::TransactionHeader>* __rust_proto_thunk__Map_i64_common_TransactionHeader_new() {
    return new google::protobuf::Map<int64_t, ::common::TransactionHeader>();
  }
  void __rust_proto_thunk__Map_i64_common_TransactionHeader_free(const google::protobuf::Map<int64_t, ::common::TransactionHeader>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_common_TransactionHeader_clear(google::protobuf::Map<int64_t, ::common::TransactionHeader> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_common_TransactionHeader_size(const google::protobuf::Map<int64_t, ::common::TransactionHeader>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_common_TransactionHeader_insert(google::protobuf::Map<int64_t, ::common::TransactionHeader> * m,
                          int64_t key, ::common::TransactionHeader value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_TransactionHeader_get(const google::protobuf::Map<int64_t, ::common::TransactionHeader>* m,
                       int64_t key, const ::common::TransactionHeader** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::TransactionHeader* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_TransactionHeader_remove(google::protobuf::Map<int64_t, ::common::TransactionHeader> * m,
                          int64_t key, ::common::TransactionHeader * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_common_TransactionHeader_iter(
      const google::protobuf::Map<int64_t, ::common::TransactionHeader>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_common_TransactionHeader_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::common::TransactionHeader** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::common::TransactionHeader>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::common::TransactionHeader>* __rust_proto_thunk__Map_u64_common_TransactionHeader_new() {
    return new google::protobuf::Map<uint64_t, ::common::TransactionHeader>();
  }
  void __rust_proto_thunk__Map_u64_common_TransactionHeader_free(const google::protobuf::Map<uint64_t, ::common::TransactionHeader>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_common_TransactionHeader_clear(google::protobuf::Map<uint64_t, ::common::TransactionHeader> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_common_TransactionHeader_size(const google::protobuf::Map<uint64_t, ::common::TransactionHeader>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_common_TransactionHeader_insert(google::protobuf::Map<uint64_t, ::common::TransactionHeader> * m,
                          uint64_t key, ::common::TransactionHeader value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_TransactionHeader_get(const google::protobuf::Map<uint64_t, ::common::TransactionHeader>* m,
                       uint64_t key, const ::common::TransactionHeader** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::TransactionHeader* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_TransactionHeader_remove(google::protobuf::Map<uint64_t, ::common::TransactionHeader> * m,
                          uint64_t key, ::common::TransactionHeader * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_common_TransactionHeader_iter(
      const google::protobuf::Map<uint64_t, ::common::TransactionHeader>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_common_TransactionHeader_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::common::TransactionHeader** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::common::TransactionHeader>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::common::TransactionHeader>* __rust_proto_thunk__Map_bool_common_TransactionHeader_new() {
    return new google::protobuf::Map<bool, ::common::TransactionHeader>();
  }
  void __rust_proto_thunk__Map_bool_common_TransactionHeader_free(const google::protobuf::Map<bool, ::common::TransactionHeader>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_common_TransactionHeader_clear(google::protobuf::Map<bool, ::common::TransactionHeader> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_common_TransactionHeader_size(const google::protobuf::Map<bool, ::common::TransactionHeader>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_common_TransactionHeader_insert(google::protobuf::Map<bool, ::common::TransactionHeader> * m,
                          bool key, ::common::TransactionHeader value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_TransactionHeader_get(const google::protobuf::Map<bool, ::common::TransactionHeader>* m,
                       bool key, const ::common::TransactionHeader** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::TransactionHeader* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_TransactionHeader_remove(google::protobuf::Map<bool, ::common::TransactionHeader> * m,
                          bool key, ::common::TransactionHeader * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_common_TransactionHeader_iter(
      const google::protobuf::Map<bool, ::common::TransactionHeader>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_common_TransactionHeader_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::common::TransactionHeader** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::common::TransactionHeader>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::common::TransactionHeader>* __rust_proto_thunk__Map_ProtoStr_common_TransactionHeader_new() {
    return new google::protobuf::Map<std::string, ::common::TransactionHeader>();
  }
  void __rust_proto_thunk__Map_ProtoStr_common_TransactionHeader_free(const google::protobuf::Map<std::string, ::common::TransactionHeader>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_common_TransactionHeader_clear(google::protobuf::Map<std::string, ::common::TransactionHeader> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_common_TransactionHeader_size(const google::protobuf::Map<std::string, ::common::TransactionHeader>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_TransactionHeader_insert(google::protobuf::Map<std::string, ::common::TransactionHeader> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::TransactionHeader value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_TransactionHeader_get(const google::protobuf::Map<std::string, ::common::TransactionHeader>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::common::TransactionHeader** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::common::TransactionHeader* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_TransactionHeader_remove(google::protobuf::Map<std::string, ::common::TransactionHeader> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::TransactionHeader * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_common_TransactionHeader_iter(
      const google::protobuf::Map<std::string, ::common::TransactionHeader>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_common_TransactionHeader_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::common::TransactionHeader** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::common::TransactionHeader>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// common.BatchedPromissory
        // clang-format off
extern "C" {
void* __rust_proto_thunk__common_BatchedPromissory_new() { return new ::common::BatchedPromissory(); }
void __rust_proto_thunk__common_BatchedPromissory_delete(void* ptr) { delete static_cast<::common::BatchedPromissory*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__common_BatchedPromissory_serialize(::common::BatchedPromissory* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__common_BatchedPromissory_parse(::common::BatchedPromissory* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__common_BatchedPromissory_copy_from(::common::BatchedPromissory* dst, const ::common::BatchedPromissory* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__common_BatchedPromissory_repeated_len(google::protobuf::RepeatedPtrField<::common::BatchedPromissory>* field) {
  return field->size();
}
const ::common::BatchedPromissory& __rust_proto_thunk__common_BatchedPromissory_repeated_get(
  google::protobuf::RepeatedPtrField<::common::BatchedPromissory>* field,
  size_t index) {
  return field->Get(index);
}
::common::BatchedPromissory* __rust_proto_thunk__common_BatchedPromissory_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::common::BatchedPromissory>* field,
  size_t index) {
  return field->Mutable(index);
}
::common::BatchedPromissory* __rust_proto_thunk__common_BatchedPromissory_repeated_add(google::protobuf::RepeatedPtrField<::common::BatchedPromissory>* field) {
  return field->Add();
}
void __rust_proto_thunk__common_BatchedPromissory_repeated_clear(google::protobuf::RepeatedPtrField<::common::BatchedPromissory>* field) {
  field->Clear();
}
void __rust_proto_thunk__common_BatchedPromissory_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::common::BatchedPromissory>& dst,
  const google::protobuf::RepeatedPtrField<::common::BatchedPromissory>& src) {
  dst = src;
}

void __rust_proto_thunk__common_BatchedPromissory_clear_promissory_batch(::common::BatchedPromissory* msg) {
  msg->clear_promissory_batch();
}
google::protobuf::RepeatedPtrField<std::string>* __rust_proto_thunk__common_BatchedPromissory_get_mut_promissory_batch(::common::BatchedPromissory* msg) {
  return msg->mutable_promissory_batch();
}
const google::protobuf::RepeatedPtrField<std::string>* __rust_proto_thunk__common_BatchedPromissory_get_promissory_batch(
    const ::common::BatchedPromissory* msg) {
  return &msg->promissory_batch();
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::common::BatchedPromissory>* __rust_proto_thunk__Map_i32_common_BatchedPromissory_new() {
    return new google::protobuf::Map<int32_t, ::common::BatchedPromissory>();
  }
  void __rust_proto_thunk__Map_i32_common_BatchedPromissory_free(const google::protobuf::Map<int32_t, ::common::BatchedPromissory>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_common_BatchedPromissory_clear(google::protobuf::Map<int32_t, ::common::BatchedPromissory> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_common_BatchedPromissory_size(const google::protobuf::Map<int32_t, ::common::BatchedPromissory>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_common_BatchedPromissory_insert(google::protobuf::Map<int32_t, ::common::BatchedPromissory> * m,
                          int32_t key, ::common::BatchedPromissory value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_BatchedPromissory_get(const google::protobuf::Map<int32_t, ::common::BatchedPromissory>* m,
                       int32_t key, const ::common::BatchedPromissory** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::BatchedPromissory* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_BatchedPromissory_remove(google::protobuf::Map<int32_t, ::common::BatchedPromissory> * m,
                          int32_t key, ::common::BatchedPromissory * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_common_BatchedPromissory_iter(
      const google::protobuf::Map<int32_t, ::common::BatchedPromissory>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_common_BatchedPromissory_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::common::BatchedPromissory** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::common::BatchedPromissory>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::common::BatchedPromissory>* __rust_proto_thunk__Map_u32_common_BatchedPromissory_new() {
    return new google::protobuf::Map<uint32_t, ::common::BatchedPromissory>();
  }
  void __rust_proto_thunk__Map_u32_common_BatchedPromissory_free(const google::protobuf::Map<uint32_t, ::common::BatchedPromissory>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_common_BatchedPromissory_clear(google::protobuf::Map<uint32_t, ::common::BatchedPromissory> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_common_BatchedPromissory_size(const google::protobuf::Map<uint32_t, ::common::BatchedPromissory>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_common_BatchedPromissory_insert(google::protobuf::Map<uint32_t, ::common::BatchedPromissory> * m,
                          uint32_t key, ::common::BatchedPromissory value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_BatchedPromissory_get(const google::protobuf::Map<uint32_t, ::common::BatchedPromissory>* m,
                       uint32_t key, const ::common::BatchedPromissory** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::BatchedPromissory* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_BatchedPromissory_remove(google::protobuf::Map<uint32_t, ::common::BatchedPromissory> * m,
                          uint32_t key, ::common::BatchedPromissory * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_common_BatchedPromissory_iter(
      const google::protobuf::Map<uint32_t, ::common::BatchedPromissory>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_common_BatchedPromissory_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::common::BatchedPromissory** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::common::BatchedPromissory>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::common::BatchedPromissory>* __rust_proto_thunk__Map_i64_common_BatchedPromissory_new() {
    return new google::protobuf::Map<int64_t, ::common::BatchedPromissory>();
  }
  void __rust_proto_thunk__Map_i64_common_BatchedPromissory_free(const google::protobuf::Map<int64_t, ::common::BatchedPromissory>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_common_BatchedPromissory_clear(google::protobuf::Map<int64_t, ::common::BatchedPromissory> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_common_BatchedPromissory_size(const google::protobuf::Map<int64_t, ::common::BatchedPromissory>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_common_BatchedPromissory_insert(google::protobuf::Map<int64_t, ::common::BatchedPromissory> * m,
                          int64_t key, ::common::BatchedPromissory value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_BatchedPromissory_get(const google::protobuf::Map<int64_t, ::common::BatchedPromissory>* m,
                       int64_t key, const ::common::BatchedPromissory** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::BatchedPromissory* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_BatchedPromissory_remove(google::protobuf::Map<int64_t, ::common::BatchedPromissory> * m,
                          int64_t key, ::common::BatchedPromissory * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_common_BatchedPromissory_iter(
      const google::protobuf::Map<int64_t, ::common::BatchedPromissory>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_common_BatchedPromissory_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::common::BatchedPromissory** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::common::BatchedPromissory>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::common::BatchedPromissory>* __rust_proto_thunk__Map_u64_common_BatchedPromissory_new() {
    return new google::protobuf::Map<uint64_t, ::common::BatchedPromissory>();
  }
  void __rust_proto_thunk__Map_u64_common_BatchedPromissory_free(const google::protobuf::Map<uint64_t, ::common::BatchedPromissory>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_common_BatchedPromissory_clear(google::protobuf::Map<uint64_t, ::common::BatchedPromissory> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_common_BatchedPromissory_size(const google::protobuf::Map<uint64_t, ::common::BatchedPromissory>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_common_BatchedPromissory_insert(google::protobuf::Map<uint64_t, ::common::BatchedPromissory> * m,
                          uint64_t key, ::common::BatchedPromissory value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_BatchedPromissory_get(const google::protobuf::Map<uint64_t, ::common::BatchedPromissory>* m,
                       uint64_t key, const ::common::BatchedPromissory** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::BatchedPromissory* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_BatchedPromissory_remove(google::protobuf::Map<uint64_t, ::common::BatchedPromissory> * m,
                          uint64_t key, ::common::BatchedPromissory * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_common_BatchedPromissory_iter(
      const google::protobuf::Map<uint64_t, ::common::BatchedPromissory>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_common_BatchedPromissory_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::common::BatchedPromissory** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::common::BatchedPromissory>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::common::BatchedPromissory>* __rust_proto_thunk__Map_bool_common_BatchedPromissory_new() {
    return new google::protobuf::Map<bool, ::common::BatchedPromissory>();
  }
  void __rust_proto_thunk__Map_bool_common_BatchedPromissory_free(const google::protobuf::Map<bool, ::common::BatchedPromissory>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_common_BatchedPromissory_clear(google::protobuf::Map<bool, ::common::BatchedPromissory> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_common_BatchedPromissory_size(const google::protobuf::Map<bool, ::common::BatchedPromissory>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_common_BatchedPromissory_insert(google::protobuf::Map<bool, ::common::BatchedPromissory> * m,
                          bool key, ::common::BatchedPromissory value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_BatchedPromissory_get(const google::protobuf::Map<bool, ::common::BatchedPromissory>* m,
                       bool key, const ::common::BatchedPromissory** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::BatchedPromissory* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_BatchedPromissory_remove(google::protobuf::Map<bool, ::common::BatchedPromissory> * m,
                          bool key, ::common::BatchedPromissory * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_common_BatchedPromissory_iter(
      const google::protobuf::Map<bool, ::common::BatchedPromissory>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_common_BatchedPromissory_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::common::BatchedPromissory** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::common::BatchedPromissory>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::common::BatchedPromissory>* __rust_proto_thunk__Map_ProtoStr_common_BatchedPromissory_new() {
    return new google::protobuf::Map<std::string, ::common::BatchedPromissory>();
  }
  void __rust_proto_thunk__Map_ProtoStr_common_BatchedPromissory_free(const google::protobuf::Map<std::string, ::common::BatchedPromissory>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_common_BatchedPromissory_clear(google::protobuf::Map<std::string, ::common::BatchedPromissory> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_common_BatchedPromissory_size(const google::protobuf::Map<std::string, ::common::BatchedPromissory>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_BatchedPromissory_insert(google::protobuf::Map<std::string, ::common::BatchedPromissory> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::BatchedPromissory value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_BatchedPromissory_get(const google::protobuf::Map<std::string, ::common::BatchedPromissory>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::common::BatchedPromissory** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::common::BatchedPromissory* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_BatchedPromissory_remove(google::protobuf::Map<std::string, ::common::BatchedPromissory> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::BatchedPromissory * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_common_BatchedPromissory_iter(
      const google::protobuf::Map<std::string, ::common::BatchedPromissory>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_common_BatchedPromissory_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::common::BatchedPromissory** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::common::BatchedPromissory>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// common.SimplePaymentRequest
        // clang-format off
extern "C" {
void* __rust_proto_thunk__common_SimplePaymentRequest_new() { return new ::common::SimplePaymentRequest(); }
void __rust_proto_thunk__common_SimplePaymentRequest_delete(void* ptr) { delete static_cast<::common::SimplePaymentRequest*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__common_SimplePaymentRequest_serialize(::common::SimplePaymentRequest* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__common_SimplePaymentRequest_parse(::common::SimplePaymentRequest* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__common_SimplePaymentRequest_copy_from(::common::SimplePaymentRequest* dst, const ::common::SimplePaymentRequest* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__common_SimplePaymentRequest_repeated_len(google::protobuf::RepeatedPtrField<::common::SimplePaymentRequest>* field) {
  return field->size();
}
const ::common::SimplePaymentRequest& __rust_proto_thunk__common_SimplePaymentRequest_repeated_get(
  google::protobuf::RepeatedPtrField<::common::SimplePaymentRequest>* field,
  size_t index) {
  return field->Get(index);
}
::common::SimplePaymentRequest* __rust_proto_thunk__common_SimplePaymentRequest_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::common::SimplePaymentRequest>* field,
  size_t index) {
  return field->Mutable(index);
}
::common::SimplePaymentRequest* __rust_proto_thunk__common_SimplePaymentRequest_repeated_add(google::protobuf::RepeatedPtrField<::common::SimplePaymentRequest>* field) {
  return field->Add();
}
void __rust_proto_thunk__common_SimplePaymentRequest_repeated_clear(google::protobuf::RepeatedPtrField<::common::SimplePaymentRequest>* field) {
  field->Clear();
}
void __rust_proto_thunk__common_SimplePaymentRequest_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::common::SimplePaymentRequest>& dst,
  const google::protobuf::RepeatedPtrField<::common::SimplePaymentRequest>& src) {
  dst = src;
}

const void* __rust_proto_thunk__common_SimplePaymentRequest_get_amount(::common::SimplePaymentRequest* msg) {
  return static_cast<const void*>(&msg->amount());
}
void* __rust_proto_thunk__common_SimplePaymentRequest_get_mut_amount(::common::SimplePaymentRequest* msg) {
  return static_cast<void*>(msg->mutable_amount());
}
void __rust_proto_thunk__common_SimplePaymentRequest_clear_amount(::common::SimplePaymentRequest* msg) { msg->clear_amount(); }
bool __rust_proto_thunk__common_SimplePaymentRequest_has_amount(::common::SimplePaymentRequest* msg) { return msg->has_amount(); }


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::common::SimplePaymentRequest>* __rust_proto_thunk__Map_i32_common_SimplePaymentRequest_new() {
    return new google::protobuf::Map<int32_t, ::common::SimplePaymentRequest>();
  }
  void __rust_proto_thunk__Map_i32_common_SimplePaymentRequest_free(const google::protobuf::Map<int32_t, ::common::SimplePaymentRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_common_SimplePaymentRequest_clear(google::protobuf::Map<int32_t, ::common::SimplePaymentRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_common_SimplePaymentRequest_size(const google::protobuf::Map<int32_t, ::common::SimplePaymentRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_common_SimplePaymentRequest_insert(google::protobuf::Map<int32_t, ::common::SimplePaymentRequest> * m,
                          int32_t key, ::common::SimplePaymentRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_SimplePaymentRequest_get(const google::protobuf::Map<int32_t, ::common::SimplePaymentRequest>* m,
                       int32_t key, const ::common::SimplePaymentRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::SimplePaymentRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_SimplePaymentRequest_remove(google::protobuf::Map<int32_t, ::common::SimplePaymentRequest> * m,
                          int32_t key, ::common::SimplePaymentRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_common_SimplePaymentRequest_iter(
      const google::protobuf::Map<int32_t, ::common::SimplePaymentRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_common_SimplePaymentRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::common::SimplePaymentRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::common::SimplePaymentRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::common::SimplePaymentRequest>* __rust_proto_thunk__Map_u32_common_SimplePaymentRequest_new() {
    return new google::protobuf::Map<uint32_t, ::common::SimplePaymentRequest>();
  }
  void __rust_proto_thunk__Map_u32_common_SimplePaymentRequest_free(const google::protobuf::Map<uint32_t, ::common::SimplePaymentRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_common_SimplePaymentRequest_clear(google::protobuf::Map<uint32_t, ::common::SimplePaymentRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_common_SimplePaymentRequest_size(const google::protobuf::Map<uint32_t, ::common::SimplePaymentRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_common_SimplePaymentRequest_insert(google::protobuf::Map<uint32_t, ::common::SimplePaymentRequest> * m,
                          uint32_t key, ::common::SimplePaymentRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_SimplePaymentRequest_get(const google::protobuf::Map<uint32_t, ::common::SimplePaymentRequest>* m,
                       uint32_t key, const ::common::SimplePaymentRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::SimplePaymentRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_SimplePaymentRequest_remove(google::protobuf::Map<uint32_t, ::common::SimplePaymentRequest> * m,
                          uint32_t key, ::common::SimplePaymentRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_common_SimplePaymentRequest_iter(
      const google::protobuf::Map<uint32_t, ::common::SimplePaymentRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_common_SimplePaymentRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::common::SimplePaymentRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::common::SimplePaymentRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::common::SimplePaymentRequest>* __rust_proto_thunk__Map_i64_common_SimplePaymentRequest_new() {
    return new google::protobuf::Map<int64_t, ::common::SimplePaymentRequest>();
  }
  void __rust_proto_thunk__Map_i64_common_SimplePaymentRequest_free(const google::protobuf::Map<int64_t, ::common::SimplePaymentRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_common_SimplePaymentRequest_clear(google::protobuf::Map<int64_t, ::common::SimplePaymentRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_common_SimplePaymentRequest_size(const google::protobuf::Map<int64_t, ::common::SimplePaymentRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_common_SimplePaymentRequest_insert(google::protobuf::Map<int64_t, ::common::SimplePaymentRequest> * m,
                          int64_t key, ::common::SimplePaymentRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_SimplePaymentRequest_get(const google::protobuf::Map<int64_t, ::common::SimplePaymentRequest>* m,
                       int64_t key, const ::common::SimplePaymentRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::SimplePaymentRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_SimplePaymentRequest_remove(google::protobuf::Map<int64_t, ::common::SimplePaymentRequest> * m,
                          int64_t key, ::common::SimplePaymentRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_common_SimplePaymentRequest_iter(
      const google::protobuf::Map<int64_t, ::common::SimplePaymentRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_common_SimplePaymentRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::common::SimplePaymentRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::common::SimplePaymentRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::common::SimplePaymentRequest>* __rust_proto_thunk__Map_u64_common_SimplePaymentRequest_new() {
    return new google::protobuf::Map<uint64_t, ::common::SimplePaymentRequest>();
  }
  void __rust_proto_thunk__Map_u64_common_SimplePaymentRequest_free(const google::protobuf::Map<uint64_t, ::common::SimplePaymentRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_common_SimplePaymentRequest_clear(google::protobuf::Map<uint64_t, ::common::SimplePaymentRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_common_SimplePaymentRequest_size(const google::protobuf::Map<uint64_t, ::common::SimplePaymentRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_common_SimplePaymentRequest_insert(google::protobuf::Map<uint64_t, ::common::SimplePaymentRequest> * m,
                          uint64_t key, ::common::SimplePaymentRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_SimplePaymentRequest_get(const google::protobuf::Map<uint64_t, ::common::SimplePaymentRequest>* m,
                       uint64_t key, const ::common::SimplePaymentRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::SimplePaymentRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_SimplePaymentRequest_remove(google::protobuf::Map<uint64_t, ::common::SimplePaymentRequest> * m,
                          uint64_t key, ::common::SimplePaymentRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_common_SimplePaymentRequest_iter(
      const google::protobuf::Map<uint64_t, ::common::SimplePaymentRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_common_SimplePaymentRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::common::SimplePaymentRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::common::SimplePaymentRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::common::SimplePaymentRequest>* __rust_proto_thunk__Map_bool_common_SimplePaymentRequest_new() {
    return new google::protobuf::Map<bool, ::common::SimplePaymentRequest>();
  }
  void __rust_proto_thunk__Map_bool_common_SimplePaymentRequest_free(const google::protobuf::Map<bool, ::common::SimplePaymentRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_common_SimplePaymentRequest_clear(google::protobuf::Map<bool, ::common::SimplePaymentRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_common_SimplePaymentRequest_size(const google::protobuf::Map<bool, ::common::SimplePaymentRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_common_SimplePaymentRequest_insert(google::protobuf::Map<bool, ::common::SimplePaymentRequest> * m,
                          bool key, ::common::SimplePaymentRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_SimplePaymentRequest_get(const google::protobuf::Map<bool, ::common::SimplePaymentRequest>* m,
                       bool key, const ::common::SimplePaymentRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::SimplePaymentRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_SimplePaymentRequest_remove(google::protobuf::Map<bool, ::common::SimplePaymentRequest> * m,
                          bool key, ::common::SimplePaymentRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_common_SimplePaymentRequest_iter(
      const google::protobuf::Map<bool, ::common::SimplePaymentRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_common_SimplePaymentRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::common::SimplePaymentRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::common::SimplePaymentRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::common::SimplePaymentRequest>* __rust_proto_thunk__Map_ProtoStr_common_SimplePaymentRequest_new() {
    return new google::protobuf::Map<std::string, ::common::SimplePaymentRequest>();
  }
  void __rust_proto_thunk__Map_ProtoStr_common_SimplePaymentRequest_free(const google::protobuf::Map<std::string, ::common::SimplePaymentRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_common_SimplePaymentRequest_clear(google::protobuf::Map<std::string, ::common::SimplePaymentRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_common_SimplePaymentRequest_size(const google::protobuf::Map<std::string, ::common::SimplePaymentRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_SimplePaymentRequest_insert(google::protobuf::Map<std::string, ::common::SimplePaymentRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::SimplePaymentRequest value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_SimplePaymentRequest_get(const google::protobuf::Map<std::string, ::common::SimplePaymentRequest>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::common::SimplePaymentRequest** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::common::SimplePaymentRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_SimplePaymentRequest_remove(google::protobuf::Map<std::string, ::common::SimplePaymentRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::SimplePaymentRequest * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_common_SimplePaymentRequest_iter(
      const google::protobuf::Map<std::string, ::common::SimplePaymentRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_common_SimplePaymentRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::common::SimplePaymentRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::common::SimplePaymentRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// common.SimplePaymentAck
        // clang-format off
extern "C" {
void* __rust_proto_thunk__common_SimplePaymentAck_new() { return new ::common::SimplePaymentAck(); }
void __rust_proto_thunk__common_SimplePaymentAck_delete(void* ptr) { delete static_cast<::common::SimplePaymentAck*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__common_SimplePaymentAck_serialize(::common::SimplePaymentAck* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__common_SimplePaymentAck_parse(::common::SimplePaymentAck* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__common_SimplePaymentAck_copy_from(::common::SimplePaymentAck* dst, const ::common::SimplePaymentAck* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__common_SimplePaymentAck_repeated_len(google::protobuf::RepeatedPtrField<::common::SimplePaymentAck>* field) {
  return field->size();
}
const ::common::SimplePaymentAck& __rust_proto_thunk__common_SimplePaymentAck_repeated_get(
  google::protobuf::RepeatedPtrField<::common::SimplePaymentAck>* field,
  size_t index) {
  return field->Get(index);
}
::common::SimplePaymentAck* __rust_proto_thunk__common_SimplePaymentAck_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::common::SimplePaymentAck>* field,
  size_t index) {
  return field->Mutable(index);
}
::common::SimplePaymentAck* __rust_proto_thunk__common_SimplePaymentAck_repeated_add(google::protobuf::RepeatedPtrField<::common::SimplePaymentAck>* field) {
  return field->Add();
}
void __rust_proto_thunk__common_SimplePaymentAck_repeated_clear(google::protobuf::RepeatedPtrField<::common::SimplePaymentAck>* field) {
  field->Clear();
}
void __rust_proto_thunk__common_SimplePaymentAck_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::common::SimplePaymentAck>& dst,
  const google::protobuf::RepeatedPtrField<::common::SimplePaymentAck>& src) {
  dst = src;
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::common::SimplePaymentAck>* __rust_proto_thunk__Map_i32_common_SimplePaymentAck_new() {
    return new google::protobuf::Map<int32_t, ::common::SimplePaymentAck>();
  }
  void __rust_proto_thunk__Map_i32_common_SimplePaymentAck_free(const google::protobuf::Map<int32_t, ::common::SimplePaymentAck>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_common_SimplePaymentAck_clear(google::protobuf::Map<int32_t, ::common::SimplePaymentAck> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_common_SimplePaymentAck_size(const google::protobuf::Map<int32_t, ::common::SimplePaymentAck>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_common_SimplePaymentAck_insert(google::protobuf::Map<int32_t, ::common::SimplePaymentAck> * m,
                          int32_t key, ::common::SimplePaymentAck value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_SimplePaymentAck_get(const google::protobuf::Map<int32_t, ::common::SimplePaymentAck>* m,
                       int32_t key, const ::common::SimplePaymentAck** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::SimplePaymentAck* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_SimplePaymentAck_remove(google::protobuf::Map<int32_t, ::common::SimplePaymentAck> * m,
                          int32_t key, ::common::SimplePaymentAck * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_common_SimplePaymentAck_iter(
      const google::protobuf::Map<int32_t, ::common::SimplePaymentAck>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_common_SimplePaymentAck_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::common::SimplePaymentAck** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::common::SimplePaymentAck>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::common::SimplePaymentAck>* __rust_proto_thunk__Map_u32_common_SimplePaymentAck_new() {
    return new google::protobuf::Map<uint32_t, ::common::SimplePaymentAck>();
  }
  void __rust_proto_thunk__Map_u32_common_SimplePaymentAck_free(const google::protobuf::Map<uint32_t, ::common::SimplePaymentAck>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_common_SimplePaymentAck_clear(google::protobuf::Map<uint32_t, ::common::SimplePaymentAck> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_common_SimplePaymentAck_size(const google::protobuf::Map<uint32_t, ::common::SimplePaymentAck>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_common_SimplePaymentAck_insert(google::protobuf::Map<uint32_t, ::common::SimplePaymentAck> * m,
                          uint32_t key, ::common::SimplePaymentAck value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_SimplePaymentAck_get(const google::protobuf::Map<uint32_t, ::common::SimplePaymentAck>* m,
                       uint32_t key, const ::common::SimplePaymentAck** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::SimplePaymentAck* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_SimplePaymentAck_remove(google::protobuf::Map<uint32_t, ::common::SimplePaymentAck> * m,
                          uint32_t key, ::common::SimplePaymentAck * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_common_SimplePaymentAck_iter(
      const google::protobuf::Map<uint32_t, ::common::SimplePaymentAck>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_common_SimplePaymentAck_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::common::SimplePaymentAck** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::common::SimplePaymentAck>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::common::SimplePaymentAck>* __rust_proto_thunk__Map_i64_common_SimplePaymentAck_new() {
    return new google::protobuf::Map<int64_t, ::common::SimplePaymentAck>();
  }
  void __rust_proto_thunk__Map_i64_common_SimplePaymentAck_free(const google::protobuf::Map<int64_t, ::common::SimplePaymentAck>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_common_SimplePaymentAck_clear(google::protobuf::Map<int64_t, ::common::SimplePaymentAck> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_common_SimplePaymentAck_size(const google::protobuf::Map<int64_t, ::common::SimplePaymentAck>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_common_SimplePaymentAck_insert(google::protobuf::Map<int64_t, ::common::SimplePaymentAck> * m,
                          int64_t key, ::common::SimplePaymentAck value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_SimplePaymentAck_get(const google::protobuf::Map<int64_t, ::common::SimplePaymentAck>* m,
                       int64_t key, const ::common::SimplePaymentAck** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::SimplePaymentAck* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_SimplePaymentAck_remove(google::protobuf::Map<int64_t, ::common::SimplePaymentAck> * m,
                          int64_t key, ::common::SimplePaymentAck * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_common_SimplePaymentAck_iter(
      const google::protobuf::Map<int64_t, ::common::SimplePaymentAck>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_common_SimplePaymentAck_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::common::SimplePaymentAck** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::common::SimplePaymentAck>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::common::SimplePaymentAck>* __rust_proto_thunk__Map_u64_common_SimplePaymentAck_new() {
    return new google::protobuf::Map<uint64_t, ::common::SimplePaymentAck>();
  }
  void __rust_proto_thunk__Map_u64_common_SimplePaymentAck_free(const google::protobuf::Map<uint64_t, ::common::SimplePaymentAck>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_common_SimplePaymentAck_clear(google::protobuf::Map<uint64_t, ::common::SimplePaymentAck> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_common_SimplePaymentAck_size(const google::protobuf::Map<uint64_t, ::common::SimplePaymentAck>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_common_SimplePaymentAck_insert(google::protobuf::Map<uint64_t, ::common::SimplePaymentAck> * m,
                          uint64_t key, ::common::SimplePaymentAck value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_SimplePaymentAck_get(const google::protobuf::Map<uint64_t, ::common::SimplePaymentAck>* m,
                       uint64_t key, const ::common::SimplePaymentAck** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::SimplePaymentAck* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_SimplePaymentAck_remove(google::protobuf::Map<uint64_t, ::common::SimplePaymentAck> * m,
                          uint64_t key, ::common::SimplePaymentAck * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_common_SimplePaymentAck_iter(
      const google::protobuf::Map<uint64_t, ::common::SimplePaymentAck>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_common_SimplePaymentAck_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::common::SimplePaymentAck** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::common::SimplePaymentAck>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::common::SimplePaymentAck>* __rust_proto_thunk__Map_bool_common_SimplePaymentAck_new() {
    return new google::protobuf::Map<bool, ::common::SimplePaymentAck>();
  }
  void __rust_proto_thunk__Map_bool_common_SimplePaymentAck_free(const google::protobuf::Map<bool, ::common::SimplePaymentAck>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_common_SimplePaymentAck_clear(google::protobuf::Map<bool, ::common::SimplePaymentAck> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_common_SimplePaymentAck_size(const google::protobuf::Map<bool, ::common::SimplePaymentAck>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_common_SimplePaymentAck_insert(google::protobuf::Map<bool, ::common::SimplePaymentAck> * m,
                          bool key, ::common::SimplePaymentAck value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_SimplePaymentAck_get(const google::protobuf::Map<bool, ::common::SimplePaymentAck>* m,
                       bool key, const ::common::SimplePaymentAck** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::SimplePaymentAck* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_SimplePaymentAck_remove(google::protobuf::Map<bool, ::common::SimplePaymentAck> * m,
                          bool key, ::common::SimplePaymentAck * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_common_SimplePaymentAck_iter(
      const google::protobuf::Map<bool, ::common::SimplePaymentAck>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_common_SimplePaymentAck_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::common::SimplePaymentAck** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::common::SimplePaymentAck>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::common::SimplePaymentAck>* __rust_proto_thunk__Map_ProtoStr_common_SimplePaymentAck_new() {
    return new google::protobuf::Map<std::string, ::common::SimplePaymentAck>();
  }
  void __rust_proto_thunk__Map_ProtoStr_common_SimplePaymentAck_free(const google::protobuf::Map<std::string, ::common::SimplePaymentAck>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_common_SimplePaymentAck_clear(google::protobuf::Map<std::string, ::common::SimplePaymentAck> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_common_SimplePaymentAck_size(const google::protobuf::Map<std::string, ::common::SimplePaymentAck>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_SimplePaymentAck_insert(google::protobuf::Map<std::string, ::common::SimplePaymentAck> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::SimplePaymentAck value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_SimplePaymentAck_get(const google::protobuf::Map<std::string, ::common::SimplePaymentAck>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::common::SimplePaymentAck** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::common::SimplePaymentAck* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_SimplePaymentAck_remove(google::protobuf::Map<std::string, ::common::SimplePaymentAck> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::SimplePaymentAck * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_common_SimplePaymentAck_iter(
      const google::protobuf::Map<std::string, ::common::SimplePaymentAck>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_common_SimplePaymentAck_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::common::SimplePaymentAck** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::common::SimplePaymentAck>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// common.SimplePaymentNack
        // clang-format off
extern "C" {
void* __rust_proto_thunk__common_SimplePaymentNack_new() { return new ::common::SimplePaymentNack(); }
void __rust_proto_thunk__common_SimplePaymentNack_delete(void* ptr) { delete static_cast<::common::SimplePaymentNack*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__common_SimplePaymentNack_serialize(::common::SimplePaymentNack* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__common_SimplePaymentNack_parse(::common::SimplePaymentNack* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__common_SimplePaymentNack_copy_from(::common::SimplePaymentNack* dst, const ::common::SimplePaymentNack* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__common_SimplePaymentNack_repeated_len(google::protobuf::RepeatedPtrField<::common::SimplePaymentNack>* field) {
  return field->size();
}
const ::common::SimplePaymentNack& __rust_proto_thunk__common_SimplePaymentNack_repeated_get(
  google::protobuf::RepeatedPtrField<::common::SimplePaymentNack>* field,
  size_t index) {
  return field->Get(index);
}
::common::SimplePaymentNack* __rust_proto_thunk__common_SimplePaymentNack_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::common::SimplePaymentNack>* field,
  size_t index) {
  return field->Mutable(index);
}
::common::SimplePaymentNack* __rust_proto_thunk__common_SimplePaymentNack_repeated_add(google::protobuf::RepeatedPtrField<::common::SimplePaymentNack>* field) {
  return field->Add();
}
void __rust_proto_thunk__common_SimplePaymentNack_repeated_clear(google::protobuf::RepeatedPtrField<::common::SimplePaymentNack>* field) {
  field->Clear();
}
void __rust_proto_thunk__common_SimplePaymentNack_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::common::SimplePaymentNack>& dst,
  const google::protobuf::RepeatedPtrField<::common::SimplePaymentNack>& src) {
  dst = src;
}

::common::SimplePaymentNack_RejectionReason __rust_proto_thunk__common_SimplePaymentNack_get_rejection_reason(::common::SimplePaymentNack* msg) { return msg->rejection_reason(); }
void __rust_proto_thunk__common_SimplePaymentNack_set_rejection_reason(::common::SimplePaymentNack* msg, ::common::SimplePaymentNack_RejectionReason val) {
  msg->set_rejection_reason(val);
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__common_SimplePaymentNack_get_other(::common::SimplePaymentNack* msg) {
  absl::string_view val = msg->other();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__common_SimplePaymentNack_set_other(::common::SimplePaymentNack* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_other(absl::string_view(s.ptr, s.len));
}


}  // extern "C"
// clang-format on

extern "C" {
  __PB_RUST_EXPOSE_SCALAR_MAP_METHODS_FOR_VALUE_TYPE(
      ::common::SimplePaymentNack_RejectionReason, common_SimplePaymentNack_RejectionReason, ::common::SimplePaymentNack_RejectionReason, value, cpp_value)
}
extern "C" {
  const google::protobuf::Map<int32_t, ::common::SimplePaymentNack>* __rust_proto_thunk__Map_i32_common_SimplePaymentNack_new() {
    return new google::protobuf::Map<int32_t, ::common::SimplePaymentNack>();
  }
  void __rust_proto_thunk__Map_i32_common_SimplePaymentNack_free(const google::protobuf::Map<int32_t, ::common::SimplePaymentNack>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_common_SimplePaymentNack_clear(google::protobuf::Map<int32_t, ::common::SimplePaymentNack> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_common_SimplePaymentNack_size(const google::protobuf::Map<int32_t, ::common::SimplePaymentNack>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_common_SimplePaymentNack_insert(google::protobuf::Map<int32_t, ::common::SimplePaymentNack> * m,
                          int32_t key, ::common::SimplePaymentNack value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_SimplePaymentNack_get(const google::protobuf::Map<int32_t, ::common::SimplePaymentNack>* m,
                       int32_t key, const ::common::SimplePaymentNack** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::SimplePaymentNack* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_SimplePaymentNack_remove(google::protobuf::Map<int32_t, ::common::SimplePaymentNack> * m,
                          int32_t key, ::common::SimplePaymentNack * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_common_SimplePaymentNack_iter(
      const google::protobuf::Map<int32_t, ::common::SimplePaymentNack>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_common_SimplePaymentNack_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::common::SimplePaymentNack** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::common::SimplePaymentNack>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::common::SimplePaymentNack>* __rust_proto_thunk__Map_u32_common_SimplePaymentNack_new() {
    return new google::protobuf::Map<uint32_t, ::common::SimplePaymentNack>();
  }
  void __rust_proto_thunk__Map_u32_common_SimplePaymentNack_free(const google::protobuf::Map<uint32_t, ::common::SimplePaymentNack>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_common_SimplePaymentNack_clear(google::protobuf::Map<uint32_t, ::common::SimplePaymentNack> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_common_SimplePaymentNack_size(const google::protobuf::Map<uint32_t, ::common::SimplePaymentNack>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_common_SimplePaymentNack_insert(google::protobuf::Map<uint32_t, ::common::SimplePaymentNack> * m,
                          uint32_t key, ::common::SimplePaymentNack value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_SimplePaymentNack_get(const google::protobuf::Map<uint32_t, ::common::SimplePaymentNack>* m,
                       uint32_t key, const ::common::SimplePaymentNack** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::SimplePaymentNack* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_SimplePaymentNack_remove(google::protobuf::Map<uint32_t, ::common::SimplePaymentNack> * m,
                          uint32_t key, ::common::SimplePaymentNack * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_common_SimplePaymentNack_iter(
      const google::protobuf::Map<uint32_t, ::common::SimplePaymentNack>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_common_SimplePaymentNack_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::common::SimplePaymentNack** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::common::SimplePaymentNack>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::common::SimplePaymentNack>* __rust_proto_thunk__Map_i64_common_SimplePaymentNack_new() {
    return new google::protobuf::Map<int64_t, ::common::SimplePaymentNack>();
  }
  void __rust_proto_thunk__Map_i64_common_SimplePaymentNack_free(const google::protobuf::Map<int64_t, ::common::SimplePaymentNack>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_common_SimplePaymentNack_clear(google::protobuf::Map<int64_t, ::common::SimplePaymentNack> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_common_SimplePaymentNack_size(const google::protobuf::Map<int64_t, ::common::SimplePaymentNack>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_common_SimplePaymentNack_insert(google::protobuf::Map<int64_t, ::common::SimplePaymentNack> * m,
                          int64_t key, ::common::SimplePaymentNack value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_SimplePaymentNack_get(const google::protobuf::Map<int64_t, ::common::SimplePaymentNack>* m,
                       int64_t key, const ::common::SimplePaymentNack** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::SimplePaymentNack* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_SimplePaymentNack_remove(google::protobuf::Map<int64_t, ::common::SimplePaymentNack> * m,
                          int64_t key, ::common::SimplePaymentNack * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_common_SimplePaymentNack_iter(
      const google::protobuf::Map<int64_t, ::common::SimplePaymentNack>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_common_SimplePaymentNack_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::common::SimplePaymentNack** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::common::SimplePaymentNack>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::common::SimplePaymentNack>* __rust_proto_thunk__Map_u64_common_SimplePaymentNack_new() {
    return new google::protobuf::Map<uint64_t, ::common::SimplePaymentNack>();
  }
  void __rust_proto_thunk__Map_u64_common_SimplePaymentNack_free(const google::protobuf::Map<uint64_t, ::common::SimplePaymentNack>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_common_SimplePaymentNack_clear(google::protobuf::Map<uint64_t, ::common::SimplePaymentNack> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_common_SimplePaymentNack_size(const google::protobuf::Map<uint64_t, ::common::SimplePaymentNack>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_common_SimplePaymentNack_insert(google::protobuf::Map<uint64_t, ::common::SimplePaymentNack> * m,
                          uint64_t key, ::common::SimplePaymentNack value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_SimplePaymentNack_get(const google::protobuf::Map<uint64_t, ::common::SimplePaymentNack>* m,
                       uint64_t key, const ::common::SimplePaymentNack** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::SimplePaymentNack* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_SimplePaymentNack_remove(google::protobuf::Map<uint64_t, ::common::SimplePaymentNack> * m,
                          uint64_t key, ::common::SimplePaymentNack * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_common_SimplePaymentNack_iter(
      const google::protobuf::Map<uint64_t, ::common::SimplePaymentNack>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_common_SimplePaymentNack_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::common::SimplePaymentNack** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::common::SimplePaymentNack>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::common::SimplePaymentNack>* __rust_proto_thunk__Map_bool_common_SimplePaymentNack_new() {
    return new google::protobuf::Map<bool, ::common::SimplePaymentNack>();
  }
  void __rust_proto_thunk__Map_bool_common_SimplePaymentNack_free(const google::protobuf::Map<bool, ::common::SimplePaymentNack>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_common_SimplePaymentNack_clear(google::protobuf::Map<bool, ::common::SimplePaymentNack> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_common_SimplePaymentNack_size(const google::protobuf::Map<bool, ::common::SimplePaymentNack>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_common_SimplePaymentNack_insert(google::protobuf::Map<bool, ::common::SimplePaymentNack> * m,
                          bool key, ::common::SimplePaymentNack value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_SimplePaymentNack_get(const google::protobuf::Map<bool, ::common::SimplePaymentNack>* m,
                       bool key, const ::common::SimplePaymentNack** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::SimplePaymentNack* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_SimplePaymentNack_remove(google::protobuf::Map<bool, ::common::SimplePaymentNack> * m,
                          bool key, ::common::SimplePaymentNack * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_common_SimplePaymentNack_iter(
      const google::protobuf::Map<bool, ::common::SimplePaymentNack>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_common_SimplePaymentNack_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::common::SimplePaymentNack** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::common::SimplePaymentNack>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::common::SimplePaymentNack>* __rust_proto_thunk__Map_ProtoStr_common_SimplePaymentNack_new() {
    return new google::protobuf::Map<std::string, ::common::SimplePaymentNack>();
  }
  void __rust_proto_thunk__Map_ProtoStr_common_SimplePaymentNack_free(const google::protobuf::Map<std::string, ::common::SimplePaymentNack>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_common_SimplePaymentNack_clear(google::protobuf::Map<std::string, ::common::SimplePaymentNack> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_common_SimplePaymentNack_size(const google::protobuf::Map<std::string, ::common::SimplePaymentNack>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_SimplePaymentNack_insert(google::protobuf::Map<std::string, ::common::SimplePaymentNack> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::SimplePaymentNack value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_SimplePaymentNack_get(const google::protobuf::Map<std::string, ::common::SimplePaymentNack>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::common::SimplePaymentNack** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::common::SimplePaymentNack* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_SimplePaymentNack_remove(google::protobuf::Map<std::string, ::common::SimplePaymentNack> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::SimplePaymentNack * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_common_SimplePaymentNack_iter(
      const google::protobuf::Map<std::string, ::common::SimplePaymentNack>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_common_SimplePaymentNack_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::common::SimplePaymentNack** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::common::SimplePaymentNack>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// common.SimplePaymentResponse
        // clang-format off
extern "C" {
void* __rust_proto_thunk__common_SimplePaymentResponse_new() { return new ::common::SimplePaymentResponse(); }
void __rust_proto_thunk__common_SimplePaymentResponse_delete(void* ptr) { delete static_cast<::common::SimplePaymentResponse*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__common_SimplePaymentResponse_serialize(::common::SimplePaymentResponse* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__common_SimplePaymentResponse_parse(::common::SimplePaymentResponse* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__common_SimplePaymentResponse_copy_from(::common::SimplePaymentResponse* dst, const ::common::SimplePaymentResponse* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__common_SimplePaymentResponse_repeated_len(google::protobuf::RepeatedPtrField<::common::SimplePaymentResponse>* field) {
  return field->size();
}
const ::common::SimplePaymentResponse& __rust_proto_thunk__common_SimplePaymentResponse_repeated_get(
  google::protobuf::RepeatedPtrField<::common::SimplePaymentResponse>* field,
  size_t index) {
  return field->Get(index);
}
::common::SimplePaymentResponse* __rust_proto_thunk__common_SimplePaymentResponse_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::common::SimplePaymentResponse>* field,
  size_t index) {
  return field->Mutable(index);
}
::common::SimplePaymentResponse* __rust_proto_thunk__common_SimplePaymentResponse_repeated_add(google::protobuf::RepeatedPtrField<::common::SimplePaymentResponse>* field) {
  return field->Add();
}
void __rust_proto_thunk__common_SimplePaymentResponse_repeated_clear(google::protobuf::RepeatedPtrField<::common::SimplePaymentResponse>* field) {
  field->Clear();
}
void __rust_proto_thunk__common_SimplePaymentResponse_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::common::SimplePaymentResponse>& dst,
  const google::protobuf::RepeatedPtrField<::common::SimplePaymentResponse>& src) {
  dst = src;
}

const void* __rust_proto_thunk__common_SimplePaymentResponse_get_received_amount(::common::SimplePaymentResponse* msg) {
  return static_cast<const void*>(&msg->received_amount());
}
void* __rust_proto_thunk__common_SimplePaymentResponse_get_mut_received_amount(::common::SimplePaymentResponse* msg) {
  return static_cast<void*>(msg->mutable_received_amount());
}
void __rust_proto_thunk__common_SimplePaymentResponse_clear_received_amount(::common::SimplePaymentResponse* msg) { msg->clear_received_amount(); }
bool __rust_proto_thunk__common_SimplePaymentResponse_has_received_amount(::common::SimplePaymentResponse* msg) { return msg->has_received_amount(); }

void __rust_proto_thunk__common_SimplePaymentResponse_clear_received_promissories(::common::SimplePaymentResponse* msg) {
  msg->clear_received_promissories();
}
google::protobuf::RepeatedPtrField<std::string>* __rust_proto_thunk__common_SimplePaymentResponse_get_mut_received_promissories(::common::SimplePaymentResponse* msg) {
  return msg->mutable_received_promissories();
}
const google::protobuf::RepeatedPtrField<std::string>* __rust_proto_thunk__common_SimplePaymentResponse_get_received_promissories(
    const ::common::SimplePaymentResponse* msg) {
  return &msg->received_promissories();
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::common::SimplePaymentResponse>* __rust_proto_thunk__Map_i32_common_SimplePaymentResponse_new() {
    return new google::protobuf::Map<int32_t, ::common::SimplePaymentResponse>();
  }
  void __rust_proto_thunk__Map_i32_common_SimplePaymentResponse_free(const google::protobuf::Map<int32_t, ::common::SimplePaymentResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_common_SimplePaymentResponse_clear(google::protobuf::Map<int32_t, ::common::SimplePaymentResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_common_SimplePaymentResponse_size(const google::protobuf::Map<int32_t, ::common::SimplePaymentResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_common_SimplePaymentResponse_insert(google::protobuf::Map<int32_t, ::common::SimplePaymentResponse> * m,
                          int32_t key, ::common::SimplePaymentResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_SimplePaymentResponse_get(const google::protobuf::Map<int32_t, ::common::SimplePaymentResponse>* m,
                       int32_t key, const ::common::SimplePaymentResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::SimplePaymentResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_SimplePaymentResponse_remove(google::protobuf::Map<int32_t, ::common::SimplePaymentResponse> * m,
                          int32_t key, ::common::SimplePaymentResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_common_SimplePaymentResponse_iter(
      const google::protobuf::Map<int32_t, ::common::SimplePaymentResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_common_SimplePaymentResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::common::SimplePaymentResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::common::SimplePaymentResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::common::SimplePaymentResponse>* __rust_proto_thunk__Map_u32_common_SimplePaymentResponse_new() {
    return new google::protobuf::Map<uint32_t, ::common::SimplePaymentResponse>();
  }
  void __rust_proto_thunk__Map_u32_common_SimplePaymentResponse_free(const google::protobuf::Map<uint32_t, ::common::SimplePaymentResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_common_SimplePaymentResponse_clear(google::protobuf::Map<uint32_t, ::common::SimplePaymentResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_common_SimplePaymentResponse_size(const google::protobuf::Map<uint32_t, ::common::SimplePaymentResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_common_SimplePaymentResponse_insert(google::protobuf::Map<uint32_t, ::common::SimplePaymentResponse> * m,
                          uint32_t key, ::common::SimplePaymentResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_SimplePaymentResponse_get(const google::protobuf::Map<uint32_t, ::common::SimplePaymentResponse>* m,
                       uint32_t key, const ::common::SimplePaymentResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::SimplePaymentResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_SimplePaymentResponse_remove(google::protobuf::Map<uint32_t, ::common::SimplePaymentResponse> * m,
                          uint32_t key, ::common::SimplePaymentResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_common_SimplePaymentResponse_iter(
      const google::protobuf::Map<uint32_t, ::common::SimplePaymentResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_common_SimplePaymentResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::common::SimplePaymentResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::common::SimplePaymentResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::common::SimplePaymentResponse>* __rust_proto_thunk__Map_i64_common_SimplePaymentResponse_new() {
    return new google::protobuf::Map<int64_t, ::common::SimplePaymentResponse>();
  }
  void __rust_proto_thunk__Map_i64_common_SimplePaymentResponse_free(const google::protobuf::Map<int64_t, ::common::SimplePaymentResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_common_SimplePaymentResponse_clear(google::protobuf::Map<int64_t, ::common::SimplePaymentResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_common_SimplePaymentResponse_size(const google::protobuf::Map<int64_t, ::common::SimplePaymentResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_common_SimplePaymentResponse_insert(google::protobuf::Map<int64_t, ::common::SimplePaymentResponse> * m,
                          int64_t key, ::common::SimplePaymentResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_SimplePaymentResponse_get(const google::protobuf::Map<int64_t, ::common::SimplePaymentResponse>* m,
                       int64_t key, const ::common::SimplePaymentResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::SimplePaymentResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_SimplePaymentResponse_remove(google::protobuf::Map<int64_t, ::common::SimplePaymentResponse> * m,
                          int64_t key, ::common::SimplePaymentResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_common_SimplePaymentResponse_iter(
      const google::protobuf::Map<int64_t, ::common::SimplePaymentResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_common_SimplePaymentResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::common::SimplePaymentResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::common::SimplePaymentResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::common::SimplePaymentResponse>* __rust_proto_thunk__Map_u64_common_SimplePaymentResponse_new() {
    return new google::protobuf::Map<uint64_t, ::common::SimplePaymentResponse>();
  }
  void __rust_proto_thunk__Map_u64_common_SimplePaymentResponse_free(const google::protobuf::Map<uint64_t, ::common::SimplePaymentResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_common_SimplePaymentResponse_clear(google::protobuf::Map<uint64_t, ::common::SimplePaymentResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_common_SimplePaymentResponse_size(const google::protobuf::Map<uint64_t, ::common::SimplePaymentResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_common_SimplePaymentResponse_insert(google::protobuf::Map<uint64_t, ::common::SimplePaymentResponse> * m,
                          uint64_t key, ::common::SimplePaymentResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_SimplePaymentResponse_get(const google::protobuf::Map<uint64_t, ::common::SimplePaymentResponse>* m,
                       uint64_t key, const ::common::SimplePaymentResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::SimplePaymentResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_SimplePaymentResponse_remove(google::protobuf::Map<uint64_t, ::common::SimplePaymentResponse> * m,
                          uint64_t key, ::common::SimplePaymentResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_common_SimplePaymentResponse_iter(
      const google::protobuf::Map<uint64_t, ::common::SimplePaymentResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_common_SimplePaymentResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::common::SimplePaymentResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::common::SimplePaymentResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::common::SimplePaymentResponse>* __rust_proto_thunk__Map_bool_common_SimplePaymentResponse_new() {
    return new google::protobuf::Map<bool, ::common::SimplePaymentResponse>();
  }
  void __rust_proto_thunk__Map_bool_common_SimplePaymentResponse_free(const google::protobuf::Map<bool, ::common::SimplePaymentResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_common_SimplePaymentResponse_clear(google::protobuf::Map<bool, ::common::SimplePaymentResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_common_SimplePaymentResponse_size(const google::protobuf::Map<bool, ::common::SimplePaymentResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_common_SimplePaymentResponse_insert(google::protobuf::Map<bool, ::common::SimplePaymentResponse> * m,
                          bool key, ::common::SimplePaymentResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_SimplePaymentResponse_get(const google::protobuf::Map<bool, ::common::SimplePaymentResponse>* m,
                       bool key, const ::common::SimplePaymentResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::SimplePaymentResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_SimplePaymentResponse_remove(google::protobuf::Map<bool, ::common::SimplePaymentResponse> * m,
                          bool key, ::common::SimplePaymentResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_common_SimplePaymentResponse_iter(
      const google::protobuf::Map<bool, ::common::SimplePaymentResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_common_SimplePaymentResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::common::SimplePaymentResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::common::SimplePaymentResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::common::SimplePaymentResponse>* __rust_proto_thunk__Map_ProtoStr_common_SimplePaymentResponse_new() {
    return new google::protobuf::Map<std::string, ::common::SimplePaymentResponse>();
  }
  void __rust_proto_thunk__Map_ProtoStr_common_SimplePaymentResponse_free(const google::protobuf::Map<std::string, ::common::SimplePaymentResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_common_SimplePaymentResponse_clear(google::protobuf::Map<std::string, ::common::SimplePaymentResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_common_SimplePaymentResponse_size(const google::protobuf::Map<std::string, ::common::SimplePaymentResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_SimplePaymentResponse_insert(google::protobuf::Map<std::string, ::common::SimplePaymentResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::SimplePaymentResponse value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_SimplePaymentResponse_get(const google::protobuf::Map<std::string, ::common::SimplePaymentResponse>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::common::SimplePaymentResponse** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::common::SimplePaymentResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_SimplePaymentResponse_remove(google::protobuf::Map<std::string, ::common::SimplePaymentResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::SimplePaymentResponse * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_common_SimplePaymentResponse_iter(
      const google::protobuf::Map<std::string, ::common::SimplePaymentResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_common_SimplePaymentResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::common::SimplePaymentResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::common::SimplePaymentResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// common.FundsChangeRequest
        // clang-format off
extern "C" {
void* __rust_proto_thunk__common_FundsChangeRequest_new() { return new ::common::FundsChangeRequest(); }
void __rust_proto_thunk__common_FundsChangeRequest_delete(void* ptr) { delete static_cast<::common::FundsChangeRequest*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__common_FundsChangeRequest_serialize(::common::FundsChangeRequest* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__common_FundsChangeRequest_parse(::common::FundsChangeRequest* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__common_FundsChangeRequest_copy_from(::common::FundsChangeRequest* dst, const ::common::FundsChangeRequest* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__common_FundsChangeRequest_repeated_len(google::protobuf::RepeatedPtrField<::common::FundsChangeRequest>* field) {
  return field->size();
}
const ::common::FundsChangeRequest& __rust_proto_thunk__common_FundsChangeRequest_repeated_get(
  google::protobuf::RepeatedPtrField<::common::FundsChangeRequest>* field,
  size_t index) {
  return field->Get(index);
}
::common::FundsChangeRequest* __rust_proto_thunk__common_FundsChangeRequest_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::common::FundsChangeRequest>* field,
  size_t index) {
  return field->Mutable(index);
}
::common::FundsChangeRequest* __rust_proto_thunk__common_FundsChangeRequest_repeated_add(google::protobuf::RepeatedPtrField<::common::FundsChangeRequest>* field) {
  return field->Add();
}
void __rust_proto_thunk__common_FundsChangeRequest_repeated_clear(google::protobuf::RepeatedPtrField<::common::FundsChangeRequest>* field) {
  field->Clear();
}
void __rust_proto_thunk__common_FundsChangeRequest_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::common::FundsChangeRequest>& dst,
  const google::protobuf::RepeatedPtrField<::common::FundsChangeRequest>& src) {
  dst = src;
}

const void* __rust_proto_thunk__common_FundsChangeRequest_get_gross_amount(::common::FundsChangeRequest* msg) {
  return static_cast<const void*>(&msg->gross_amount());
}
void* __rust_proto_thunk__common_FundsChangeRequest_get_mut_gross_amount(::common::FundsChangeRequest* msg) {
  return static_cast<void*>(msg->mutable_gross_amount());
}
void __rust_proto_thunk__common_FundsChangeRequest_clear_gross_amount(::common::FundsChangeRequest* msg) { msg->clear_gross_amount(); }
bool __rust_proto_thunk__common_FundsChangeRequest_has_gross_amount(::common::FundsChangeRequest* msg) { return msg->has_gross_amount(); }

const void* __rust_proto_thunk__common_FundsChangeRequest_get_exact_amount(::common::FundsChangeRequest* msg) {
  return static_cast<const void*>(&msg->exact_amount());
}
void* __rust_proto_thunk__common_FundsChangeRequest_get_mut_exact_amount(::common::FundsChangeRequest* msg) {
  return static_cast<void*>(msg->mutable_exact_amount());
}
void __rust_proto_thunk__common_FundsChangeRequest_clear_exact_amount(::common::FundsChangeRequest* msg) { msg->clear_exact_amount(); }
bool __rust_proto_thunk__common_FundsChangeRequest_has_exact_amount(::common::FundsChangeRequest* msg) { return msg->has_exact_amount(); }

const void* __rust_proto_thunk__common_FundsChangeRequest_get_fees(::common::FundsChangeRequest* msg) {
  return static_cast<const void*>(&msg->fees());
}
void* __rust_proto_thunk__common_FundsChangeRequest_get_mut_fees(::common::FundsChangeRequest* msg) {
  return static_cast<void*>(msg->mutable_fees());
}
void __rust_proto_thunk__common_FundsChangeRequest_clear_fees(::common::FundsChangeRequest* msg) { msg->clear_fees(); }
bool __rust_proto_thunk__common_FundsChangeRequest_has_fees(::common::FundsChangeRequest* msg) { return msg->has_fees(); }


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::common::FundsChangeRequest>* __rust_proto_thunk__Map_i32_common_FundsChangeRequest_new() {
    return new google::protobuf::Map<int32_t, ::common::FundsChangeRequest>();
  }
  void __rust_proto_thunk__Map_i32_common_FundsChangeRequest_free(const google::protobuf::Map<int32_t, ::common::FundsChangeRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_common_FundsChangeRequest_clear(google::protobuf::Map<int32_t, ::common::FundsChangeRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_common_FundsChangeRequest_size(const google::protobuf::Map<int32_t, ::common::FundsChangeRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_common_FundsChangeRequest_insert(google::protobuf::Map<int32_t, ::common::FundsChangeRequest> * m,
                          int32_t key, ::common::FundsChangeRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_FundsChangeRequest_get(const google::protobuf::Map<int32_t, ::common::FundsChangeRequest>* m,
                       int32_t key, const ::common::FundsChangeRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::FundsChangeRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_FundsChangeRequest_remove(google::protobuf::Map<int32_t, ::common::FundsChangeRequest> * m,
                          int32_t key, ::common::FundsChangeRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_common_FundsChangeRequest_iter(
      const google::protobuf::Map<int32_t, ::common::FundsChangeRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_common_FundsChangeRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::common::FundsChangeRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::common::FundsChangeRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::common::FundsChangeRequest>* __rust_proto_thunk__Map_u32_common_FundsChangeRequest_new() {
    return new google::protobuf::Map<uint32_t, ::common::FundsChangeRequest>();
  }
  void __rust_proto_thunk__Map_u32_common_FundsChangeRequest_free(const google::protobuf::Map<uint32_t, ::common::FundsChangeRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_common_FundsChangeRequest_clear(google::protobuf::Map<uint32_t, ::common::FundsChangeRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_common_FundsChangeRequest_size(const google::protobuf::Map<uint32_t, ::common::FundsChangeRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_common_FundsChangeRequest_insert(google::protobuf::Map<uint32_t, ::common::FundsChangeRequest> * m,
                          uint32_t key, ::common::FundsChangeRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_FundsChangeRequest_get(const google::protobuf::Map<uint32_t, ::common::FundsChangeRequest>* m,
                       uint32_t key, const ::common::FundsChangeRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::FundsChangeRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_FundsChangeRequest_remove(google::protobuf::Map<uint32_t, ::common::FundsChangeRequest> * m,
                          uint32_t key, ::common::FundsChangeRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_common_FundsChangeRequest_iter(
      const google::protobuf::Map<uint32_t, ::common::FundsChangeRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_common_FundsChangeRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::common::FundsChangeRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::common::FundsChangeRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::common::FundsChangeRequest>* __rust_proto_thunk__Map_i64_common_FundsChangeRequest_new() {
    return new google::protobuf::Map<int64_t, ::common::FundsChangeRequest>();
  }
  void __rust_proto_thunk__Map_i64_common_FundsChangeRequest_free(const google::protobuf::Map<int64_t, ::common::FundsChangeRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_common_FundsChangeRequest_clear(google::protobuf::Map<int64_t, ::common::FundsChangeRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_common_FundsChangeRequest_size(const google::protobuf::Map<int64_t, ::common::FundsChangeRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_common_FundsChangeRequest_insert(google::protobuf::Map<int64_t, ::common::FundsChangeRequest> * m,
                          int64_t key, ::common::FundsChangeRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_FundsChangeRequest_get(const google::protobuf::Map<int64_t, ::common::FundsChangeRequest>* m,
                       int64_t key, const ::common::FundsChangeRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::FundsChangeRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_FundsChangeRequest_remove(google::protobuf::Map<int64_t, ::common::FundsChangeRequest> * m,
                          int64_t key, ::common::FundsChangeRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_common_FundsChangeRequest_iter(
      const google::protobuf::Map<int64_t, ::common::FundsChangeRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_common_FundsChangeRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::common::FundsChangeRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::common::FundsChangeRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::common::FundsChangeRequest>* __rust_proto_thunk__Map_u64_common_FundsChangeRequest_new() {
    return new google::protobuf::Map<uint64_t, ::common::FundsChangeRequest>();
  }
  void __rust_proto_thunk__Map_u64_common_FundsChangeRequest_free(const google::protobuf::Map<uint64_t, ::common::FundsChangeRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_common_FundsChangeRequest_clear(google::protobuf::Map<uint64_t, ::common::FundsChangeRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_common_FundsChangeRequest_size(const google::protobuf::Map<uint64_t, ::common::FundsChangeRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_common_FundsChangeRequest_insert(google::protobuf::Map<uint64_t, ::common::FundsChangeRequest> * m,
                          uint64_t key, ::common::FundsChangeRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_FundsChangeRequest_get(const google::protobuf::Map<uint64_t, ::common::FundsChangeRequest>* m,
                       uint64_t key, const ::common::FundsChangeRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::FundsChangeRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_FundsChangeRequest_remove(google::protobuf::Map<uint64_t, ::common::FundsChangeRequest> * m,
                          uint64_t key, ::common::FundsChangeRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_common_FundsChangeRequest_iter(
      const google::protobuf::Map<uint64_t, ::common::FundsChangeRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_common_FundsChangeRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::common::FundsChangeRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::common::FundsChangeRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::common::FundsChangeRequest>* __rust_proto_thunk__Map_bool_common_FundsChangeRequest_new() {
    return new google::protobuf::Map<bool, ::common::FundsChangeRequest>();
  }
  void __rust_proto_thunk__Map_bool_common_FundsChangeRequest_free(const google::protobuf::Map<bool, ::common::FundsChangeRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_common_FundsChangeRequest_clear(google::protobuf::Map<bool, ::common::FundsChangeRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_common_FundsChangeRequest_size(const google::protobuf::Map<bool, ::common::FundsChangeRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_common_FundsChangeRequest_insert(google::protobuf::Map<bool, ::common::FundsChangeRequest> * m,
                          bool key, ::common::FundsChangeRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_FundsChangeRequest_get(const google::protobuf::Map<bool, ::common::FundsChangeRequest>* m,
                       bool key, const ::common::FundsChangeRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::FundsChangeRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_FundsChangeRequest_remove(google::protobuf::Map<bool, ::common::FundsChangeRequest> * m,
                          bool key, ::common::FundsChangeRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_common_FundsChangeRequest_iter(
      const google::protobuf::Map<bool, ::common::FundsChangeRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_common_FundsChangeRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::common::FundsChangeRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::common::FundsChangeRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::common::FundsChangeRequest>* __rust_proto_thunk__Map_ProtoStr_common_FundsChangeRequest_new() {
    return new google::protobuf::Map<std::string, ::common::FundsChangeRequest>();
  }
  void __rust_proto_thunk__Map_ProtoStr_common_FundsChangeRequest_free(const google::protobuf::Map<std::string, ::common::FundsChangeRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_common_FundsChangeRequest_clear(google::protobuf::Map<std::string, ::common::FundsChangeRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_common_FundsChangeRequest_size(const google::protobuf::Map<std::string, ::common::FundsChangeRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_FundsChangeRequest_insert(google::protobuf::Map<std::string, ::common::FundsChangeRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::FundsChangeRequest value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_FundsChangeRequest_get(const google::protobuf::Map<std::string, ::common::FundsChangeRequest>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::common::FundsChangeRequest** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::common::FundsChangeRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_FundsChangeRequest_remove(google::protobuf::Map<std::string, ::common::FundsChangeRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::FundsChangeRequest * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_common_FundsChangeRequest_iter(
      const google::protobuf::Map<std::string, ::common::FundsChangeRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_common_FundsChangeRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::common::FundsChangeRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::common::FundsChangeRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// common.FundsChangeAck
        // clang-format off
extern "C" {
void* __rust_proto_thunk__common_FundsChangeAck_new() { return new ::common::FundsChangeAck(); }
void __rust_proto_thunk__common_FundsChangeAck_delete(void* ptr) { delete static_cast<::common::FundsChangeAck*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__common_FundsChangeAck_serialize(::common::FundsChangeAck* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__common_FundsChangeAck_parse(::common::FundsChangeAck* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__common_FundsChangeAck_copy_from(::common::FundsChangeAck* dst, const ::common::FundsChangeAck* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__common_FundsChangeAck_repeated_len(google::protobuf::RepeatedPtrField<::common::FundsChangeAck>* field) {
  return field->size();
}
const ::common::FundsChangeAck& __rust_proto_thunk__common_FundsChangeAck_repeated_get(
  google::protobuf::RepeatedPtrField<::common::FundsChangeAck>* field,
  size_t index) {
  return field->Get(index);
}
::common::FundsChangeAck* __rust_proto_thunk__common_FundsChangeAck_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::common::FundsChangeAck>* field,
  size_t index) {
  return field->Mutable(index);
}
::common::FundsChangeAck* __rust_proto_thunk__common_FundsChangeAck_repeated_add(google::protobuf::RepeatedPtrField<::common::FundsChangeAck>* field) {
  return field->Add();
}
void __rust_proto_thunk__common_FundsChangeAck_repeated_clear(google::protobuf::RepeatedPtrField<::common::FundsChangeAck>* field) {
  field->Clear();
}
void __rust_proto_thunk__common_FundsChangeAck_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::common::FundsChangeAck>& dst,
  const google::protobuf::RepeatedPtrField<::common::FundsChangeAck>& src) {
  dst = src;
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::common::FundsChangeAck>* __rust_proto_thunk__Map_i32_common_FundsChangeAck_new() {
    return new google::protobuf::Map<int32_t, ::common::FundsChangeAck>();
  }
  void __rust_proto_thunk__Map_i32_common_FundsChangeAck_free(const google::protobuf::Map<int32_t, ::common::FundsChangeAck>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_common_FundsChangeAck_clear(google::protobuf::Map<int32_t, ::common::FundsChangeAck> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_common_FundsChangeAck_size(const google::protobuf::Map<int32_t, ::common::FundsChangeAck>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_common_FundsChangeAck_insert(google::protobuf::Map<int32_t, ::common::FundsChangeAck> * m,
                          int32_t key, ::common::FundsChangeAck value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_FundsChangeAck_get(const google::protobuf::Map<int32_t, ::common::FundsChangeAck>* m,
                       int32_t key, const ::common::FundsChangeAck** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::FundsChangeAck* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_FundsChangeAck_remove(google::protobuf::Map<int32_t, ::common::FundsChangeAck> * m,
                          int32_t key, ::common::FundsChangeAck * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_common_FundsChangeAck_iter(
      const google::protobuf::Map<int32_t, ::common::FundsChangeAck>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_common_FundsChangeAck_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::common::FundsChangeAck** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::common::FundsChangeAck>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::common::FundsChangeAck>* __rust_proto_thunk__Map_u32_common_FundsChangeAck_new() {
    return new google::protobuf::Map<uint32_t, ::common::FundsChangeAck>();
  }
  void __rust_proto_thunk__Map_u32_common_FundsChangeAck_free(const google::protobuf::Map<uint32_t, ::common::FundsChangeAck>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_common_FundsChangeAck_clear(google::protobuf::Map<uint32_t, ::common::FundsChangeAck> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_common_FundsChangeAck_size(const google::protobuf::Map<uint32_t, ::common::FundsChangeAck>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_common_FundsChangeAck_insert(google::protobuf::Map<uint32_t, ::common::FundsChangeAck> * m,
                          uint32_t key, ::common::FundsChangeAck value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_FundsChangeAck_get(const google::protobuf::Map<uint32_t, ::common::FundsChangeAck>* m,
                       uint32_t key, const ::common::FundsChangeAck** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::FundsChangeAck* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_FundsChangeAck_remove(google::protobuf::Map<uint32_t, ::common::FundsChangeAck> * m,
                          uint32_t key, ::common::FundsChangeAck * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_common_FundsChangeAck_iter(
      const google::protobuf::Map<uint32_t, ::common::FundsChangeAck>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_common_FundsChangeAck_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::common::FundsChangeAck** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::common::FundsChangeAck>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::common::FundsChangeAck>* __rust_proto_thunk__Map_i64_common_FundsChangeAck_new() {
    return new google::protobuf::Map<int64_t, ::common::FundsChangeAck>();
  }
  void __rust_proto_thunk__Map_i64_common_FundsChangeAck_free(const google::protobuf::Map<int64_t, ::common::FundsChangeAck>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_common_FundsChangeAck_clear(google::protobuf::Map<int64_t, ::common::FundsChangeAck> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_common_FundsChangeAck_size(const google::protobuf::Map<int64_t, ::common::FundsChangeAck>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_common_FundsChangeAck_insert(google::protobuf::Map<int64_t, ::common::FundsChangeAck> * m,
                          int64_t key, ::common::FundsChangeAck value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_FundsChangeAck_get(const google::protobuf::Map<int64_t, ::common::FundsChangeAck>* m,
                       int64_t key, const ::common::FundsChangeAck** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::FundsChangeAck* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_FundsChangeAck_remove(google::protobuf::Map<int64_t, ::common::FundsChangeAck> * m,
                          int64_t key, ::common::FundsChangeAck * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_common_FundsChangeAck_iter(
      const google::protobuf::Map<int64_t, ::common::FundsChangeAck>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_common_FundsChangeAck_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::common::FundsChangeAck** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::common::FundsChangeAck>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::common::FundsChangeAck>* __rust_proto_thunk__Map_u64_common_FundsChangeAck_new() {
    return new google::protobuf::Map<uint64_t, ::common::FundsChangeAck>();
  }
  void __rust_proto_thunk__Map_u64_common_FundsChangeAck_free(const google::protobuf::Map<uint64_t, ::common::FundsChangeAck>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_common_FundsChangeAck_clear(google::protobuf::Map<uint64_t, ::common::FundsChangeAck> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_common_FundsChangeAck_size(const google::protobuf::Map<uint64_t, ::common::FundsChangeAck>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_common_FundsChangeAck_insert(google::protobuf::Map<uint64_t, ::common::FundsChangeAck> * m,
                          uint64_t key, ::common::FundsChangeAck value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_FundsChangeAck_get(const google::protobuf::Map<uint64_t, ::common::FundsChangeAck>* m,
                       uint64_t key, const ::common::FundsChangeAck** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::FundsChangeAck* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_FundsChangeAck_remove(google::protobuf::Map<uint64_t, ::common::FundsChangeAck> * m,
                          uint64_t key, ::common::FundsChangeAck * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_common_FundsChangeAck_iter(
      const google::protobuf::Map<uint64_t, ::common::FundsChangeAck>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_common_FundsChangeAck_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::common::FundsChangeAck** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::common::FundsChangeAck>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::common::FundsChangeAck>* __rust_proto_thunk__Map_bool_common_FundsChangeAck_new() {
    return new google::protobuf::Map<bool, ::common::FundsChangeAck>();
  }
  void __rust_proto_thunk__Map_bool_common_FundsChangeAck_free(const google::protobuf::Map<bool, ::common::FundsChangeAck>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_common_FundsChangeAck_clear(google::protobuf::Map<bool, ::common::FundsChangeAck> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_common_FundsChangeAck_size(const google::protobuf::Map<bool, ::common::FundsChangeAck>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_common_FundsChangeAck_insert(google::protobuf::Map<bool, ::common::FundsChangeAck> * m,
                          bool key, ::common::FundsChangeAck value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_FundsChangeAck_get(const google::protobuf::Map<bool, ::common::FundsChangeAck>* m,
                       bool key, const ::common::FundsChangeAck** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::FundsChangeAck* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_FundsChangeAck_remove(google::protobuf::Map<bool, ::common::FundsChangeAck> * m,
                          bool key, ::common::FundsChangeAck * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_common_FundsChangeAck_iter(
      const google::protobuf::Map<bool, ::common::FundsChangeAck>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_common_FundsChangeAck_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::common::FundsChangeAck** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::common::FundsChangeAck>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::common::FundsChangeAck>* __rust_proto_thunk__Map_ProtoStr_common_FundsChangeAck_new() {
    return new google::protobuf::Map<std::string, ::common::FundsChangeAck>();
  }
  void __rust_proto_thunk__Map_ProtoStr_common_FundsChangeAck_free(const google::protobuf::Map<std::string, ::common::FundsChangeAck>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_common_FundsChangeAck_clear(google::protobuf::Map<std::string, ::common::FundsChangeAck> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_common_FundsChangeAck_size(const google::protobuf::Map<std::string, ::common::FundsChangeAck>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_FundsChangeAck_insert(google::protobuf::Map<std::string, ::common::FundsChangeAck> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::FundsChangeAck value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_FundsChangeAck_get(const google::protobuf::Map<std::string, ::common::FundsChangeAck>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::common::FundsChangeAck** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::common::FundsChangeAck* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_FundsChangeAck_remove(google::protobuf::Map<std::string, ::common::FundsChangeAck> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::FundsChangeAck * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_common_FundsChangeAck_iter(
      const google::protobuf::Map<std::string, ::common::FundsChangeAck>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_common_FundsChangeAck_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::common::FundsChangeAck** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::common::FundsChangeAck>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// common.FundsChangeNack
        // clang-format off
extern "C" {
void* __rust_proto_thunk__common_FundsChangeNack_new() { return new ::common::FundsChangeNack(); }
void __rust_proto_thunk__common_FundsChangeNack_delete(void* ptr) { delete static_cast<::common::FundsChangeNack*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__common_FundsChangeNack_serialize(::common::FundsChangeNack* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__common_FundsChangeNack_parse(::common::FundsChangeNack* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__common_FundsChangeNack_copy_from(::common::FundsChangeNack* dst, const ::common::FundsChangeNack* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__common_FundsChangeNack_repeated_len(google::protobuf::RepeatedPtrField<::common::FundsChangeNack>* field) {
  return field->size();
}
const ::common::FundsChangeNack& __rust_proto_thunk__common_FundsChangeNack_repeated_get(
  google::protobuf::RepeatedPtrField<::common::FundsChangeNack>* field,
  size_t index) {
  return field->Get(index);
}
::common::FundsChangeNack* __rust_proto_thunk__common_FundsChangeNack_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::common::FundsChangeNack>* field,
  size_t index) {
  return field->Mutable(index);
}
::common::FundsChangeNack* __rust_proto_thunk__common_FundsChangeNack_repeated_add(google::protobuf::RepeatedPtrField<::common::FundsChangeNack>* field) {
  return field->Add();
}
void __rust_proto_thunk__common_FundsChangeNack_repeated_clear(google::protobuf::RepeatedPtrField<::common::FundsChangeNack>* field) {
  field->Clear();
}
void __rust_proto_thunk__common_FundsChangeNack_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::common::FundsChangeNack>& dst,
  const google::protobuf::RepeatedPtrField<::common::FundsChangeNack>& src) {
  dst = src;
}

::common::FundsChangeNack_RejectionReason __rust_proto_thunk__common_FundsChangeNack_get_rejection_reason(::common::FundsChangeNack* msg) { return msg->rejection_reason(); }
void __rust_proto_thunk__common_FundsChangeNack_set_rejection_reason(::common::FundsChangeNack* msg, ::common::FundsChangeNack_RejectionReason val) {
  msg->set_rejection_reason(val);
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__common_FundsChangeNack_get_other(::common::FundsChangeNack* msg) {
  absl::string_view val = msg->other();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__common_FundsChangeNack_set_other(::common::FundsChangeNack* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_other(absl::string_view(s.ptr, s.len));
}


}  // extern "C"
// clang-format on

extern "C" {
  __PB_RUST_EXPOSE_SCALAR_MAP_METHODS_FOR_VALUE_TYPE(
      ::common::FundsChangeNack_RejectionReason, common_FundsChangeNack_RejectionReason, ::common::FundsChangeNack_RejectionReason, value, cpp_value)
}
extern "C" {
  const google::protobuf::Map<int32_t, ::common::FundsChangeNack>* __rust_proto_thunk__Map_i32_common_FundsChangeNack_new() {
    return new google::protobuf::Map<int32_t, ::common::FundsChangeNack>();
  }
  void __rust_proto_thunk__Map_i32_common_FundsChangeNack_free(const google::protobuf::Map<int32_t, ::common::FundsChangeNack>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_common_FundsChangeNack_clear(google::protobuf::Map<int32_t, ::common::FundsChangeNack> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_common_FundsChangeNack_size(const google::protobuf::Map<int32_t, ::common::FundsChangeNack>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_common_FundsChangeNack_insert(google::protobuf::Map<int32_t, ::common::FundsChangeNack> * m,
                          int32_t key, ::common::FundsChangeNack value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_FundsChangeNack_get(const google::protobuf::Map<int32_t, ::common::FundsChangeNack>* m,
                       int32_t key, const ::common::FundsChangeNack** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::FundsChangeNack* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_FundsChangeNack_remove(google::protobuf::Map<int32_t, ::common::FundsChangeNack> * m,
                          int32_t key, ::common::FundsChangeNack * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_common_FundsChangeNack_iter(
      const google::protobuf::Map<int32_t, ::common::FundsChangeNack>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_common_FundsChangeNack_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::common::FundsChangeNack** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::common::FundsChangeNack>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::common::FundsChangeNack>* __rust_proto_thunk__Map_u32_common_FundsChangeNack_new() {
    return new google::protobuf::Map<uint32_t, ::common::FundsChangeNack>();
  }
  void __rust_proto_thunk__Map_u32_common_FundsChangeNack_free(const google::protobuf::Map<uint32_t, ::common::FundsChangeNack>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_common_FundsChangeNack_clear(google::protobuf::Map<uint32_t, ::common::FundsChangeNack> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_common_FundsChangeNack_size(const google::protobuf::Map<uint32_t, ::common::FundsChangeNack>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_common_FundsChangeNack_insert(google::protobuf::Map<uint32_t, ::common::FundsChangeNack> * m,
                          uint32_t key, ::common::FundsChangeNack value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_FundsChangeNack_get(const google::protobuf::Map<uint32_t, ::common::FundsChangeNack>* m,
                       uint32_t key, const ::common::FundsChangeNack** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::FundsChangeNack* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_FundsChangeNack_remove(google::protobuf::Map<uint32_t, ::common::FundsChangeNack> * m,
                          uint32_t key, ::common::FundsChangeNack * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_common_FundsChangeNack_iter(
      const google::protobuf::Map<uint32_t, ::common::FundsChangeNack>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_common_FundsChangeNack_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::common::FundsChangeNack** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::common::FundsChangeNack>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::common::FundsChangeNack>* __rust_proto_thunk__Map_i64_common_FundsChangeNack_new() {
    return new google::protobuf::Map<int64_t, ::common::FundsChangeNack>();
  }
  void __rust_proto_thunk__Map_i64_common_FundsChangeNack_free(const google::protobuf::Map<int64_t, ::common::FundsChangeNack>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_common_FundsChangeNack_clear(google::protobuf::Map<int64_t, ::common::FundsChangeNack> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_common_FundsChangeNack_size(const google::protobuf::Map<int64_t, ::common::FundsChangeNack>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_common_FundsChangeNack_insert(google::protobuf::Map<int64_t, ::common::FundsChangeNack> * m,
                          int64_t key, ::common::FundsChangeNack value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_FundsChangeNack_get(const google::protobuf::Map<int64_t, ::common::FundsChangeNack>* m,
                       int64_t key, const ::common::FundsChangeNack** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::FundsChangeNack* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_FundsChangeNack_remove(google::protobuf::Map<int64_t, ::common::FundsChangeNack> * m,
                          int64_t key, ::common::FundsChangeNack * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_common_FundsChangeNack_iter(
      const google::protobuf::Map<int64_t, ::common::FundsChangeNack>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_common_FundsChangeNack_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::common::FundsChangeNack** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::common::FundsChangeNack>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::common::FundsChangeNack>* __rust_proto_thunk__Map_u64_common_FundsChangeNack_new() {
    return new google::protobuf::Map<uint64_t, ::common::FundsChangeNack>();
  }
  void __rust_proto_thunk__Map_u64_common_FundsChangeNack_free(const google::protobuf::Map<uint64_t, ::common::FundsChangeNack>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_common_FundsChangeNack_clear(google::protobuf::Map<uint64_t, ::common::FundsChangeNack> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_common_FundsChangeNack_size(const google::protobuf::Map<uint64_t, ::common::FundsChangeNack>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_common_FundsChangeNack_insert(google::protobuf::Map<uint64_t, ::common::FundsChangeNack> * m,
                          uint64_t key, ::common::FundsChangeNack value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_FundsChangeNack_get(const google::protobuf::Map<uint64_t, ::common::FundsChangeNack>* m,
                       uint64_t key, const ::common::FundsChangeNack** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::FundsChangeNack* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_FundsChangeNack_remove(google::protobuf::Map<uint64_t, ::common::FundsChangeNack> * m,
                          uint64_t key, ::common::FundsChangeNack * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_common_FundsChangeNack_iter(
      const google::protobuf::Map<uint64_t, ::common::FundsChangeNack>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_common_FundsChangeNack_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::common::FundsChangeNack** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::common::FundsChangeNack>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::common::FundsChangeNack>* __rust_proto_thunk__Map_bool_common_FundsChangeNack_new() {
    return new google::protobuf::Map<bool, ::common::FundsChangeNack>();
  }
  void __rust_proto_thunk__Map_bool_common_FundsChangeNack_free(const google::protobuf::Map<bool, ::common::FundsChangeNack>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_common_FundsChangeNack_clear(google::protobuf::Map<bool, ::common::FundsChangeNack> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_common_FundsChangeNack_size(const google::protobuf::Map<bool, ::common::FundsChangeNack>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_common_FundsChangeNack_insert(google::protobuf::Map<bool, ::common::FundsChangeNack> * m,
                          bool key, ::common::FundsChangeNack value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_FundsChangeNack_get(const google::protobuf::Map<bool, ::common::FundsChangeNack>* m,
                       bool key, const ::common::FundsChangeNack** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::FundsChangeNack* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_FundsChangeNack_remove(google::protobuf::Map<bool, ::common::FundsChangeNack> * m,
                          bool key, ::common::FundsChangeNack * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_common_FundsChangeNack_iter(
      const google::protobuf::Map<bool, ::common::FundsChangeNack>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_common_FundsChangeNack_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::common::FundsChangeNack** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::common::FundsChangeNack>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::common::FundsChangeNack>* __rust_proto_thunk__Map_ProtoStr_common_FundsChangeNack_new() {
    return new google::protobuf::Map<std::string, ::common::FundsChangeNack>();
  }
  void __rust_proto_thunk__Map_ProtoStr_common_FundsChangeNack_free(const google::protobuf::Map<std::string, ::common::FundsChangeNack>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_common_FundsChangeNack_clear(google::protobuf::Map<std::string, ::common::FundsChangeNack> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_common_FundsChangeNack_size(const google::protobuf::Map<std::string, ::common::FundsChangeNack>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_FundsChangeNack_insert(google::protobuf::Map<std::string, ::common::FundsChangeNack> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::FundsChangeNack value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_FundsChangeNack_get(const google::protobuf::Map<std::string, ::common::FundsChangeNack>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::common::FundsChangeNack** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::common::FundsChangeNack* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_FundsChangeNack_remove(google::protobuf::Map<std::string, ::common::FundsChangeNack> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::FundsChangeNack * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_common_FundsChangeNack_iter(
      const google::protobuf::Map<std::string, ::common::FundsChangeNack>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_common_FundsChangeNack_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::common::FundsChangeNack** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::common::FundsChangeNack>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// common.CancelTransaction
        // clang-format off
extern "C" {
void* __rust_proto_thunk__common_CancelTransaction_new() { return new ::common::CancelTransaction(); }
void __rust_proto_thunk__common_CancelTransaction_delete(void* ptr) { delete static_cast<::common::CancelTransaction*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__common_CancelTransaction_serialize(::common::CancelTransaction* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__common_CancelTransaction_parse(::common::CancelTransaction* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__common_CancelTransaction_copy_from(::common::CancelTransaction* dst, const ::common::CancelTransaction* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__common_CancelTransaction_repeated_len(google::protobuf::RepeatedPtrField<::common::CancelTransaction>* field) {
  return field->size();
}
const ::common::CancelTransaction& __rust_proto_thunk__common_CancelTransaction_repeated_get(
  google::protobuf::RepeatedPtrField<::common::CancelTransaction>* field,
  size_t index) {
  return field->Get(index);
}
::common::CancelTransaction* __rust_proto_thunk__common_CancelTransaction_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::common::CancelTransaction>* field,
  size_t index) {
  return field->Mutable(index);
}
::common::CancelTransaction* __rust_proto_thunk__common_CancelTransaction_repeated_add(google::protobuf::RepeatedPtrField<::common::CancelTransaction>* field) {
  return field->Add();
}
void __rust_proto_thunk__common_CancelTransaction_repeated_clear(google::protobuf::RepeatedPtrField<::common::CancelTransaction>* field) {
  field->Clear();
}
void __rust_proto_thunk__common_CancelTransaction_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::common::CancelTransaction>& dst,
  const google::protobuf::RepeatedPtrField<::common::CancelTransaction>& src) {
  dst = src;
}

::common::CancelTransaction_RejectionReason __rust_proto_thunk__common_CancelTransaction_get_rejection_reason(::common::CancelTransaction* msg) { return msg->rejection_reason(); }
void __rust_proto_thunk__common_CancelTransaction_set_rejection_reason(::common::CancelTransaction* msg, ::common::CancelTransaction_RejectionReason val) {
  msg->set_rejection_reason(val);
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__common_CancelTransaction_get_other(::common::CancelTransaction* msg) {
  absl::string_view val = msg->other();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__common_CancelTransaction_set_other(::common::CancelTransaction* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_other(absl::string_view(s.ptr, s.len));
}


}  // extern "C"
// clang-format on

extern "C" {
  __PB_RUST_EXPOSE_SCALAR_MAP_METHODS_FOR_VALUE_TYPE(
      ::common::CancelTransaction_RejectionReason, common_CancelTransaction_RejectionReason, ::common::CancelTransaction_RejectionReason, value, cpp_value)
}
extern "C" {
  const google::protobuf::Map<int32_t, ::common::CancelTransaction>* __rust_proto_thunk__Map_i32_common_CancelTransaction_new() {
    return new google::protobuf::Map<int32_t, ::common::CancelTransaction>();
  }
  void __rust_proto_thunk__Map_i32_common_CancelTransaction_free(const google::protobuf::Map<int32_t, ::common::CancelTransaction>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_common_CancelTransaction_clear(google::protobuf::Map<int32_t, ::common::CancelTransaction> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_common_CancelTransaction_size(const google::protobuf::Map<int32_t, ::common::CancelTransaction>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_common_CancelTransaction_insert(google::protobuf::Map<int32_t, ::common::CancelTransaction> * m,
                          int32_t key, ::common::CancelTransaction value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_CancelTransaction_get(const google::protobuf::Map<int32_t, ::common::CancelTransaction>* m,
                       int32_t key, const ::common::CancelTransaction** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::CancelTransaction* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_CancelTransaction_remove(google::protobuf::Map<int32_t, ::common::CancelTransaction> * m,
                          int32_t key, ::common::CancelTransaction * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_common_CancelTransaction_iter(
      const google::protobuf::Map<int32_t, ::common::CancelTransaction>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_common_CancelTransaction_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::common::CancelTransaction** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::common::CancelTransaction>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::common::CancelTransaction>* __rust_proto_thunk__Map_u32_common_CancelTransaction_new() {
    return new google::protobuf::Map<uint32_t, ::common::CancelTransaction>();
  }
  void __rust_proto_thunk__Map_u32_common_CancelTransaction_free(const google::protobuf::Map<uint32_t, ::common::CancelTransaction>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_common_CancelTransaction_clear(google::protobuf::Map<uint32_t, ::common::CancelTransaction> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_common_CancelTransaction_size(const google::protobuf::Map<uint32_t, ::common::CancelTransaction>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_common_CancelTransaction_insert(google::protobuf::Map<uint32_t, ::common::CancelTransaction> * m,
                          uint32_t key, ::common::CancelTransaction value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_CancelTransaction_get(const google::protobuf::Map<uint32_t, ::common::CancelTransaction>* m,
                       uint32_t key, const ::common::CancelTransaction** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::CancelTransaction* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_CancelTransaction_remove(google::protobuf::Map<uint32_t, ::common::CancelTransaction> * m,
                          uint32_t key, ::common::CancelTransaction * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_common_CancelTransaction_iter(
      const google::protobuf::Map<uint32_t, ::common::CancelTransaction>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_common_CancelTransaction_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::common::CancelTransaction** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::common::CancelTransaction>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::common::CancelTransaction>* __rust_proto_thunk__Map_i64_common_CancelTransaction_new() {
    return new google::protobuf::Map<int64_t, ::common::CancelTransaction>();
  }
  void __rust_proto_thunk__Map_i64_common_CancelTransaction_free(const google::protobuf::Map<int64_t, ::common::CancelTransaction>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_common_CancelTransaction_clear(google::protobuf::Map<int64_t, ::common::CancelTransaction> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_common_CancelTransaction_size(const google::protobuf::Map<int64_t, ::common::CancelTransaction>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_common_CancelTransaction_insert(google::protobuf::Map<int64_t, ::common::CancelTransaction> * m,
                          int64_t key, ::common::CancelTransaction value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_CancelTransaction_get(const google::protobuf::Map<int64_t, ::common::CancelTransaction>* m,
                       int64_t key, const ::common::CancelTransaction** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::CancelTransaction* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_CancelTransaction_remove(google::protobuf::Map<int64_t, ::common::CancelTransaction> * m,
                          int64_t key, ::common::CancelTransaction * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_common_CancelTransaction_iter(
      const google::protobuf::Map<int64_t, ::common::CancelTransaction>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_common_CancelTransaction_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::common::CancelTransaction** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::common::CancelTransaction>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::common::CancelTransaction>* __rust_proto_thunk__Map_u64_common_CancelTransaction_new() {
    return new google::protobuf::Map<uint64_t, ::common::CancelTransaction>();
  }
  void __rust_proto_thunk__Map_u64_common_CancelTransaction_free(const google::protobuf::Map<uint64_t, ::common::CancelTransaction>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_common_CancelTransaction_clear(google::protobuf::Map<uint64_t, ::common::CancelTransaction> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_common_CancelTransaction_size(const google::protobuf::Map<uint64_t, ::common::CancelTransaction>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_common_CancelTransaction_insert(google::protobuf::Map<uint64_t, ::common::CancelTransaction> * m,
                          uint64_t key, ::common::CancelTransaction value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_CancelTransaction_get(const google::protobuf::Map<uint64_t, ::common::CancelTransaction>* m,
                       uint64_t key, const ::common::CancelTransaction** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::CancelTransaction* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_CancelTransaction_remove(google::protobuf::Map<uint64_t, ::common::CancelTransaction> * m,
                          uint64_t key, ::common::CancelTransaction * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_common_CancelTransaction_iter(
      const google::protobuf::Map<uint64_t, ::common::CancelTransaction>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_common_CancelTransaction_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::common::CancelTransaction** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::common::CancelTransaction>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::common::CancelTransaction>* __rust_proto_thunk__Map_bool_common_CancelTransaction_new() {
    return new google::protobuf::Map<bool, ::common::CancelTransaction>();
  }
  void __rust_proto_thunk__Map_bool_common_CancelTransaction_free(const google::protobuf::Map<bool, ::common::CancelTransaction>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_common_CancelTransaction_clear(google::protobuf::Map<bool, ::common::CancelTransaction> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_common_CancelTransaction_size(const google::protobuf::Map<bool, ::common::CancelTransaction>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_common_CancelTransaction_insert(google::protobuf::Map<bool, ::common::CancelTransaction> * m,
                          bool key, ::common::CancelTransaction value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_CancelTransaction_get(const google::protobuf::Map<bool, ::common::CancelTransaction>* m,
                       bool key, const ::common::CancelTransaction** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::CancelTransaction* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_CancelTransaction_remove(google::protobuf::Map<bool, ::common::CancelTransaction> * m,
                          bool key, ::common::CancelTransaction * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_common_CancelTransaction_iter(
      const google::protobuf::Map<bool, ::common::CancelTransaction>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_common_CancelTransaction_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::common::CancelTransaction** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::common::CancelTransaction>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::common::CancelTransaction>* __rust_proto_thunk__Map_ProtoStr_common_CancelTransaction_new() {
    return new google::protobuf::Map<std::string, ::common::CancelTransaction>();
  }
  void __rust_proto_thunk__Map_ProtoStr_common_CancelTransaction_free(const google::protobuf::Map<std::string, ::common::CancelTransaction>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_common_CancelTransaction_clear(google::protobuf::Map<std::string, ::common::CancelTransaction> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_common_CancelTransaction_size(const google::protobuf::Map<std::string, ::common::CancelTransaction>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_CancelTransaction_insert(google::protobuf::Map<std::string, ::common::CancelTransaction> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::CancelTransaction value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_CancelTransaction_get(const google::protobuf::Map<std::string, ::common::CancelTransaction>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::common::CancelTransaction** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::common::CancelTransaction* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_CancelTransaction_remove(google::protobuf::Map<std::string, ::common::CancelTransaction> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::CancelTransaction * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_common_CancelTransaction_iter(
      const google::protobuf::Map<std::string, ::common::CancelTransaction>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_common_CancelTransaction_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::common::CancelTransaction** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::common::CancelTransaction>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// common.Commitment
        // clang-format off
extern "C" {
void* __rust_proto_thunk__common_Commitment_new() { return new ::common::Commitment(); }
void __rust_proto_thunk__common_Commitment_delete(void* ptr) { delete static_cast<::common::Commitment*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__common_Commitment_serialize(::common::Commitment* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__common_Commitment_parse(::common::Commitment* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__common_Commitment_copy_from(::common::Commitment* dst, const ::common::Commitment* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__common_Commitment_repeated_len(google::protobuf::RepeatedPtrField<::common::Commitment>* field) {
  return field->size();
}
const ::common::Commitment& __rust_proto_thunk__common_Commitment_repeated_get(
  google::protobuf::RepeatedPtrField<::common::Commitment>* field,
  size_t index) {
  return field->Get(index);
}
::common::Commitment* __rust_proto_thunk__common_Commitment_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::common::Commitment>* field,
  size_t index) {
  return field->Mutable(index);
}
::common::Commitment* __rust_proto_thunk__common_Commitment_repeated_add(google::protobuf::RepeatedPtrField<::common::Commitment>* field) {
  return field->Add();
}
void __rust_proto_thunk__common_Commitment_repeated_clear(google::protobuf::RepeatedPtrField<::common::Commitment>* field) {
  field->Clear();
}
void __rust_proto_thunk__common_Commitment_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::common::Commitment>& dst,
  const google::protobuf::RepeatedPtrField<::common::Commitment>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__common_Commitment_get_sender(::common::Commitment* msg) {
  absl::string_view val = msg->sender();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__common_Commitment_set_sender(::common::Commitment* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_sender(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__common_Commitment_get_recipient(::common::Commitment* msg) {
  absl::string_view val = msg->recipient();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__common_Commitment_set_recipient(::common::Commitment* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_recipient(absl::string_view(s.ptr, s.len));
}

const void* __rust_proto_thunk__common_Commitment_get_amount(::common::Commitment* msg) {
  return static_cast<const void*>(&msg->amount());
}
void* __rust_proto_thunk__common_Commitment_get_mut_amount(::common::Commitment* msg) {
  return static_cast<void*>(msg->mutable_amount());
}
void __rust_proto_thunk__common_Commitment_clear_amount(::common::Commitment* msg) { msg->clear_amount(); }
bool __rust_proto_thunk__common_Commitment_has_amount(::common::Commitment* msg) { return msg->has_amount(); }


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::common::Commitment>* __rust_proto_thunk__Map_i32_common_Commitment_new() {
    return new google::protobuf::Map<int32_t, ::common::Commitment>();
  }
  void __rust_proto_thunk__Map_i32_common_Commitment_free(const google::protobuf::Map<int32_t, ::common::Commitment>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_common_Commitment_clear(google::protobuf::Map<int32_t, ::common::Commitment> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_common_Commitment_size(const google::protobuf::Map<int32_t, ::common::Commitment>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_common_Commitment_insert(google::protobuf::Map<int32_t, ::common::Commitment> * m,
                          int32_t key, ::common::Commitment value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_Commitment_get(const google::protobuf::Map<int32_t, ::common::Commitment>* m,
                       int32_t key, const ::common::Commitment** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::Commitment* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_Commitment_remove(google::protobuf::Map<int32_t, ::common::Commitment> * m,
                          int32_t key, ::common::Commitment * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_common_Commitment_iter(
      const google::protobuf::Map<int32_t, ::common::Commitment>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_common_Commitment_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::common::Commitment** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::common::Commitment>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::common::Commitment>* __rust_proto_thunk__Map_u32_common_Commitment_new() {
    return new google::protobuf::Map<uint32_t, ::common::Commitment>();
  }
  void __rust_proto_thunk__Map_u32_common_Commitment_free(const google::protobuf::Map<uint32_t, ::common::Commitment>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_common_Commitment_clear(google::protobuf::Map<uint32_t, ::common::Commitment> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_common_Commitment_size(const google::protobuf::Map<uint32_t, ::common::Commitment>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_common_Commitment_insert(google::protobuf::Map<uint32_t, ::common::Commitment> * m,
                          uint32_t key, ::common::Commitment value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_Commitment_get(const google::protobuf::Map<uint32_t, ::common::Commitment>* m,
                       uint32_t key, const ::common::Commitment** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::Commitment* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_Commitment_remove(google::protobuf::Map<uint32_t, ::common::Commitment> * m,
                          uint32_t key, ::common::Commitment * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_common_Commitment_iter(
      const google::protobuf::Map<uint32_t, ::common::Commitment>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_common_Commitment_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::common::Commitment** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::common::Commitment>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::common::Commitment>* __rust_proto_thunk__Map_i64_common_Commitment_new() {
    return new google::protobuf::Map<int64_t, ::common::Commitment>();
  }
  void __rust_proto_thunk__Map_i64_common_Commitment_free(const google::protobuf::Map<int64_t, ::common::Commitment>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_common_Commitment_clear(google::protobuf::Map<int64_t, ::common::Commitment> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_common_Commitment_size(const google::protobuf::Map<int64_t, ::common::Commitment>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_common_Commitment_insert(google::protobuf::Map<int64_t, ::common::Commitment> * m,
                          int64_t key, ::common::Commitment value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_Commitment_get(const google::protobuf::Map<int64_t, ::common::Commitment>* m,
                       int64_t key, const ::common::Commitment** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::Commitment* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_Commitment_remove(google::protobuf::Map<int64_t, ::common::Commitment> * m,
                          int64_t key, ::common::Commitment * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_common_Commitment_iter(
      const google::protobuf::Map<int64_t, ::common::Commitment>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_common_Commitment_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::common::Commitment** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::common::Commitment>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::common::Commitment>* __rust_proto_thunk__Map_u64_common_Commitment_new() {
    return new google::protobuf::Map<uint64_t, ::common::Commitment>();
  }
  void __rust_proto_thunk__Map_u64_common_Commitment_free(const google::protobuf::Map<uint64_t, ::common::Commitment>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_common_Commitment_clear(google::protobuf::Map<uint64_t, ::common::Commitment> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_common_Commitment_size(const google::protobuf::Map<uint64_t, ::common::Commitment>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_common_Commitment_insert(google::protobuf::Map<uint64_t, ::common::Commitment> * m,
                          uint64_t key, ::common::Commitment value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_Commitment_get(const google::protobuf::Map<uint64_t, ::common::Commitment>* m,
                       uint64_t key, const ::common::Commitment** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::Commitment* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_Commitment_remove(google::protobuf::Map<uint64_t, ::common::Commitment> * m,
                          uint64_t key, ::common::Commitment * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_common_Commitment_iter(
      const google::protobuf::Map<uint64_t, ::common::Commitment>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_common_Commitment_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::common::Commitment** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::common::Commitment>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::common::Commitment>* __rust_proto_thunk__Map_bool_common_Commitment_new() {
    return new google::protobuf::Map<bool, ::common::Commitment>();
  }
  void __rust_proto_thunk__Map_bool_common_Commitment_free(const google::protobuf::Map<bool, ::common::Commitment>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_common_Commitment_clear(google::protobuf::Map<bool, ::common::Commitment> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_common_Commitment_size(const google::protobuf::Map<bool, ::common::Commitment>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_common_Commitment_insert(google::protobuf::Map<bool, ::common::Commitment> * m,
                          bool key, ::common::Commitment value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_Commitment_get(const google::protobuf::Map<bool, ::common::Commitment>* m,
                       bool key, const ::common::Commitment** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::Commitment* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_Commitment_remove(google::protobuf::Map<bool, ::common::Commitment> * m,
                          bool key, ::common::Commitment * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_common_Commitment_iter(
      const google::protobuf::Map<bool, ::common::Commitment>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_common_Commitment_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::common::Commitment** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::common::Commitment>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::common::Commitment>* __rust_proto_thunk__Map_ProtoStr_common_Commitment_new() {
    return new google::protobuf::Map<std::string, ::common::Commitment>();
  }
  void __rust_proto_thunk__Map_ProtoStr_common_Commitment_free(const google::protobuf::Map<std::string, ::common::Commitment>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_common_Commitment_clear(google::protobuf::Map<std::string, ::common::Commitment> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_common_Commitment_size(const google::protobuf::Map<std::string, ::common::Commitment>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_Commitment_insert(google::protobuf::Map<std::string, ::common::Commitment> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::Commitment value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_Commitment_get(const google::protobuf::Map<std::string, ::common::Commitment>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::common::Commitment** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::common::Commitment* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_Commitment_remove(google::protobuf::Map<std::string, ::common::Commitment> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::Commitment * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_common_Commitment_iter(
      const google::protobuf::Map<std::string, ::common::Commitment>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_common_Commitment_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::common::Commitment** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::common::Commitment>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// common.TimeoutCondition
        // clang-format off
extern "C" {
void* __rust_proto_thunk__common_TimeoutCondition_new() { return new ::common::TimeoutCondition(); }
void __rust_proto_thunk__common_TimeoutCondition_delete(void* ptr) { delete static_cast<::common::TimeoutCondition*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__common_TimeoutCondition_serialize(::common::TimeoutCondition* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__common_TimeoutCondition_parse(::common::TimeoutCondition* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__common_TimeoutCondition_copy_from(::common::TimeoutCondition* dst, const ::common::TimeoutCondition* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__common_TimeoutCondition_repeated_len(google::protobuf::RepeatedPtrField<::common::TimeoutCondition>* field) {
  return field->size();
}
const ::common::TimeoutCondition& __rust_proto_thunk__common_TimeoutCondition_repeated_get(
  google::protobuf::RepeatedPtrField<::common::TimeoutCondition>* field,
  size_t index) {
  return field->Get(index);
}
::common::TimeoutCondition* __rust_proto_thunk__common_TimeoutCondition_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::common::TimeoutCondition>* field,
  size_t index) {
  return field->Mutable(index);
}
::common::TimeoutCondition* __rust_proto_thunk__common_TimeoutCondition_repeated_add(google::protobuf::RepeatedPtrField<::common::TimeoutCondition>* field) {
  return field->Add();
}
void __rust_proto_thunk__common_TimeoutCondition_repeated_clear(google::protobuf::RepeatedPtrField<::common::TimeoutCondition>* field) {
  field->Clear();
}
void __rust_proto_thunk__common_TimeoutCondition_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::common::TimeoutCondition>& dst,
  const google::protobuf::RepeatedPtrField<::common::TimeoutCondition>& src) {
  dst = src;
}

::uint64_t __rust_proto_thunk__common_TimeoutCondition_get_timeout(::common::TimeoutCondition* msg) { return msg->timeout(); }
void __rust_proto_thunk__common_TimeoutCondition_set_timeout(::common::TimeoutCondition* msg, ::uint64_t val) {
  msg->set_timeout(val);
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::common::TimeoutCondition>* __rust_proto_thunk__Map_i32_common_TimeoutCondition_new() {
    return new google::protobuf::Map<int32_t, ::common::TimeoutCondition>();
  }
  void __rust_proto_thunk__Map_i32_common_TimeoutCondition_free(const google::protobuf::Map<int32_t, ::common::TimeoutCondition>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_common_TimeoutCondition_clear(google::protobuf::Map<int32_t, ::common::TimeoutCondition> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_common_TimeoutCondition_size(const google::protobuf::Map<int32_t, ::common::TimeoutCondition>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_common_TimeoutCondition_insert(google::protobuf::Map<int32_t, ::common::TimeoutCondition> * m,
                          int32_t key, ::common::TimeoutCondition value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_TimeoutCondition_get(const google::protobuf::Map<int32_t, ::common::TimeoutCondition>* m,
                       int32_t key, const ::common::TimeoutCondition** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::TimeoutCondition* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_TimeoutCondition_remove(google::protobuf::Map<int32_t, ::common::TimeoutCondition> * m,
                          int32_t key, ::common::TimeoutCondition * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_common_TimeoutCondition_iter(
      const google::protobuf::Map<int32_t, ::common::TimeoutCondition>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_common_TimeoutCondition_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::common::TimeoutCondition** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::common::TimeoutCondition>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::common::TimeoutCondition>* __rust_proto_thunk__Map_u32_common_TimeoutCondition_new() {
    return new google::protobuf::Map<uint32_t, ::common::TimeoutCondition>();
  }
  void __rust_proto_thunk__Map_u32_common_TimeoutCondition_free(const google::protobuf::Map<uint32_t, ::common::TimeoutCondition>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_common_TimeoutCondition_clear(google::protobuf::Map<uint32_t, ::common::TimeoutCondition> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_common_TimeoutCondition_size(const google::protobuf::Map<uint32_t, ::common::TimeoutCondition>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_common_TimeoutCondition_insert(google::protobuf::Map<uint32_t, ::common::TimeoutCondition> * m,
                          uint32_t key, ::common::TimeoutCondition value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_TimeoutCondition_get(const google::protobuf::Map<uint32_t, ::common::TimeoutCondition>* m,
                       uint32_t key, const ::common::TimeoutCondition** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::TimeoutCondition* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_TimeoutCondition_remove(google::protobuf::Map<uint32_t, ::common::TimeoutCondition> * m,
                          uint32_t key, ::common::TimeoutCondition * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_common_TimeoutCondition_iter(
      const google::protobuf::Map<uint32_t, ::common::TimeoutCondition>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_common_TimeoutCondition_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::common::TimeoutCondition** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::common::TimeoutCondition>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::common::TimeoutCondition>* __rust_proto_thunk__Map_i64_common_TimeoutCondition_new() {
    return new google::protobuf::Map<int64_t, ::common::TimeoutCondition>();
  }
  void __rust_proto_thunk__Map_i64_common_TimeoutCondition_free(const google::protobuf::Map<int64_t, ::common::TimeoutCondition>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_common_TimeoutCondition_clear(google::protobuf::Map<int64_t, ::common::TimeoutCondition> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_common_TimeoutCondition_size(const google::protobuf::Map<int64_t, ::common::TimeoutCondition>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_common_TimeoutCondition_insert(google::protobuf::Map<int64_t, ::common::TimeoutCondition> * m,
                          int64_t key, ::common::TimeoutCondition value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_TimeoutCondition_get(const google::protobuf::Map<int64_t, ::common::TimeoutCondition>* m,
                       int64_t key, const ::common::TimeoutCondition** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::TimeoutCondition* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_TimeoutCondition_remove(google::protobuf::Map<int64_t, ::common::TimeoutCondition> * m,
                          int64_t key, ::common::TimeoutCondition * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_common_TimeoutCondition_iter(
      const google::protobuf::Map<int64_t, ::common::TimeoutCondition>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_common_TimeoutCondition_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::common::TimeoutCondition** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::common::TimeoutCondition>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::common::TimeoutCondition>* __rust_proto_thunk__Map_u64_common_TimeoutCondition_new() {
    return new google::protobuf::Map<uint64_t, ::common::TimeoutCondition>();
  }
  void __rust_proto_thunk__Map_u64_common_TimeoutCondition_free(const google::protobuf::Map<uint64_t, ::common::TimeoutCondition>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_common_TimeoutCondition_clear(google::protobuf::Map<uint64_t, ::common::TimeoutCondition> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_common_TimeoutCondition_size(const google::protobuf::Map<uint64_t, ::common::TimeoutCondition>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_common_TimeoutCondition_insert(google::protobuf::Map<uint64_t, ::common::TimeoutCondition> * m,
                          uint64_t key, ::common::TimeoutCondition value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_TimeoutCondition_get(const google::protobuf::Map<uint64_t, ::common::TimeoutCondition>* m,
                       uint64_t key, const ::common::TimeoutCondition** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::TimeoutCondition* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_TimeoutCondition_remove(google::protobuf::Map<uint64_t, ::common::TimeoutCondition> * m,
                          uint64_t key, ::common::TimeoutCondition * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_common_TimeoutCondition_iter(
      const google::protobuf::Map<uint64_t, ::common::TimeoutCondition>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_common_TimeoutCondition_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::common::TimeoutCondition** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::common::TimeoutCondition>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::common::TimeoutCondition>* __rust_proto_thunk__Map_bool_common_TimeoutCondition_new() {
    return new google::protobuf::Map<bool, ::common::TimeoutCondition>();
  }
  void __rust_proto_thunk__Map_bool_common_TimeoutCondition_free(const google::protobuf::Map<bool, ::common::TimeoutCondition>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_common_TimeoutCondition_clear(google::protobuf::Map<bool, ::common::TimeoutCondition> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_common_TimeoutCondition_size(const google::protobuf::Map<bool, ::common::TimeoutCondition>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_common_TimeoutCondition_insert(google::protobuf::Map<bool, ::common::TimeoutCondition> * m,
                          bool key, ::common::TimeoutCondition value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_TimeoutCondition_get(const google::protobuf::Map<bool, ::common::TimeoutCondition>* m,
                       bool key, const ::common::TimeoutCondition** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::TimeoutCondition* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_TimeoutCondition_remove(google::protobuf::Map<bool, ::common::TimeoutCondition> * m,
                          bool key, ::common::TimeoutCondition * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_common_TimeoutCondition_iter(
      const google::protobuf::Map<bool, ::common::TimeoutCondition>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_common_TimeoutCondition_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::common::TimeoutCondition** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::common::TimeoutCondition>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::common::TimeoutCondition>* __rust_proto_thunk__Map_ProtoStr_common_TimeoutCondition_new() {
    return new google::protobuf::Map<std::string, ::common::TimeoutCondition>();
  }
  void __rust_proto_thunk__Map_ProtoStr_common_TimeoutCondition_free(const google::protobuf::Map<std::string, ::common::TimeoutCondition>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_common_TimeoutCondition_clear(google::protobuf::Map<std::string, ::common::TimeoutCondition> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_common_TimeoutCondition_size(const google::protobuf::Map<std::string, ::common::TimeoutCondition>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_TimeoutCondition_insert(google::protobuf::Map<std::string, ::common::TimeoutCondition> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::TimeoutCondition value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_TimeoutCondition_get(const google::protobuf::Map<std::string, ::common::TimeoutCondition>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::common::TimeoutCondition** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::common::TimeoutCondition* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_TimeoutCondition_remove(google::protobuf::Map<std::string, ::common::TimeoutCondition> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::TimeoutCondition * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_common_TimeoutCondition_iter(
      const google::protobuf::Map<std::string, ::common::TimeoutCondition>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_common_TimeoutCondition_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::common::TimeoutCondition** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::common::TimeoutCondition>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// common.HashCondition
        // clang-format off
extern "C" {
void* __rust_proto_thunk__common_HashCondition_new() { return new ::common::HashCondition(); }
void __rust_proto_thunk__common_HashCondition_delete(void* ptr) { delete static_cast<::common::HashCondition*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__common_HashCondition_serialize(::common::HashCondition* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__common_HashCondition_parse(::common::HashCondition* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__common_HashCondition_copy_from(::common::HashCondition* dst, const ::common::HashCondition* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__common_HashCondition_repeated_len(google::protobuf::RepeatedPtrField<::common::HashCondition>* field) {
  return field->size();
}
const ::common::HashCondition& __rust_proto_thunk__common_HashCondition_repeated_get(
  google::protobuf::RepeatedPtrField<::common::HashCondition>* field,
  size_t index) {
  return field->Get(index);
}
::common::HashCondition* __rust_proto_thunk__common_HashCondition_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::common::HashCondition>* field,
  size_t index) {
  return field->Mutable(index);
}
::common::HashCondition* __rust_proto_thunk__common_HashCondition_repeated_add(google::protobuf::RepeatedPtrField<::common::HashCondition>* field) {
  return field->Add();
}
void __rust_proto_thunk__common_HashCondition_repeated_clear(google::protobuf::RepeatedPtrField<::common::HashCondition>* field) {
  field->Clear();
}
void __rust_proto_thunk__common_HashCondition_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::common::HashCondition>& dst,
  const google::protobuf::RepeatedPtrField<::common::HashCondition>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__common_HashCondition_get_presenter(::common::HashCondition* msg) {
  absl::string_view val = msg->presenter();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__common_HashCondition_set_presenter(::common::HashCondition* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_presenter(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__common_HashCondition_get_hash(::common::HashCondition* msg) {
  absl::string_view val = msg->hash();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__common_HashCondition_set_hash(::common::HashCondition* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_hash(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__common_HashCondition_get_algorithm_oid(::common::HashCondition* msg) {
  absl::string_view val = msg->algorithm_oid();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__common_HashCondition_set_algorithm_oid(::common::HashCondition* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_algorithm_oid(absl::string_view(s.ptr, s.len));
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::common::HashCondition>* __rust_proto_thunk__Map_i32_common_HashCondition_new() {
    return new google::protobuf::Map<int32_t, ::common::HashCondition>();
  }
  void __rust_proto_thunk__Map_i32_common_HashCondition_free(const google::protobuf::Map<int32_t, ::common::HashCondition>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_common_HashCondition_clear(google::protobuf::Map<int32_t, ::common::HashCondition> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_common_HashCondition_size(const google::protobuf::Map<int32_t, ::common::HashCondition>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_common_HashCondition_insert(google::protobuf::Map<int32_t, ::common::HashCondition> * m,
                          int32_t key, ::common::HashCondition value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_HashCondition_get(const google::protobuf::Map<int32_t, ::common::HashCondition>* m,
                       int32_t key, const ::common::HashCondition** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::HashCondition* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_HashCondition_remove(google::protobuf::Map<int32_t, ::common::HashCondition> * m,
                          int32_t key, ::common::HashCondition * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_common_HashCondition_iter(
      const google::protobuf::Map<int32_t, ::common::HashCondition>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_common_HashCondition_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::common::HashCondition** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::common::HashCondition>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::common::HashCondition>* __rust_proto_thunk__Map_u32_common_HashCondition_new() {
    return new google::protobuf::Map<uint32_t, ::common::HashCondition>();
  }
  void __rust_proto_thunk__Map_u32_common_HashCondition_free(const google::protobuf::Map<uint32_t, ::common::HashCondition>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_common_HashCondition_clear(google::protobuf::Map<uint32_t, ::common::HashCondition> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_common_HashCondition_size(const google::protobuf::Map<uint32_t, ::common::HashCondition>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_common_HashCondition_insert(google::protobuf::Map<uint32_t, ::common::HashCondition> * m,
                          uint32_t key, ::common::HashCondition value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_HashCondition_get(const google::protobuf::Map<uint32_t, ::common::HashCondition>* m,
                       uint32_t key, const ::common::HashCondition** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::HashCondition* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_HashCondition_remove(google::protobuf::Map<uint32_t, ::common::HashCondition> * m,
                          uint32_t key, ::common::HashCondition * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_common_HashCondition_iter(
      const google::protobuf::Map<uint32_t, ::common::HashCondition>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_common_HashCondition_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::common::HashCondition** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::common::HashCondition>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::common::HashCondition>* __rust_proto_thunk__Map_i64_common_HashCondition_new() {
    return new google::protobuf::Map<int64_t, ::common::HashCondition>();
  }
  void __rust_proto_thunk__Map_i64_common_HashCondition_free(const google::protobuf::Map<int64_t, ::common::HashCondition>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_common_HashCondition_clear(google::protobuf::Map<int64_t, ::common::HashCondition> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_common_HashCondition_size(const google::protobuf::Map<int64_t, ::common::HashCondition>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_common_HashCondition_insert(google::protobuf::Map<int64_t, ::common::HashCondition> * m,
                          int64_t key, ::common::HashCondition value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_HashCondition_get(const google::protobuf::Map<int64_t, ::common::HashCondition>* m,
                       int64_t key, const ::common::HashCondition** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::HashCondition* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_HashCondition_remove(google::protobuf::Map<int64_t, ::common::HashCondition> * m,
                          int64_t key, ::common::HashCondition * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_common_HashCondition_iter(
      const google::protobuf::Map<int64_t, ::common::HashCondition>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_common_HashCondition_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::common::HashCondition** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::common::HashCondition>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::common::HashCondition>* __rust_proto_thunk__Map_u64_common_HashCondition_new() {
    return new google::protobuf::Map<uint64_t, ::common::HashCondition>();
  }
  void __rust_proto_thunk__Map_u64_common_HashCondition_free(const google::protobuf::Map<uint64_t, ::common::HashCondition>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_common_HashCondition_clear(google::protobuf::Map<uint64_t, ::common::HashCondition> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_common_HashCondition_size(const google::protobuf::Map<uint64_t, ::common::HashCondition>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_common_HashCondition_insert(google::protobuf::Map<uint64_t, ::common::HashCondition> * m,
                          uint64_t key, ::common::HashCondition value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_HashCondition_get(const google::protobuf::Map<uint64_t, ::common::HashCondition>* m,
                       uint64_t key, const ::common::HashCondition** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::HashCondition* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_HashCondition_remove(google::protobuf::Map<uint64_t, ::common::HashCondition> * m,
                          uint64_t key, ::common::HashCondition * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_common_HashCondition_iter(
      const google::protobuf::Map<uint64_t, ::common::HashCondition>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_common_HashCondition_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::common::HashCondition** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::common::HashCondition>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::common::HashCondition>* __rust_proto_thunk__Map_bool_common_HashCondition_new() {
    return new google::protobuf::Map<bool, ::common::HashCondition>();
  }
  void __rust_proto_thunk__Map_bool_common_HashCondition_free(const google::protobuf::Map<bool, ::common::HashCondition>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_common_HashCondition_clear(google::protobuf::Map<bool, ::common::HashCondition> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_common_HashCondition_size(const google::protobuf::Map<bool, ::common::HashCondition>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_common_HashCondition_insert(google::protobuf::Map<bool, ::common::HashCondition> * m,
                          bool key, ::common::HashCondition value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_HashCondition_get(const google::protobuf::Map<bool, ::common::HashCondition>* m,
                       bool key, const ::common::HashCondition** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::HashCondition* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_HashCondition_remove(google::protobuf::Map<bool, ::common::HashCondition> * m,
                          bool key, ::common::HashCondition * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_common_HashCondition_iter(
      const google::protobuf::Map<bool, ::common::HashCondition>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_common_HashCondition_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::common::HashCondition** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::common::HashCondition>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::common::HashCondition>* __rust_proto_thunk__Map_ProtoStr_common_HashCondition_new() {
    return new google::protobuf::Map<std::string, ::common::HashCondition>();
  }
  void __rust_proto_thunk__Map_ProtoStr_common_HashCondition_free(const google::protobuf::Map<std::string, ::common::HashCondition>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_common_HashCondition_clear(google::protobuf::Map<std::string, ::common::HashCondition> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_common_HashCondition_size(const google::protobuf::Map<std::string, ::common::HashCondition>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_HashCondition_insert(google::protobuf::Map<std::string, ::common::HashCondition> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::HashCondition value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_HashCondition_get(const google::protobuf::Map<std::string, ::common::HashCondition>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::common::HashCondition** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::common::HashCondition* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_HashCondition_remove(google::protobuf::Map<std::string, ::common::HashCondition> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::HashCondition * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_common_HashCondition_iter(
      const google::protobuf::Map<std::string, ::common::HashCondition>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_common_HashCondition_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::common::HashCondition** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::common::HashCondition>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// common.MintCondition
        // clang-format off
extern "C" {
void* __rust_proto_thunk__common_MintCondition_new() { return new ::common::MintCondition(); }
void __rust_proto_thunk__common_MintCondition_delete(void* ptr) { delete static_cast<::common::MintCondition*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__common_MintCondition_serialize(::common::MintCondition* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__common_MintCondition_parse(::common::MintCondition* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__common_MintCondition_copy_from(::common::MintCondition* dst, const ::common::MintCondition* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__common_MintCondition_repeated_len(google::protobuf::RepeatedPtrField<::common::MintCondition>* field) {
  return field->size();
}
const ::common::MintCondition& __rust_proto_thunk__common_MintCondition_repeated_get(
  google::protobuf::RepeatedPtrField<::common::MintCondition>* field,
  size_t index) {
  return field->Get(index);
}
::common::MintCondition* __rust_proto_thunk__common_MintCondition_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::common::MintCondition>* field,
  size_t index) {
  return field->Mutable(index);
}
::common::MintCondition* __rust_proto_thunk__common_MintCondition_repeated_add(google::protobuf::RepeatedPtrField<::common::MintCondition>* field) {
  return field->Add();
}
void __rust_proto_thunk__common_MintCondition_repeated_clear(google::protobuf::RepeatedPtrField<::common::MintCondition>* field) {
  field->Clear();
}
void __rust_proto_thunk__common_MintCondition_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::common::MintCondition>& dst,
  const google::protobuf::RepeatedPtrField<::common::MintCondition>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__common_MintCondition_get_issuer(::common::MintCondition* msg) {
  absl::string_view val = msg->issuer();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__common_MintCondition_set_issuer(::common::MintCondition* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_issuer(absl::string_view(s.ptr, s.len));
}

const void* __rust_proto_thunk__common_MintCondition_get_distribution(::common::MintCondition* msg) {
  return static_cast<const void*>(&msg->distribution());
}
void* __rust_proto_thunk__common_MintCondition_get_mut_distribution(::common::MintCondition* msg) {
  return static_cast<void*>(msg->mutable_distribution());
}
void __rust_proto_thunk__common_MintCondition_clear_distribution(::common::MintCondition* msg) { msg->clear_distribution(); }
bool __rust_proto_thunk__common_MintCondition_has_distribution(::common::MintCondition* msg) { return msg->has_distribution(); }


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::common::MintCondition>* __rust_proto_thunk__Map_i32_common_MintCondition_new() {
    return new google::protobuf::Map<int32_t, ::common::MintCondition>();
  }
  void __rust_proto_thunk__Map_i32_common_MintCondition_free(const google::protobuf::Map<int32_t, ::common::MintCondition>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_common_MintCondition_clear(google::protobuf::Map<int32_t, ::common::MintCondition> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_common_MintCondition_size(const google::protobuf::Map<int32_t, ::common::MintCondition>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_common_MintCondition_insert(google::protobuf::Map<int32_t, ::common::MintCondition> * m,
                          int32_t key, ::common::MintCondition value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_MintCondition_get(const google::protobuf::Map<int32_t, ::common::MintCondition>* m,
                       int32_t key, const ::common::MintCondition** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::MintCondition* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_MintCondition_remove(google::protobuf::Map<int32_t, ::common::MintCondition> * m,
                          int32_t key, ::common::MintCondition * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_common_MintCondition_iter(
      const google::protobuf::Map<int32_t, ::common::MintCondition>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_common_MintCondition_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::common::MintCondition** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::common::MintCondition>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::common::MintCondition>* __rust_proto_thunk__Map_u32_common_MintCondition_new() {
    return new google::protobuf::Map<uint32_t, ::common::MintCondition>();
  }
  void __rust_proto_thunk__Map_u32_common_MintCondition_free(const google::protobuf::Map<uint32_t, ::common::MintCondition>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_common_MintCondition_clear(google::protobuf::Map<uint32_t, ::common::MintCondition> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_common_MintCondition_size(const google::protobuf::Map<uint32_t, ::common::MintCondition>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_common_MintCondition_insert(google::protobuf::Map<uint32_t, ::common::MintCondition> * m,
                          uint32_t key, ::common::MintCondition value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_MintCondition_get(const google::protobuf::Map<uint32_t, ::common::MintCondition>* m,
                       uint32_t key, const ::common::MintCondition** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::MintCondition* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_MintCondition_remove(google::protobuf::Map<uint32_t, ::common::MintCondition> * m,
                          uint32_t key, ::common::MintCondition * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_common_MintCondition_iter(
      const google::protobuf::Map<uint32_t, ::common::MintCondition>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_common_MintCondition_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::common::MintCondition** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::common::MintCondition>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::common::MintCondition>* __rust_proto_thunk__Map_i64_common_MintCondition_new() {
    return new google::protobuf::Map<int64_t, ::common::MintCondition>();
  }
  void __rust_proto_thunk__Map_i64_common_MintCondition_free(const google::protobuf::Map<int64_t, ::common::MintCondition>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_common_MintCondition_clear(google::protobuf::Map<int64_t, ::common::MintCondition> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_common_MintCondition_size(const google::protobuf::Map<int64_t, ::common::MintCondition>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_common_MintCondition_insert(google::protobuf::Map<int64_t, ::common::MintCondition> * m,
                          int64_t key, ::common::MintCondition value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_MintCondition_get(const google::protobuf::Map<int64_t, ::common::MintCondition>* m,
                       int64_t key, const ::common::MintCondition** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::MintCondition* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_MintCondition_remove(google::protobuf::Map<int64_t, ::common::MintCondition> * m,
                          int64_t key, ::common::MintCondition * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_common_MintCondition_iter(
      const google::protobuf::Map<int64_t, ::common::MintCondition>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_common_MintCondition_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::common::MintCondition** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::common::MintCondition>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::common::MintCondition>* __rust_proto_thunk__Map_u64_common_MintCondition_new() {
    return new google::protobuf::Map<uint64_t, ::common::MintCondition>();
  }
  void __rust_proto_thunk__Map_u64_common_MintCondition_free(const google::protobuf::Map<uint64_t, ::common::MintCondition>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_common_MintCondition_clear(google::protobuf::Map<uint64_t, ::common::MintCondition> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_common_MintCondition_size(const google::protobuf::Map<uint64_t, ::common::MintCondition>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_common_MintCondition_insert(google::protobuf::Map<uint64_t, ::common::MintCondition> * m,
                          uint64_t key, ::common::MintCondition value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_MintCondition_get(const google::protobuf::Map<uint64_t, ::common::MintCondition>* m,
                       uint64_t key, const ::common::MintCondition** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::MintCondition* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_MintCondition_remove(google::protobuf::Map<uint64_t, ::common::MintCondition> * m,
                          uint64_t key, ::common::MintCondition * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_common_MintCondition_iter(
      const google::protobuf::Map<uint64_t, ::common::MintCondition>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_common_MintCondition_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::common::MintCondition** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::common::MintCondition>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::common::MintCondition>* __rust_proto_thunk__Map_bool_common_MintCondition_new() {
    return new google::protobuf::Map<bool, ::common::MintCondition>();
  }
  void __rust_proto_thunk__Map_bool_common_MintCondition_free(const google::protobuf::Map<bool, ::common::MintCondition>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_common_MintCondition_clear(google::protobuf::Map<bool, ::common::MintCondition> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_common_MintCondition_size(const google::protobuf::Map<bool, ::common::MintCondition>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_common_MintCondition_insert(google::protobuf::Map<bool, ::common::MintCondition> * m,
                          bool key, ::common::MintCondition value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_MintCondition_get(const google::protobuf::Map<bool, ::common::MintCondition>* m,
                       bool key, const ::common::MintCondition** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::MintCondition* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_MintCondition_remove(google::protobuf::Map<bool, ::common::MintCondition> * m,
                          bool key, ::common::MintCondition * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_common_MintCondition_iter(
      const google::protobuf::Map<bool, ::common::MintCondition>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_common_MintCondition_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::common::MintCondition** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::common::MintCondition>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::common::MintCondition>* __rust_proto_thunk__Map_ProtoStr_common_MintCondition_new() {
    return new google::protobuf::Map<std::string, ::common::MintCondition>();
  }
  void __rust_proto_thunk__Map_ProtoStr_common_MintCondition_free(const google::protobuf::Map<std::string, ::common::MintCondition>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_common_MintCondition_clear(google::protobuf::Map<std::string, ::common::MintCondition> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_common_MintCondition_size(const google::protobuf::Map<std::string, ::common::MintCondition>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_MintCondition_insert(google::protobuf::Map<std::string, ::common::MintCondition> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::MintCondition value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_MintCondition_get(const google::protobuf::Map<std::string, ::common::MintCondition>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::common::MintCondition** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::common::MintCondition* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_MintCondition_remove(google::protobuf::Map<std::string, ::common::MintCondition> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::MintCondition * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_common_MintCondition_iter(
      const google::protobuf::Map<std::string, ::common::MintCondition>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_common_MintCondition_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::common::MintCondition** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::common::MintCondition>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// common.RedeemCondition
        // clang-format off
extern "C" {
void* __rust_proto_thunk__common_RedeemCondition_new() { return new ::common::RedeemCondition(); }
void __rust_proto_thunk__common_RedeemCondition_delete(void* ptr) { delete static_cast<::common::RedeemCondition*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__common_RedeemCondition_serialize(::common::RedeemCondition* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__common_RedeemCondition_parse(::common::RedeemCondition* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__common_RedeemCondition_copy_from(::common::RedeemCondition* dst, const ::common::RedeemCondition* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__common_RedeemCondition_repeated_len(google::protobuf::RepeatedPtrField<::common::RedeemCondition>* field) {
  return field->size();
}
const ::common::RedeemCondition& __rust_proto_thunk__common_RedeemCondition_repeated_get(
  google::protobuf::RepeatedPtrField<::common::RedeemCondition>* field,
  size_t index) {
  return field->Get(index);
}
::common::RedeemCondition* __rust_proto_thunk__common_RedeemCondition_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::common::RedeemCondition>* field,
  size_t index) {
  return field->Mutable(index);
}
::common::RedeemCondition* __rust_proto_thunk__common_RedeemCondition_repeated_add(google::protobuf::RepeatedPtrField<::common::RedeemCondition>* field) {
  return field->Add();
}
void __rust_proto_thunk__common_RedeemCondition_repeated_clear(google::protobuf::RepeatedPtrField<::common::RedeemCondition>* field) {
  field->Clear();
}
void __rust_proto_thunk__common_RedeemCondition_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::common::RedeemCondition>& dst,
  const google::protobuf::RepeatedPtrField<::common::RedeemCondition>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__common_RedeemCondition_get_issuer(::common::RedeemCondition* msg) {
  absl::string_view val = msg->issuer();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__common_RedeemCondition_set_issuer(::common::RedeemCondition* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_issuer(absl::string_view(s.ptr, s.len));
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::common::RedeemCondition>* __rust_proto_thunk__Map_i32_common_RedeemCondition_new() {
    return new google::protobuf::Map<int32_t, ::common::RedeemCondition>();
  }
  void __rust_proto_thunk__Map_i32_common_RedeemCondition_free(const google::protobuf::Map<int32_t, ::common::RedeemCondition>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_common_RedeemCondition_clear(google::protobuf::Map<int32_t, ::common::RedeemCondition> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_common_RedeemCondition_size(const google::protobuf::Map<int32_t, ::common::RedeemCondition>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_common_RedeemCondition_insert(google::protobuf::Map<int32_t, ::common::RedeemCondition> * m,
                          int32_t key, ::common::RedeemCondition value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_RedeemCondition_get(const google::protobuf::Map<int32_t, ::common::RedeemCondition>* m,
                       int32_t key, const ::common::RedeemCondition** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::RedeemCondition* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_RedeemCondition_remove(google::protobuf::Map<int32_t, ::common::RedeemCondition> * m,
                          int32_t key, ::common::RedeemCondition * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_common_RedeemCondition_iter(
      const google::protobuf::Map<int32_t, ::common::RedeemCondition>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_common_RedeemCondition_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::common::RedeemCondition** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::common::RedeemCondition>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::common::RedeemCondition>* __rust_proto_thunk__Map_u32_common_RedeemCondition_new() {
    return new google::protobuf::Map<uint32_t, ::common::RedeemCondition>();
  }
  void __rust_proto_thunk__Map_u32_common_RedeemCondition_free(const google::protobuf::Map<uint32_t, ::common::RedeemCondition>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_common_RedeemCondition_clear(google::protobuf::Map<uint32_t, ::common::RedeemCondition> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_common_RedeemCondition_size(const google::protobuf::Map<uint32_t, ::common::RedeemCondition>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_common_RedeemCondition_insert(google::protobuf::Map<uint32_t, ::common::RedeemCondition> * m,
                          uint32_t key, ::common::RedeemCondition value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_RedeemCondition_get(const google::protobuf::Map<uint32_t, ::common::RedeemCondition>* m,
                       uint32_t key, const ::common::RedeemCondition** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::RedeemCondition* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_RedeemCondition_remove(google::protobuf::Map<uint32_t, ::common::RedeemCondition> * m,
                          uint32_t key, ::common::RedeemCondition * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_common_RedeemCondition_iter(
      const google::protobuf::Map<uint32_t, ::common::RedeemCondition>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_common_RedeemCondition_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::common::RedeemCondition** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::common::RedeemCondition>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::common::RedeemCondition>* __rust_proto_thunk__Map_i64_common_RedeemCondition_new() {
    return new google::protobuf::Map<int64_t, ::common::RedeemCondition>();
  }
  void __rust_proto_thunk__Map_i64_common_RedeemCondition_free(const google::protobuf::Map<int64_t, ::common::RedeemCondition>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_common_RedeemCondition_clear(google::protobuf::Map<int64_t, ::common::RedeemCondition> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_common_RedeemCondition_size(const google::protobuf::Map<int64_t, ::common::RedeemCondition>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_common_RedeemCondition_insert(google::protobuf::Map<int64_t, ::common::RedeemCondition> * m,
                          int64_t key, ::common::RedeemCondition value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_RedeemCondition_get(const google::protobuf::Map<int64_t, ::common::RedeemCondition>* m,
                       int64_t key, const ::common::RedeemCondition** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::RedeemCondition* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_RedeemCondition_remove(google::protobuf::Map<int64_t, ::common::RedeemCondition> * m,
                          int64_t key, ::common::RedeemCondition * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_common_RedeemCondition_iter(
      const google::protobuf::Map<int64_t, ::common::RedeemCondition>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_common_RedeemCondition_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::common::RedeemCondition** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::common::RedeemCondition>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::common::RedeemCondition>* __rust_proto_thunk__Map_u64_common_RedeemCondition_new() {
    return new google::protobuf::Map<uint64_t, ::common::RedeemCondition>();
  }
  void __rust_proto_thunk__Map_u64_common_RedeemCondition_free(const google::protobuf::Map<uint64_t, ::common::RedeemCondition>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_common_RedeemCondition_clear(google::protobuf::Map<uint64_t, ::common::RedeemCondition> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_common_RedeemCondition_size(const google::protobuf::Map<uint64_t, ::common::RedeemCondition>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_common_RedeemCondition_insert(google::protobuf::Map<uint64_t, ::common::RedeemCondition> * m,
                          uint64_t key, ::common::RedeemCondition value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_RedeemCondition_get(const google::protobuf::Map<uint64_t, ::common::RedeemCondition>* m,
                       uint64_t key, const ::common::RedeemCondition** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::RedeemCondition* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_RedeemCondition_remove(google::protobuf::Map<uint64_t, ::common::RedeemCondition> * m,
                          uint64_t key, ::common::RedeemCondition * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_common_RedeemCondition_iter(
      const google::protobuf::Map<uint64_t, ::common::RedeemCondition>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_common_RedeemCondition_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::common::RedeemCondition** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::common::RedeemCondition>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::common::RedeemCondition>* __rust_proto_thunk__Map_bool_common_RedeemCondition_new() {
    return new google::protobuf::Map<bool, ::common::RedeemCondition>();
  }
  void __rust_proto_thunk__Map_bool_common_RedeemCondition_free(const google::protobuf::Map<bool, ::common::RedeemCondition>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_common_RedeemCondition_clear(google::protobuf::Map<bool, ::common::RedeemCondition> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_common_RedeemCondition_size(const google::protobuf::Map<bool, ::common::RedeemCondition>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_common_RedeemCondition_insert(google::protobuf::Map<bool, ::common::RedeemCondition> * m,
                          bool key, ::common::RedeemCondition value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_RedeemCondition_get(const google::protobuf::Map<bool, ::common::RedeemCondition>* m,
                       bool key, const ::common::RedeemCondition** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::RedeemCondition* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_RedeemCondition_remove(google::protobuf::Map<bool, ::common::RedeemCondition> * m,
                          bool key, ::common::RedeemCondition * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_common_RedeemCondition_iter(
      const google::protobuf::Map<bool, ::common::RedeemCondition>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_common_RedeemCondition_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::common::RedeemCondition** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::common::RedeemCondition>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::common::RedeemCondition>* __rust_proto_thunk__Map_ProtoStr_common_RedeemCondition_new() {
    return new google::protobuf::Map<std::string, ::common::RedeemCondition>();
  }
  void __rust_proto_thunk__Map_ProtoStr_common_RedeemCondition_free(const google::protobuf::Map<std::string, ::common::RedeemCondition>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_common_RedeemCondition_clear(google::protobuf::Map<std::string, ::common::RedeemCondition> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_common_RedeemCondition_size(const google::protobuf::Map<std::string, ::common::RedeemCondition>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_RedeemCondition_insert(google::protobuf::Map<std::string, ::common::RedeemCondition> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::RedeemCondition value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_RedeemCondition_get(const google::protobuf::Map<std::string, ::common::RedeemCondition>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::common::RedeemCondition** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::common::RedeemCondition* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_RedeemCondition_remove(google::protobuf::Map<std::string, ::common::RedeemCondition> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::RedeemCondition * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_common_RedeemCondition_iter(
      const google::protobuf::Map<std::string, ::common::RedeemCondition>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_common_RedeemCondition_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::common::RedeemCondition** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::common::RedeemCondition>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// common.Condition
        // clang-format off
extern "C" {
void* __rust_proto_thunk__common_Condition_new() { return new ::common::Condition(); }
void __rust_proto_thunk__common_Condition_delete(void* ptr) { delete static_cast<::common::Condition*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__common_Condition_serialize(::common::Condition* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__common_Condition_parse(::common::Condition* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__common_Condition_copy_from(::common::Condition* dst, const ::common::Condition* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__common_Condition_repeated_len(google::protobuf::RepeatedPtrField<::common::Condition>* field) {
  return field->size();
}
const ::common::Condition& __rust_proto_thunk__common_Condition_repeated_get(
  google::protobuf::RepeatedPtrField<::common::Condition>* field,
  size_t index) {
  return field->Get(index);
}
::common::Condition* __rust_proto_thunk__common_Condition_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::common::Condition>* field,
  size_t index) {
  return field->Mutable(index);
}
::common::Condition* __rust_proto_thunk__common_Condition_repeated_add(google::protobuf::RepeatedPtrField<::common::Condition>* field) {
  return field->Add();
}
void __rust_proto_thunk__common_Condition_repeated_clear(google::protobuf::RepeatedPtrField<::common::Condition>* field) {
  field->Clear();
}
void __rust_proto_thunk__common_Condition_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::common::Condition>& dst,
  const google::protobuf::RepeatedPtrField<::common::Condition>& src) {
  dst = src;
}

const void* __rust_proto_thunk__common_Condition_get_timeout(::common::Condition* msg) {
  return static_cast<const void*>(&msg->timeout());
}
void* __rust_proto_thunk__common_Condition_get_mut_timeout(::common::Condition* msg) {
  return static_cast<void*>(msg->mutable_timeout());
}
void __rust_proto_thunk__common_Condition_clear_timeout(::common::Condition* msg) { msg->clear_timeout(); }
bool __rust_proto_thunk__common_Condition_has_timeout(::common::Condition* msg) { return msg->has_timeout(); }

const void* __rust_proto_thunk__common_Condition_get_hash(::common::Condition* msg) {
  return static_cast<const void*>(&msg->hash());
}
void* __rust_proto_thunk__common_Condition_get_mut_hash(::common::Condition* msg) {
  return static_cast<void*>(msg->mutable_hash());
}
void __rust_proto_thunk__common_Condition_clear_hash(::common::Condition* msg) { msg->clear_hash(); }
bool __rust_proto_thunk__common_Condition_has_hash(::common::Condition* msg) { return msg->has_hash(); }

const void* __rust_proto_thunk__common_Condition_get_mint(::common::Condition* msg) {
  return static_cast<const void*>(&msg->mint());
}
void* __rust_proto_thunk__common_Condition_get_mut_mint(::common::Condition* msg) {
  return static_cast<void*>(msg->mutable_mint());
}
void __rust_proto_thunk__common_Condition_clear_mint(::common::Condition* msg) { msg->clear_mint(); }
bool __rust_proto_thunk__common_Condition_has_mint(::common::Condition* msg) { return msg->has_mint(); }

const void* __rust_proto_thunk__common_Condition_get_redeem(::common::Condition* msg) {
  return static_cast<const void*>(&msg->redeem());
}
void* __rust_proto_thunk__common_Condition_get_mut_redeem(::common::Condition* msg) {
  return static_cast<void*>(msg->mutable_redeem());
}
void __rust_proto_thunk__common_Condition_clear_redeem(::common::Condition* msg) { msg->clear_redeem(); }
bool __rust_proto_thunk__common_Condition_has_redeem(::common::Condition* msg) { return msg->has_redeem(); }


::common::Condition::ConditionCase __rust_proto_thunk__common_Condition_case_condition(::common::Condition* msg) {
  return msg->condition_case();
}
}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::common::Condition>* __rust_proto_thunk__Map_i32_common_Condition_new() {
    return new google::protobuf::Map<int32_t, ::common::Condition>();
  }
  void __rust_proto_thunk__Map_i32_common_Condition_free(const google::protobuf::Map<int32_t, ::common::Condition>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_common_Condition_clear(google::protobuf::Map<int32_t, ::common::Condition> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_common_Condition_size(const google::protobuf::Map<int32_t, ::common::Condition>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_common_Condition_insert(google::protobuf::Map<int32_t, ::common::Condition> * m,
                          int32_t key, ::common::Condition value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_Condition_get(const google::protobuf::Map<int32_t, ::common::Condition>* m,
                       int32_t key, const ::common::Condition** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::Condition* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_Condition_remove(google::protobuf::Map<int32_t, ::common::Condition> * m,
                          int32_t key, ::common::Condition * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_common_Condition_iter(
      const google::protobuf::Map<int32_t, ::common::Condition>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_common_Condition_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::common::Condition** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::common::Condition>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::common::Condition>* __rust_proto_thunk__Map_u32_common_Condition_new() {
    return new google::protobuf::Map<uint32_t, ::common::Condition>();
  }
  void __rust_proto_thunk__Map_u32_common_Condition_free(const google::protobuf::Map<uint32_t, ::common::Condition>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_common_Condition_clear(google::protobuf::Map<uint32_t, ::common::Condition> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_common_Condition_size(const google::protobuf::Map<uint32_t, ::common::Condition>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_common_Condition_insert(google::protobuf::Map<uint32_t, ::common::Condition> * m,
                          uint32_t key, ::common::Condition value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_Condition_get(const google::protobuf::Map<uint32_t, ::common::Condition>* m,
                       uint32_t key, const ::common::Condition** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::Condition* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_Condition_remove(google::protobuf::Map<uint32_t, ::common::Condition> * m,
                          uint32_t key, ::common::Condition * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_common_Condition_iter(
      const google::protobuf::Map<uint32_t, ::common::Condition>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_common_Condition_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::common::Condition** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::common::Condition>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::common::Condition>* __rust_proto_thunk__Map_i64_common_Condition_new() {
    return new google::protobuf::Map<int64_t, ::common::Condition>();
  }
  void __rust_proto_thunk__Map_i64_common_Condition_free(const google::protobuf::Map<int64_t, ::common::Condition>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_common_Condition_clear(google::protobuf::Map<int64_t, ::common::Condition> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_common_Condition_size(const google::protobuf::Map<int64_t, ::common::Condition>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_common_Condition_insert(google::protobuf::Map<int64_t, ::common::Condition> * m,
                          int64_t key, ::common::Condition value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_Condition_get(const google::protobuf::Map<int64_t, ::common::Condition>* m,
                       int64_t key, const ::common::Condition** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::Condition* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_Condition_remove(google::protobuf::Map<int64_t, ::common::Condition> * m,
                          int64_t key, ::common::Condition * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_common_Condition_iter(
      const google::protobuf::Map<int64_t, ::common::Condition>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_common_Condition_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::common::Condition** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::common::Condition>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::common::Condition>* __rust_proto_thunk__Map_u64_common_Condition_new() {
    return new google::protobuf::Map<uint64_t, ::common::Condition>();
  }
  void __rust_proto_thunk__Map_u64_common_Condition_free(const google::protobuf::Map<uint64_t, ::common::Condition>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_common_Condition_clear(google::protobuf::Map<uint64_t, ::common::Condition> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_common_Condition_size(const google::protobuf::Map<uint64_t, ::common::Condition>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_common_Condition_insert(google::protobuf::Map<uint64_t, ::common::Condition> * m,
                          uint64_t key, ::common::Condition value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_Condition_get(const google::protobuf::Map<uint64_t, ::common::Condition>* m,
                       uint64_t key, const ::common::Condition** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::Condition* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_Condition_remove(google::protobuf::Map<uint64_t, ::common::Condition> * m,
                          uint64_t key, ::common::Condition * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_common_Condition_iter(
      const google::protobuf::Map<uint64_t, ::common::Condition>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_common_Condition_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::common::Condition** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::common::Condition>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::common::Condition>* __rust_proto_thunk__Map_bool_common_Condition_new() {
    return new google::protobuf::Map<bool, ::common::Condition>();
  }
  void __rust_proto_thunk__Map_bool_common_Condition_free(const google::protobuf::Map<bool, ::common::Condition>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_common_Condition_clear(google::protobuf::Map<bool, ::common::Condition> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_common_Condition_size(const google::protobuf::Map<bool, ::common::Condition>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_common_Condition_insert(google::protobuf::Map<bool, ::common::Condition> * m,
                          bool key, ::common::Condition value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_Condition_get(const google::protobuf::Map<bool, ::common::Condition>* m,
                       bool key, const ::common::Condition** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::Condition* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_Condition_remove(google::protobuf::Map<bool, ::common::Condition> * m,
                          bool key, ::common::Condition * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_common_Condition_iter(
      const google::protobuf::Map<bool, ::common::Condition>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_common_Condition_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::common::Condition** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::common::Condition>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::common::Condition>* __rust_proto_thunk__Map_ProtoStr_common_Condition_new() {
    return new google::protobuf::Map<std::string, ::common::Condition>();
  }
  void __rust_proto_thunk__Map_ProtoStr_common_Condition_free(const google::protobuf::Map<std::string, ::common::Condition>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_common_Condition_clear(google::protobuf::Map<std::string, ::common::Condition> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_common_Condition_size(const google::protobuf::Map<std::string, ::common::Condition>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_Condition_insert(google::protobuf::Map<std::string, ::common::Condition> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::Condition value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_Condition_get(const google::protobuf::Map<std::string, ::common::Condition>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::common::Condition** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::common::Condition* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_Condition_remove(google::protobuf::Map<std::string, ::common::Condition> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::Condition * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_common_Condition_iter(
      const google::protobuf::Map<std::string, ::common::Condition>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_common_Condition_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::common::Condition** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::common::Condition>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// common.Contract
        // clang-format off
extern "C" {
void* __rust_proto_thunk__common_Contract_new() { return new ::common::Contract(); }
void __rust_proto_thunk__common_Contract_delete(void* ptr) { delete static_cast<::common::Contract*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__common_Contract_serialize(::common::Contract* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__common_Contract_parse(::common::Contract* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__common_Contract_copy_from(::common::Contract* dst, const ::common::Contract* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__common_Contract_repeated_len(google::protobuf::RepeatedPtrField<::common::Contract>* field) {
  return field->size();
}
const ::common::Contract& __rust_proto_thunk__common_Contract_repeated_get(
  google::protobuf::RepeatedPtrField<::common::Contract>* field,
  size_t index) {
  return field->Get(index);
}
::common::Contract* __rust_proto_thunk__common_Contract_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::common::Contract>* field,
  size_t index) {
  return field->Mutable(index);
}
::common::Contract* __rust_proto_thunk__common_Contract_repeated_add(google::protobuf::RepeatedPtrField<::common::Contract>* field) {
  return field->Add();
}
void __rust_proto_thunk__common_Contract_repeated_clear(google::protobuf::RepeatedPtrField<::common::Contract>* field) {
  field->Clear();
}
void __rust_proto_thunk__common_Contract_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::common::Contract>& dst,
  const google::protobuf::RepeatedPtrField<::common::Contract>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__common_Contract_get_urn(::common::Contract* msg) {
  absl::string_view val = msg->urn();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__common_Contract_set_urn(::common::Contract* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_urn(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__common_Contract_get_id(::common::Contract* msg) {
  absl::string_view val = msg->id();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__common_Contract_set_id(::common::Contract* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_id(absl::string_view(s.ptr, s.len));
}

void __rust_proto_thunk__common_Contract_clear_commitments(::common::Contract* msg) {
  msg->clear_commitments();
}
google::protobuf::RepeatedPtrField<::common::Commitment>* __rust_proto_thunk__common_Contract_get_mut_commitments(::common::Contract* msg) {
  return msg->mutable_commitments();
}
const google::protobuf::RepeatedPtrField<::common::Commitment>* __rust_proto_thunk__common_Contract_get_commitments(
    const ::common::Contract* msg) {
  return &msg->commitments();
}

void __rust_proto_thunk__common_Contract_clear_conditions(::common::Contract* msg) {
  msg->clear_conditions();
}
google::protobuf::RepeatedPtrField<::common::Condition>* __rust_proto_thunk__common_Contract_get_mut_conditions(::common::Contract* msg) {
  return msg->mutable_conditions();
}
const google::protobuf::RepeatedPtrField<::common::Condition>* __rust_proto_thunk__common_Contract_get_conditions(
    const ::common::Contract* msg) {
  return &msg->conditions();
}

const void* __rust_proto_thunk__common_Contract_get_signatories(const ::common::Contract* msg) {
  return &msg->signatories();
}
void* __rust_proto_thunk__common_Contract_get_mut_signatories(::common::Contract* msg) { return msg->mutable_signatories(); }

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__common_Contract_get_memo(::common::Contract* msg) {
  absl::string_view val = msg->memo();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__common_Contract_set_memo(::common::Contract* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_memo(absl::string_view(s.ptr, s.len));
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::common::Contract>* __rust_proto_thunk__Map_i32_common_Contract_new() {
    return new google::protobuf::Map<int32_t, ::common::Contract>();
  }
  void __rust_proto_thunk__Map_i32_common_Contract_free(const google::protobuf::Map<int32_t, ::common::Contract>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_common_Contract_clear(google::protobuf::Map<int32_t, ::common::Contract> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_common_Contract_size(const google::protobuf::Map<int32_t, ::common::Contract>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_common_Contract_insert(google::protobuf::Map<int32_t, ::common::Contract> * m,
                          int32_t key, ::common::Contract value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_Contract_get(const google::protobuf::Map<int32_t, ::common::Contract>* m,
                       int32_t key, const ::common::Contract** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::Contract* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_Contract_remove(google::protobuf::Map<int32_t, ::common::Contract> * m,
                          int32_t key, ::common::Contract * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_common_Contract_iter(
      const google::protobuf::Map<int32_t, ::common::Contract>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_common_Contract_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::common::Contract** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::common::Contract>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::common::Contract>* __rust_proto_thunk__Map_u32_common_Contract_new() {
    return new google::protobuf::Map<uint32_t, ::common::Contract>();
  }
  void __rust_proto_thunk__Map_u32_common_Contract_free(const google::protobuf::Map<uint32_t, ::common::Contract>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_common_Contract_clear(google::protobuf::Map<uint32_t, ::common::Contract> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_common_Contract_size(const google::protobuf::Map<uint32_t, ::common::Contract>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_common_Contract_insert(google::protobuf::Map<uint32_t, ::common::Contract> * m,
                          uint32_t key, ::common::Contract value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_Contract_get(const google::protobuf::Map<uint32_t, ::common::Contract>* m,
                       uint32_t key, const ::common::Contract** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::Contract* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_Contract_remove(google::protobuf::Map<uint32_t, ::common::Contract> * m,
                          uint32_t key, ::common::Contract * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_common_Contract_iter(
      const google::protobuf::Map<uint32_t, ::common::Contract>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_common_Contract_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::common::Contract** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::common::Contract>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::common::Contract>* __rust_proto_thunk__Map_i64_common_Contract_new() {
    return new google::protobuf::Map<int64_t, ::common::Contract>();
  }
  void __rust_proto_thunk__Map_i64_common_Contract_free(const google::protobuf::Map<int64_t, ::common::Contract>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_common_Contract_clear(google::protobuf::Map<int64_t, ::common::Contract> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_common_Contract_size(const google::protobuf::Map<int64_t, ::common::Contract>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_common_Contract_insert(google::protobuf::Map<int64_t, ::common::Contract> * m,
                          int64_t key, ::common::Contract value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_Contract_get(const google::protobuf::Map<int64_t, ::common::Contract>* m,
                       int64_t key, const ::common::Contract** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::Contract* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_Contract_remove(google::protobuf::Map<int64_t, ::common::Contract> * m,
                          int64_t key, ::common::Contract * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_common_Contract_iter(
      const google::protobuf::Map<int64_t, ::common::Contract>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_common_Contract_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::common::Contract** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::common::Contract>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::common::Contract>* __rust_proto_thunk__Map_u64_common_Contract_new() {
    return new google::protobuf::Map<uint64_t, ::common::Contract>();
  }
  void __rust_proto_thunk__Map_u64_common_Contract_free(const google::protobuf::Map<uint64_t, ::common::Contract>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_common_Contract_clear(google::protobuf::Map<uint64_t, ::common::Contract> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_common_Contract_size(const google::protobuf::Map<uint64_t, ::common::Contract>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_common_Contract_insert(google::protobuf::Map<uint64_t, ::common::Contract> * m,
                          uint64_t key, ::common::Contract value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_Contract_get(const google::protobuf::Map<uint64_t, ::common::Contract>* m,
                       uint64_t key, const ::common::Contract** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::Contract* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_Contract_remove(google::protobuf::Map<uint64_t, ::common::Contract> * m,
                          uint64_t key, ::common::Contract * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_common_Contract_iter(
      const google::protobuf::Map<uint64_t, ::common::Contract>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_common_Contract_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::common::Contract** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::common::Contract>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::common::Contract>* __rust_proto_thunk__Map_bool_common_Contract_new() {
    return new google::protobuf::Map<bool, ::common::Contract>();
  }
  void __rust_proto_thunk__Map_bool_common_Contract_free(const google::protobuf::Map<bool, ::common::Contract>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_common_Contract_clear(google::protobuf::Map<bool, ::common::Contract> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_common_Contract_size(const google::protobuf::Map<bool, ::common::Contract>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_common_Contract_insert(google::protobuf::Map<bool, ::common::Contract> * m,
                          bool key, ::common::Contract value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_Contract_get(const google::protobuf::Map<bool, ::common::Contract>* m,
                       bool key, const ::common::Contract** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::Contract* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_Contract_remove(google::protobuf::Map<bool, ::common::Contract> * m,
                          bool key, ::common::Contract * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_common_Contract_iter(
      const google::protobuf::Map<bool, ::common::Contract>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_common_Contract_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::common::Contract** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::common::Contract>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::common::Contract>* __rust_proto_thunk__Map_ProtoStr_common_Contract_new() {
    return new google::protobuf::Map<std::string, ::common::Contract>();
  }
  void __rust_proto_thunk__Map_ProtoStr_common_Contract_free(const google::protobuf::Map<std::string, ::common::Contract>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_common_Contract_clear(google::protobuf::Map<std::string, ::common::Contract> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_common_Contract_size(const google::protobuf::Map<std::string, ::common::Contract>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_Contract_insert(google::protobuf::Map<std::string, ::common::Contract> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::Contract value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_Contract_get(const google::protobuf::Map<std::string, ::common::Contract>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::common::Contract** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::common::Contract* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_Contract_remove(google::protobuf::Map<std::string, ::common::Contract> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::Contract * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_common_Contract_iter(
      const google::protobuf::Map<std::string, ::common::Contract>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_common_Contract_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::common::Contract** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::common::Contract>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// common.AssociatedUetrConnection
        // clang-format off
extern "C" {
void* __rust_proto_thunk__common_AssociatedUetrConnection_new() { return new ::common::AssociatedUetrConnection(); }
void __rust_proto_thunk__common_AssociatedUetrConnection_delete(void* ptr) { delete static_cast<::common::AssociatedUetrConnection*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__common_AssociatedUetrConnection_serialize(::common::AssociatedUetrConnection* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__common_AssociatedUetrConnection_parse(::common::AssociatedUetrConnection* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__common_AssociatedUetrConnection_copy_from(::common::AssociatedUetrConnection* dst, const ::common::AssociatedUetrConnection* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__common_AssociatedUetrConnection_repeated_len(google::protobuf::RepeatedPtrField<::common::AssociatedUetrConnection>* field) {
  return field->size();
}
const ::common::AssociatedUetrConnection& __rust_proto_thunk__common_AssociatedUetrConnection_repeated_get(
  google::protobuf::RepeatedPtrField<::common::AssociatedUetrConnection>* field,
  size_t index) {
  return field->Get(index);
}
::common::AssociatedUetrConnection* __rust_proto_thunk__common_AssociatedUetrConnection_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::common::AssociatedUetrConnection>* field,
  size_t index) {
  return field->Mutable(index);
}
::common::AssociatedUetrConnection* __rust_proto_thunk__common_AssociatedUetrConnection_repeated_add(google::protobuf::RepeatedPtrField<::common::AssociatedUetrConnection>* field) {
  return field->Add();
}
void __rust_proto_thunk__common_AssociatedUetrConnection_repeated_clear(google::protobuf::RepeatedPtrField<::common::AssociatedUetrConnection>* field) {
  field->Clear();
}
void __rust_proto_thunk__common_AssociatedUetrConnection_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::common::AssociatedUetrConnection>& dst,
  const google::protobuf::RepeatedPtrField<::common::AssociatedUetrConnection>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__common_AssociatedUetrConnection_get_uetr(::common::AssociatedUetrConnection* msg) {
  absl::string_view val = msg->uetr();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__common_AssociatedUetrConnection_set_uetr(::common::AssociatedUetrConnection* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_uetr(absl::string_view(s.ptr, s.len));
}

const void* __rust_proto_thunk__common_AssociatedUetrConnection_get_packet(::common::AssociatedUetrConnection* msg) {
  return static_cast<const void*>(&msg->packet());
}
void* __rust_proto_thunk__common_AssociatedUetrConnection_get_mut_packet(::common::AssociatedUetrConnection* msg) {
  return static_cast<void*>(msg->mutable_packet());
}
void __rust_proto_thunk__common_AssociatedUetrConnection_clear_packet(::common::AssociatedUetrConnection* msg) { msg->clear_packet(); }
bool __rust_proto_thunk__common_AssociatedUetrConnection_has_packet(::common::AssociatedUetrConnection* msg) { return msg->has_packet(); }


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::common::AssociatedUetrConnection>* __rust_proto_thunk__Map_i32_common_AssociatedUetrConnection_new() {
    return new google::protobuf::Map<int32_t, ::common::AssociatedUetrConnection>();
  }
  void __rust_proto_thunk__Map_i32_common_AssociatedUetrConnection_free(const google::protobuf::Map<int32_t, ::common::AssociatedUetrConnection>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_common_AssociatedUetrConnection_clear(google::protobuf::Map<int32_t, ::common::AssociatedUetrConnection> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_common_AssociatedUetrConnection_size(const google::protobuf::Map<int32_t, ::common::AssociatedUetrConnection>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_common_AssociatedUetrConnection_insert(google::protobuf::Map<int32_t, ::common::AssociatedUetrConnection> * m,
                          int32_t key, ::common::AssociatedUetrConnection value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_AssociatedUetrConnection_get(const google::protobuf::Map<int32_t, ::common::AssociatedUetrConnection>* m,
                       int32_t key, const ::common::AssociatedUetrConnection** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::AssociatedUetrConnection* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_common_AssociatedUetrConnection_remove(google::protobuf::Map<int32_t, ::common::AssociatedUetrConnection> * m,
                          int32_t key, ::common::AssociatedUetrConnection * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_common_AssociatedUetrConnection_iter(
      const google::protobuf::Map<int32_t, ::common::AssociatedUetrConnection>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_common_AssociatedUetrConnection_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::common::AssociatedUetrConnection** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::common::AssociatedUetrConnection>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::common::AssociatedUetrConnection>* __rust_proto_thunk__Map_u32_common_AssociatedUetrConnection_new() {
    return new google::protobuf::Map<uint32_t, ::common::AssociatedUetrConnection>();
  }
  void __rust_proto_thunk__Map_u32_common_AssociatedUetrConnection_free(const google::protobuf::Map<uint32_t, ::common::AssociatedUetrConnection>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_common_AssociatedUetrConnection_clear(google::protobuf::Map<uint32_t, ::common::AssociatedUetrConnection> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_common_AssociatedUetrConnection_size(const google::protobuf::Map<uint32_t, ::common::AssociatedUetrConnection>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_common_AssociatedUetrConnection_insert(google::protobuf::Map<uint32_t, ::common::AssociatedUetrConnection> * m,
                          uint32_t key, ::common::AssociatedUetrConnection value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_AssociatedUetrConnection_get(const google::protobuf::Map<uint32_t, ::common::AssociatedUetrConnection>* m,
                       uint32_t key, const ::common::AssociatedUetrConnection** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::AssociatedUetrConnection* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_common_AssociatedUetrConnection_remove(google::protobuf::Map<uint32_t, ::common::AssociatedUetrConnection> * m,
                          uint32_t key, ::common::AssociatedUetrConnection * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_common_AssociatedUetrConnection_iter(
      const google::protobuf::Map<uint32_t, ::common::AssociatedUetrConnection>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_common_AssociatedUetrConnection_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::common::AssociatedUetrConnection** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::common::AssociatedUetrConnection>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::common::AssociatedUetrConnection>* __rust_proto_thunk__Map_i64_common_AssociatedUetrConnection_new() {
    return new google::protobuf::Map<int64_t, ::common::AssociatedUetrConnection>();
  }
  void __rust_proto_thunk__Map_i64_common_AssociatedUetrConnection_free(const google::protobuf::Map<int64_t, ::common::AssociatedUetrConnection>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_common_AssociatedUetrConnection_clear(google::protobuf::Map<int64_t, ::common::AssociatedUetrConnection> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_common_AssociatedUetrConnection_size(const google::protobuf::Map<int64_t, ::common::AssociatedUetrConnection>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_common_AssociatedUetrConnection_insert(google::protobuf::Map<int64_t, ::common::AssociatedUetrConnection> * m,
                          int64_t key, ::common::AssociatedUetrConnection value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_AssociatedUetrConnection_get(const google::protobuf::Map<int64_t, ::common::AssociatedUetrConnection>* m,
                       int64_t key, const ::common::AssociatedUetrConnection** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::AssociatedUetrConnection* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_common_AssociatedUetrConnection_remove(google::protobuf::Map<int64_t, ::common::AssociatedUetrConnection> * m,
                          int64_t key, ::common::AssociatedUetrConnection * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_common_AssociatedUetrConnection_iter(
      const google::protobuf::Map<int64_t, ::common::AssociatedUetrConnection>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_common_AssociatedUetrConnection_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::common::AssociatedUetrConnection** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::common::AssociatedUetrConnection>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::common::AssociatedUetrConnection>* __rust_proto_thunk__Map_u64_common_AssociatedUetrConnection_new() {
    return new google::protobuf::Map<uint64_t, ::common::AssociatedUetrConnection>();
  }
  void __rust_proto_thunk__Map_u64_common_AssociatedUetrConnection_free(const google::protobuf::Map<uint64_t, ::common::AssociatedUetrConnection>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_common_AssociatedUetrConnection_clear(google::protobuf::Map<uint64_t, ::common::AssociatedUetrConnection> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_common_AssociatedUetrConnection_size(const google::protobuf::Map<uint64_t, ::common::AssociatedUetrConnection>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_common_AssociatedUetrConnection_insert(google::protobuf::Map<uint64_t, ::common::AssociatedUetrConnection> * m,
                          uint64_t key, ::common::AssociatedUetrConnection value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_AssociatedUetrConnection_get(const google::protobuf::Map<uint64_t, ::common::AssociatedUetrConnection>* m,
                       uint64_t key, const ::common::AssociatedUetrConnection** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::AssociatedUetrConnection* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_common_AssociatedUetrConnection_remove(google::protobuf::Map<uint64_t, ::common::AssociatedUetrConnection> * m,
                          uint64_t key, ::common::AssociatedUetrConnection * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_common_AssociatedUetrConnection_iter(
      const google::protobuf::Map<uint64_t, ::common::AssociatedUetrConnection>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_common_AssociatedUetrConnection_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::common::AssociatedUetrConnection** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::common::AssociatedUetrConnection>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::common::AssociatedUetrConnection>* __rust_proto_thunk__Map_bool_common_AssociatedUetrConnection_new() {
    return new google::protobuf::Map<bool, ::common::AssociatedUetrConnection>();
  }
  void __rust_proto_thunk__Map_bool_common_AssociatedUetrConnection_free(const google::protobuf::Map<bool, ::common::AssociatedUetrConnection>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_common_AssociatedUetrConnection_clear(google::protobuf::Map<bool, ::common::AssociatedUetrConnection> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_common_AssociatedUetrConnection_size(const google::protobuf::Map<bool, ::common::AssociatedUetrConnection>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_common_AssociatedUetrConnection_insert(google::protobuf::Map<bool, ::common::AssociatedUetrConnection> * m,
                          bool key, ::common::AssociatedUetrConnection value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_AssociatedUetrConnection_get(const google::protobuf::Map<bool, ::common::AssociatedUetrConnection>* m,
                       bool key, const ::common::AssociatedUetrConnection** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::common::AssociatedUetrConnection* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_common_AssociatedUetrConnection_remove(google::protobuf::Map<bool, ::common::AssociatedUetrConnection> * m,
                          bool key, ::common::AssociatedUetrConnection * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_common_AssociatedUetrConnection_iter(
      const google::protobuf::Map<bool, ::common::AssociatedUetrConnection>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_common_AssociatedUetrConnection_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::common::AssociatedUetrConnection** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::common::AssociatedUetrConnection>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::common::AssociatedUetrConnection>* __rust_proto_thunk__Map_ProtoStr_common_AssociatedUetrConnection_new() {
    return new google::protobuf::Map<std::string, ::common::AssociatedUetrConnection>();
  }
  void __rust_proto_thunk__Map_ProtoStr_common_AssociatedUetrConnection_free(const google::protobuf::Map<std::string, ::common::AssociatedUetrConnection>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_common_AssociatedUetrConnection_clear(google::protobuf::Map<std::string, ::common::AssociatedUetrConnection> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_common_AssociatedUetrConnection_size(const google::protobuf::Map<std::string, ::common::AssociatedUetrConnection>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_AssociatedUetrConnection_insert(google::protobuf::Map<std::string, ::common::AssociatedUetrConnection> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::AssociatedUetrConnection value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_AssociatedUetrConnection_get(const google::protobuf::Map<std::string, ::common::AssociatedUetrConnection>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::common::AssociatedUetrConnection** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::common::AssociatedUetrConnection* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_common_AssociatedUetrConnection_remove(google::protobuf::Map<std::string, ::common::AssociatedUetrConnection> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::common::AssociatedUetrConnection * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_common_AssociatedUetrConnection_iter(
      const google::protobuf::Map<std::string, ::common::AssociatedUetrConnection>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_common_AssociatedUetrConnection_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::common::AssociatedUetrConnection** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::common::AssociatedUetrConnection>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// common.PacketType
extern "C" {
  __PB_RUST_EXPOSE_SCALAR_MAP_METHODS_FOR_VALUE_TYPE(
      ::common::PacketType, common_PacketType, ::common::PacketType, value, cpp_value)
}

// common.HashAlgorithm
extern "C" {
  __PB_RUST_EXPOSE_SCALAR_MAP_METHODS_FOR_VALUE_TYPE(
      ::common::HashAlgorithm, common_HashAlgorithm, ::common::HashAlgorithm, value, cpp_value)
}

// common.ContractType
extern "C" {
  __PB_RUST_EXPOSE_SCALAR_MAP_METHODS_FOR_VALUE_TYPE(
      ::common::ContractType, common_ContractType, ::common::ContractType, value, cpp_value)
}

