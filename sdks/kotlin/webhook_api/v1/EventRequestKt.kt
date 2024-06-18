// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: webhook_api/v1/webhook.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package webhook_api.v1;

@kotlin.jvm.JvmName("-initializeeventRequest")
public inline fun eventRequest(block: webhook_api.v1.EventRequestKt.Dsl.() -> kotlin.Unit): webhook_api.v1.WebhookOuterClass.EventRequest =
  webhook_api.v1.EventRequestKt.Dsl._create(webhook_api.v1.WebhookOuterClass.EventRequest.newBuilder()).apply { block() }._build()
/**
 * ```
 * [Example]
 * {
 * "event": {
 * "event_type": 0,
 * "signature": "SignatureLenVariesByAlgorithm",
 * "event_id": "EVENT_ID",
 * "delivery_date": "2007-01-02T15:04:05Z",
 * "owner_id": "eyJwtHeader.eyJwtPayload.Secret",
 * "payload": {
 * "wallet_created_payload": {
 * "name": "NAME",
 * "did": "did:method-name:zDIDMultibase58Encoded",
 * "created": "2006-01-02T15:04:05Z"
 * }
 * }
 * }
 * }
 * ```
 *
 * Protobuf type `webhook_api.v1.EventRequest`
 */
public object EventRequestKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: webhook_api.v1.WebhookOuterClass.EventRequest.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: webhook_api.v1.WebhookOuterClass.EventRequest.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): webhook_api.v1.WebhookOuterClass.EventRequest = _builder.build()

    /**
     * ```
     * Event.
     * ```
     *
     * `.webhook_api.v1.Event event = 1;`
     */
    public var event: webhook_api.v1.WebhookOuterClass.Event
      @JvmName("getEvent")
      get() = _builder.getEvent()
      @JvmName("setEvent")
      set(value) {
        _builder.setEvent(value)
      }
    /**
     * ```
     * Event.
     * ```
     *
     * `.webhook_api.v1.Event event = 1;`
     */
    public fun clearEvent() {
      _builder.clearEvent()
    }
    /**
     * ```
     * Event.
     * ```
     *
     * `.webhook_api.v1.Event event = 1;`
     * @return Whether the event field is set.
     */
    public fun hasEvent(): kotlin.Boolean {
      return _builder.hasEvent()
    }
    public val EventRequestKt.Dsl.eventOrNull: webhook_api.v1.WebhookOuterClass.Event?
      get() = _builder.eventOrNull
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun webhook_api.v1.WebhookOuterClass.EventRequest.copy(block: `webhook_api.v1`.EventRequestKt.Dsl.() -> kotlin.Unit): webhook_api.v1.WebhookOuterClass.EventRequest =
  `webhook_api.v1`.EventRequestKt.Dsl._create(this.toBuilder()).apply { block() }._build()

public val webhook_api.v1.WebhookOuterClass.EventRequestOrBuilder.eventOrNull: webhook_api.v1.WebhookOuterClass.Event?
  get() = if (hasEvent()) getEvent() else null

