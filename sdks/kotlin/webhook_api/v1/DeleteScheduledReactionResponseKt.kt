// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: webhook_api/v1/webhook.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package webhook_api.v1;

@kotlin.jvm.JvmName("-initializedeleteScheduledReactionResponse")
public inline fun deleteScheduledReactionResponse(block: webhook_api.v1.DeleteScheduledReactionResponseKt.Dsl.() -> kotlin.Unit): webhook_api.v1.WebhookOuterClass.DeleteScheduledReactionResponse =
  webhook_api.v1.DeleteScheduledReactionResponseKt.Dsl._create(webhook_api.v1.WebhookOuterClass.DeleteScheduledReactionResponse.newBuilder()).apply { block() }._build()
/**
 * ```
 * [Example]
 * {}
 * ```
 *
 * Protobuf type `webhook_api.v1.DeleteScheduledReactionResponse`
 */
public object DeleteScheduledReactionResponseKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: webhook_api.v1.WebhookOuterClass.DeleteScheduledReactionResponse.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: webhook_api.v1.WebhookOuterClass.DeleteScheduledReactionResponse.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): webhook_api.v1.WebhookOuterClass.DeleteScheduledReactionResponse = _builder.build()
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun webhook_api.v1.WebhookOuterClass.DeleteScheduledReactionResponse.copy(block: `webhook_api.v1`.DeleteScheduledReactionResponseKt.Dsl.() -> kotlin.Unit): webhook_api.v1.WebhookOuterClass.DeleteScheduledReactionResponse =
  `webhook_api.v1`.DeleteScheduledReactionResponseKt.Dsl._create(this.toBuilder()).apply { block() }._build()

