// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: common/v1/common.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package common;

@kotlin.jvm.JvmName("-initializemanyUetrConnection")
public inline fun manyUetrConnection(block: common.ManyUetrConnectionKt.Dsl.() -> kotlin.Unit): common.Common.ManyUetrConnection =
  common.ManyUetrConnectionKt.Dsl._create(common.Common.ManyUetrConnection.newBuilder()).apply { block() }._build()
/**
 * ```
 * [Example]
 * {
 * "associated": [
 * "UETR"
 * ]
 * }
 * ```
 *
 * Protobuf type `common.ManyUetrConnection`
 */
public object ManyUetrConnectionKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: common.Common.ManyUetrConnection.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: common.Common.ManyUetrConnection.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): common.Common.ManyUetrConnection = _builder.build()

    /**
     * An uninstantiable, behaviorless type to represent the field in
     * generics.
     */
    @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
    public class AssociatedProxy private constructor() : com.google.protobuf.kotlin.DslProxy()
    /**
     * ```
     * Associated UETRs in UUID representation
     * ```
     *
     * `repeated string associated = 3;`
     * @return A list containing the associated.
     */
    public val associated: com.google.protobuf.kotlin.DslList<kotlin.String, AssociatedProxy>
      @kotlin.jvm.JvmSynthetic
      get() = com.google.protobuf.kotlin.DslList(
        _builder.getAssociatedList()
      )
    /**
     * ```
     * Associated UETRs in UUID representation
     * ```
     *
     * `repeated string associated = 3;`
     * @param value The associated to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("addAssociated")
    public fun com.google.protobuf.kotlin.DslList<kotlin.String, AssociatedProxy>.add(value: kotlin.String) {
      _builder.addAssociated(value)
    }
    /**
     * ```
     * Associated UETRs in UUID representation
     * ```
     *
     * `repeated string associated = 3;`
     * @param value The associated to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("plusAssignAssociated")
    @Suppress("NOTHING_TO_INLINE")
    public inline operator fun com.google.protobuf.kotlin.DslList<kotlin.String, AssociatedProxy>.plusAssign(value: kotlin.String) {
      add(value)
    }
    /**
     * ```
     * Associated UETRs in UUID representation
     * ```
     *
     * `repeated string associated = 3;`
     * @param values The associated to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("addAllAssociated")
    public fun com.google.protobuf.kotlin.DslList<kotlin.String, AssociatedProxy>.addAll(values: kotlin.collections.Iterable<kotlin.String>) {
      _builder.addAllAssociated(values)
    }
    /**
     * ```
     * Associated UETRs in UUID representation
     * ```
     *
     * `repeated string associated = 3;`
     * @param values The associated to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("plusAssignAllAssociated")
    @Suppress("NOTHING_TO_INLINE")
    public inline operator fun com.google.protobuf.kotlin.DslList<kotlin.String, AssociatedProxy>.plusAssign(values: kotlin.collections.Iterable<kotlin.String>) {
      addAll(values)
    }
    /**
     * ```
     * Associated UETRs in UUID representation
     * ```
     *
     * `repeated string associated = 3;`
     * @param index The index to set the value at.
     * @param value The associated to set.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("setAssociated")
    public operator fun com.google.protobuf.kotlin.DslList<kotlin.String, AssociatedProxy>.set(index: kotlin.Int, value: kotlin.String) {
      _builder.setAssociated(index, value)
    }/**
     * ```
     * Associated UETRs in UUID representation
     * ```
     *
     * `repeated string associated = 3;`
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("clearAssociated")
    public fun com.google.protobuf.kotlin.DslList<kotlin.String, AssociatedProxy>.clear() {
      _builder.clearAssociated()
    }}
}
@kotlin.jvm.JvmSynthetic
public inline fun common.Common.ManyUetrConnection.copy(block: `common`.ManyUetrConnectionKt.Dsl.() -> kotlin.Unit): common.Common.ManyUetrConnection =
  `common`.ManyUetrConnectionKt.Dsl._create(this.toBuilder()).apply { block() }._build()

