// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: webhook_api/v1/webhook.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package webhook_api.v1;

@kotlin.jvm.JvmName("-initializecreateContractReactionPayload")
public inline fun createContractReactionPayload(block: webhook_api.v1.CreateContractReactionPayloadKt.Dsl.() -> kotlin.Unit): webhook_api.v1.WebhookOuterClass.CreateContractReactionPayload =
  webhook_api.v1.CreateContractReactionPayloadKt.Dsl._create(webhook_api.v1.WebhookOuterClass.CreateContractReactionPayload.newBuilder()).apply { block() }._build()
/**
 * ```
 * [Example]
 * {
 * "sender_wallet_id": "SENDER_WALLET_ID1",
 * "commitments": [
 * {
 * "sender": "zSenderPublicKey",
 * "recipient": "zRecipientPublicKey",
 * "amount": {
 * "currency_code":"USD",
 * "amount":100,
 * "decimals":2
 * }
 * },
 * {
 * "sender": "zSenderPublicKey2",
 * "recipient": "zRecipientPublicKey2",
 * "amount": {
 * "currency_code":"USD",
 * "amount":500,
 * "decimals":2
 * }
 * }
 * ],
 * "conditions": [
 * {
 * "condition": {
 * "hash": {
 * "presenter": "PresenterPublicKey",
 * "hash": "13550350a8681c84"
 * }
 * }
 * },
 * {
 * "condition": {
 * "hash": {
 * "presenter": "PresenterPublicKey",
 * "hash": "13550350a8681c84"
 * }
 * }
 * }
 * ],
 * "timeout_secs": 10000,
 * "memo": "MEMO"
 * }
 * ```
 *
 * Protobuf type `webhook_api.v1.CreateContractReactionPayload`
 */
public object CreateContractReactionPayloadKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: webhook_api.v1.WebhookOuterClass.CreateContractReactionPayload.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: webhook_api.v1.WebhookOuterClass.CreateContractReactionPayload.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): webhook_api.v1.WebhookOuterClass.CreateContractReactionPayload = _builder.build()

    /**
     * ```
     * Wallet ID.
     * ```
     *
     * `string sender_wallet_id = 1;`
     */
    public var senderWalletId: kotlin.String
      @JvmName("getSenderWalletId")
      get() = _builder.getSenderWalletId()
      @JvmName("setSenderWalletId")
      set(value) {
        _builder.setSenderWalletId(value)
      }
    /**
     * ```
     * Wallet ID.
     * ```
     *
     * `string sender_wallet_id = 1;`
     */
    public fun clearSenderWalletId() {
      _builder.clearSenderWalletId()
    }

    /**
     * An uninstantiable, behaviorless type to represent the field in
     * generics.
     */
    @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
    public class CommitmentsProxy private constructor() : com.google.protobuf.kotlin.DslProxy()
    /**
     * ```
     * List of commitments.
     * ```
     *
     * `repeated .common.Commitment commitments = 2;`
     */
     public val commitments: com.google.protobuf.kotlin.DslList<common.PacketOuterClass.Commitment, CommitmentsProxy>
      @kotlin.jvm.JvmSynthetic
      get() = com.google.protobuf.kotlin.DslList(
        _builder.getCommitmentsList()
      )
    /**
     * ```
     * List of commitments.
     * ```
     *
     * `repeated .common.Commitment commitments = 2;`
     * @param value The commitments to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("addCommitments")
    public fun com.google.protobuf.kotlin.DslList<common.PacketOuterClass.Commitment, CommitmentsProxy>.add(value: common.PacketOuterClass.Commitment) {
      _builder.addCommitments(value)
    }
    /**
     * ```
     * List of commitments.
     * ```
     *
     * `repeated .common.Commitment commitments = 2;`
     * @param value The commitments to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("plusAssignCommitments")
    @Suppress("NOTHING_TO_INLINE")
    public inline operator fun com.google.protobuf.kotlin.DslList<common.PacketOuterClass.Commitment, CommitmentsProxy>.plusAssign(value: common.PacketOuterClass.Commitment) {
      add(value)
    }
    /**
     * ```
     * List of commitments.
     * ```
     *
     * `repeated .common.Commitment commitments = 2;`
     * @param values The commitments to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("addAllCommitments")
    public fun com.google.protobuf.kotlin.DslList<common.PacketOuterClass.Commitment, CommitmentsProxy>.addAll(values: kotlin.collections.Iterable<common.PacketOuterClass.Commitment>) {
      _builder.addAllCommitments(values)
    }
    /**
     * ```
     * List of commitments.
     * ```
     *
     * `repeated .common.Commitment commitments = 2;`
     * @param values The commitments to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("plusAssignAllCommitments")
    @Suppress("NOTHING_TO_INLINE")
    public inline operator fun com.google.protobuf.kotlin.DslList<common.PacketOuterClass.Commitment, CommitmentsProxy>.plusAssign(values: kotlin.collections.Iterable<common.PacketOuterClass.Commitment>) {
      addAll(values)
    }
    /**
     * ```
     * List of commitments.
     * ```
     *
     * `repeated .common.Commitment commitments = 2;`
     * @param index The index to set the value at.
     * @param value The commitments to set.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("setCommitments")
    public operator fun com.google.protobuf.kotlin.DslList<common.PacketOuterClass.Commitment, CommitmentsProxy>.set(index: kotlin.Int, value: common.PacketOuterClass.Commitment) {
      _builder.setCommitments(index, value)
    }
    /**
     * ```
     * List of commitments.
     * ```
     *
     * `repeated .common.Commitment commitments = 2;`
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("clearCommitments")
    public fun com.google.protobuf.kotlin.DslList<common.PacketOuterClass.Commitment, CommitmentsProxy>.clear() {
      _builder.clearCommitments()
    }


    /**
     * An uninstantiable, behaviorless type to represent the field in
     * generics.
     */
    @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
    public class ConditionsProxy private constructor() : com.google.protobuf.kotlin.DslProxy()
    /**
     * ```
     * List of conditions.
     * ```
     *
     * `repeated .common.Condition conditions = 3;`
     */
     public val conditions: com.google.protobuf.kotlin.DslList<common.PacketOuterClass.Condition, ConditionsProxy>
      @kotlin.jvm.JvmSynthetic
      get() = com.google.protobuf.kotlin.DslList(
        _builder.getConditionsList()
      )
    /**
     * ```
     * List of conditions.
     * ```
     *
     * `repeated .common.Condition conditions = 3;`
     * @param value The conditions to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("addConditions")
    public fun com.google.protobuf.kotlin.DslList<common.PacketOuterClass.Condition, ConditionsProxy>.add(value: common.PacketOuterClass.Condition) {
      _builder.addConditions(value)
    }
    /**
     * ```
     * List of conditions.
     * ```
     *
     * `repeated .common.Condition conditions = 3;`
     * @param value The conditions to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("plusAssignConditions")
    @Suppress("NOTHING_TO_INLINE")
    public inline operator fun com.google.protobuf.kotlin.DslList<common.PacketOuterClass.Condition, ConditionsProxy>.plusAssign(value: common.PacketOuterClass.Condition) {
      add(value)
    }
    /**
     * ```
     * List of conditions.
     * ```
     *
     * `repeated .common.Condition conditions = 3;`
     * @param values The conditions to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("addAllConditions")
    public fun com.google.protobuf.kotlin.DslList<common.PacketOuterClass.Condition, ConditionsProxy>.addAll(values: kotlin.collections.Iterable<common.PacketOuterClass.Condition>) {
      _builder.addAllConditions(values)
    }
    /**
     * ```
     * List of conditions.
     * ```
     *
     * `repeated .common.Condition conditions = 3;`
     * @param values The conditions to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("plusAssignAllConditions")
    @Suppress("NOTHING_TO_INLINE")
    public inline operator fun com.google.protobuf.kotlin.DslList<common.PacketOuterClass.Condition, ConditionsProxy>.plusAssign(values: kotlin.collections.Iterable<common.PacketOuterClass.Condition>) {
      addAll(values)
    }
    /**
     * ```
     * List of conditions.
     * ```
     *
     * `repeated .common.Condition conditions = 3;`
     * @param index The index to set the value at.
     * @param value The conditions to set.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("setConditions")
    public operator fun com.google.protobuf.kotlin.DslList<common.PacketOuterClass.Condition, ConditionsProxy>.set(index: kotlin.Int, value: common.PacketOuterClass.Condition) {
      _builder.setConditions(index, value)
    }
    /**
     * ```
     * List of conditions.
     * ```
     *
     * `repeated .common.Condition conditions = 3;`
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("clearConditions")
    public fun com.google.protobuf.kotlin.DslList<common.PacketOuterClass.Condition, ConditionsProxy>.clear() {
      _builder.clearConditions()
    }


    /**
     * ```
     * Timeout in seconds for contracts created by the scheduled reaction.
     * ```
     *
     * `uint32 timeout_secs = 4;`
     */
    public var timeoutSecs: kotlin.Int
      @JvmName("getTimeoutSecs")
      get() = _builder.getTimeoutSecs()
      @JvmName("setTimeoutSecs")
      set(value) {
        _builder.setTimeoutSecs(value)
      }
    /**
     * ```
     * Timeout in seconds for contracts created by the scheduled reaction.
     * ```
     *
     * `uint32 timeout_secs = 4;`
     */
    public fun clearTimeoutSecs() {
      _builder.clearTimeoutSecs()
    }

    /**
     * ```
     * Memo for contracts created by the scheduled reaction.
     * ```
     *
     * `string memo = 5;`
     */
    public var memo: kotlin.String
      @JvmName("getMemo")
      get() = _builder.getMemo()
      @JvmName("setMemo")
      set(value) {
        _builder.setMemo(value)
      }
    /**
     * ```
     * Memo for contracts created by the scheduled reaction.
     * ```
     *
     * `string memo = 5;`
     */
    public fun clearMemo() {
      _builder.clearMemo()
    }
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun webhook_api.v1.WebhookOuterClass.CreateContractReactionPayload.copy(block: `webhook_api.v1`.CreateContractReactionPayloadKt.Dsl.() -> kotlin.Unit): webhook_api.v1.WebhookOuterClass.CreateContractReactionPayload =
  `webhook_api.v1`.CreateContractReactionPayloadKt.Dsl._create(this.toBuilder()).apply { block() }._build()
