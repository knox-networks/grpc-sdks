// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: business_api/v1/business.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package business_api.v1;

@kotlin.jvm.JvmName("-initializesendNewTransactionRequest")
public inline fun sendNewTransactionRequest(block: business_api.v1.SendNewTransactionRequestKt.Dsl.() -> kotlin.Unit): business_api.v1.BusinessOuterClass.SendNewTransactionRequest =
  business_api.v1.SendNewTransactionRequestKt.Dsl._create(business_api.v1.BusinessOuterClass.SendNewTransactionRequest.newBuilder()).apply { block() }._build()
/**
 * ```
 * [Example]
 * {
 * "participant": "zParticipantPublicKey",
 * "memo": "SAMPLE MEMO",
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
 * ]
 * }
 * ```
 *
 * Protobuf type `business_api.v1.SendNewTransactionRequest`
 */
public object SendNewTransactionRequestKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: business_api.v1.BusinessOuterClass.SendNewTransactionRequest.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: business_api.v1.BusinessOuterClass.SendNewTransactionRequest.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): business_api.v1.BusinessOuterClass.SendNewTransactionRequest = _builder.build()

    /**
     * ```
     * Participant Public Key.
     * ```
     *
     * `string participant = 1;`
     */
    public var participant: kotlin.String
      @JvmName("getParticipant")
      get() = _builder.getParticipant()
      @JvmName("setParticipant")
      set(value) {
        _builder.setParticipant(value)
      }
    /**
     * ```
     * Participant Public Key.
     * ```
     *
     * `string participant = 1;`
     */
    public fun clearParticipant() {
      _builder.clearParticipant()
    }

    /**
     * ```
     * Transaction Memo.
     * ```
     *
     * `string memo = 2;`
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
     * Transaction Memo.
     * ```
     *
     * `string memo = 2;`
     */
    public fun clearMemo() {
      _builder.clearMemo()
    }

    /**
     * An uninstantiable, behaviorless type to represent the field in
     * generics.
     */
    @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
    public class CommitmentsProxy private constructor() : com.google.protobuf.kotlin.DslProxy()
    /**
     * ```
     * Commitments.
     * ```
     *
     * `repeated .common.Commitment commitments = 3;`
     */
     public val commitments: com.google.protobuf.kotlin.DslList<common.PacketOuterClass.Commitment, CommitmentsProxy>
      @kotlin.jvm.JvmSynthetic
      get() = com.google.protobuf.kotlin.DslList(
        _builder.getCommitmentsList()
      )
    /**
     * ```
     * Commitments.
     * ```
     *
     * `repeated .common.Commitment commitments = 3;`
     * @param value The commitments to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("addCommitments")
    public fun com.google.protobuf.kotlin.DslList<common.PacketOuterClass.Commitment, CommitmentsProxy>.add(value: common.PacketOuterClass.Commitment) {
      _builder.addCommitments(value)
    }
    /**
     * ```
     * Commitments.
     * ```
     *
     * `repeated .common.Commitment commitments = 3;`
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
     * Commitments.
     * ```
     *
     * `repeated .common.Commitment commitments = 3;`
     * @param values The commitments to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("addAllCommitments")
    public fun com.google.protobuf.kotlin.DslList<common.PacketOuterClass.Commitment, CommitmentsProxy>.addAll(values: kotlin.collections.Iterable<common.PacketOuterClass.Commitment>) {
      _builder.addAllCommitments(values)
    }
    /**
     * ```
     * Commitments.
     * ```
     *
     * `repeated .common.Commitment commitments = 3;`
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
     * Commitments.
     * ```
     *
     * `repeated .common.Commitment commitments = 3;`
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
     * Commitments.
     * ```
     *
     * `repeated .common.Commitment commitments = 3;`
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
     * Conditions.
     * ```
     *
     * `repeated .common.Condition conditions = 4;`
     */
     public val conditions: com.google.protobuf.kotlin.DslList<common.PacketOuterClass.Condition, ConditionsProxy>
      @kotlin.jvm.JvmSynthetic
      get() = com.google.protobuf.kotlin.DslList(
        _builder.getConditionsList()
      )
    /**
     * ```
     * Conditions.
     * ```
     *
     * `repeated .common.Condition conditions = 4;`
     * @param value The conditions to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("addConditions")
    public fun com.google.protobuf.kotlin.DslList<common.PacketOuterClass.Condition, ConditionsProxy>.add(value: common.PacketOuterClass.Condition) {
      _builder.addConditions(value)
    }
    /**
     * ```
     * Conditions.
     * ```
     *
     * `repeated .common.Condition conditions = 4;`
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
     * Conditions.
     * ```
     *
     * `repeated .common.Condition conditions = 4;`
     * @param values The conditions to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("addAllConditions")
    public fun com.google.protobuf.kotlin.DslList<common.PacketOuterClass.Condition, ConditionsProxy>.addAll(values: kotlin.collections.Iterable<common.PacketOuterClass.Condition>) {
      _builder.addAllConditions(values)
    }
    /**
     * ```
     * Conditions.
     * ```
     *
     * `repeated .common.Condition conditions = 4;`
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
     * Conditions.
     * ```
     *
     * `repeated .common.Condition conditions = 4;`
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
     * Conditions.
     * ```
     *
     * `repeated .common.Condition conditions = 4;`
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("clearConditions")
    public fun com.google.protobuf.kotlin.DslList<common.PacketOuterClass.Condition, ConditionsProxy>.clear() {
      _builder.clearConditions()
    }

  }
}
@kotlin.jvm.JvmSynthetic
public inline fun business_api.v1.BusinessOuterClass.SendNewTransactionRequest.copy(block: `business_api.v1`.SendNewTransactionRequestKt.Dsl.() -> kotlin.Unit): business_api.v1.BusinessOuterClass.SendNewTransactionRequest =
  `business_api.v1`.SendNewTransactionRequestKt.Dsl._create(this.toBuilder()).apply { block() }._build()

