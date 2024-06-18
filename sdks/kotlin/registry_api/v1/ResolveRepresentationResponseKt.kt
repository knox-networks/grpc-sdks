// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: registry_api/v1/registry.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package registry_api.v1;

@kotlin.jvm.JvmName("-initializeresolveRepresentationResponse")
public inline fun resolveRepresentationResponse(block: registry_api.v1.ResolveRepresentationResponseKt.Dsl.() -> kotlin.Unit): registry_api.v1.Registry.ResolveRepresentationResponse =
  registry_api.v1.ResolveRepresentationResponseKt.Dsl._create(registry_api.v1.Registry.ResolveRepresentationResponse.newBuilder()).apply { block() }._build()
/**
 * ```
 * [Example]
 * {
 * "didDocumentStream": "DID_DOCUMENT_STREAM",
 * "didResolutionMetadata": {
 * "contentType": "MEDIA_TYPE",
 * "duration": 1000,
 * "didUrl": {
 * "did": "did:method-name:zDIDMultibase58Encoded",
 * "methodName": "EXAMPLE_METHOD_NAME",
 * "methodSpecificId": "METHOD_SPECIFIC_IDENTIFIER"
 * },
 * "error": "OPTIONAL_ERROR_MESSAGE"
 * },
 * "didDocumentMetadata": { 
 * "created": { 
 * "seconds": 0, 
 * "nanos": 0 
 * }, 
 * "updated": { 
 * "seconds": 0, 
 * "nanos": 0 
 * }
 * }
 * }
 * ```
 *
 * Protobuf type `registry_api.v1.ResolveRepresentationResponse`
 */
public object ResolveRepresentationResponseKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: registry_api.v1.Registry.ResolveRepresentationResponse.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: registry_api.v1.Registry.ResolveRepresentationResponse.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): registry_api.v1.Registry.ResolveRepresentationResponse = _builder.build()

    /**
     * ```
     * A set of data containing mechanisms to verify the DID and discover related services.
     * ```
     *
     * `string didDocumentStream = 1;`
     */
    public var didDocumentStream: kotlin.String
      @JvmName("getDidDocumentStream")
      get() = _builder.getDidDocumentStream()
      @JvmName("setDidDocumentStream")
      set(value) {
        _builder.setDidDocumentStream(value)
      }
    /**
     * ```
     * A set of data containing mechanisms to verify the DID and discover related services.
     * ```
     *
     * `string didDocumentStream = 1;`
     */
    public fun clearDidDocumentStream() {
      _builder.clearDidDocumentStream()
    }

    /**
     * ```
     * Resolution Metadata object, with defined timestamps and content type.
     * ```
     *
     * `.registry_api.v1.ResolutionMetadata didResolutionMetadata = 2;`
     */
    public var didResolutionMetadata: registry_api.v1.Registry.ResolutionMetadata
      @JvmName("getDidResolutionMetadata")
      get() = _builder.getDidResolutionMetadata()
      @JvmName("setDidResolutionMetadata")
      set(value) {
        _builder.setDidResolutionMetadata(value)
      }
    /**
     * ```
     * Resolution Metadata object, with defined timestamps and content type.
     * ```
     *
     * `.registry_api.v1.ResolutionMetadata didResolutionMetadata = 2;`
     */
    public fun clearDidResolutionMetadata() {
      _builder.clearDidResolutionMetadata()
    }
    /**
     * ```
     * Resolution Metadata object, with defined timestamps and content type.
     * ```
     *
     * `.registry_api.v1.ResolutionMetadata didResolutionMetadata = 2;`
     * @return Whether the didResolutionMetadata field is set.
     */
    public fun hasDidResolutionMetadata(): kotlin.Boolean {
      return _builder.hasDidResolutionMetadata()
    }
    public val ResolveRepresentationResponseKt.Dsl.didResolutionMetadataOrNull: registry_api.v1.Registry.ResolutionMetadata?
      get() = _builder.didResolutionMetadataOrNull

    /**
     * ```
     * This structure contains metadata about the DID document contained in the didDocument property
     * ```
     *
     * `.registry_api.v1.DidDocumentMetadata didDocumentMetadata = 3;`
     */
    public var didDocumentMetadata: registry_api.v1.Registry.DidDocumentMetadata
      @JvmName("getDidDocumentMetadata")
      get() = _builder.getDidDocumentMetadata()
      @JvmName("setDidDocumentMetadata")
      set(value) {
        _builder.setDidDocumentMetadata(value)
      }
    /**
     * ```
     * This structure contains metadata about the DID document contained in the didDocument property
     * ```
     *
     * `.registry_api.v1.DidDocumentMetadata didDocumentMetadata = 3;`
     */
    public fun clearDidDocumentMetadata() {
      _builder.clearDidDocumentMetadata()
    }
    /**
     * ```
     * This structure contains metadata about the DID document contained in the didDocument property
     * ```
     *
     * `.registry_api.v1.DidDocumentMetadata didDocumentMetadata = 3;`
     * @return Whether the didDocumentMetadata field is set.
     */
    public fun hasDidDocumentMetadata(): kotlin.Boolean {
      return _builder.hasDidDocumentMetadata()
    }
    public val ResolveRepresentationResponseKt.Dsl.didDocumentMetadataOrNull: registry_api.v1.Registry.DidDocumentMetadata?
      get() = _builder.didDocumentMetadataOrNull
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun registry_api.v1.Registry.ResolveRepresentationResponse.copy(block: `registry_api.v1`.ResolveRepresentationResponseKt.Dsl.() -> kotlin.Unit): registry_api.v1.Registry.ResolveRepresentationResponse =
  `registry_api.v1`.ResolveRepresentationResponseKt.Dsl._create(this.toBuilder()).apply { block() }._build()

public val registry_api.v1.Registry.ResolveRepresentationResponseOrBuilder.didResolutionMetadataOrNull: registry_api.v1.Registry.ResolutionMetadata?
  get() = if (hasDidResolutionMetadata()) getDidResolutionMetadata() else null

public val registry_api.v1.Registry.ResolveRepresentationResponseOrBuilder.didDocumentMetadataOrNull: registry_api.v1.Registry.DidDocumentMetadata?
  get() = if (hasDidDocumentMetadata()) getDidDocumentMetadata() else null

