// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: webhook_api/v1/webhook.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package webhook_api.v1;

@kotlin.jvm.JvmName("-initializelistWebhooksRequest")
public inline fun listWebhooksRequest(block: webhook_api.v1.ListWebhooksRequestKt.Dsl.() -> kotlin.Unit): webhook_api.v1.WebhookOuterClass.ListWebhooksRequest =
  webhook_api.v1.ListWebhooksRequestKt.Dsl._create(webhook_api.v1.WebhookOuterClass.ListWebhooksRequest.newBuilder()).apply { block() }._build()
/**
 * ```
 * [Example]
 * {
 * "api_key": "API_KEY",
 * "page": 0,
 * "page_size": 10000
 * }
 * ```
 *
 * Protobuf type `webhook_api.v1.ListWebhooksRequest`
 */
public object ListWebhooksRequestKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: webhook_api.v1.WebhookOuterClass.ListWebhooksRequest.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: webhook_api.v1.WebhookOuterClass.ListWebhooksRequest.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): webhook_api.v1.WebhookOuterClass.ListWebhooksRequest = _builder.build()

    /**
     * ```
     * API Key as a JWT
     * ```
     *
     * `string api_key = 1;`
     */
    public var apiKey: kotlin.String
      @JvmName("getApiKey")
      get() = _builder.getApiKey()
      @JvmName("setApiKey")
      set(value) {
        _builder.setApiKey(value)
      }
    /**
     * ```
     * API Key as a JWT
     * ```
     *
     * `string api_key = 1;`
     */
    public fun clearApiKey() {
      _builder.clearApiKey()
    }

    /**
     * ```
     * Page number for pagination.
     * ```
     *
     * `int32 page = 2;`
     */
    public var page: kotlin.Int
      @JvmName("getPage")
      get() = _builder.getPage()
      @JvmName("setPage")
      set(value) {
        _builder.setPage(value)
      }
    /**
     * ```
     * Page number for pagination.
     * ```
     *
     * `int32 page = 2;`
     */
    public fun clearPage() {
      _builder.clearPage()
    }

    /**
     * ```
     * Size of the page for pagination.
     * ```
     *
     * `int32 page_size = 3;`
     */
    public var pageSize: kotlin.Int
      @JvmName("getPageSize")
      get() = _builder.getPageSize()
      @JvmName("setPageSize")
      set(value) {
        _builder.setPageSize(value)
      }
    /**
     * ```
     * Size of the page for pagination.
     * ```
     *
     * `int32 page_size = 3;`
     */
    public fun clearPageSize() {
      _builder.clearPageSize()
    }
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun webhook_api.v1.WebhookOuterClass.ListWebhooksRequest.copy(block: `webhook_api.v1`.ListWebhooksRequestKt.Dsl.() -> kotlin.Unit): webhook_api.v1.WebhookOuterClass.ListWebhooksRequest =
  `webhook_api.v1`.ListWebhooksRequestKt.Dsl._create(this.toBuilder()).apply { block() }._build()

