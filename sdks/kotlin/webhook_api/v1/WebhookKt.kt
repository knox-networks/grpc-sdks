// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: webhook_api/v1/webhook.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package webhook_api.v1;

@kotlin.jvm.JvmName("-initializewebhook")
public inline fun webhook(block: webhook_api.v1.WebhookKt.Dsl.() -> kotlin.Unit): webhook_api.v1.WebhookOuterClass.Webhook =
  webhook_api.v1.WebhookKt.Dsl._create(webhook_api.v1.WebhookOuterClass.Webhook.newBuilder()).apply { block() }._build()
/**
 * ```
 * [Example]
 * {
 * "id": "ID1",
 * "owner_id": "eyJwtHeader.eyJwtPayload.Secret1",
 * "name": "NAME1",
 * "webhook_url": "WEBHOOK_URL1",
 * "supported_events": [
 * 0,
 * 1
 * ],
 * "delivery_method": 0
 * }
 * ```
 *
 * Protobuf type `webhook_api.v1.Webhook`
 */
public object WebhookKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: webhook_api.v1.WebhookOuterClass.Webhook.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: webhook_api.v1.WebhookOuterClass.Webhook.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): webhook_api.v1.WebhookOuterClass.Webhook = _builder.build()

    /**
     * ```
     * Webhook ID.
     * ```
     *
     * `string id = 1;`
     */
    public var id: kotlin.String
      @JvmName("getId")
      get() = _builder.getId()
      @JvmName("setId")
      set(value) {
        _builder.setId(value)
      }
    /**
     * ```
     * Webhook ID.
     * ```
     *
     * `string id = 1;`
     */
    public fun clearId() {
      _builder.clearId()
    }

    /**
     * ```
     * Owner ID.
     * ```
     *
     * `string owner_id = 2;`
     */
    public var ownerId: kotlin.String
      @JvmName("getOwnerId")
      get() = _builder.getOwnerId()
      @JvmName("setOwnerId")
      set(value) {
        _builder.setOwnerId(value)
      }
    /**
     * ```
     * Owner ID.
     * ```
     *
     * `string owner_id = 2;`
     */
    public fun clearOwnerId() {
      _builder.clearOwnerId()
    }

    /**
     * ```
     * Webhook Name.
     * ```
     *
     * `string name = 3;`
     */
    public var name: kotlin.String
      @JvmName("getName")
      get() = _builder.getName()
      @JvmName("setName")
      set(value) {
        _builder.setName(value)
      }
    /**
     * ```
     * Webhook Name.
     * ```
     *
     * `string name = 3;`
     */
    public fun clearName() {
      _builder.clearName()
    }

    /**
     * ```
     * Webhook URL.
     * ```
     *
     * `string webhook_url = 4;`
     */
    public var webhookUrl: kotlin.String
      @JvmName("getWebhookUrl")
      get() = _builder.getWebhookUrl()
      @JvmName("setWebhookUrl")
      set(value) {
        _builder.setWebhookUrl(value)
      }
    /**
     * ```
     * Webhook URL.
     * ```
     *
     * `string webhook_url = 4;`
     */
    public fun clearWebhookUrl() {
      _builder.clearWebhookUrl()
    }

    /**
     * An uninstantiable, behaviorless type to represent the field in
     * generics.
     */
    @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
    public class SupportedEventsProxy private constructor() : com.google.protobuf.kotlin.DslProxy()
    /**
     * ```
     * List of supported Events.
     * ```
     *
     * `repeated .webhook_api.v1.EventType supported_events = 5;`
     */
    public val supportedEvents: com.google.protobuf.kotlin.DslList<webhook_api.v1.WebhookOuterClass.EventType, SupportedEventsProxy>
      @kotlin.jvm.JvmSynthetic
      get() = com.google.protobuf.kotlin.DslList(
        _builder.getSupportedEventsList()
      )
    /**
     * ```
     * List of supported Events.
     * ```
     *
     * `repeated .webhook_api.v1.EventType supported_events = 5;`
     * @param value The supportedEvents to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("addSupportedEvents")
    public fun com.google.protobuf.kotlin.DslList<webhook_api.v1.WebhookOuterClass.EventType, SupportedEventsProxy>.add(value: webhook_api.v1.WebhookOuterClass.EventType) {
      _builder.addSupportedEvents(value)
    }/**
     * ```
     * List of supported Events.
     * ```
     *
     * `repeated .webhook_api.v1.EventType supported_events = 5;`
     * @param value The supportedEvents to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("plusAssignSupportedEvents")
    @Suppress("NOTHING_TO_INLINE")
    public inline operator fun com.google.protobuf.kotlin.DslList<webhook_api.v1.WebhookOuterClass.EventType, SupportedEventsProxy>.plusAssign(value: webhook_api.v1.WebhookOuterClass.EventType) {
      add(value)
    }/**
     * ```
     * List of supported Events.
     * ```
     *
     * `repeated .webhook_api.v1.EventType supported_events = 5;`
     * @param values The supportedEvents to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("addAllSupportedEvents")
    public fun com.google.protobuf.kotlin.DslList<webhook_api.v1.WebhookOuterClass.EventType, SupportedEventsProxy>.addAll(values: kotlin.collections.Iterable<webhook_api.v1.WebhookOuterClass.EventType>) {
      _builder.addAllSupportedEvents(values)
    }/**
     * ```
     * List of supported Events.
     * ```
     *
     * `repeated .webhook_api.v1.EventType supported_events = 5;`
     * @param values The supportedEvents to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("plusAssignAllSupportedEvents")
    @Suppress("NOTHING_TO_INLINE")
    public inline operator fun com.google.protobuf.kotlin.DslList<webhook_api.v1.WebhookOuterClass.EventType, SupportedEventsProxy>.plusAssign(values: kotlin.collections.Iterable<webhook_api.v1.WebhookOuterClass.EventType>) {
      addAll(values)
    }/**
     * ```
     * List of supported Events.
     * ```
     *
     * `repeated .webhook_api.v1.EventType supported_events = 5;`
     * @param index The index to set the value at.
     * @param value The supportedEvents to set.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("setSupportedEvents")
    public operator fun com.google.protobuf.kotlin.DslList<webhook_api.v1.WebhookOuterClass.EventType, SupportedEventsProxy>.set(index: kotlin.Int, value: webhook_api.v1.WebhookOuterClass.EventType) {
      _builder.setSupportedEvents(index, value)
    }/**
     * ```
     * List of supported Events.
     * ```
     *
     * `repeated .webhook_api.v1.EventType supported_events = 5;`
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("clearSupportedEvents")
    public fun com.google.protobuf.kotlin.DslList<webhook_api.v1.WebhookOuterClass.EventType, SupportedEventsProxy>.clear() {
      _builder.clearSupportedEvents()
    }
    /**
     * ```
     * Specification of Delivery method.
     * ```
     *
     * `.webhook_api.v1.DeliveryMethodType delivery_method = 6;`
     */
    public var deliveryMethod: webhook_api.v1.WebhookOuterClass.DeliveryMethodType
      @JvmName("getDeliveryMethod")
      get() = _builder.getDeliveryMethod()
      @JvmName("setDeliveryMethod")
      set(value) {
        _builder.setDeliveryMethod(value)
      }
    public var deliveryMethodValue: kotlin.Int
      @JvmName("getDeliveryMethodValue")
      get() = _builder.getDeliveryMethodValue()
      @JvmName("setDeliveryMethodValue")
      set(value) {
        _builder.setDeliveryMethodValue(value)
      }
    /**
     * ```
     * Specification of Delivery method.
     * ```
     *
     * `.webhook_api.v1.DeliveryMethodType delivery_method = 6;`
     */
    public fun clearDeliveryMethod() {
      _builder.clearDeliveryMethod()
    }
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun webhook_api.v1.WebhookOuterClass.Webhook.copy(block: `webhook_api.v1`.WebhookKt.Dsl.() -> kotlin.Unit): webhook_api.v1.WebhookOuterClass.Webhook =
  `webhook_api.v1`.WebhookKt.Dsl._create(this.toBuilder()).apply { block() }._build()
