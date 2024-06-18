// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: registry_api/v1/registry.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package registry_api.v1;

@kotlin.jvm.JvmName("-initializeresolutionMetadata")
public inline fun resolutionMetadata(block: registry_api.v1.ResolutionMetadataKt.Dsl.() -> kotlin.Unit): registry_api.v1.Registry.ResolutionMetadata =
  registry_api.v1.ResolutionMetadataKt.Dsl._create(registry_api.v1.Registry.ResolutionMetadata.newBuilder()).apply { block() }._build()
/**
 * ```
 * [Example]
 * {
 * "contentType": "MEDIA_TYPE",
 * "duration": 1000,
 * "didUrl": {
 * "did": "did:method-name:zDIDMultibase58Encoded",
 * "methodName": "EXAMPLE_METHOD_NAME",
 * "methodSpecificId": "METHOD_SPECIFIC_IDENTIFIER"
 * },
 * "error": "OPTIONAL_ERROR_MESSAGE"
 * }
 * ```
 *
 * Protobuf type `registry_api.v1.ResolutionMetadata`
 */
public object ResolutionMetadataKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: registry_api.v1.Registry.ResolutionMetadata.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: registry_api.v1.Registry.ResolutionMetadata.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): registry_api.v1.Registry.ResolutionMetadata = _builder.build()

    /**
     * ```
     * Media type of the W3C DID Document.
     * ```
     *
     * `optional string contentType = 1;`
     */
    public var contentType: kotlin.String
      @JvmName("getContentType")
      get() = _builder.getContentType()
      @JvmName("setContentType")
      set(value) {
        _builder.setContentType(value)
      }
    /**
     * ```
     * Media type of the W3C DID Document.
     * ```
     *
     * `optional string contentType = 1;`
     */
    public fun clearContentType() {
      _builder.clearContentType()
    }
    /**
     * ```
     * Media type of the W3C DID Document.
     * ```
     *
     * `optional string contentType = 1;`
     * @return Whether the contentType field is set.
     */
    public fun hasContentType(): kotlin.Boolean {
      return _builder.hasContentType()
    }

    /**
     * ```
     * Duration of the Resolution.
     * ```
     *
     * `optional int64 duration = 2;`
     */
    public var duration: kotlin.Long
      @JvmName("getDuration")
      get() = _builder.getDuration()
      @JvmName("setDuration")
      set(value) {
        _builder.setDuration(value)
      }
    /**
     * ```
     * Duration of the Resolution.
     * ```
     *
     * `optional int64 duration = 2;`
     */
    public fun clearDuration() {
      _builder.clearDuration()
    }
    /**
     * ```
     * Duration of the Resolution.
     * ```
     *
     * `optional int64 duration = 2;`
     * @return Whether the duration field is set.
     */
    public fun hasDuration(): kotlin.Boolean {
      return _builder.hasDuration()
    }

    /**
     * ```
     * Resolution DID URL.
     * ```
     *
     * `optional .registry_api.v1.ResolutionMetadataDidUrl didUrl = 3;`
     */
    public var didUrl: registry_api.v1.Registry.ResolutionMetadataDidUrl
      @JvmName("getDidUrl")
      get() = _builder.getDidUrl()
      @JvmName("setDidUrl")
      set(value) {
        _builder.setDidUrl(value)
      }
    /**
     * ```
     * Resolution DID URL.
     * ```
     *
     * `optional .registry_api.v1.ResolutionMetadataDidUrl didUrl = 3;`
     */
    public fun clearDidUrl() {
      _builder.clearDidUrl()
    }
    /**
     * ```
     * Resolution DID URL.
     * ```
     *
     * `optional .registry_api.v1.ResolutionMetadataDidUrl didUrl = 3;`
     * @return Whether the didUrl field is set.
     */
    public fun hasDidUrl(): kotlin.Boolean {
      return _builder.hasDidUrl()
    }
    public val ResolutionMetadataKt.Dsl.didUrlOrNull: registry_api.v1.Registry.ResolutionMetadataDidUrl?
      get() = _builder.didUrlOrNull

    /**
     * ```
     * Error Message.
     * ```
     *
     * `optional string error = 4;`
     */
    public var error: kotlin.String
      @JvmName("getError")
      get() = _builder.getError()
      @JvmName("setError")
      set(value) {
        _builder.setError(value)
      }
    /**
     * ```
     * Error Message.
     * ```
     *
     * `optional string error = 4;`
     */
    public fun clearError() {
      _builder.clearError()
    }
    /**
     * ```
     * Error Message.
     * ```
     *
     * `optional string error = 4;`
     * @return Whether the error field is set.
     */
    public fun hasError(): kotlin.Boolean {
      return _builder.hasError()
    }
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun registry_api.v1.Registry.ResolutionMetadata.copy(block: `registry_api.v1`.ResolutionMetadataKt.Dsl.() -> kotlin.Unit): registry_api.v1.Registry.ResolutionMetadata =
  `registry_api.v1`.ResolutionMetadataKt.Dsl._create(this.toBuilder()).apply { block() }._build()

public val registry_api.v1.Registry.ResolutionMetadataOrBuilder.didUrlOrNull: registry_api.v1.Registry.ResolutionMetadataDidUrl?
  get() = if (hasDidUrl()) getDidUrl() else null

