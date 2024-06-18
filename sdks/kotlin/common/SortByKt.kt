// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: common/v1/common.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package common;

@kotlin.jvm.JvmName("-initializesortBy")
public inline fun sortBy(block: common.SortByKt.Dsl.() -> kotlin.Unit): common.Common.SortBy =
  common.SortByKt.Dsl._create(common.Common.SortBy.newBuilder()).apply { block() }._build()
/**
 * ```
 * [Example]
 * {
 * "items": [
 * {
 * "field": "FIELD1",
 * "direction": 0
 * },
 * {
 * "field": "FIELD2",
 * "direction": 0
 * }
 * ]
 * }
 * ```
 *
 * Protobuf type `common.SortBy`
 */
public object SortByKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: common.Common.SortBy.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: common.Common.SortBy.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): common.Common.SortBy = _builder.build()

    /**
     * An uninstantiable, behaviorless type to represent the field in
     * generics.
     */
    @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
    public class ItemsProxy private constructor() : com.google.protobuf.kotlin.DslProxy()
    /**
     * ```
     * List of sort items.
     * ```
     *
     * `repeated .common.SortItem items = 1;`
     */
     public val items: com.google.protobuf.kotlin.DslList<common.Common.SortItem, ItemsProxy>
      @kotlin.jvm.JvmSynthetic
      get() = com.google.protobuf.kotlin.DslList(
        _builder.getItemsList()
      )
    /**
     * ```
     * List of sort items.
     * ```
     *
     * `repeated .common.SortItem items = 1;`
     * @param value The items to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("addItems")
    public fun com.google.protobuf.kotlin.DslList<common.Common.SortItem, ItemsProxy>.add(value: common.Common.SortItem) {
      _builder.addItems(value)
    }
    /**
     * ```
     * List of sort items.
     * ```
     *
     * `repeated .common.SortItem items = 1;`
     * @param value The items to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("plusAssignItems")
    @Suppress("NOTHING_TO_INLINE")
    public inline operator fun com.google.protobuf.kotlin.DslList<common.Common.SortItem, ItemsProxy>.plusAssign(value: common.Common.SortItem) {
      add(value)
    }
    /**
     * ```
     * List of sort items.
     * ```
     *
     * `repeated .common.SortItem items = 1;`
     * @param values The items to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("addAllItems")
    public fun com.google.protobuf.kotlin.DslList<common.Common.SortItem, ItemsProxy>.addAll(values: kotlin.collections.Iterable<common.Common.SortItem>) {
      _builder.addAllItems(values)
    }
    /**
     * ```
     * List of sort items.
     * ```
     *
     * `repeated .common.SortItem items = 1;`
     * @param values The items to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("plusAssignAllItems")
    @Suppress("NOTHING_TO_INLINE")
    public inline operator fun com.google.protobuf.kotlin.DslList<common.Common.SortItem, ItemsProxy>.plusAssign(values: kotlin.collections.Iterable<common.Common.SortItem>) {
      addAll(values)
    }
    /**
     * ```
     * List of sort items.
     * ```
     *
     * `repeated .common.SortItem items = 1;`
     * @param index The index to set the value at.
     * @param value The items to set.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("setItems")
    public operator fun com.google.protobuf.kotlin.DslList<common.Common.SortItem, ItemsProxy>.set(index: kotlin.Int, value: common.Common.SortItem) {
      _builder.setItems(index, value)
    }
    /**
     * ```
     * List of sort items.
     * ```
     *
     * `repeated .common.SortItem items = 1;`
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("clearItems")
    public fun com.google.protobuf.kotlin.DslList<common.Common.SortItem, ItemsProxy>.clear() {
      _builder.clearItems()
    }

  }
}
@kotlin.jvm.JvmSynthetic
public inline fun common.Common.SortBy.copy(block: `common`.SortByKt.Dsl.() -> kotlin.Unit): common.Common.SortBy =
  `common`.SortByKt.Dsl._create(this.toBuilder()).apply { block() }._build()

