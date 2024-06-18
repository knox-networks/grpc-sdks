// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: google/protobuf/struct.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package com.google.protobuf;

@kotlin.jvm.JvmName("-initializestruct")
public inline fun struct(block: com.google.protobuf.StructKt.Dsl.() -> kotlin.Unit): com.google.protobuf.Struct =
  com.google.protobuf.StructKt.Dsl._create(com.google.protobuf.Struct.newBuilder()).apply { block() }._build()
/**
 * ```
 * `Struct` represents a structured data value, consisting of fields
 * which map to dynamically typed values. In some languages, `Struct`
 * might be supported by a native representation. For example, in
 * scripting languages like JS a struct is represented as an
 * object. The details of that representation are described together
 * with the proto support for the language.
 *
 * The JSON representation for `Struct` is JSON object.
 * ```
 *
 * Protobuf type `google.protobuf.Struct`
 */
public object StructKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: com.google.protobuf.Struct.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: com.google.protobuf.Struct.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): com.google.protobuf.Struct = _builder.build()

    /**
     * An uninstantiable, behaviorless type to represent the field in
     * generics.
     */
    @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
    public class FieldsProxy private constructor() : com.google.protobuf.kotlin.DslProxy()
    /**
     * ```
     * Unordered map of dynamically typed values.
     * ```
     *
     * `map<string, .google.protobuf.Value> fields = 1;`
     */
     public val fields: com.google.protobuf.kotlin.DslMap<kotlin.String, com.google.protobuf.Value, FieldsProxy>
      @kotlin.jvm.JvmSynthetic
      @JvmName("getFieldsMap")
      get() = com.google.protobuf.kotlin.DslMap(
        _builder.getFieldsMap()
      )
    /**
     * ```
     * Unordered map of dynamically typed values.
     * ```
     *
     * `map<string, .google.protobuf.Value> fields = 1;`
     */
    @JvmName("putFields")
    public fun com.google.protobuf.kotlin.DslMap<kotlin.String, com.google.protobuf.Value, FieldsProxy>
      .put(key: kotlin.String, value: com.google.protobuf.Value) {
         _builder.putFields(key, value)
       }
    /**
     * ```
     * Unordered map of dynamically typed values.
     * ```
     *
     * `map<string, .google.protobuf.Value> fields = 1;`
     */
    @kotlin.jvm.JvmSynthetic
    @JvmName("setFields")
    @Suppress("NOTHING_TO_INLINE")
    public inline operator fun com.google.protobuf.kotlin.DslMap<kotlin.String, com.google.protobuf.Value, FieldsProxy>
      .set(key: kotlin.String, value: com.google.protobuf.Value) {
         put(key, value)
       }
    /**
     * ```
     * Unordered map of dynamically typed values.
     * ```
     *
     * `map<string, .google.protobuf.Value> fields = 1;`
     */
    @kotlin.jvm.JvmSynthetic
    @JvmName("removeFields")
    public fun com.google.protobuf.kotlin.DslMap<kotlin.String, com.google.protobuf.Value, FieldsProxy>
      .remove(key: kotlin.String) {
         _builder.removeFields(key)
       }
    /**
     * ```
     * Unordered map of dynamically typed values.
     * ```
     *
     * `map<string, .google.protobuf.Value> fields = 1;`
     */
    @kotlin.jvm.JvmSynthetic
    @JvmName("putAllFields")
    public fun com.google.protobuf.kotlin.DslMap<kotlin.String, com.google.protobuf.Value, FieldsProxy>
      .putAll(map: kotlin.collections.Map<kotlin.String, com.google.protobuf.Value>) {
         _builder.putAllFields(map)
       }
    /**
     * ```
     * Unordered map of dynamically typed values.
     * ```
     *
     * `map<string, .google.protobuf.Value> fields = 1;`
     */
    @kotlin.jvm.JvmSynthetic
    @JvmName("clearFields")
    public fun com.google.protobuf.kotlin.DslMap<kotlin.String, com.google.protobuf.Value, FieldsProxy>
      .clear() {
         _builder.clearFields()
       }
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun com.google.protobuf.Struct.copy(block: `com.google.protobuf`.StructKt.Dsl.() -> kotlin.Unit): com.google.protobuf.Struct =
  `com.google.protobuf`.StructKt.Dsl._create(this.toBuilder()).apply { block() }._build()

