// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: webhook_api/v1/webhook.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package webhook_api.v1;

@kotlin.jvm.JvmName("-initializecreateScheduledReactionResponse")
public inline fun createScheduledReactionResponse(block: webhook_api.v1.CreateScheduledReactionResponseKt.Dsl.() -> kotlin.Unit): webhook_api.v1.WebhookOuterClass.CreateScheduledReactionResponse =
  webhook_api.v1.CreateScheduledReactionResponseKt.Dsl._create(webhook_api.v1.WebhookOuterClass.CreateScheduledReactionResponse.newBuilder()).apply { block() }._build()
/**
 * ```
 * [Example]
 * {
 * "scheduled_reaction": {
 * "id": "REACTION_ID2",
 * "name": "REACTION_NAME2",
 * "reaction_type": 1,
 * "cron_tab": "*****",
 * "reaction_payload": {
 * "create_contract_payload": {
 * "sender_wallet_id": "SENDER_WALLET_ID2",
 * "commitments": [
 * {
 * "sender": "zSenderPublicKey",
 * "recipient": "zRecipientPublicKey",
 * "amount": {
 * "currency_code":"USD",
 * "amount":100,
 * "decimals":2
 * }
 * },
 * {
 * "sender": "zSenderPublicKey2",
 * "recipient": "zRecipientPublicKey2",
 * "amount": {
 * "currency_code":"USD",
 * "amount":500,
 * "decimals":2
 * }
 * }
 * ],
 * "conditions": [
 * {
 * "condition": {
 * "hash": {
 * "presenter": "PresenterPublicKey",
 * "hash": "13550350a8681c84"
 * }
 * }
 * },
 * {
 * "condition": {
 * "hash": {
 * "presenter": "PresenterPublicKey",
 * "hash": "13550350a8681c84"
 * }
 * }
 * }
 * ],
 * "timeout_secs": 10000,
 * "memo": "MEMO"
 * }
 * }
 * }
 * }
 * ```
 *
 * Protobuf type `webhook_api.v1.CreateScheduledReactionResponse`
 */
public object CreateScheduledReactionResponseKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: webhook_api.v1.WebhookOuterClass.CreateScheduledReactionResponse.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: webhook_api.v1.WebhookOuterClass.CreateScheduledReactionResponse.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): webhook_api.v1.WebhookOuterClass.CreateScheduledReactionResponse = _builder.build()

    /**
     * ```
     * Scheduled Reaction.
     * ```
     *
     * `.webhook_api.v1.ScheduledReaction scheduled_reaction = 1;`
     */
    public var scheduledReaction: webhook_api.v1.WebhookOuterClass.ScheduledReaction
      @JvmName("getScheduledReaction")
      get() = _builder.getScheduledReaction()
      @JvmName("setScheduledReaction")
      set(value) {
        _builder.setScheduledReaction(value)
      }
    /**
     * ```
     * Scheduled Reaction.
     * ```
     *
     * `.webhook_api.v1.ScheduledReaction scheduled_reaction = 1;`
     */
    public fun clearScheduledReaction() {
      _builder.clearScheduledReaction()
    }
    /**
     * ```
     * Scheduled Reaction.
     * ```
     *
     * `.webhook_api.v1.ScheduledReaction scheduled_reaction = 1;`
     * @return Whether the scheduledReaction field is set.
     */
    public fun hasScheduledReaction(): kotlin.Boolean {
      return _builder.hasScheduledReaction()
    }
    public val CreateScheduledReactionResponseKt.Dsl.scheduledReactionOrNull: webhook_api.v1.WebhookOuterClass.ScheduledReaction?
      get() = _builder.scheduledReactionOrNull
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun webhook_api.v1.WebhookOuterClass.CreateScheduledReactionResponse.copy(block: `webhook_api.v1`.CreateScheduledReactionResponseKt.Dsl.() -> kotlin.Unit): webhook_api.v1.WebhookOuterClass.CreateScheduledReactionResponse =
  `webhook_api.v1`.CreateScheduledReactionResponseKt.Dsl._create(this.toBuilder()).apply { block() }._build()

public val webhook_api.v1.WebhookOuterClass.CreateScheduledReactionResponseOrBuilder.scheduledReactionOrNull: webhook_api.v1.WebhookOuterClass.ScheduledReaction?
  get() = if (hasScheduledReaction()) getScheduledReaction() else null

