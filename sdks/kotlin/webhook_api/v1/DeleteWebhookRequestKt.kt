// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: webhook_api/v1/webhook.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package webhook_api.v1;

@kotlin.jvm.JvmName("-initializedeleteWebhookRequest")
public inline fun deleteWebhookRequest(block: webhook_api.v1.DeleteWebhookRequestKt.Dsl.() -> kotlin.Unit): webhook_api.v1.WebhookOuterClass.DeleteWebhookRequest =
  webhook_api.v1.DeleteWebhookRequestKt.Dsl._create(webhook_api.v1.WebhookOuterClass.DeleteWebhookRequest.newBuilder()).apply { block() }._build()
/**
 * ```
 * [Example]
 * {
 * "api_key": "API_KEY",
 * "webhook_id": "WEBHOOK_ID"
 * }
 * ```
 *
 * Protobuf type `webhook_api.v1.DeleteWebhookRequest`
 */
public object DeleteWebhookRequestKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: webhook_api.v1.WebhookOuterClass.DeleteWebhookRequest.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: webhook_api.v1.WebhookOuterClass.DeleteWebhookRequest.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): webhook_api.v1.WebhookOuterClass.DeleteWebhookRequest = _builder.build()

    /**
     * ```
     * API Key, JWT.
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
     * API Key, JWT.
     * ```
     *
     * `string api_key = 1;`
     */
    public fun clearApiKey() {
      _builder.clearApiKey()
    }

    /**
     * ```
     * Webhook ID.
     * ```
     *
     * `string webhook_id = 2;`
     */
    public var webhookId: kotlin.String
      @JvmName("getWebhookId")
      get() = _builder.getWebhookId()
      @JvmName("setWebhookId")
      set(value) {
        _builder.setWebhookId(value)
      }
    /**
     * ```
     * Webhook ID.
     * ```
     *
     * `string webhook_id = 2;`
     */
    public fun clearWebhookId() {
      _builder.clearWebhookId()
    }
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun webhook_api.v1.WebhookOuterClass.DeleteWebhookRequest.copy(block: `webhook_api.v1`.DeleteWebhookRequestKt.Dsl.() -> kotlin.Unit): webhook_api.v1.WebhookOuterClass.DeleteWebhookRequest =
  `webhook_api.v1`.DeleteWebhookRequestKt.Dsl._create(this.toBuilder()).apply { block() }._build()

