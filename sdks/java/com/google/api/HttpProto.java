// Generated by the protocol buffer compiler.  DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: google/api/http.proto
// Protobuf Java Version: 4.27.1

package com.google.api;

public final class HttpProto {
  private HttpProto() {}
  static {
    com.google.protobuf.RuntimeVersion.validateProtobufGencodeVersion(
      com.google.protobuf.RuntimeVersion.RuntimeDomain.PUBLIC,
      /* major= */ 4,
      /* minor= */ 27,
      /* patch= */ 1,
      /* suffix= */ "",
      HttpProto.class.getName());
  }
  public static void registerAllExtensions(
      com.google.protobuf.ExtensionRegistryLite registry) {
  }

  public static void registerAllExtensions(
      com.google.protobuf.ExtensionRegistry registry) {
    registerAllExtensions(
        (com.google.protobuf.ExtensionRegistryLite) registry);
  }
  static final com.google.protobuf.Descriptors.Descriptor
    internal_static_google_api_Http_descriptor;
  static final 
    com.google.protobuf.GeneratedMessage.FieldAccessorTable
      internal_static_google_api_Http_fieldAccessorTable;
  static final com.google.protobuf.Descriptors.Descriptor
    internal_static_google_api_HttpRule_descriptor;
  static final 
    com.google.protobuf.GeneratedMessage.FieldAccessorTable
      internal_static_google_api_HttpRule_fieldAccessorTable;
  static final com.google.protobuf.Descriptors.Descriptor
    internal_static_google_api_CustomHttpPattern_descriptor;
  static final 
    com.google.protobuf.GeneratedMessage.FieldAccessorTable
      internal_static_google_api_CustomHttpPattern_fieldAccessorTable;

  public static com.google.protobuf.Descriptors.FileDescriptor
      getDescriptor() {
    return descriptor;
  }
  private static  com.google.protobuf.Descriptors.FileDescriptor
      descriptor;
  static {
    java.lang.String[] descriptorData = {
      "\n\025google/api/http.proto\022\ngoogle.api\"T\n\004H" +
      "ttp\022#\n\005rules\030\001 \003(\0132\024.google.api.HttpRule" +
      "\022\'\n\037fully_decode_reserved_expansion\030\002 \001(" +
      "\010\"\201\002\n\010HttpRule\022\020\n\010selector\030\001 \001(\t\022\r\n\003get\030" +
      "\002 \001(\tH\000\022\r\n\003put\030\003 \001(\tH\000\022\016\n\004post\030\004 \001(\tH\000\022\020" +
      "\n\006delete\030\005 \001(\tH\000\022\017\n\005patch\030\006 \001(\tH\000\022/\n\006cus" +
      "tom\030\010 \001(\0132\035.google.api.CustomHttpPattern" +
      "H\000\022\014\n\004body\030\007 \001(\t\022\025\n\rresponse_body\030\014 \001(\t\022" +
      "1\n\023additional_bindings\030\013 \003(\0132\024.google.ap" +
      "i.HttpRuleB\t\n\007pattern\"/\n\021CustomHttpPatte" +
      "rn\022\014\n\004kind\030\001 \001(\t\022\014\n\004path\030\002 \001(\tBj\n\016com.go" +
      "ogle.apiB\tHttpProtoP\001ZAgoogle.golang.org" +
      "/genproto/googleapis/api/annotations;ann" +
      "otations\370\001\001\242\002\004GAPIb\006proto3"
    };
    descriptor = com.google.protobuf.Descriptors.FileDescriptor
      .internalBuildGeneratedFileFrom(descriptorData,
        new com.google.protobuf.Descriptors.FileDescriptor[] {
        });
    internal_static_google_api_Http_descriptor =
      getDescriptor().getMessageTypes().get(0);
    internal_static_google_api_Http_fieldAccessorTable = new
      com.google.protobuf.GeneratedMessage.FieldAccessorTable(
        internal_static_google_api_Http_descriptor,
        new java.lang.String[] { "Rules", "FullyDecodeReservedExpansion", });
    internal_static_google_api_HttpRule_descriptor =
      getDescriptor().getMessageTypes().get(1);
    internal_static_google_api_HttpRule_fieldAccessorTable = new
      com.google.protobuf.GeneratedMessage.FieldAccessorTable(
        internal_static_google_api_HttpRule_descriptor,
        new java.lang.String[] { "Selector", "Get", "Put", "Post", "Delete", "Patch", "Custom", "Body", "ResponseBody", "AdditionalBindings", "Pattern", });
    internal_static_google_api_CustomHttpPattern_descriptor =
      getDescriptor().getMessageTypes().get(2);
    internal_static_google_api_CustomHttpPattern_fieldAccessorTable = new
      com.google.protobuf.GeneratedMessage.FieldAccessorTable(
        internal_static_google_api_CustomHttpPattern_descriptor,
        new java.lang.String[] { "Kind", "Path", });
    descriptor.resolveAllFeaturesImmutable();
  }

  // @@protoc_insertion_point(outer_class_scope)
}
