// <auto-generated>
//     Generated by the protocol buffer compiler.  DO NOT EDIT!
//     source: faucet_api/v1/faucet.proto
// </auto-generated>
#pragma warning disable 1591, 0612, 3021
#region Designer generated code

using pb = global::Google.Protobuf;
using pbc = global::Google.Protobuf.Collections;
using pbr = global::Google.Protobuf.Reflection;
using scg = global::System.Collections.Generic;
namespace FaucetApi.V1 {

  /// <summary>Holder for reflection information generated from faucet_api/v1/faucet.proto</summary>
  public static partial class FaucetReflection {

    #region Descriptor
    /// <summary>File descriptor for faucet_api/v1/faucet.proto</summary>
    public static pbr::FileDescriptor Descriptor {
      get { return descriptor; }
    }
    private static pbr::FileDescriptor descriptor;

    static FaucetReflection() {
      byte[] descriptorData = global::System.Convert.FromBase64String(
          string.Concat(
            "ChpmYXVjZXRfYXBpL3YxL2ZhdWNldC5wcm90bxINZmF1Y2V0X2FwaS52MRoW",
            "Y29tbW9uL3YxL2NvbW1vbi5wcm90byKEAQoNRmF1Y2V0UmVxdWVzdBIeCgZh",
            "bW91bnQYASABKAsyDi5jb21tb24uQW1vdW50EhEKCXJlY2lwaWVudBgCIAEo",
            "CRIvCgxkaXN0cmlidXRpb24YAyABKAsyFC5jb21tb24uRGlzdHJpYnV0aW9u",
            "SACIAQFCDwoNX2Rpc3RyaWJ1dGlvbiIuCg5GYXVjZXRSZXNwb25zZRIcChRu",
            "dW1fcHJvbWlzc29yeV9maWxlcxgBIAEoDTJPCgZGYXVjZXQSRQoGRmF1Y2V0",
            "EhwuZmF1Y2V0X2FwaS52MS5GYXVjZXRSZXF1ZXN0Gh0uZmF1Y2V0X2FwaS52",
            "MS5GYXVjZXRSZXNwb25zZUI5WjdnaXRodWIuY29tL2tub3gtbmV0d29ya3Mv",
            "Z3JwYy1zZGtzL3Nkay9nby9mYXVjZXRfYXBpL3YxYgZwcm90bzM="));
      descriptor = pbr::FileDescriptor.FromGeneratedCode(descriptorData,
          new pbr::FileDescriptor[] { global::Common.CommonReflection.Descriptor, },
          new pbr::GeneratedClrTypeInfo(null, null, new pbr::GeneratedClrTypeInfo[] {
            new pbr::GeneratedClrTypeInfo(typeof(global::FaucetApi.V1.FaucetRequest), global::FaucetApi.V1.FaucetRequest.Parser, new[]{ "Amount", "Recipient", "Distribution" }, new[]{ "Distribution" }, null, null, null),
            new pbr::GeneratedClrTypeInfo(typeof(global::FaucetApi.V1.FaucetResponse), global::FaucetApi.V1.FaucetResponse.Parser, new[]{ "NumPromissoryFiles" }, null, null, null, null)
          }));
    }
    #endregion

  }
  #region Messages
  /// <summary>
  /// Faucet request with the total amount and recipient.
  /// </summary>
  public sealed partial class FaucetRequest : pb::IMessage<FaucetRequest>
  #if !GOOGLE_PROTOBUF_REFSTRUCT_COMPATIBILITY_MODE
      , pb::IBufferMessage
  #endif
  {
    private static readonly pb::MessageParser<FaucetRequest> _parser = new pb::MessageParser<FaucetRequest>(() => new FaucetRequest());
    private pb::UnknownFieldSet _unknownFields;
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public static pb::MessageParser<FaucetRequest> Parser { get { return _parser; } }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public static pbr::MessageDescriptor Descriptor {
      get { return global::FaucetApi.V1.FaucetReflection.Descriptor.MessageTypes[0]; }
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    pbr::MessageDescriptor pb::IMessage.Descriptor {
      get { return Descriptor; }
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public FaucetRequest() {
      OnConstruction();
    }

    partial void OnConstruction();

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public FaucetRequest(FaucetRequest other) : this() {
      amount_ = other.amount_ != null ? other.amount_.Clone() : null;
      recipient_ = other.recipient_;
      distribution_ = other.distribution_ != null ? other.distribution_.Clone() : null;
      _unknownFields = pb::UnknownFieldSet.Clone(other._unknownFields);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public FaucetRequest Clone() {
      return new FaucetRequest(this);
    }

    /// <summary>Field number for the "amount" field.</summary>
    public const int AmountFieldNumber = 1;
    private global::Common.Amount amount_;
    /// <summary>
    /// Total amount to be issued.
    /// </summary>
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public global::Common.Amount Amount {
      get { return amount_; }
      set {
        amount_ = value;
      }
    }

    /// <summary>Field number for the "recipient" field.</summary>
    public const int RecipientFieldNumber = 2;
    private string recipient_ = "";
    /// <summary>
    /// Verifier public key of the intended recipient of the issued promissory
    /// files (multibase encoding).
    /// </summary>
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public string Recipient {
      get { return recipient_; }
      set {
        recipient_ = pb::ProtoPreconditions.CheckNotNull(value, "value");
      }
    }

    /// <summary>Field number for the "distribution" field.</summary>
    public const int DistributionFieldNumber = 3;
    private global::Common.Distribution distribution_;
    /// <summary>
    /// sum must match corresponding amount.amount
    /// </summary>
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public global::Common.Distribution Distribution {
      get { return distribution_; }
      set {
        distribution_ = value;
      }
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public override bool Equals(object other) {
      return Equals(other as FaucetRequest);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public bool Equals(FaucetRequest other) {
      if (ReferenceEquals(other, null)) {
        return false;
      }
      if (ReferenceEquals(other, this)) {
        return true;
      }
      if (!object.Equals(Amount, other.Amount)) return false;
      if (Recipient != other.Recipient) return false;
      if (!object.Equals(Distribution, other.Distribution)) return false;
      return Equals(_unknownFields, other._unknownFields);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public override int GetHashCode() {
      int hash = 1;
      if (amount_ != null) hash ^= Amount.GetHashCode();
      if (Recipient.Length != 0) hash ^= Recipient.GetHashCode();
      if (distribution_ != null) hash ^= Distribution.GetHashCode();
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
      if (amount_ != null) {
        output.WriteRawTag(10);
        output.WriteMessage(Amount);
      }
      if (Recipient.Length != 0) {
        output.WriteRawTag(18);
        output.WriteString(Recipient);
      }
      if (distribution_ != null) {
        output.WriteRawTag(26);
        output.WriteMessage(Distribution);
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
      if (amount_ != null) {
        output.WriteRawTag(10);
        output.WriteMessage(Amount);
      }
      if (Recipient.Length != 0) {
        output.WriteRawTag(18);
        output.WriteString(Recipient);
      }
      if (distribution_ != null) {
        output.WriteRawTag(26);
        output.WriteMessage(Distribution);
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
      if (amount_ != null) {
        size += 1 + pb::CodedOutputStream.ComputeMessageSize(Amount);
      }
      if (Recipient.Length != 0) {
        size += 1 + pb::CodedOutputStream.ComputeStringSize(Recipient);
      }
      if (distribution_ != null) {
        size += 1 + pb::CodedOutputStream.ComputeMessageSize(Distribution);
      }
      if (_unknownFields != null) {
        size += _unknownFields.CalculateSize();
      }
      return size;
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public void MergeFrom(FaucetRequest other) {
      if (other == null) {
        return;
      }
      if (other.amount_ != null) {
        if (amount_ == null) {
          Amount = new global::Common.Amount();
        }
        Amount.MergeFrom(other.Amount);
      }
      if (other.Recipient.Length != 0) {
        Recipient = other.Recipient;
      }
      if (other.distribution_ != null) {
        if (distribution_ == null) {
          Distribution = new global::Common.Distribution();
        }
        Distribution.MergeFrom(other.Distribution);
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
            if (amount_ == null) {
              Amount = new global::Common.Amount();
            }
            input.ReadMessage(Amount);
            break;
          }
          case 18: {
            Recipient = input.ReadString();
            break;
          }
          case 26: {
            if (distribution_ == null) {
              Distribution = new global::Common.Distribution();
            }
            input.ReadMessage(Distribution);
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
            if (amount_ == null) {
              Amount = new global::Common.Amount();
            }
            input.ReadMessage(Amount);
            break;
          }
          case 18: {
            Recipient = input.ReadString();
            break;
          }
          case 26: {
            if (distribution_ == null) {
              Distribution = new global::Common.Distribution();
            }
            input.ReadMessage(Distribution);
            break;
          }
        }
      }
    }
    #endif

  }

  /// <summary>
  /// Faucet response including the number of promissory files issued.
  /// </summary>
  public sealed partial class FaucetResponse : pb::IMessage<FaucetResponse>
  #if !GOOGLE_PROTOBUF_REFSTRUCT_COMPATIBILITY_MODE
      , pb::IBufferMessage
  #endif
  {
    private static readonly pb::MessageParser<FaucetResponse> _parser = new pb::MessageParser<FaucetResponse>(() => new FaucetResponse());
    private pb::UnknownFieldSet _unknownFields;
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public static pb::MessageParser<FaucetResponse> Parser { get { return _parser; } }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public static pbr::MessageDescriptor Descriptor {
      get { return global::FaucetApi.V1.FaucetReflection.Descriptor.MessageTypes[1]; }
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    pbr::MessageDescriptor pb::IMessage.Descriptor {
      get { return Descriptor; }
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public FaucetResponse() {
      OnConstruction();
    }

    partial void OnConstruction();

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public FaucetResponse(FaucetResponse other) : this() {
      numPromissoryFiles_ = other.numPromissoryFiles_;
      _unknownFields = pb::UnknownFieldSet.Clone(other._unknownFields);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public FaucetResponse Clone() {
      return new FaucetResponse(this);
    }

    /// <summary>Field number for the "num_promissory_files" field.</summary>
    public const int NumPromissoryFilesFieldNumber = 1;
    private uint numPromissoryFiles_;
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public uint NumPromissoryFiles {
      get { return numPromissoryFiles_; }
      set {
        numPromissoryFiles_ = value;
      }
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public override bool Equals(object other) {
      return Equals(other as FaucetResponse);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public bool Equals(FaucetResponse other) {
      if (ReferenceEquals(other, null)) {
        return false;
      }
      if (ReferenceEquals(other, this)) {
        return true;
      }
      if (NumPromissoryFiles != other.NumPromissoryFiles) return false;
      return Equals(_unknownFields, other._unknownFields);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public override int GetHashCode() {
      int hash = 1;
      if (NumPromissoryFiles != 0) hash ^= NumPromissoryFiles.GetHashCode();
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
      if (NumPromissoryFiles != 0) {
        output.WriteRawTag(8);
        output.WriteUInt32(NumPromissoryFiles);
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
      if (NumPromissoryFiles != 0) {
        output.WriteRawTag(8);
        output.WriteUInt32(NumPromissoryFiles);
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
      if (NumPromissoryFiles != 0) {
        size += 1 + pb::CodedOutputStream.ComputeUInt32Size(NumPromissoryFiles);
      }
      if (_unknownFields != null) {
        size += _unknownFields.CalculateSize();
      }
      return size;
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public void MergeFrom(FaucetResponse other) {
      if (other == null) {
        return;
      }
      if (other.NumPromissoryFiles != 0) {
        NumPromissoryFiles = other.NumPromissoryFiles;
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
          case 8: {
            NumPromissoryFiles = input.ReadUInt32();
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
          case 8: {
            NumPromissoryFiles = input.ReadUInt32();
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
