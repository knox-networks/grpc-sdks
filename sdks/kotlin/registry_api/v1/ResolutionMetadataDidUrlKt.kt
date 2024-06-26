// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: registry_api/v1/registry.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package registry_api.v1;

@kotlin.jvm.JvmName("-initializeresolutionMetadataDidUrl")
public inline fun resolutionMetadataDidUrl(block: registry_api.v1.ResolutionMetadataDidUrlKt.Dsl.() -> kotlin.Unit): registry_api.v1.Registry.ResolutionMetadataDidUrl =
  registry_api.v1.ResolutionMetadataDidUrlKt.Dsl._create(registry_api.v1.Registry.ResolutionMetadataDidUrl.newBuilder()).apply { block() }._build()
/**
 * ```
 * [Example]
 * {
 * "did": "did:method-name:zDIDMultibase58Encoded",
 * "methodName": "EXAMPLE_METHOD_NAME",
 * "methodSpecificId": "METHOD_SPECIFIC_IDENTIFIER"
 * }
 * ```
 *
 * Protobuf type `registry_api.v1.ResolutionMetadataDidUrl`
 */
public object ResolutionMetadataDidUrlKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: registry_api.v1.Registry.ResolutionMetadataDidUrl.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: registry_api.v1.Registry.ResolutionMetadataDidUrl.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): registry_api.v1.Registry.ResolutionMetadataDidUrl = _builder.build()

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
     * W3C Decentralized Scheme.
     * ```
     *
     * `string methodName = 2;`
     */
    public var methodName: kotlin.String
      @JvmName("getMethodName")
      get() = _builder.getMethodName()
      @JvmName("setMethodName")
      set(value) {
        _builder.setMethodName(value)
      }
    /**
     * ```
     * W3C Decentralized Scheme.
     * ```
     *
     * `string methodName = 2;`
     */
    public fun clearMethodName() {
      _builder.clearMethodName()
    }

    /**
     * ```
     * Method specific identifier.
     * ```
     *
     * `string methodSpecificId = 3;`
     */
    public var methodSpecificId: kotlin.String
      @JvmName("getMethodSpecificId")
      get() = _builder.getMethodSpecificId()
      @JvmName("setMethodSpecificId")
      set(value) {
        _builder.setMethodSpecificId(value)
      }
    /**
     * ```
     * Method specific identifier.
     * ```
     *
     * `string methodSpecificId = 3;`
     */
    public fun clearMethodSpecificId() {
      _builder.clearMethodSpecificId()
    }
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun registry_api.v1.Registry.ResolutionMetadataDidUrl.copy(block: `registry_api.v1`.ResolutionMetadataDidUrlKt.Dsl.() -> kotlin.Unit): registry_api.v1.Registry.ResolutionMetadataDidUrl =
  `registry_api.v1`.ResolutionMetadataDidUrlKt.Dsl._create(this.toBuilder()).apply { block() }._build()

