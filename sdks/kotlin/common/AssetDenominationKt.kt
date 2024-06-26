// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: common/v1/common.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package common;

@kotlin.jvm.JvmName("-initializeassetDenomination")
public inline fun assetDenomination(block: common.AssetDenominationKt.Dsl.() -> kotlin.Unit): common.Common.AssetDenomination =
  common.AssetDenominationKt.Dsl._create(common.Common.AssetDenomination.newBuilder()).apply { block() }._build()
/**
 * ```
 * [Example]
 * {
 * "denominations": [1, 5, 10, 25, 100, 200, 500, 1000],
 * "precision": 2
 * }
 * ```
 *
 * Protobuf type `common.AssetDenomination`
 */
public object AssetDenominationKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: common.Common.AssetDenomination.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: common.Common.AssetDenomination.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): common.Common.AssetDenomination = _builder.build()

    /**
     * An uninstantiable, behaviorless type to represent the field in
     * generics.
     */
    @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
    public class DenominationsProxy private constructor() : com.google.protobuf.kotlin.DslProxy()
    /**
     * ```
     * Denominations allowed, e.g. [1, 5, 10, 25, 100, etc.].
     * ```
     *
     * `repeated int64 denominations = 1;`
     */
     public val denominations: com.google.protobuf.kotlin.DslList<kotlin.Long, DenominationsProxy>
      @kotlin.jvm.JvmSynthetic
      get() = com.google.protobuf.kotlin.DslList(
        _builder.getDenominationsList()
      )
    /**
     * ```
     * Denominations allowed, e.g. [1, 5, 10, 25, 100, etc.].
     * ```
     *
     * `repeated int64 denominations = 1;`
     * @param value The denominations to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("addDenominations")
    public fun com.google.protobuf.kotlin.DslList<kotlin.Long, DenominationsProxy>.add(value: kotlin.Long) {
      _builder.addDenominations(value)
    }/**
     * ```
     * Denominations allowed, e.g. [1, 5, 10, 25, 100, etc.].
     * ```
     *
     * `repeated int64 denominations = 1;`
     * @param value The denominations to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("plusAssignDenominations")
    @Suppress("NOTHING_TO_INLINE")
    public inline operator fun com.google.protobuf.kotlin.DslList<kotlin.Long, DenominationsProxy>.plusAssign(value: kotlin.Long) {
      add(value)
    }/**
     * ```
     * Denominations allowed, e.g. [1, 5, 10, 25, 100, etc.].
     * ```
     *
     * `repeated int64 denominations = 1;`
     * @param values The denominations to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("addAllDenominations")
    public fun com.google.protobuf.kotlin.DslList<kotlin.Long, DenominationsProxy>.addAll(values: kotlin.collections.Iterable<kotlin.Long>) {
      _builder.addAllDenominations(values)
    }/**
     * ```
     * Denominations allowed, e.g. [1, 5, 10, 25, 100, etc.].
     * ```
     *
     * `repeated int64 denominations = 1;`
     * @param values The denominations to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("plusAssignAllDenominations")
    @Suppress("NOTHING_TO_INLINE")
    public inline operator fun com.google.protobuf.kotlin.DslList<kotlin.Long, DenominationsProxy>.plusAssign(values: kotlin.collections.Iterable<kotlin.Long>) {
      addAll(values)
    }/**
     * ```
     * Denominations allowed, e.g. [1, 5, 10, 25, 100, etc.].
     * ```
     *
     * `repeated int64 denominations = 1;`
     * @param index The index to set the value at.
     * @param value The denominations to set.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("setDenominations")
    public operator fun com.google.protobuf.kotlin.DslList<kotlin.Long, DenominationsProxy>.set(index: kotlin.Int, value: kotlin.Long) {
      _builder.setDenominations(index, value)
    }/**
     * ```
     * Denominations allowed, e.g. [1, 5, 10, 25, 100, etc.].
     * ```
     *
     * `repeated int64 denominations = 1;`
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("clearDenominations")
    public fun com.google.protobuf.kotlin.DslList<kotlin.Long, DenominationsProxy>.clear() {
      _builder.clearDenominations()
    }
    /**
     * ```
     * Precision for Asset (decimals places), e.g. USD $1.00 = 2.
     * ```
     *
     * `uint32 precision = 2;`
     */
    public var precision: kotlin.Int
      @JvmName("getPrecision")
      get() = _builder.getPrecision()
      @JvmName("setPrecision")
      set(value) {
        _builder.setPrecision(value)
      }
    /**
     * ```
     * Precision for Asset (decimals places), e.g. USD $1.00 = 2.
     * ```
     *
     * `uint32 precision = 2;`
     */
    public fun clearPrecision() {
      _builder.clearPrecision()
    }
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun common.Common.AssetDenomination.copy(block: `common`.AssetDenominationKt.Dsl.() -> kotlin.Unit): common.Common.AssetDenomination =
  `common`.AssetDenominationKt.Dsl._create(this.toBuilder()).apply { block() }._build()

