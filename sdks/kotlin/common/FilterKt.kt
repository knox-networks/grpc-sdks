// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: common/v1/common.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package common;

@kotlin.jvm.JvmName("-initializefilter")
public inline fun filter(block: common.FilterKt.Dsl.() -> kotlin.Unit): common.Common.Filter =
  common.FilterKt.Dsl._create(common.Common.Filter.newBuilder()).apply { block() }._build()
/**
 * ```
 * [Example]
 * {
 * "filters": [
 * {
 * "field": "FIELD1",
 * "operator": 0,
 * "value": {
 * "str_value": "STRINGVAL"
 * }
 * },
 * {
 * "field": "FIELD2",
 * "operator": 0,
 * "value": {
 * "int_value": 10
 * }
 * },
 * {
 * "field": "FIELD3",
 * "operator": 0,
 * "value": {
 * "bool_value": false
 * }
 * }
 * ],
 * "operator": 0
 * }
 * ```
 *
 * Protobuf type `common.Filter`
 */
public object FilterKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: common.Common.Filter.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: common.Common.Filter.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): common.Common.Filter = _builder.build()

    /**
     * An uninstantiable, behaviorless type to represent the field in
     * generics.
     */
    @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
    public class FiltersProxy private constructor() : com.google.protobuf.kotlin.DslProxy()
    /**
     * ```
     * List of Filters.
     * ```
     *
     * `repeated .common.FilterItem filters = 1;`
     */
     public val filters: com.google.protobuf.kotlin.DslList<common.Common.FilterItem, FiltersProxy>
      @kotlin.jvm.JvmSynthetic
      get() = com.google.protobuf.kotlin.DslList(
        _builder.getFiltersList()
      )
    /**
     * ```
     * List of Filters.
     * ```
     *
     * `repeated .common.FilterItem filters = 1;`
     * @param value The filters to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("addFilters")
    public fun com.google.protobuf.kotlin.DslList<common.Common.FilterItem, FiltersProxy>.add(value: common.Common.FilterItem) {
      _builder.addFilters(value)
    }
    /**
     * ```
     * List of Filters.
     * ```
     *
     * `repeated .common.FilterItem filters = 1;`
     * @param value The filters to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("plusAssignFilters")
    @Suppress("NOTHING_TO_INLINE")
    public inline operator fun com.google.protobuf.kotlin.DslList<common.Common.FilterItem, FiltersProxy>.plusAssign(value: common.Common.FilterItem) {
      add(value)
    }
    /**
     * ```
     * List of Filters.
     * ```
     *
     * `repeated .common.FilterItem filters = 1;`
     * @param values The filters to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("addAllFilters")
    public fun com.google.protobuf.kotlin.DslList<common.Common.FilterItem, FiltersProxy>.addAll(values: kotlin.collections.Iterable<common.Common.FilterItem>) {
      _builder.addAllFilters(values)
    }
    /**
     * ```
     * List of Filters.
     * ```
     *
     * `repeated .common.FilterItem filters = 1;`
     * @param values The filters to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("plusAssignAllFilters")
    @Suppress("NOTHING_TO_INLINE")
    public inline operator fun com.google.protobuf.kotlin.DslList<common.Common.FilterItem, FiltersProxy>.plusAssign(values: kotlin.collections.Iterable<common.Common.FilterItem>) {
      addAll(values)
    }
    /**
     * ```
     * List of Filters.
     * ```
     *
     * `repeated .common.FilterItem filters = 1;`
     * @param index The index to set the value at.
     * @param value The filters to set.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("setFilters")
    public operator fun com.google.protobuf.kotlin.DslList<common.Common.FilterItem, FiltersProxy>.set(index: kotlin.Int, value: common.Common.FilterItem) {
      _builder.setFilters(index, value)
    }
    /**
     * ```
     * List of Filters.
     * ```
     *
     * `repeated .common.FilterItem filters = 1;`
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("clearFilters")
    public fun com.google.protobuf.kotlin.DslList<common.Common.FilterItem, FiltersProxy>.clear() {
      _builder.clearFilters()
    }


    /**
     * ```
     * Operator for Filter.
     * ```
     *
     * `.common.LogicalOperator operator = 2;`
     */
    public var operator: common.Common.LogicalOperator
      @JvmName("getOperator")
      get() = _builder.getOperator()
      @JvmName("setOperator")
      set(value) {
        _builder.setOperator(value)
      }
    public var operatorValue: kotlin.Int
      @JvmName("getOperatorValue")
      get() = _builder.getOperatorValue()
      @JvmName("setOperatorValue")
      set(value) {
        _builder.setOperatorValue(value)
      }
    /**
     * ```
     * Operator for Filter.
     * ```
     *
     * `.common.LogicalOperator operator = 2;`
     */
    public fun clearOperator() {
      _builder.clearOperator()
    }
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun common.Common.Filter.copy(block: `common`.FilterKt.Dsl.() -> kotlin.Unit): common.Common.Filter =
  `common`.FilterKt.Dsl._create(this.toBuilder()).apply { block() }._build()

