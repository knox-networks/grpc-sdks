// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: authority_api/v1/authority.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package authority_api.v1;

@kotlin.jvm.JvmName("-initializeauthorizeRequest")
public inline fun authorizeRequest(block: authority_api.v1.AuthorizeRequestKt.Dsl.() -> kotlin.Unit): authority_api.v1.AuthorityOuterClass.AuthorizeRequest =
  authority_api.v1.AuthorizeRequestKt.Dsl._create(authority_api.v1.AuthorityOuterClass.AuthorizeRequest.newBuilder()).apply { block() }._build()
/**
 * ```
 * [Example]
 * {
 * "promissory_file": "LengthOfFileBytesVaries="
 * }
 * ```
 *
 * Protobuf type `authority_api.v1.AuthorizeRequest`
 */
public object AuthorizeRequestKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: authority_api.v1.AuthorityOuterClass.AuthorizeRequest.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: authority_api.v1.AuthorityOuterClass.AuthorizeRequest.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): authority_api.v1.AuthorityOuterClass.AuthorizeRequest = _builder.build()

    /**
     * ```
     * Digital banknote to authorize.
     * ```
     *
     * `bytes promissory_file = 1;`
     */
    public var promissoryFile: com.google.protobuf.ByteString
      @JvmName("getPromissoryFile")
      get() = _builder.getPromissoryFile()
      @JvmName("setPromissoryFile")
      set(value) {
        _builder.setPromissoryFile(value)
      }
    /**
     * ```
     * Digital banknote to authorize.
     * ```
     *
     * `bytes promissory_file = 1;`
     */
    public fun clearPromissoryFile() {
      _builder.clearPromissoryFile()
    }
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun authority_api.v1.AuthorityOuterClass.AuthorizeRequest.copy(block: `authority_api.v1`.AuthorizeRequestKt.Dsl.() -> kotlin.Unit): authority_api.v1.AuthorityOuterClass.AuthorizeRequest =
  `authority_api.v1`.AuthorizeRequestKt.Dsl._create(this.toBuilder()).apply { block() }._build()

