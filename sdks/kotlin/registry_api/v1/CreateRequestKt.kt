// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: registry_api/v1/registry.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package registry_api.v1;

@kotlin.jvm.JvmName("-initializecreateRequest")
public inline fun createRequest(block: registry_api.v1.CreateRequestKt.Dsl.() -> kotlin.Unit): registry_api.v1.Registry.CreateRequest =
  registry_api.v1.CreateRequestKt.Dsl._create(registry_api.v1.Registry.CreateRequest.newBuilder()).apply { block() }._build()
/**
 * ```
 * [Example]
 * {
 * "did": "did:method-name:zDIDMultibase58Encoded",
 * "document": "DOCUMENT"
 * }
 * ```
 *
 * Protobuf type `registry_api.v1.CreateRequest`
 */
public object CreateRequestKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: registry_api.v1.Registry.CreateRequest.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: registry_api.v1.Registry.CreateRequest.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): registry_api.v1.Registry.CreateRequest = _builder.build()

    /**
     * ```
     * W3C Decentralized Identifier (DID) of the wallet.
     * ```
     *
     * `string did = 1;`
     */
    public var did: kotlin.String
      @JvmName("getDid")
      get() = _builder.getDid()
      @JvmName("setDid")
      set(value) {
        _builder.setDid(value)
      }
    /**
     * ```
     * W3C Decentralized Identifier (DID) of the wallet.
     * ```
     *
     * `string did = 1;`
     */
    public fun clearDid() {
      _builder.clearDid()
    }

    /**
     * ```
     * A set of data containing mechanisms to verify the DID and discover related services. 
     * ```
     *
     * `string document = 2;`
     */
    public var document: kotlin.String
      @JvmName("getDocument")
      get() = _builder.getDocument()
      @JvmName("setDocument")
      set(value) {
        _builder.setDocument(value)
      }
    /**
     * ```
     * A set of data containing mechanisms to verify the DID and discover related services. 
     * ```
     *
     * `string document = 2;`
     */
    public fun clearDocument() {
      _builder.clearDocument()
    }
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun registry_api.v1.Registry.CreateRequest.copy(block: `registry_api.v1`.CreateRequestKt.Dsl.() -> kotlin.Unit): registry_api.v1.Registry.CreateRequest =
  `registry_api.v1`.CreateRequestKt.Dsl._create(this.toBuilder()).apply { block() }._build()
