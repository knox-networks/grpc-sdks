// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: common/v1/packet.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package common;

@kotlin.jvm.JvmName("-initializeassociatedUetrConnection")
public inline fun associatedUetrConnection(block: common.AssociatedUetrConnectionKt.Dsl.() -> kotlin.Unit): common.PacketOuterClass.AssociatedUetrConnection =
  common.AssociatedUetrConnectionKt.Dsl._create(common.PacketOuterClass.AssociatedUetrConnection.newBuilder()).apply { block() }._build()
/**
 * ```
 * [Example]
 * {
 * "instance_id": "INSTANCE_ID",
 * "uetr": "UETR",
 * packet: {
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
 * "transaction_header": {}
 * }
 * }
 * ```
 *
 * Protobuf type `common.AssociatedUetrConnection`
 */
public object AssociatedUetrConnectionKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: common.PacketOuterClass.AssociatedUetrConnection.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: common.PacketOuterClass.AssociatedUetrConnection.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): common.PacketOuterClass.AssociatedUetrConnection = _builder.build()

    /**
     * ```
     * Uetr in UUID v4 human readable representation
     * ```
     *
     * `string uetr = 1;`
     */
    public var uetr: kotlin.String
      @JvmName("getUetr")
      get() = _builder.getUetr()
      @JvmName("setUetr")
      set(value) {
        _builder.setUetr(value)
      }
    /**
     * ```
     * Uetr in UUID v4 human readable representation
     * ```
     *
     * `string uetr = 1;`
     */
    public fun clearUetr() {
      _builder.clearUetr()
    }

    /**
     * ```
     * Optional packet
     * ```
     *
     * `optional .common.Packet packet = 2;`
     */
    public var packet: common.PacketOuterClass.Packet
      @JvmName("getPacket")
      get() = _builder.getPacket()
      @JvmName("setPacket")
      set(value) {
        _builder.setPacket(value)
      }
    /**
     * ```
     * Optional packet
     * ```
     *
     * `optional .common.Packet packet = 2;`
     */
    public fun clearPacket() {
      _builder.clearPacket()
    }
    /**
     * ```
     * Optional packet
     * ```
     *
     * `optional .common.Packet packet = 2;`
     * @return Whether the packet field is set.
     */
    public fun hasPacket(): kotlin.Boolean {
      return _builder.hasPacket()
    }
    public val AssociatedUetrConnectionKt.Dsl.packetOrNull: common.PacketOuterClass.Packet?
      get() = _builder.packetOrNull
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun common.PacketOuterClass.AssociatedUetrConnection.copy(block: `common`.AssociatedUetrConnectionKt.Dsl.() -> kotlin.Unit): common.PacketOuterClass.AssociatedUetrConnection =
  `common`.AssociatedUetrConnectionKt.Dsl._create(this.toBuilder()).apply { block() }._build()

public val common.PacketOuterClass.AssociatedUetrConnectionOrBuilder.packetOrNull: common.PacketOuterClass.Packet?
  get() = if (hasPacket()) getPacket() else null

