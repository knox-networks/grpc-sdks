// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: webhook_api/v1/webhook.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package webhook_api.v1;

@kotlin.jvm.JvmName("-initializepingPayload")
public inline fun pingPayload(block: webhook_api.v1.PingPayloadKt.Dsl.() -> kotlin.Unit): webhook_api.v1.WebhookOuterClass.PingPayload =
  webhook_api.v1.PingPayloadKt.Dsl._create(webhook_api.v1.WebhookOuterClass.PingPayload.newBuilder()).apply { block() }._build()
/**
 * ```
 * [Example]
 * {
 * "message": "MESSAGE"
 * }
 * ```
 *
 * Protobuf type `webhook_api.v1.PingPayload`
 */
public object PingPayloadKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: webhook_api.v1.WebhookOuterClass.PingPayload.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: webhook_api.v1.WebhookOuterClass.PingPayload.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): webhook_api.v1.WebhookOuterClass.PingPayload = _builder.build()

    /**
     * ```
     * Message payload
     * ```
     *
     * `string message = 1;`
     */
    public var message: kotlin.String
      @JvmName("getMessage")
      get() = _builder.getMessage()
      @JvmName("setMessage")
      set(value) {
        _builder.setMessage(value)
      }
    /**
     * ```
     * Message payload
     * ```
     *
     * `string message = 1;`
     */
    public fun clearMessage() {
      _builder.clearMessage()
    }
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun webhook_api.v1.WebhookOuterClass.PingPayload.copy(block: `webhook_api.v1`.PingPayloadKt.Dsl.() -> kotlin.Unit): webhook_api.v1.WebhookOuterClass.PingPayload =
  `webhook_api.v1`.PingPayloadKt.Dsl._create(this.toBuilder()).apply { block() }._build()

