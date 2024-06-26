// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: authority_api/v1/authority.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package authority_api.v1;

@kotlin.jvm.JvmName("-initializegetIssuerLimitsResponse")
public inline fun getIssuerLimitsResponse(block: authority_api.v1.GetIssuerLimitsResponseKt.Dsl.() -> kotlin.Unit): authority_api.v1.AuthorityOuterClass.GetIssuerLimitsResponse =
  authority_api.v1.GetIssuerLimitsResponseKt.Dsl._create(authority_api.v1.AuthorityOuterClass.GetIssuerLimitsResponse.newBuilder()).apply { block() }._build()
/**
 * ```
 * [Example]
 * {
 * "limit": [
 * {
 * "currency_code":"USD",
 * "amount": 100,
 * "decimals":2
 * }
 * ],
 * "issuer_public_key": "zIssuerPublicKeyMultibase58Encoded"
 * }
 * ```
 *
 * Protobuf type `authority_api.v1.GetIssuerLimitsResponse`
 */
public object GetIssuerLimitsResponseKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: authority_api.v1.AuthorityOuterClass.GetIssuerLimitsResponse.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: authority_api.v1.AuthorityOuterClass.GetIssuerLimitsResponse.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): authority_api.v1.AuthorityOuterClass.GetIssuerLimitsResponse = _builder.build()

    /**
     * An uninstantiable, behaviorless type to represent the field in
     * generics.
     */
    @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
    public class LimitProxy private constructor() : com.google.protobuf.kotlin.DslProxy()
    /**
     * ```
     * Approved issuance amount limit.
     * ```
     *
     * `repeated .common.Amount limit = 1;`
     */
     public val limit: com.google.protobuf.kotlin.DslList<common.Common.Amount, LimitProxy>
      @kotlin.jvm.JvmSynthetic
      get() = com.google.protobuf.kotlin.DslList(
        _builder.getLimitList()
      )
    /**
     * ```
     * Approved issuance amount limit.
     * ```
     *
     * `repeated .common.Amount limit = 1;`
     * @param value The limit to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("addLimit")
    public fun com.google.protobuf.kotlin.DslList<common.Common.Amount, LimitProxy>.add(value: common.Common.Amount) {
      _builder.addLimit(value)
    }
    /**
     * ```
     * Approved issuance amount limit.
     * ```
     *
     * `repeated .common.Amount limit = 1;`
     * @param value The limit to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("plusAssignLimit")
    @Suppress("NOTHING_TO_INLINE")
    public inline operator fun com.google.protobuf.kotlin.DslList<common.Common.Amount, LimitProxy>.plusAssign(value: common.Common.Amount) {
      add(value)
    }
    /**
     * ```
     * Approved issuance amount limit.
     * ```
     *
     * `repeated .common.Amount limit = 1;`
     * @param values The limit to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("addAllLimit")
    public fun com.google.protobuf.kotlin.DslList<common.Common.Amount, LimitProxy>.addAll(values: kotlin.collections.Iterable<common.Common.Amount>) {
      _builder.addAllLimit(values)
    }
    /**
     * ```
     * Approved issuance amount limit.
     * ```
     *
     * `repeated .common.Amount limit = 1;`
     * @param values The limit to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("plusAssignAllLimit")
    @Suppress("NOTHING_TO_INLINE")
    public inline operator fun com.google.protobuf.kotlin.DslList<common.Common.Amount, LimitProxy>.plusAssign(values: kotlin.collections.Iterable<common.Common.Amount>) {
      addAll(values)
    }
    /**
     * ```
     * Approved issuance amount limit.
     * ```
     *
     * `repeated .common.Amount limit = 1;`
     * @param index The index to set the value at.
     * @param value The limit to set.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("setLimit")
    public operator fun com.google.protobuf.kotlin.DslList<common.Common.Amount, LimitProxy>.set(index: kotlin.Int, value: common.Common.Amount) {
      _builder.setLimit(index, value)
    }
    /**
     * ```
     * Approved issuance amount limit.
     * ```
     *
     * `repeated .common.Amount limit = 1;`
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("clearLimit")
    public fun com.google.protobuf.kotlin.DslList<common.Common.Amount, LimitProxy>.clear() {
      _builder.clearLimit()
    }


    /**
     * ```
     * Verifier of the Issuer.
     * ```
     *
     * `string issuer_public_key = 2;`
     */
    public var issuerPublicKey: kotlin.String
      @JvmName("getIssuerPublicKey")
      get() = _builder.getIssuerPublicKey()
      @JvmName("setIssuerPublicKey")
      set(value) {
        _builder.setIssuerPublicKey(value)
      }
    /**
     * ```
     * Verifier of the Issuer.
     * ```
     *
     * `string issuer_public_key = 2;`
     */
    public fun clearIssuerPublicKey() {
      _builder.clearIssuerPublicKey()
    }
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun authority_api.v1.AuthorityOuterClass.GetIssuerLimitsResponse.copy(block: `authority_api.v1`.GetIssuerLimitsResponseKt.Dsl.() -> kotlin.Unit): authority_api.v1.AuthorityOuterClass.GetIssuerLimitsResponse =
  `authority_api.v1`.GetIssuerLimitsResponseKt.Dsl._create(this.toBuilder()).apply { block() }._build()

