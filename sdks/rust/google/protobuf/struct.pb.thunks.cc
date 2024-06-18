#include "google/protobuf/struct.proto.h"
#include "google/protobuf/rust/cpp_kernel/cpp_api.h"
// google.protobuf.Struct
        // clang-format off
extern "C" {
void* __rust_proto_thunk__google_protobuf_Struct_new() { return new ::google::protobuf::Struct(); }
void __rust_proto_thunk__google_protobuf_Struct_delete(void* ptr) { delete static_cast<::google::protobuf::Struct*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__google_protobuf_Struct_serialize(::google::protobuf::Struct* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__google_protobuf_Struct_parse(::google::protobuf::Struct* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__google_protobuf_Struct_copy_from(::google::protobuf::Struct* dst, const ::google::protobuf::Struct* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__google_protobuf_Struct_repeated_len(google::protobuf::RepeatedPtrField<::google::protobuf::Struct>* field) {
  return field->size();
}
const ::google::protobuf::Struct& __rust_proto_thunk__google_protobuf_Struct_repeated_get(
  google::protobuf::RepeatedPtrField<::google::protobuf::Struct>* field,
  size_t index) {
  return field->Get(index);
}
::google::protobuf::Struct* __rust_proto_thunk__google_protobuf_Struct_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::google::protobuf::Struct>* field,
  size_t index) {
  return field->Mutable(index);
}
::google::protobuf::Struct* __rust_proto_thunk__google_protobuf_Struct_repeated_add(google::protobuf::RepeatedPtrField<::google::protobuf::Struct>* field) {
  return field->Add();
}
void __rust_proto_thunk__google_protobuf_Struct_repeated_clear(google::protobuf::RepeatedPtrField<::google::protobuf::Struct>* field) {
  field->Clear();
}
void __rust_proto_thunk__google_protobuf_Struct_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::google::protobuf::Struct>& dst,
  const google::protobuf::RepeatedPtrField<::google::protobuf::Struct>& src) {
  dst = src;
}

const void* __rust_proto_thunk__google_protobuf_Struct_get_fields(const ::google::protobuf::Struct* msg) {
  return &msg->fields();
}
void* __rust_proto_thunk__google_protobuf_Struct_get_mut_fields(::google::protobuf::Struct* msg) { return msg->mutable_fields(); }


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::google::protobuf::Struct>* __rust_proto_thunk__Map_i32_google_protobuf_Struct_new() {
    return new google::protobuf::Map<int32_t, ::google::protobuf::Struct>();
  }
  void __rust_proto_thunk__Map_i32_google_protobuf_Struct_free(const google::protobuf::Map<int32_t, ::google::protobuf::Struct>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_google_protobuf_Struct_clear(google::protobuf::Map<int32_t, ::google::protobuf::Struct> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_google_protobuf_Struct_size(const google::protobuf::Map<int32_t, ::google::protobuf::Struct>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_google_protobuf_Struct_insert(google::protobuf::Map<int32_t, ::google::protobuf::Struct> * m,
                          int32_t key, ::google::protobuf::Struct value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_google_protobuf_Struct_get(const google::protobuf::Map<int32_t, ::google::protobuf::Struct>* m,
                       int32_t key, const ::google::protobuf::Struct** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::google::protobuf::Struct* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_google_protobuf_Struct_remove(google::protobuf::Map<int32_t, ::google::protobuf::Struct> * m,
                          int32_t key, ::google::protobuf::Struct * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_google_protobuf_Struct_iter(
      const google::protobuf::Map<int32_t, ::google::protobuf::Struct>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_google_protobuf_Struct_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::google::protobuf::Struct** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::google::protobuf::Struct>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::google::protobuf::Struct>* __rust_proto_thunk__Map_u32_google_protobuf_Struct_new() {
    return new google::protobuf::Map<uint32_t, ::google::protobuf::Struct>();
  }
  void __rust_proto_thunk__Map_u32_google_protobuf_Struct_free(const google::protobuf::Map<uint32_t, ::google::protobuf::Struct>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_google_protobuf_Struct_clear(google::protobuf::Map<uint32_t, ::google::protobuf::Struct> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_google_protobuf_Struct_size(const google::protobuf::Map<uint32_t, ::google::protobuf::Struct>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_google_protobuf_Struct_insert(google::protobuf::Map<uint32_t, ::google::protobuf::Struct> * m,
                          uint32_t key, ::google::protobuf::Struct value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_google_protobuf_Struct_get(const google::protobuf::Map<uint32_t, ::google::protobuf::Struct>* m,
                       uint32_t key, const ::google::protobuf::Struct** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::google::protobuf::Struct* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_google_protobuf_Struct_remove(google::protobuf::Map<uint32_t, ::google::protobuf::Struct> * m,
                          uint32_t key, ::google::protobuf::Struct * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_google_protobuf_Struct_iter(
      const google::protobuf::Map<uint32_t, ::google::protobuf::Struct>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_google_protobuf_Struct_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::google::protobuf::Struct** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::google::protobuf::Struct>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::google::protobuf::Struct>* __rust_proto_thunk__Map_i64_google_protobuf_Struct_new() {
    return new google::protobuf::Map<int64_t, ::google::protobuf::Struct>();
  }
  void __rust_proto_thunk__Map_i64_google_protobuf_Struct_free(const google::protobuf::Map<int64_t, ::google::protobuf::Struct>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_google_protobuf_Struct_clear(google::protobuf::Map<int64_t, ::google::protobuf::Struct> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_google_protobuf_Struct_size(const google::protobuf::Map<int64_t, ::google::protobuf::Struct>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_google_protobuf_Struct_insert(google::protobuf::Map<int64_t, ::google::protobuf::Struct> * m,
                          int64_t key, ::google::protobuf::Struct value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_google_protobuf_Struct_get(const google::protobuf::Map<int64_t, ::google::protobuf::Struct>* m,
                       int64_t key, const ::google::protobuf::Struct** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::google::protobuf::Struct* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_google_protobuf_Struct_remove(google::protobuf::Map<int64_t, ::google::protobuf::Struct> * m,
                          int64_t key, ::google::protobuf::Struct * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_google_protobuf_Struct_iter(
      const google::protobuf::Map<int64_t, ::google::protobuf::Struct>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_google_protobuf_Struct_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::google::protobuf::Struct** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::google::protobuf::Struct>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::google::protobuf::Struct>* __rust_proto_thunk__Map_u64_google_protobuf_Struct_new() {
    return new google::protobuf::Map<uint64_t, ::google::protobuf::Struct>();
  }
  void __rust_proto_thunk__Map_u64_google_protobuf_Struct_free(const google::protobuf::Map<uint64_t, ::google::protobuf::Struct>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_google_protobuf_Struct_clear(google::protobuf::Map<uint64_t, ::google::protobuf::Struct> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_google_protobuf_Struct_size(const google::protobuf::Map<uint64_t, ::google::protobuf::Struct>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_google_protobuf_Struct_insert(google::protobuf::Map<uint64_t, ::google::protobuf::Struct> * m,
                          uint64_t key, ::google::protobuf::Struct value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_google_protobuf_Struct_get(const google::protobuf::Map<uint64_t, ::google::protobuf::Struct>* m,
                       uint64_t key, const ::google::protobuf::Struct** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::google::protobuf::Struct* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_google_protobuf_Struct_remove(google::protobuf::Map<uint64_t, ::google::protobuf::Struct> * m,
                          uint64_t key, ::google::protobuf::Struct * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_google_protobuf_Struct_iter(
      const google::protobuf::Map<uint64_t, ::google::protobuf::Struct>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_google_protobuf_Struct_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::google::protobuf::Struct** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::google::protobuf::Struct>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::google::protobuf::Struct>* __rust_proto_thunk__Map_bool_google_protobuf_Struct_new() {
    return new google::protobuf::Map<bool, ::google::protobuf::Struct>();
  }
  void __rust_proto_thunk__Map_bool_google_protobuf_Struct_free(const google::protobuf::Map<bool, ::google::protobuf::Struct>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_google_protobuf_Struct_clear(google::protobuf::Map<bool, ::google::protobuf::Struct> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_google_protobuf_Struct_size(const google::protobuf::Map<bool, ::google::protobuf::Struct>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_google_protobuf_Struct_insert(google::protobuf::Map<bool, ::google::protobuf::Struct> * m,
                          bool key, ::google::protobuf::Struct value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_google_protobuf_Struct_get(const google::protobuf::Map<bool, ::google::protobuf::Struct>* m,
                       bool key, const ::google::protobuf::Struct** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::google::protobuf::Struct* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_google_protobuf_Struct_remove(google::protobuf::Map<bool, ::google::protobuf::Struct> * m,
                          bool key, ::google::protobuf::Struct * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_google_protobuf_Struct_iter(
      const google::protobuf::Map<bool, ::google::protobuf::Struct>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_google_protobuf_Struct_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::google::protobuf::Struct** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::google::protobuf::Struct>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::google::protobuf::Struct>* __rust_proto_thunk__Map_ProtoStr_google_protobuf_Struct_new() {
    return new google::protobuf::Map<std::string, ::google::protobuf::Struct>();
  }
  void __rust_proto_thunk__Map_ProtoStr_google_protobuf_Struct_free(const google::protobuf::Map<std::string, ::google::protobuf::Struct>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_google_protobuf_Struct_clear(google::protobuf::Map<std::string, ::google::protobuf::Struct> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_google_protobuf_Struct_size(const google::protobuf::Map<std::string, ::google::protobuf::Struct>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_google_protobuf_Struct_insert(google::protobuf::Map<std::string, ::google::protobuf::Struct> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::google::protobuf::Struct value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_google_protobuf_Struct_get(const google::protobuf::Map<std::string, ::google::protobuf::Struct>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::google::protobuf::Struct** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::google::protobuf::Struct* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_google_protobuf_Struct_remove(google::protobuf::Map<std::string, ::google::protobuf::Struct> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::google::protobuf::Struct * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_google_protobuf_Struct_iter(
      const google::protobuf::Map<std::string, ::google::protobuf::Struct>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_google_protobuf_Struct_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::google::protobuf::Struct** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::google::protobuf::Struct>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// google.protobuf.Value
        // clang-format off
extern "C" {
void* __rust_proto_thunk__google_protobuf_Value_new() { return new ::google::protobuf::Value(); }
void __rust_proto_thunk__google_protobuf_Value_delete(void* ptr) { delete static_cast<::google::protobuf::Value*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__google_protobuf_Value_serialize(::google::protobuf::Value* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__google_protobuf_Value_parse(::google::protobuf::Value* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__google_protobuf_Value_copy_from(::google::protobuf::Value* dst, const ::google::protobuf::Value* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__google_protobuf_Value_repeated_len(google::protobuf::RepeatedPtrField<::google::protobuf::Value>* field) {
  return field->size();
}
const ::google::protobuf::Value& __rust_proto_thunk__google_protobuf_Value_repeated_get(
  google::protobuf::RepeatedPtrField<::google::protobuf::Value>* field,
  size_t index) {
  return field->Get(index);
}
::google::protobuf::Value* __rust_proto_thunk__google_protobuf_Value_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::google::protobuf::Value>* field,
  size_t index) {
  return field->Mutable(index);
}
::google::protobuf::Value* __rust_proto_thunk__google_protobuf_Value_repeated_add(google::protobuf::RepeatedPtrField<::google::protobuf::Value>* field) {
  return field->Add();
}
void __rust_proto_thunk__google_protobuf_Value_repeated_clear(google::protobuf::RepeatedPtrField<::google::protobuf::Value>* field) {
  field->Clear();
}
void __rust_proto_thunk__google_protobuf_Value_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::google::protobuf::Value>& dst,
  const google::protobuf::RepeatedPtrField<::google::protobuf::Value>& src) {
  dst = src;
}

bool __rust_proto_thunk__google_protobuf_Value_has_null_value(::google::protobuf::Value* msg) {
  return msg->has_null_value();
}
void __rust_proto_thunk__google_protobuf_Value_clear_null_value(::google::protobuf::Value* msg) { msg->clear_null_value(); }
::google::protobuf::NullValue __rust_proto_thunk__google_protobuf_Value_get_null_value(::google::protobuf::Value* msg) { return msg->null_value(); }
void __rust_proto_thunk__google_protobuf_Value_set_null_value(::google::protobuf::Value* msg, ::google::protobuf::NullValue val) {
  msg->set_null_value(val);
}

bool __rust_proto_thunk__google_protobuf_Value_has_number_value(::google::protobuf::Value* msg) {
  return msg->has_number_value();
}
void __rust_proto_thunk__google_protobuf_Value_clear_number_value(::google::protobuf::Value* msg) { msg->clear_number_value(); }
double __rust_proto_thunk__google_protobuf_Value_get_number_value(::google::protobuf::Value* msg) { return msg->number_value(); }
void __rust_proto_thunk__google_protobuf_Value_set_number_value(::google::protobuf::Value* msg, double val) {
  msg->set_number_value(val);
}

bool __rust_proto_thunk__google_protobuf_Value_has_string_value(::google::protobuf::Value* msg) {
  return msg->has_string_value();
}
void __rust_proto_thunk__google_protobuf_Value_clear_string_value(::google::protobuf::Value* msg) { msg->clear_string_value(); }
::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__google_protobuf_Value_get_string_value(::google::protobuf::Value* msg) {
  absl::string_view val = msg->string_value();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__google_protobuf_Value_set_string_value(::google::protobuf::Value* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_string_value(absl::string_view(s.ptr, s.len));
}

bool __rust_proto_thunk__google_protobuf_Value_has_bool_value(::google::protobuf::Value* msg) {
  return msg->has_bool_value();
}
void __rust_proto_thunk__google_protobuf_Value_clear_bool_value(::google::protobuf::Value* msg) { msg->clear_bool_value(); }
bool __rust_proto_thunk__google_protobuf_Value_get_bool_value(::google::protobuf::Value* msg) { return msg->bool_value(); }
void __rust_proto_thunk__google_protobuf_Value_set_bool_value(::google::protobuf::Value* msg, bool val) {
  msg->set_bool_value(val);
}

const void* __rust_proto_thunk__google_protobuf_Value_get_struct_value(::google::protobuf::Value* msg) {
  return static_cast<const void*>(&msg->struct_value());
}
void* __rust_proto_thunk__google_protobuf_Value_get_mut_struct_value(::google::protobuf::Value* msg) {
  return static_cast<void*>(msg->mutable_struct_value());
}
void __rust_proto_thunk__google_protobuf_Value_clear_struct_value(::google::protobuf::Value* msg) { msg->clear_struct_value(); }
bool __rust_proto_thunk__google_protobuf_Value_has_struct_value(::google::protobuf::Value* msg) { return msg->has_struct_value(); }

const void* __rust_proto_thunk__google_protobuf_Value_get_list_value(::google::protobuf::Value* msg) {
  return static_cast<const void*>(&msg->list_value());
}
void* __rust_proto_thunk__google_protobuf_Value_get_mut_list_value(::google::protobuf::Value* msg) {
  return static_cast<void*>(msg->mutable_list_value());
}
void __rust_proto_thunk__google_protobuf_Value_clear_list_value(::google::protobuf::Value* msg) { msg->clear_list_value(); }
bool __rust_proto_thunk__google_protobuf_Value_has_list_value(::google::protobuf::Value* msg) { return msg->has_list_value(); }


::google::protobuf::Value::KindCase __rust_proto_thunk__google_protobuf_Value_case_kind(::google::protobuf::Value* msg) {
  return msg->kind_case();
}
}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::google::protobuf::Value>* __rust_proto_thunk__Map_i32_google_protobuf_Value_new() {
    return new google::protobuf::Map<int32_t, ::google::protobuf::Value>();
  }
  void __rust_proto_thunk__Map_i32_google_protobuf_Value_free(const google::protobuf::Map<int32_t, ::google::protobuf::Value>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_google_protobuf_Value_clear(google::protobuf::Map<int32_t, ::google::protobuf::Value> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_google_protobuf_Value_size(const google::protobuf::Map<int32_t, ::google::protobuf::Value>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_google_protobuf_Value_insert(google::protobuf::Map<int32_t, ::google::protobuf::Value> * m,
                          int32_t key, ::google::protobuf::Value value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_google_protobuf_Value_get(const google::protobuf::Map<int32_t, ::google::protobuf::Value>* m,
                       int32_t key, const ::google::protobuf::Value** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::google::protobuf::Value* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_google_protobuf_Value_remove(google::protobuf::Map<int32_t, ::google::protobuf::Value> * m,
                          int32_t key, ::google::protobuf::Value * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_google_protobuf_Value_iter(
      const google::protobuf::Map<int32_t, ::google::protobuf::Value>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_google_protobuf_Value_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::google::protobuf::Value** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::google::protobuf::Value>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::google::protobuf::Value>* __rust_proto_thunk__Map_u32_google_protobuf_Value_new() {
    return new google::protobuf::Map<uint32_t, ::google::protobuf::Value>();
  }
  void __rust_proto_thunk__Map_u32_google_protobuf_Value_free(const google::protobuf::Map<uint32_t, ::google::protobuf::Value>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_google_protobuf_Value_clear(google::protobuf::Map<uint32_t, ::google::protobuf::Value> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_google_protobuf_Value_size(const google::protobuf::Map<uint32_t, ::google::protobuf::Value>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_google_protobuf_Value_insert(google::protobuf::Map<uint32_t, ::google::protobuf::Value> * m,
                          uint32_t key, ::google::protobuf::Value value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_google_protobuf_Value_get(const google::protobuf::Map<uint32_t, ::google::protobuf::Value>* m,
                       uint32_t key, const ::google::protobuf::Value** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::google::protobuf::Value* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_google_protobuf_Value_remove(google::protobuf::Map<uint32_t, ::google::protobuf::Value> * m,
                          uint32_t key, ::google::protobuf::Value * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_google_protobuf_Value_iter(
      const google::protobuf::Map<uint32_t, ::google::protobuf::Value>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_google_protobuf_Value_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::google::protobuf::Value** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::google::protobuf::Value>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::google::protobuf::Value>* __rust_proto_thunk__Map_i64_google_protobuf_Value_new() {
    return new google::protobuf::Map<int64_t, ::google::protobuf::Value>();
  }
  void __rust_proto_thunk__Map_i64_google_protobuf_Value_free(const google::protobuf::Map<int64_t, ::google::protobuf::Value>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_google_protobuf_Value_clear(google::protobuf::Map<int64_t, ::google::protobuf::Value> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_google_protobuf_Value_size(const google::protobuf::Map<int64_t, ::google::protobuf::Value>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_google_protobuf_Value_insert(google::protobuf::Map<int64_t, ::google::protobuf::Value> * m,
                          int64_t key, ::google::protobuf::Value value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_google_protobuf_Value_get(const google::protobuf::Map<int64_t, ::google::protobuf::Value>* m,
                       int64_t key, const ::google::protobuf::Value** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::google::protobuf::Value* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_google_protobuf_Value_remove(google::protobuf::Map<int64_t, ::google::protobuf::Value> * m,
                          int64_t key, ::google::protobuf::Value * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_google_protobuf_Value_iter(
      const google::protobuf::Map<int64_t, ::google::protobuf::Value>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_google_protobuf_Value_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::google::protobuf::Value** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::google::protobuf::Value>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::google::protobuf::Value>* __rust_proto_thunk__Map_u64_google_protobuf_Value_new() {
    return new google::protobuf::Map<uint64_t, ::google::protobuf::Value>();
  }
  void __rust_proto_thunk__Map_u64_google_protobuf_Value_free(const google::protobuf::Map<uint64_t, ::google::protobuf::Value>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_google_protobuf_Value_clear(google::protobuf::Map<uint64_t, ::google::protobuf::Value> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_google_protobuf_Value_size(const google::protobuf::Map<uint64_t, ::google::protobuf::Value>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_google_protobuf_Value_insert(google::protobuf::Map<uint64_t, ::google::protobuf::Value> * m,
                          uint64_t key, ::google::protobuf::Value value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_google_protobuf_Value_get(const google::protobuf::Map<uint64_t, ::google::protobuf::Value>* m,
                       uint64_t key, const ::google::protobuf::Value** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::google::protobuf::Value* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_google_protobuf_Value_remove(google::protobuf::Map<uint64_t, ::google::protobuf::Value> * m,
                          uint64_t key, ::google::protobuf::Value * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_google_protobuf_Value_iter(
      const google::protobuf::Map<uint64_t, ::google::protobuf::Value>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_google_protobuf_Value_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::google::protobuf::Value** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::google::protobuf::Value>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::google::protobuf::Value>* __rust_proto_thunk__Map_bool_google_protobuf_Value_new() {
    return new google::protobuf::Map<bool, ::google::protobuf::Value>();
  }
  void __rust_proto_thunk__Map_bool_google_protobuf_Value_free(const google::protobuf::Map<bool, ::google::protobuf::Value>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_google_protobuf_Value_clear(google::protobuf::Map<bool, ::google::protobuf::Value> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_google_protobuf_Value_size(const google::protobuf::Map<bool, ::google::protobuf::Value>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_google_protobuf_Value_insert(google::protobuf::Map<bool, ::google::protobuf::Value> * m,
                          bool key, ::google::protobuf::Value value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_google_protobuf_Value_get(const google::protobuf::Map<bool, ::google::protobuf::Value>* m,
                       bool key, const ::google::protobuf::Value** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::google::protobuf::Value* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_google_protobuf_Value_remove(google::protobuf::Map<bool, ::google::protobuf::Value> * m,
                          bool key, ::google::protobuf::Value * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_google_protobuf_Value_iter(
      const google::protobuf::Map<bool, ::google::protobuf::Value>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_google_protobuf_Value_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::google::protobuf::Value** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::google::protobuf::Value>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::google::protobuf::Value>* __rust_proto_thunk__Map_ProtoStr_google_protobuf_Value_new() {
    return new google::protobuf::Map<std::string, ::google::protobuf::Value>();
  }
  void __rust_proto_thunk__Map_ProtoStr_google_protobuf_Value_free(const google::protobuf::Map<std::string, ::google::protobuf::Value>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_google_protobuf_Value_clear(google::protobuf::Map<std::string, ::google::protobuf::Value> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_google_protobuf_Value_size(const google::protobuf::Map<std::string, ::google::protobuf::Value>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_google_protobuf_Value_insert(google::protobuf::Map<std::string, ::google::protobuf::Value> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::google::protobuf::Value value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_google_protobuf_Value_get(const google::protobuf::Map<std::string, ::google::protobuf::Value>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::google::protobuf::Value** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::google::protobuf::Value* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_google_protobuf_Value_remove(google::protobuf::Map<std::string, ::google::protobuf::Value> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::google::protobuf::Value * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_google_protobuf_Value_iter(
      const google::protobuf::Map<std::string, ::google::protobuf::Value>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_google_protobuf_Value_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::google::protobuf::Value** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::google::protobuf::Value>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// google.protobuf.ListValue
        // clang-format off
extern "C" {
void* __rust_proto_thunk__google_protobuf_ListValue_new() { return new ::google::protobuf::ListValue(); }
void __rust_proto_thunk__google_protobuf_ListValue_delete(void* ptr) { delete static_cast<::google::protobuf::ListValue*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__google_protobuf_ListValue_serialize(::google::protobuf::ListValue* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__google_protobuf_ListValue_parse(::google::protobuf::ListValue* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__google_protobuf_ListValue_copy_from(::google::protobuf::ListValue* dst, const ::google::protobuf::ListValue* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__google_protobuf_ListValue_repeated_len(google::protobuf::RepeatedPtrField<::google::protobuf::ListValue>* field) {
  return field->size();
}
const ::google::protobuf::ListValue& __rust_proto_thunk__google_protobuf_ListValue_repeated_get(
  google::protobuf::RepeatedPtrField<::google::protobuf::ListValue>* field,
  size_t index) {
  return field->Get(index);
}
::google::protobuf::ListValue* __rust_proto_thunk__google_protobuf_ListValue_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::google::protobuf::ListValue>* field,
  size_t index) {
  return field->Mutable(index);
}
::google::protobuf::ListValue* __rust_proto_thunk__google_protobuf_ListValue_repeated_add(google::protobuf::RepeatedPtrField<::google::protobuf::ListValue>* field) {
  return field->Add();
}
void __rust_proto_thunk__google_protobuf_ListValue_repeated_clear(google::protobuf::RepeatedPtrField<::google::protobuf::ListValue>* field) {
  field->Clear();
}
void __rust_proto_thunk__google_protobuf_ListValue_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::google::protobuf::ListValue>& dst,
  const google::protobuf::RepeatedPtrField<::google::protobuf::ListValue>& src) {
  dst = src;
}

void __rust_proto_thunk__google_protobuf_ListValue_clear_values(::google::protobuf::ListValue* msg) {
  msg->clear_values();
}
google::protobuf::RepeatedPtrField<::google::protobuf::Value>* __rust_proto_thunk__google_protobuf_ListValue_get_mut_values(::google::protobuf::ListValue* msg) {
  return msg->mutable_values();
}
const google::protobuf::RepeatedPtrField<::google::protobuf::Value>* __rust_proto_thunk__google_protobuf_ListValue_get_values(
    const ::google::protobuf::ListValue* msg) {
  return &msg->values();
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::google::protobuf::ListValue>* __rust_proto_thunk__Map_i32_google_protobuf_ListValue_new() {
    return new google::protobuf::Map<int32_t, ::google::protobuf::ListValue>();
  }
  void __rust_proto_thunk__Map_i32_google_protobuf_ListValue_free(const google::protobuf::Map<int32_t, ::google::protobuf::ListValue>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_google_protobuf_ListValue_clear(google::protobuf::Map<int32_t, ::google::protobuf::ListValue> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_google_protobuf_ListValue_size(const google::protobuf::Map<int32_t, ::google::protobuf::ListValue>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_google_protobuf_ListValue_insert(google::protobuf::Map<int32_t, ::google::protobuf::ListValue> * m,
                          int32_t key, ::google::protobuf::ListValue value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_google_protobuf_ListValue_get(const google::protobuf::Map<int32_t, ::google::protobuf::ListValue>* m,
                       int32_t key, const ::google::protobuf::ListValue** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::google::protobuf::ListValue* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_google_protobuf_ListValue_remove(google::protobuf::Map<int32_t, ::google::protobuf::ListValue> * m,
                          int32_t key, ::google::protobuf::ListValue * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_google_protobuf_ListValue_iter(
      const google::protobuf::Map<int32_t, ::google::protobuf::ListValue>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_google_protobuf_ListValue_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::google::protobuf::ListValue** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::google::protobuf::ListValue>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::google::protobuf::ListValue>* __rust_proto_thunk__Map_u32_google_protobuf_ListValue_new() {
    return new google::protobuf::Map<uint32_t, ::google::protobuf::ListValue>();
  }
  void __rust_proto_thunk__Map_u32_google_protobuf_ListValue_free(const google::protobuf::Map<uint32_t, ::google::protobuf::ListValue>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_google_protobuf_ListValue_clear(google::protobuf::Map<uint32_t, ::google::protobuf::ListValue> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_google_protobuf_ListValue_size(const google::protobuf::Map<uint32_t, ::google::protobuf::ListValue>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_google_protobuf_ListValue_insert(google::protobuf::Map<uint32_t, ::google::protobuf::ListValue> * m,
                          uint32_t key, ::google::protobuf::ListValue value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_google_protobuf_ListValue_get(const google::protobuf::Map<uint32_t, ::google::protobuf::ListValue>* m,
                       uint32_t key, const ::google::protobuf::ListValue** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::google::protobuf::ListValue* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_google_protobuf_ListValue_remove(google::protobuf::Map<uint32_t, ::google::protobuf::ListValue> * m,
                          uint32_t key, ::google::protobuf::ListValue * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_google_protobuf_ListValue_iter(
      const google::protobuf::Map<uint32_t, ::google::protobuf::ListValue>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_google_protobuf_ListValue_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::google::protobuf::ListValue** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::google::protobuf::ListValue>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::google::protobuf::ListValue>* __rust_proto_thunk__Map_i64_google_protobuf_ListValue_new() {
    return new google::protobuf::Map<int64_t, ::google::protobuf::ListValue>();
  }
  void __rust_proto_thunk__Map_i64_google_protobuf_ListValue_free(const google::protobuf::Map<int64_t, ::google::protobuf::ListValue>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_google_protobuf_ListValue_clear(google::protobuf::Map<int64_t, ::google::protobuf::ListValue> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_google_protobuf_ListValue_size(const google::protobuf::Map<int64_t, ::google::protobuf::ListValue>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_google_protobuf_ListValue_insert(google::protobuf::Map<int64_t, ::google::protobuf::ListValue> * m,
                          int64_t key, ::google::protobuf::ListValue value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_google_protobuf_ListValue_get(const google::protobuf::Map<int64_t, ::google::protobuf::ListValue>* m,
                       int64_t key, const ::google::protobuf::ListValue** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::google::protobuf::ListValue* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_google_protobuf_ListValue_remove(google::protobuf::Map<int64_t, ::google::protobuf::ListValue> * m,
                          int64_t key, ::google::protobuf::ListValue * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_google_protobuf_ListValue_iter(
      const google::protobuf::Map<int64_t, ::google::protobuf::ListValue>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_google_protobuf_ListValue_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::google::protobuf::ListValue** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::google::protobuf::ListValue>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::google::protobuf::ListValue>* __rust_proto_thunk__Map_u64_google_protobuf_ListValue_new() {
    return new google::protobuf::Map<uint64_t, ::google::protobuf::ListValue>();
  }
  void __rust_proto_thunk__Map_u64_google_protobuf_ListValue_free(const google::protobuf::Map<uint64_t, ::google::protobuf::ListValue>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_google_protobuf_ListValue_clear(google::protobuf::Map<uint64_t, ::google::protobuf::ListValue> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_google_protobuf_ListValue_size(const google::protobuf::Map<uint64_t, ::google::protobuf::ListValue>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_google_protobuf_ListValue_insert(google::protobuf::Map<uint64_t, ::google::protobuf::ListValue> * m,
                          uint64_t key, ::google::protobuf::ListValue value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_google_protobuf_ListValue_get(const google::protobuf::Map<uint64_t, ::google::protobuf::ListValue>* m,
                       uint64_t key, const ::google::protobuf::ListValue** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::google::protobuf::ListValue* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_google_protobuf_ListValue_remove(google::protobuf::Map<uint64_t, ::google::protobuf::ListValue> * m,
                          uint64_t key, ::google::protobuf::ListValue * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_google_protobuf_ListValue_iter(
      const google::protobuf::Map<uint64_t, ::google::protobuf::ListValue>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_google_protobuf_ListValue_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::google::protobuf::ListValue** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::google::protobuf::ListValue>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::google::protobuf::ListValue>* __rust_proto_thunk__Map_bool_google_protobuf_ListValue_new() {
    return new google::protobuf::Map<bool, ::google::protobuf::ListValue>();
  }
  void __rust_proto_thunk__Map_bool_google_protobuf_ListValue_free(const google::protobuf::Map<bool, ::google::protobuf::ListValue>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_google_protobuf_ListValue_clear(google::protobuf::Map<bool, ::google::protobuf::ListValue> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_google_protobuf_ListValue_size(const google::protobuf::Map<bool, ::google::protobuf::ListValue>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_google_protobuf_ListValue_insert(google::protobuf::Map<bool, ::google::protobuf::ListValue> * m,
                          bool key, ::google::protobuf::ListValue value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_google_protobuf_ListValue_get(const google::protobuf::Map<bool, ::google::protobuf::ListValue>* m,
                       bool key, const ::google::protobuf::ListValue** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::google::protobuf::ListValue* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_google_protobuf_ListValue_remove(google::protobuf::Map<bool, ::google::protobuf::ListValue> * m,
                          bool key, ::google::protobuf::ListValue * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_google_protobuf_ListValue_iter(
      const google::protobuf::Map<bool, ::google::protobuf::ListValue>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_google_protobuf_ListValue_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::google::protobuf::ListValue** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::google::protobuf::ListValue>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::google::protobuf::ListValue>* __rust_proto_thunk__Map_ProtoStr_google_protobuf_ListValue_new() {
    return new google::protobuf::Map<std::string, ::google::protobuf::ListValue>();
  }
  void __rust_proto_thunk__Map_ProtoStr_google_protobuf_ListValue_free(const google::protobuf::Map<std::string, ::google::protobuf::ListValue>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_google_protobuf_ListValue_clear(google::protobuf::Map<std::string, ::google::protobuf::ListValue> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_google_protobuf_ListValue_size(const google::protobuf::Map<std::string, ::google::protobuf::ListValue>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_google_protobuf_ListValue_insert(google::protobuf::Map<std::string, ::google::protobuf::ListValue> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::google::protobuf::ListValue value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_google_protobuf_ListValue_get(const google::protobuf::Map<std::string, ::google::protobuf::ListValue>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::google::protobuf::ListValue** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::google::protobuf::ListValue* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_google_protobuf_ListValue_remove(google::protobuf::Map<std::string, ::google::protobuf::ListValue> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::google::protobuf::ListValue * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_google_protobuf_ListValue_iter(
      const google::protobuf::Map<std::string, ::google::protobuf::ListValue>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_google_protobuf_ListValue_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::google::protobuf::ListValue** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::google::protobuf::ListValue>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// google.protobuf.NullValue
extern "C" {
  __PB_RUST_EXPOSE_SCALAR_MAP_METHODS_FOR_VALUE_TYPE(
      ::google::protobuf::NullValue, google_protobuf_NullValue, ::google::protobuf::NullValue, value, cpp_value)
}

