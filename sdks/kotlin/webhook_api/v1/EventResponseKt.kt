// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: webhook_api/v1/webhook.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package webhook_api.v1;

@kotlin.jvm.JvmName("-initializeeventResponse")
public inline fun eventResponse(block: webhook_api.v1.EventResponseKt.Dsl.() -> kotlin.Unit): webhook_api.v1.WebhookOuterClass.EventResponse =
  webhook_api.v1.EventResponseKt.Dsl._create(webhook_api.v1.WebhookOuterClass.EventResponse.newBuilder()).apply { block() }._build()
/**
 * ```
 * [Example]
 * {}
 * ```
 *
 * Protobuf type `webhook_api.v1.EventResponse`
 */
public object EventResponseKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: webhook_api.v1.WebhookOuterClass.EventResponse.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: webhook_api.v1.WebhookOuterClass.EventResponse.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): webhook_api.v1.WebhookOuterClass.EventResponse = _builder.build()
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun webhook_api.v1.WebhookOuterClass.EventResponse.copy(block: `webhook_api.v1`.EventResponseKt.Dsl.() -> kotlin.Unit): webhook_api.v1.WebhookOuterClass.EventResponse =
  `webhook_api.v1`.EventResponseKt.Dsl._create(this.toBuilder()).apply { block() }._build()
