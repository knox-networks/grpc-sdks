// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: common/v1/packet.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package common;

@kotlin.jvm.JvmName("-initializepacket")
public inline fun packet(block: common.PacketKt.Dsl.() -> kotlin.Unit): common.PacketOuterClass.Packet =
  common.PacketKt.Dsl._create(common.PacketOuterClass.Packet.newBuilder()).apply { block() }._build()
/**
 * ```
 * [Example]
 * {
 * "packet_type": 100,
 * "data": "LengthVariesBasedOnData=",
 * "sender": {
 * "signature_system": 1,
 * "verifier": "BytesLengthOfSenderVerifierVariesByAlgo="
 * },
 * "recipient": {
 * "signature_system": 1,
 * "verifier": "AlgoVariesRecipientVerifierBytesLen="
 * },
 * "nonce": "NonceBytesLenVaryByAlgo=",
 * "transaction_header": {
 * "starter": {
 * "signature_system": 1,
 * "verifier": "BytesLenStarterVerifierVariesByAlgo="
 * },
 * "responder": {
 * "signature_system": 1,
 * "verifier": "AlgoVariesResponderVerifierBytesLen="
 * },
 * "uetr": "UETR",
 * "timestamp": "2006-01-02T15:04:05Z",
 * "signature": "ByteLengthSignatureVariesByAlgo=",
 * "user_reference": "USER_REFERENCE",
 * "related_uetrs": [
 * "RELATED_UETR1",
 * "RELATED_UETR2"
 * ]
 * }
 * }
 * ```
 *
 * Protobuf type `common.Packet`
 */
public object PacketKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: common.PacketOuterClass.Packet.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: common.PacketOuterClass.Packet.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): common.PacketOuterClass.Packet = _builder.build()

    /**
     * ```
     * Packet type for determining deserialization scheme and processing
     * ```
     *
     * `.common.PacketType packet_type = 1;`
     */
    public var packetType: common.PacketOuterClass.PacketType
      @JvmName("getPacketType")
      get() = _builder.getPacketType()
      @JvmName("setPacketType")
      set(value) {
        _builder.setPacketType(value)
      }
    public var packetTypeValue: kotlin.Int
      @JvmName("getPacketTypeValue")
      get() = _builder.getPacketTypeValue()
      @JvmName("setPacketTypeValue")
      set(value) {
        _builder.setPacketTypeValue(value)
      }
    /**
     * ```
     * Packet type for determining deserialization scheme and processing
     * ```
     *
     * `.common.PacketType packet_type = 1;`
     */
    public fun clearPacketType() {
      _builder.clearPacketType()
    }

    /**
     * ```
     * Raw byte data corresponding to message type
     * ```
     *
     * `bytes data = 2;`
     */
    public var data: com.google.protobuf.ByteString
      @JvmName("getData")
      get() = _builder.getData()
      @JvmName("setData")
      set(value) {
        _builder.setData(value)
      }
    /**
     * ```
     * Raw byte data corresponding to message type
     * ```
     *
     * `bytes data = 2;`
     */
    public fun clearData() {
      _builder.clearData()
    }

    /**
     * ```
     * Verifier of sender
     * ```
     *
     * `.common.DynamicVerifier sender = 4;`
     */
    public var sender: common.Common.DynamicVerifier
      @JvmName("getSender")
      get() = _builder.getSender()
      @JvmName("setSender")
      set(value) {
        _builder.setSender(value)
      }
    /**
     * ```
     * Verifier of sender
     * ```
     *
     * `.common.DynamicVerifier sender = 4;`
     */
    public fun clearSender() {
      _builder.clearSender()
    }
    /**
     * ```
     * Verifier of sender
     * ```
     *
     * `.common.DynamicVerifier sender = 4;`
     * @return Whether the sender field is set.
     */
    public fun hasSender(): kotlin.Boolean {
      return _builder.hasSender()
    }
    public val PacketKt.Dsl.senderOrNull: common.Common.DynamicVerifier?
      get() = _builder.senderOrNull

    /**
     * ```
     * Verifier of recipient
     * ```
     *
     * `.common.DynamicVerifier recipient = 5;`
     */
    public var recipient: common.Common.DynamicVerifier
      @JvmName("getRecipient")
      get() = _builder.getRecipient()
      @JvmName("setRecipient")
      set(value) {
        _builder.setRecipient(value)
      }
    /**
     * ```
     * Verifier of recipient
     * ```
     *
     * `.common.DynamicVerifier recipient = 5;`
     */
    public fun clearRecipient() {
      _builder.clearRecipient()
    }
    /**
     * ```
     * Verifier of recipient
     * ```
     *
     * `.common.DynamicVerifier recipient = 5;`
     * @return Whether the recipient field is set.
     */
    public fun hasRecipient(): kotlin.Boolean {
      return _builder.hasRecipient()
    }
    public val PacketKt.Dsl.recipientOrNull: common.Common.DynamicVerifier?
      get() = _builder.recipientOrNull

    /**
     * ```
     * Nonce used to guard against replay attacks
     * ```
     *
     * `bytes nonce = 6;`
     */
    public var nonce: com.google.protobuf.ByteString
      @JvmName("getNonce")
      get() = _builder.getNonce()
      @JvmName("setNonce")
      set(value) {
        _builder.setNonce(value)
      }
    /**
     * ```
     * Nonce used to guard against replay attacks
     * ```
     *
     * `bytes nonce = 6;`
     */
    public fun clearNonce() {
      _builder.clearNonce()
    }

    /**
     * ```
     * optional transaction header
     * ```
     *
     * `.common.TransactionHeader transaction_header = 7;`
     */
    public var transactionHeader: common.PacketOuterClass.TransactionHeader
      @JvmName("getTransactionHeader")
      get() = _builder.getTransactionHeader()
      @JvmName("setTransactionHeader")
      set(value) {
        _builder.setTransactionHeader(value)
      }
    /**
     * ```
     * optional transaction header
     * ```
     *
     * `.common.TransactionHeader transaction_header = 7;`
     */
    public fun clearTransactionHeader() {
      _builder.clearTransactionHeader()
    }
    /**
     * ```
     * optional transaction header
     * ```
     *
     * `.common.TransactionHeader transaction_header = 7;`
     * @return Whether the transactionHeader field is set.
     */
    public fun hasTransactionHeader(): kotlin.Boolean {
      return _builder.hasTransactionHeader()
    }
    public val PacketKt.Dsl.transactionHeaderOrNull: common.PacketOuterClass.TransactionHeader?
      get() = _builder.transactionHeaderOrNull
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun common.PacketOuterClass.Packet.copy(block: `common`.PacketKt.Dsl.() -> kotlin.Unit): common.PacketOuterClass.Packet =
  `common`.PacketKt.Dsl._create(this.toBuilder()).apply { block() }._build()

public val common.PacketOuterClass.PacketOrBuilder.senderOrNull: common.Common.DynamicVerifier?
  get() = if (hasSender()) getSender() else null

public val common.PacketOuterClass.PacketOrBuilder.recipientOrNull: common.Common.DynamicVerifier?
  get() = if (hasRecipient()) getRecipient() else null

public val common.PacketOuterClass.PacketOrBuilder.transactionHeaderOrNull: common.PacketOuterClass.TransactionHeader?
  get() = if (hasTransactionHeader()) getTransactionHeader() else null

