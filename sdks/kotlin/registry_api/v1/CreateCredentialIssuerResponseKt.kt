// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: registry_api/v1/registry.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package registry_api.v1;

@kotlin.jvm.JvmName("-initializecreateCredentialIssuerResponse")
public inline fun createCredentialIssuerResponse(block: registry_api.v1.CreateCredentialIssuerResponseKt.Dsl.() -> kotlin.Unit): registry_api.v1.Registry.CreateCredentialIssuerResponse =
  registry_api.v1.CreateCredentialIssuerResponseKt.Dsl._create(registry_api.v1.Registry.CreateCredentialIssuerResponse.newBuilder()).apply { block() }._build()
/**
 * ```
 * [Example]
 * {
 * "ok": true
 * }
 * ```
 *
 * Protobuf type `registry_api.v1.CreateCredentialIssuerResponse`
 */
public object CreateCredentialIssuerResponseKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: registry_api.v1.Registry.CreateCredentialIssuerResponse.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: registry_api.v1.Registry.CreateCredentialIssuerResponse.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): registry_api.v1.Registry.CreateCredentialIssuerResponse = _builder.build()

    /**
     * `bool ok = 1;`
     */
    public var ok: kotlin.Boolean
      @JvmName("getOk")
      get() = _builder.getOk()
      @JvmName("setOk")
      set(value) {
        _builder.setOk(value)
      }
    /**
     * `bool ok = 1;`
     */
    public fun clearOk() {
      _builder.clearOk()
    }
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun registry_api.v1.Registry.CreateCredentialIssuerResponse.copy(block: `registry_api.v1`.CreateCredentialIssuerResponseKt.Dsl.() -> kotlin.Unit): registry_api.v1.Registry.CreateCredentialIssuerResponse =
  `registry_api.v1`.CreateCredentialIssuerResponseKt.Dsl._create(this.toBuilder()).apply { block() }._build()

