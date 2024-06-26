// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: business_api/v1/business.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package business_api.v1;

@kotlin.jvm.JvmName("-initializelistResponse")
public inline fun listResponse(block: business_api.v1.ListResponseKt.Dsl.() -> kotlin.Unit): business_api.v1.BusinessOuterClass.ListResponse =
  business_api.v1.ListResponseKt.Dsl._create(business_api.v1.BusinessOuterClass.ListResponse.newBuilder()).apply { block() }._build()
/**
 * ```
 * [Example]
 * {
 * "list": [
 * "item1",
 * "item2",
 * "item3"
 * ]
 * }
 * ```
 *
 * Protobuf type `business_api.v1.ListResponse`
 */
public object ListResponseKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: business_api.v1.BusinessOuterClass.ListResponse.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: business_api.v1.BusinessOuterClass.ListResponse.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): business_api.v1.BusinessOuterClass.ListResponse = _builder.build()

    /**
     * An uninstantiable, behaviorless type to represent the field in
     * generics.
     */
    @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
    public class ListProxy private constructor() : com.google.protobuf.kotlin.DslProxy()
    /**
     * ```
     * List.
     * ```
     *
     * `repeated string list = 1;`
     * @return A list containing the list.
     */
    public val list: com.google.protobuf.kotlin.DslList<kotlin.String, ListProxy>
      @kotlin.jvm.JvmSynthetic
      get() = com.google.protobuf.kotlin.DslList(
        _builder.getListList()
      )
    /**
     * ```
     * List.
     * ```
     *
     * `repeated string list = 1;`
     * @param value The list to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("addList")
    public fun com.google.protobuf.kotlin.DslList<kotlin.String, ListProxy>.add(value: kotlin.String) {
      _builder.addList(value)
    }
    /**
     * ```
     * List.
     * ```
     *
     * `repeated string list = 1;`
     * @param value The list to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("plusAssignList")
    @Suppress("NOTHING_TO_INLINE")
    public inline operator fun com.google.protobuf.kotlin.DslList<kotlin.String, ListProxy>.plusAssign(value: kotlin.String) {
      add(value)
    }
    /**
     * ```
     * List.
     * ```
     *
     * `repeated string list = 1;`
     * @param values The list to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("addAllList")
    public fun com.google.protobuf.kotlin.DslList<kotlin.String, ListProxy>.addAll(values: kotlin.collections.Iterable<kotlin.String>) {
      _builder.addAllList(values)
    }
    /**
     * ```
     * List.
     * ```
     *
     * `repeated string list = 1;`
     * @param values The list to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("plusAssignAllList")
    @Suppress("NOTHING_TO_INLINE")
    public inline operator fun com.google.protobuf.kotlin.DslList<kotlin.String, ListProxy>.plusAssign(values: kotlin.collections.Iterable<kotlin.String>) {
      addAll(values)
    }
    /**
     * ```
     * List.
     * ```
     *
     * `repeated string list = 1;`
     * @param index The index to set the value at.
     * @param value The list to set.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("setList")
    public operator fun com.google.protobuf.kotlin.DslList<kotlin.String, ListProxy>.set(index: kotlin.Int, value: kotlin.String) {
      _builder.setList(index, value)
    }/**
     * ```
     * List.
     * ```
     *
     * `repeated string list = 1;`
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("clearList")
    public fun com.google.protobuf.kotlin.DslList<kotlin.String, ListProxy>.clear() {
      _builder.clearList()
    }}
}
@kotlin.jvm.JvmSynthetic
public inline fun business_api.v1.BusinessOuterClass.ListResponse.copy(block: `business_api.v1`.ListResponseKt.Dsl.() -> kotlin.Unit): business_api.v1.BusinessOuterClass.ListResponse =
  `business_api.v1`.ListResponseKt.Dsl._create(this.toBuilder()).apply { block() }._build()

