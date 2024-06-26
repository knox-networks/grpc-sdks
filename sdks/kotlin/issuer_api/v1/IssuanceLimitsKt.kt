// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: issuer_api/v1/issuer.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package issuer_api.v1;

@kotlin.jvm.JvmName("-initializeissuanceLimits")
public inline fun issuanceLimits(block: issuer_api.v1.IssuanceLimitsKt.Dsl.() -> kotlin.Unit): issuer_api.v1.Issuer.IssuanceLimits =
  issuer_api.v1.IssuanceLimitsKt.Dsl._create(issuer_api.v1.Issuer.IssuanceLimits.newBuilder()).apply { block() }._build()
/**
 * Protobuf type `issuer_api.v1.IssuanceLimits`
 */
public object IssuanceLimitsKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: issuer_api.v1.Issuer.IssuanceLimits.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: issuer_api.v1.Issuer.IssuanceLimits.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): issuer_api.v1.Issuer.IssuanceLimits = _builder.build()

    /**
     * An uninstantiable, behaviorless type to represent the field in
     * generics.
     */
    @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
    public class InnerProxy private constructor() : com.google.protobuf.kotlin.DslProxy()
    /**
     * ```
     * The amounts up to which issuance is authorized in respective currencies.
     * ```
     *
     * `repeated .common.Amount inner = 2;`
     */
     public val inner: com.google.protobuf.kotlin.DslList<common.Common.Amount, InnerProxy>
      @kotlin.jvm.JvmSynthetic
      get() = com.google.protobuf.kotlin.DslList(
        _builder.getInnerList()
      )
    /**
     * ```
     * The amounts up to which issuance is authorized in respective currencies.
     * ```
     *
     * `repeated .common.Amount inner = 2;`
     * @param value The inner to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("addInner")
    public fun com.google.protobuf.kotlin.DslList<common.Common.Amount, InnerProxy>.add(value: common.Common.Amount) {
      _builder.addInner(value)
    }
    /**
     * ```
     * The amounts up to which issuance is authorized in respective currencies.
     * ```
     *
     * `repeated .common.Amount inner = 2;`
     * @param value The inner to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("plusAssignInner")
    @Suppress("NOTHING_TO_INLINE")
    public inline operator fun com.google.protobuf.kotlin.DslList<common.Common.Amount, InnerProxy>.plusAssign(value: common.Common.Amount) {
      add(value)
    }
    /**
     * ```
     * The amounts up to which issuance is authorized in respective currencies.
     * ```
     *
     * `repeated .common.Amount inner = 2;`
     * @param values The inner to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("addAllInner")
    public fun com.google.protobuf.kotlin.DslList<common.Common.Amount, InnerProxy>.addAll(values: kotlin.collections.Iterable<common.Common.Amount>) {
      _builder.addAllInner(values)
    }
    /**
     * ```
     * The amounts up to which issuance is authorized in respective currencies.
     * ```
     *
     * `repeated .common.Amount inner = 2;`
     * @param values The inner to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("plusAssignAllInner")
    @Suppress("NOTHING_TO_INLINE")
    public inline operator fun com.google.protobuf.kotlin.DslList<common.Common.Amount, InnerProxy>.plusAssign(values: kotlin.collections.Iterable<common.Common.Amount>) {
      addAll(values)
    }
    /**
     * ```
     * The amounts up to which issuance is authorized in respective currencies.
     * ```
     *
     * `repeated .common.Amount inner = 2;`
     * @param index The index to set the value at.
     * @param value The inner to set.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("setInner")
    public operator fun com.google.protobuf.kotlin.DslList<common.Common.Amount, InnerProxy>.set(index: kotlin.Int, value: common.Common.Amount) {
      _builder.setInner(index, value)
    }
    /**
     * ```
     * The amounts up to which issuance is authorized in respective currencies.
     * ```
     *
     * `repeated .common.Amount inner = 2;`
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("clearInner")
    public fun com.google.protobuf.kotlin.DslList<common.Common.Amount, InnerProxy>.clear() {
      _builder.clearInner()
    }

  }
}
@kotlin.jvm.JvmSynthetic
public inline fun issuer_api.v1.Issuer.IssuanceLimits.copy(block: `issuer_api.v1`.IssuanceLimitsKt.Dsl.() -> kotlin.Unit): issuer_api.v1.Issuer.IssuanceLimits =
  `issuer_api.v1`.IssuanceLimitsKt.Dsl._create(this.toBuilder()).apply { block() }._build()

