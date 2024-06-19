// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: wallet_api/v1/wallet.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package wallet_api.v1;

@kotlin.jvm.JvmName("-initializeprepareTransactionRequest")
public inline fun prepareTransactionRequest(block: wallet_api.v1.PrepareTransactionRequestKt.Dsl.() -> kotlin.Unit): wallet_api.v1.WalletOuterClass.PrepareTransactionRequest =
  wallet_api.v1.PrepareTransactionRequestKt.Dsl._create(wallet_api.v1.WalletOuterClass.PrepareTransactionRequest.newBuilder()).apply { block() }._build()
/**
 * ```
 * [Example]
 * {
 * "api_key": "API_KEY",
 * "wallet_id": "zWalletIDMultibase58Encoded",
 * "commitments": [
 * {
 * "sender": "zSenderPublicKeyMultibase58Encoded",
 * "recipient": "zRecipientPublicKeyMultibase58Encoded",
 * "amount": {
 * "currency_code": "USD",
 * "amount": 100,
 * "decimals": 2
 * }
 * }
 * ],
 * "conditions": [
 * {
 * "condition": {
 * "timeout": {
 * "timeout": 1687180000
 * }
 * }
 * }
 * ],
 * "memo": "Some contract"
 * }
 * ```
 *
 * Protobuf type `wallet_api.v1.PrepareTransactionRequest`
 */
public object PrepareTransactionRequestKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: wallet_api.v1.WalletOuterClass.PrepareTransactionRequest.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: wallet_api.v1.WalletOuterClass.PrepareTransactionRequest.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): wallet_api.v1.WalletOuterClass.PrepareTransactionRequest = _builder.build()

    /**
     * ```
     * API key given to the wallet owner at the time of owner creation.
     * ```
     *
     * `string api_key = 1;`
     */
    public var apiKey: kotlin.String
      @JvmName("getApiKey")
      get() = _builder.getApiKey()
      @JvmName("setApiKey")
      set(value) {
        _builder.setApiKey(value)
      }
    /**
     * ```
     * API key given to the wallet owner at the time of owner creation.
     * ```
     *
     * `string api_key = 1;`
     */
    public fun clearApiKey() {
      _builder.clearApiKey()
    }

    /**
     * ```
     * UUID of the wallet.
     * ```
     *
     * `string wallet_id = 2;`
     */
    public var walletId: kotlin.String
      @JvmName("getWalletId")
      get() = _builder.getWalletId()
      @JvmName("setWalletId")
      set(value) {
        _builder.setWalletId(value)
      }
    /**
     * ```
     * UUID of the wallet.
     * ```
     *
     * `string wallet_id = 2;`
     */
    public fun clearWalletId() {
      _builder.clearWalletId()
    }

    /**
     * An uninstantiable, behaviorless type to represent the field in
     * generics.
     */
    @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
    public class CommitmentsProxy private constructor() : com.google.protobuf.kotlin.DslProxy()
    /**
     * ```
     * List of commitments
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
     * List of commitments
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
     * List of commitments
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
     * List of commitments
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
     * List of commitments
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
     * List of commitments
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
     * List of commitments
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
     * List of conditions
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
     * List of conditions
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
     * List of conditions
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
     * List of conditions
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
     * List of conditions
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
     * List of conditions
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
     * List of conditions
     * ```
     *
     * `repeated .common.Condition conditions = 4;`
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("clearConditions")
    public fun com.google.protobuf.kotlin.DslList<common.PacketOuterClass.Condition, ConditionsProxy>.clear() {
      _builder.clearConditions()
    }


    /**
     * ```
     * Human-readable memo field
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
     * Human-readable memo field
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
public inline fun wallet_api.v1.WalletOuterClass.PrepareTransactionRequest.copy(block: `wallet_api.v1`.PrepareTransactionRequestKt.Dsl.() -> kotlin.Unit): wallet_api.v1.WalletOuterClass.PrepareTransactionRequest =
  `wallet_api.v1`.PrepareTransactionRequestKt.Dsl._create(this.toBuilder()).apply { block() }._build()
