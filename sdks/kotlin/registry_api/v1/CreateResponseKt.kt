// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: registry_api/v1/registry.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package registry_api.v1;

@kotlin.jvm.JvmName("-initializecreateResponse")
public inline fun createResponse(block: registry_api.v1.CreateResponseKt.Dsl.() -> kotlin.Unit): registry_api.v1.Registry.CreateResponse =
  registry_api.v1.CreateResponseKt.Dsl._create(registry_api.v1.Registry.CreateResponse.newBuilder()).apply { block() }._build()
/**
 * ```
 * [Example]
 * {}
 * ```
 *
 * Protobuf type `registry_api.v1.CreateResponse`
 */
public object CreateResponseKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: registry_api.v1.Registry.CreateResponse.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: registry_api.v1.Registry.CreateResponse.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): registry_api.v1.Registry.CreateResponse = _builder.build()
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun registry_api.v1.Registry.CreateResponse.copy(block: `registry_api.v1`.CreateResponseKt.Dsl.() -> kotlin.Unit): registry_api.v1.Registry.CreateResponse =
  `registry_api.v1`.CreateResponseKt.Dsl._create(this.toBuilder()).apply { block() }._build()
