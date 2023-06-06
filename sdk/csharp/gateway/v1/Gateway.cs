// <auto-generated>
//     Generated by the protocol buffer compiler.  DO NOT EDIT!
//     source: gateway/v1/gateway.proto
// </auto-generated>
#pragma warning disable 1591, 0612, 3021
#region Designer generated code

using pb = global::Google.Protobuf;
using pbc = global::Google.Protobuf.Collections;
using pbr = global::Google.Protobuf.Reflection;
using scg = global::System.Collections.Generic;
namespace Gateway {

  /// <summary>Holder for reflection information generated from gateway/v1/gateway.proto</summary>
  public static partial class GatewayReflection {

    #region Descriptor
    /// <summary>File descriptor for gateway/v1/gateway.proto</summary>
    public static pbr::FileDescriptor Descriptor {
      get { return descriptor; }
    }
    private static pbr::FileDescriptor descriptor;

    static GatewayReflection() {
      byte[] descriptorData = global::System.Convert.FromBase64String(
          string.Concat(
            "ChhnYXRld2F5L3YxL2dhdGV3YXkucHJvdG8SB2dhdGV3YXkaFmNvbW1vbi92",
            "MS9jb21tb24ucHJvdG8aFmNvbW1vbi92MS9wYWNrZXQucHJvdG8iZAoITG9n",
            "RW50cnkSEAoIdmVyaWZpZXIYASABKAkSDAoEbm90ZRgCIAEoCRIRCgl0aW1l",
            "c3RhbXAYAyABKAkSDgoGb3JpZ2luGAQgASgJEhUKDWNvbm5lY3Rpb25faWQY",
            "BSABKAkiFAoSQ29ubmVjdGlvbnNSZXF1ZXN0Im0KE0Nvbm5lY3Rpb25zUmVz",
            "cG9uc2USDgoGYWN0aXZlGAEgAygJEiIKB2NyZWF0ZWQYAiADKAsyES5nYXRl",
            "d2F5LkxvZ0VudHJ5EiIKB3JlbW92ZWQYAyADKAsyES5nYXRld2F5LkxvZ0Vu",
            "dHJ5MtgBCg5Nb25ldGFlR2F0ZXdheRJNCgxBdXRoZW50aWNhdGUSHS5jb21t",
            "b24uQXV0aGVudGljYXRpb25SZXF1ZXN0Gh4uY29tbW9uLkF1dGhlbnRpY2F0",
            "aW9uUmVzcG9uc2USKgoERGF0YRIOLmNvbW1vbi5QYWNrZXQaDi5jb21tb24u",
            "UGFja2V0KAEwARJLCg5HZXRDb25uZWN0aW9ucxIbLmdhdGV3YXkuQ29ubmVj",
            "dGlvbnNSZXF1ZXN0GhwuZ2F0ZXdheS5Db25uZWN0aW9uc1Jlc3BvbnNlQjZa",
            "NGdpdGh1Yi5jb20va25veC1uZXR3b3Jrcy9ncnBjLXNka3Mvc2RrL2dvL2dh",
            "dGV3YXkvdjFiBnByb3RvMw=="));
      descriptor = pbr::FileDescriptor.FromGeneratedCode(descriptorData,
          new pbr::FileDescriptor[] { global::Common.CommonReflection.Descriptor, global::Common.PacketReflection.Descriptor, },
          new pbr::GeneratedClrTypeInfo(null, null, new pbr::GeneratedClrTypeInfo[] {
            new pbr::GeneratedClrTypeInfo(typeof(global::Gateway.LogEntry), global::Gateway.LogEntry.Parser, new[]{ "Verifier", "Note", "Timestamp", "Origin", "ConnectionId" }, null, null, null, null),
            new pbr::GeneratedClrTypeInfo(typeof(global::Gateway.ConnectionsRequest), global::Gateway.ConnectionsRequest.Parser, null, null, null, null, null),
            new pbr::GeneratedClrTypeInfo(typeof(global::Gateway.ConnectionsResponse), global::Gateway.ConnectionsResponse.Parser, new[]{ "Active", "Created", "Removed" }, null, null, null, null)
          }));
    }
    #endregion

  }
  #region Messages
  /// <summary>
  /// [Example]
  ///{
  ///"verifier": "VERIFIER", 
  ///"note": "NOTE", 
  ///"timestamp": "2006-01-02T15:04:05Z", 
  ///"origin": "ORIGIN", 
  ///"connection_id": "CONNECTION_ID_UUID"
  ///}
  /// </summary>
  public sealed partial class LogEntry : pb::IMessage<LogEntry>
  #if !GOOGLE_PROTOBUF_REFSTRUCT_COMPATIBILITY_MODE
      , pb::IBufferMessage
  #endif
  {
    private static readonly pb::MessageParser<LogEntry> _parser = new pb::MessageParser<LogEntry>(() => new LogEntry());
    private pb::UnknownFieldSet _unknownFields;
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public static pb::MessageParser<LogEntry> Parser { get { return _parser; } }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public static pbr::MessageDescriptor Descriptor {
      get { return global::Gateway.GatewayReflection.Descriptor.MessageTypes[0]; }
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    pbr::MessageDescriptor pb::IMessage.Descriptor {
      get { return Descriptor; }
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public LogEntry() {
      OnConstruction();
    }

    partial void OnConstruction();

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public LogEntry(LogEntry other) : this() {
      verifier_ = other.verifier_;
      note_ = other.note_;
      timestamp_ = other.timestamp_;
      origin_ = other.origin_;
      connectionId_ = other.connectionId_;
      _unknownFields = pb::UnknownFieldSet.Clone(other._unknownFields);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public LogEntry Clone() {
      return new LogEntry(this);
    }

    /// <summary>Field number for the "verifier" field.</summary>
    public const int VerifierFieldNumber = 1;
    private string verifier_ = "";
    /// <summary>
    /// Public Key
    /// </summary>
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public string Verifier {
      get { return verifier_; }
      set {
        verifier_ = pb::ProtoPreconditions.CheckNotNull(value, "value");
      }
    }

    /// <summary>Field number for the "note" field.</summary>
    public const int NoteFieldNumber = 2;
    private string note_ = "";
    /// <summary>
    /// Specific action being recorded.
    /// </summary>
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public string Note {
      get { return note_; }
      set {
        note_ = pb::ProtoPreconditions.CheckNotNull(value, "value");
      }
    }

    /// <summary>Field number for the "timestamp" field.</summary>
    public const int TimestampFieldNumber = 3;
    private string timestamp_ = "";
    /// <summary>
    /// RFC3339 timestamp of the log entry.
    /// </summary>
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public string Timestamp {
      get { return timestamp_; }
      set {
        timestamp_ = pb::ProtoPreconditions.CheckNotNull(value, "value");
      }
    }

    /// <summary>Field number for the "origin" field.</summary>
    public const int OriginFieldNumber = 4;
    private string origin_ = "";
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public string Origin {
      get { return origin_; }
      set {
        origin_ = pb::ProtoPreconditions.CheckNotNull(value, "value");
      }
    }

    /// <summary>Field number for the "connection_id" field.</summary>
    public const int ConnectionIdFieldNumber = 5;
    private string connectionId_ = "";
    /// <summary>
    /// UUID representing a unique client connection
    /// </summary>
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public string ConnectionId {
      get { return connectionId_; }
      set {
        connectionId_ = pb::ProtoPreconditions.CheckNotNull(value, "value");
      }
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public override bool Equals(object other) {
      return Equals(other as LogEntry);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public bool Equals(LogEntry other) {
      if (ReferenceEquals(other, null)) {
        return false;
      }
      if (ReferenceEquals(other, this)) {
        return true;
      }
      if (Verifier != other.Verifier) return false;
      if (Note != other.Note) return false;
      if (Timestamp != other.Timestamp) return false;
      if (Origin != other.Origin) return false;
      if (ConnectionId != other.ConnectionId) return false;
      return Equals(_unknownFields, other._unknownFields);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public override int GetHashCode() {
      int hash = 1;
      if (Verifier.Length != 0) hash ^= Verifier.GetHashCode();
      if (Note.Length != 0) hash ^= Note.GetHashCode();
      if (Timestamp.Length != 0) hash ^= Timestamp.GetHashCode();
      if (Origin.Length != 0) hash ^= Origin.GetHashCode();
      if (ConnectionId.Length != 0) hash ^= ConnectionId.GetHashCode();
      if (_unknownFields != null) {
        hash ^= _unknownFields.GetHashCode();
      }
      return hash;
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public override string ToString() {
      return pb::JsonFormatter.ToDiagnosticString(this);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public void WriteTo(pb::CodedOutputStream output) {
    #if !GOOGLE_PROTOBUF_REFSTRUCT_COMPATIBILITY_MODE
      output.WriteRawMessage(this);
    #else
      if (Verifier.Length != 0) {
        output.WriteRawTag(10);
        output.WriteString(Verifier);
      }
      if (Note.Length != 0) {
        output.WriteRawTag(18);
        output.WriteString(Note);
      }
      if (Timestamp.Length != 0) {
        output.WriteRawTag(26);
        output.WriteString(Timestamp);
      }
      if (Origin.Length != 0) {
        output.WriteRawTag(34);
        output.WriteString(Origin);
      }
      if (ConnectionId.Length != 0) {
        output.WriteRawTag(42);
        output.WriteString(ConnectionId);
      }
      if (_unknownFields != null) {
        _unknownFields.WriteTo(output);
      }
    #endif
    }

    #if !GOOGLE_PROTOBUF_REFSTRUCT_COMPATIBILITY_MODE
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    void pb::IBufferMessage.InternalWriteTo(ref pb::WriteContext output) {
      if (Verifier.Length != 0) {
        output.WriteRawTag(10);
        output.WriteString(Verifier);
      }
      if (Note.Length != 0) {
        output.WriteRawTag(18);
        output.WriteString(Note);
      }
      if (Timestamp.Length != 0) {
        output.WriteRawTag(26);
        output.WriteString(Timestamp);
      }
      if (Origin.Length != 0) {
        output.WriteRawTag(34);
        output.WriteString(Origin);
      }
      if (ConnectionId.Length != 0) {
        output.WriteRawTag(42);
        output.WriteString(ConnectionId);
      }
      if (_unknownFields != null) {
        _unknownFields.WriteTo(ref output);
      }
    }
    #endif

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public int CalculateSize() {
      int size = 0;
      if (Verifier.Length != 0) {
        size += 1 + pb::CodedOutputStream.ComputeStringSize(Verifier);
      }
      if (Note.Length != 0) {
        size += 1 + pb::CodedOutputStream.ComputeStringSize(Note);
      }
      if (Timestamp.Length != 0) {
        size += 1 + pb::CodedOutputStream.ComputeStringSize(Timestamp);
      }
      if (Origin.Length != 0) {
        size += 1 + pb::CodedOutputStream.ComputeStringSize(Origin);
      }
      if (ConnectionId.Length != 0) {
        size += 1 + pb::CodedOutputStream.ComputeStringSize(ConnectionId);
      }
      if (_unknownFields != null) {
        size += _unknownFields.CalculateSize();
      }
      return size;
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public void MergeFrom(LogEntry other) {
      if (other == null) {
        return;
      }
      if (other.Verifier.Length != 0) {
        Verifier = other.Verifier;
      }
      if (other.Note.Length != 0) {
        Note = other.Note;
      }
      if (other.Timestamp.Length != 0) {
        Timestamp = other.Timestamp;
      }
      if (other.Origin.Length != 0) {
        Origin = other.Origin;
      }
      if (other.ConnectionId.Length != 0) {
        ConnectionId = other.ConnectionId;
      }
      _unknownFields = pb::UnknownFieldSet.MergeFrom(_unknownFields, other._unknownFields);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public void MergeFrom(pb::CodedInputStream input) {
    #if !GOOGLE_PROTOBUF_REFSTRUCT_COMPATIBILITY_MODE
      input.ReadRawMessage(this);
    #else
      uint tag;
      while ((tag = input.ReadTag()) != 0) {
        switch(tag) {
          default:
            _unknownFields = pb::UnknownFieldSet.MergeFieldFrom(_unknownFields, input);
            break;
          case 10: {
            Verifier = input.ReadString();
            break;
          }
          case 18: {
            Note = input.ReadString();
            break;
          }
          case 26: {
            Timestamp = input.ReadString();
            break;
          }
          case 34: {
            Origin = input.ReadString();
            break;
          }
          case 42: {
            ConnectionId = input.ReadString();
            break;
          }
        }
      }
    #endif
    }

    #if !GOOGLE_PROTOBUF_REFSTRUCT_COMPATIBILITY_MODE
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    void pb::IBufferMessage.InternalMergeFrom(ref pb::ParseContext input) {
      uint tag;
      while ((tag = input.ReadTag()) != 0) {
        switch(tag) {
          default:
            _unknownFields = pb::UnknownFieldSet.MergeFieldFrom(_unknownFields, ref input);
            break;
          case 10: {
            Verifier = input.ReadString();
            break;
          }
          case 18: {
            Note = input.ReadString();
            break;
          }
          case 26: {
            Timestamp = input.ReadString();
            break;
          }
          case 34: {
            Origin = input.ReadString();
            break;
          }
          case 42: {
            ConnectionId = input.ReadString();
            break;
          }
        }
      }
    }
    #endif

  }

  /// <summary>
  /// [Example]
  ///{}
  /// </summary>
  public sealed partial class ConnectionsRequest : pb::IMessage<ConnectionsRequest>
  #if !GOOGLE_PROTOBUF_REFSTRUCT_COMPATIBILITY_MODE
      , pb::IBufferMessage
  #endif
  {
    private static readonly pb::MessageParser<ConnectionsRequest> _parser = new pb::MessageParser<ConnectionsRequest>(() => new ConnectionsRequest());
    private pb::UnknownFieldSet _unknownFields;
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public static pb::MessageParser<ConnectionsRequest> Parser { get { return _parser; } }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public static pbr::MessageDescriptor Descriptor {
      get { return global::Gateway.GatewayReflection.Descriptor.MessageTypes[1]; }
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    pbr::MessageDescriptor pb::IMessage.Descriptor {
      get { return Descriptor; }
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public ConnectionsRequest() {
      OnConstruction();
    }

    partial void OnConstruction();

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public ConnectionsRequest(ConnectionsRequest other) : this() {
      _unknownFields = pb::UnknownFieldSet.Clone(other._unknownFields);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public ConnectionsRequest Clone() {
      return new ConnectionsRequest(this);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public override bool Equals(object other) {
      return Equals(other as ConnectionsRequest);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public bool Equals(ConnectionsRequest other) {
      if (ReferenceEquals(other, null)) {
        return false;
      }
      if (ReferenceEquals(other, this)) {
        return true;
      }
      return Equals(_unknownFields, other._unknownFields);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public override int GetHashCode() {
      int hash = 1;
      if (_unknownFields != null) {
        hash ^= _unknownFields.GetHashCode();
      }
      return hash;
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public override string ToString() {
      return pb::JsonFormatter.ToDiagnosticString(this);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public void WriteTo(pb::CodedOutputStream output) {
    #if !GOOGLE_PROTOBUF_REFSTRUCT_COMPATIBILITY_MODE
      output.WriteRawMessage(this);
    #else
      if (_unknownFields != null) {
        _unknownFields.WriteTo(output);
      }
    #endif
    }

    #if !GOOGLE_PROTOBUF_REFSTRUCT_COMPATIBILITY_MODE
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    void pb::IBufferMessage.InternalWriteTo(ref pb::WriteContext output) {
      if (_unknownFields != null) {
        _unknownFields.WriteTo(ref output);
      }
    }
    #endif

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public int CalculateSize() {
      int size = 0;
      if (_unknownFields != null) {
        size += _unknownFields.CalculateSize();
      }
      return size;
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public void MergeFrom(ConnectionsRequest other) {
      if (other == null) {
        return;
      }
      _unknownFields = pb::UnknownFieldSet.MergeFrom(_unknownFields, other._unknownFields);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public void MergeFrom(pb::CodedInputStream input) {
    #if !GOOGLE_PROTOBUF_REFSTRUCT_COMPATIBILITY_MODE
      input.ReadRawMessage(this);
    #else
      uint tag;
      while ((tag = input.ReadTag()) != 0) {
        switch(tag) {
          default:
            _unknownFields = pb::UnknownFieldSet.MergeFieldFrom(_unknownFields, input);
            break;
        }
      }
    #endif
    }

    #if !GOOGLE_PROTOBUF_REFSTRUCT_COMPATIBILITY_MODE
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    void pb::IBufferMessage.InternalMergeFrom(ref pb::ParseContext input) {
      uint tag;
      while ((tag = input.ReadTag()) != 0) {
        switch(tag) {
          default:
            _unknownFields = pb::UnknownFieldSet.MergeFieldFrom(_unknownFields, ref input);
            break;
        }
      }
    }
    #endif

  }

  /// <summary>
  /// [Example]
  ///{
  ///"active": [
  ///"ActiveConnection1", 
  ///"ActiveConnection2"
  ///], 
  ///"created": [
  ///{
  ///"verifier": "VERIFIER", 
  ///"note": "CreatedConnection1", 
  ///"timestamp": "2006-01-02T15:04:05Z", 
  ///"origin": "ORIGIN", 
  ///"connection_id": "CONNECTION_ID_UUID"
  ///}, 
  ///{
  ///"verifier": "VERIFIER", 
  ///"note": "CreatedConnection2", 
  ///"timestamp": "2006-01-02T15:04:05Z", 
  ///"origin": "ORIGIN", 
  ///"connection_id": 
  ///"CONNECTION_ID_UUID"
  ///}
  ///], 
  ///"removed": [
  ///{
  ///"verifier": "VERIFIER", 
  ///"note": "RemovedConnection1", 
  ///"timestamp": "2006-01-02T15:04:05Z", 
  ///"origin": "ORIGIN", 
  ///"connection_id": "CONNECTION_ID_UUID"
  ///}, 
  ///{
  ///"verifier": "VERIFIER", 
  ///"note": "RemovedConnection2", 
  ///"timestamp": "2006-01-02T15:04:05Z", 
  ///"origin": "ORIGIN", 
  ///"connection_id": "CONNECTION_ID_UUID"
  ///}
  ///]
  ///}
  /// </summary>
  public sealed partial class ConnectionsResponse : pb::IMessage<ConnectionsResponse>
  #if !GOOGLE_PROTOBUF_REFSTRUCT_COMPATIBILITY_MODE
      , pb::IBufferMessage
  #endif
  {
    private static readonly pb::MessageParser<ConnectionsResponse> _parser = new pb::MessageParser<ConnectionsResponse>(() => new ConnectionsResponse());
    private pb::UnknownFieldSet _unknownFields;
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public static pb::MessageParser<ConnectionsResponse> Parser { get { return _parser; } }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public static pbr::MessageDescriptor Descriptor {
      get { return global::Gateway.GatewayReflection.Descriptor.MessageTypes[2]; }
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    pbr::MessageDescriptor pb::IMessage.Descriptor {
      get { return Descriptor; }
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public ConnectionsResponse() {
      OnConstruction();
    }

    partial void OnConstruction();

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public ConnectionsResponse(ConnectionsResponse other) : this() {
      active_ = other.active_.Clone();
      created_ = other.created_.Clone();
      removed_ = other.removed_.Clone();
      _unknownFields = pb::UnknownFieldSet.Clone(other._unknownFields);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public ConnectionsResponse Clone() {
      return new ConnectionsResponse(this);
    }

    /// <summary>Field number for the "active" field.</summary>
    public const int ActiveFieldNumber = 1;
    private static readonly pb::FieldCodec<string> _repeated_active_codec
        = pb::FieldCodec.ForString(10);
    private readonly pbc::RepeatedField<string> active_ = new pbc::RepeatedField<string>();
    /// <summary>
    /// Currently active connections.
    /// </summary>
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public pbc::RepeatedField<string> Active {
      get { return active_; }
    }

    /// <summary>Field number for the "created" field.</summary>
    public const int CreatedFieldNumber = 2;
    private static readonly pb::FieldCodec<global::Gateway.LogEntry> _repeated_created_codec
        = pb::FieldCodec.ForMessage(18, global::Gateway.LogEntry.Parser);
    private readonly pbc::RepeatedField<global::Gateway.LogEntry> created_ = new pbc::RepeatedField<global::Gateway.LogEntry>();
    /// <summary>
    /// The number of connections that have been created.
    /// </summary>
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public pbc::RepeatedField<global::Gateway.LogEntry> Created {
      get { return created_; }
    }

    /// <summary>Field number for the "removed" field.</summary>
    public const int RemovedFieldNumber = 3;
    private static readonly pb::FieldCodec<global::Gateway.LogEntry> _repeated_removed_codec
        = pb::FieldCodec.ForMessage(26, global::Gateway.LogEntry.Parser);
    private readonly pbc::RepeatedField<global::Gateway.LogEntry> removed_ = new pbc::RepeatedField<global::Gateway.LogEntry>();
    /// <summary>
    /// The number of connections that have been removed.
    /// </summary>
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public pbc::RepeatedField<global::Gateway.LogEntry> Removed {
      get { return removed_; }
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public override bool Equals(object other) {
      return Equals(other as ConnectionsResponse);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public bool Equals(ConnectionsResponse other) {
      if (ReferenceEquals(other, null)) {
        return false;
      }
      if (ReferenceEquals(other, this)) {
        return true;
      }
      if(!active_.Equals(other.active_)) return false;
      if(!created_.Equals(other.created_)) return false;
      if(!removed_.Equals(other.removed_)) return false;
      return Equals(_unknownFields, other._unknownFields);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public override int GetHashCode() {
      int hash = 1;
      hash ^= active_.GetHashCode();
      hash ^= created_.GetHashCode();
      hash ^= removed_.GetHashCode();
      if (_unknownFields != null) {
        hash ^= _unknownFields.GetHashCode();
      }
      return hash;
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public override string ToString() {
      return pb::JsonFormatter.ToDiagnosticString(this);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public void WriteTo(pb::CodedOutputStream output) {
    #if !GOOGLE_PROTOBUF_REFSTRUCT_COMPATIBILITY_MODE
      output.WriteRawMessage(this);
    #else
      active_.WriteTo(output, _repeated_active_codec);
      created_.WriteTo(output, _repeated_created_codec);
      removed_.WriteTo(output, _repeated_removed_codec);
      if (_unknownFields != null) {
        _unknownFields.WriteTo(output);
      }
    #endif
    }

    #if !GOOGLE_PROTOBUF_REFSTRUCT_COMPATIBILITY_MODE
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    void pb::IBufferMessage.InternalWriteTo(ref pb::WriteContext output) {
      active_.WriteTo(ref output, _repeated_active_codec);
      created_.WriteTo(ref output, _repeated_created_codec);
      removed_.WriteTo(ref output, _repeated_removed_codec);
      if (_unknownFields != null) {
        _unknownFields.WriteTo(ref output);
      }
    }
    #endif

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public int CalculateSize() {
      int size = 0;
      size += active_.CalculateSize(_repeated_active_codec);
      size += created_.CalculateSize(_repeated_created_codec);
      size += removed_.CalculateSize(_repeated_removed_codec);
      if (_unknownFields != null) {
        size += _unknownFields.CalculateSize();
      }
      return size;
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public void MergeFrom(ConnectionsResponse other) {
      if (other == null) {
        return;
      }
      active_.Add(other.active_);
      created_.Add(other.created_);
      removed_.Add(other.removed_);
      _unknownFields = pb::UnknownFieldSet.MergeFrom(_unknownFields, other._unknownFields);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public void MergeFrom(pb::CodedInputStream input) {
    #if !GOOGLE_PROTOBUF_REFSTRUCT_COMPATIBILITY_MODE
      input.ReadRawMessage(this);
    #else
      uint tag;
      while ((tag = input.ReadTag()) != 0) {
        switch(tag) {
          default:
            _unknownFields = pb::UnknownFieldSet.MergeFieldFrom(_unknownFields, input);
            break;
          case 10: {
            active_.AddEntriesFrom(input, _repeated_active_codec);
            break;
          }
          case 18: {
            created_.AddEntriesFrom(input, _repeated_created_codec);
            break;
          }
          case 26: {
            removed_.AddEntriesFrom(input, _repeated_removed_codec);
            break;
          }
        }
      }
    #endif
    }

    #if !GOOGLE_PROTOBUF_REFSTRUCT_COMPATIBILITY_MODE
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    void pb::IBufferMessage.InternalMergeFrom(ref pb::ParseContext input) {
      uint tag;
      while ((tag = input.ReadTag()) != 0) {
        switch(tag) {
          default:
            _unknownFields = pb::UnknownFieldSet.MergeFieldFrom(_unknownFields, ref input);
            break;
          case 10: {
            active_.AddEntriesFrom(ref input, _repeated_active_codec);
            break;
          }
          case 18: {
            created_.AddEntriesFrom(ref input, _repeated_created_codec);
            break;
          }
          case 26: {
            removed_.AddEntriesFrom(ref input, _repeated_removed_codec);
            break;
          }
        }
      }
    }
    #endif

  }

  #endregion

}

#endregion Designer generated code
