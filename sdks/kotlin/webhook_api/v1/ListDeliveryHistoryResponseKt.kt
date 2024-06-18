// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: webhook_api/v1/webhook.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package webhook_api.v1;

@kotlin.jvm.JvmName("-initializelistDeliveryHistoryResponse")
public inline fun listDeliveryHistoryResponse(block: webhook_api.v1.ListDeliveryHistoryResponseKt.Dsl.() -> kotlin.Unit): webhook_api.v1.WebhookOuterClass.ListDeliveryHistoryResponse =
  webhook_api.v1.ListDeliveryHistoryResponseKt.Dsl._create(webhook_api.v1.WebhookOuterClass.ListDeliveryHistoryResponse.newBuilder()).apply { block() }._build()
/**
 * ```
 * [Example]
 * {
 * "delivery_history": [
 * {
 * "id": "ID1",
 * "webhook_id": "WEBHOOK_ID1",
 * "event_id": "EVENT_ID1",
 * "owner_id": "OWNER_ID1",
 * "delivery_stage": 0,
 * "created": "2019-10-12T07:20:50.52Z"
 * }
 * ],
 * "total_owned_delivery_history": 1
 * }
 * ```
 *
 * Protobuf type `webhook_api.v1.ListDeliveryHistoryResponse`
 */
public object ListDeliveryHistoryResponseKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: webhook_api.v1.WebhookOuterClass.ListDeliveryHistoryResponse.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: webhook_api.v1.WebhookOuterClass.ListDeliveryHistoryResponse.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): webhook_api.v1.WebhookOuterClass.ListDeliveryHistoryResponse = _builder.build()

    /**
     * An uninstantiable, behaviorless type to represent the field in
     * generics.
     */
    @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
    public class DeliveryHistoryProxy private constructor() : com.google.protobuf.kotlin.DslProxy()
    /**
     * ```
     * List of Delivery History returned.
     * ```
     *
     * `repeated .webhook_api.v1.ListDeliveryHistoryResponse.DeliveryHistory delivery_history = 1;`
     */
     public val deliveryHistory: com.google.protobuf.kotlin.DslList<webhook_api.v1.WebhookOuterClass.ListDeliveryHistoryResponse.DeliveryHistory, DeliveryHistoryProxy>
      @kotlin.jvm.JvmSynthetic
      get() = com.google.protobuf.kotlin.DslList(
        _builder.getDeliveryHistoryList()
      )
    /**
     * ```
     * List of Delivery History returned.
     * ```
     *
     * `repeated .webhook_api.v1.ListDeliveryHistoryResponse.DeliveryHistory delivery_history = 1;`
     * @param value The deliveryHistory to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("addDeliveryHistory")
    public fun com.google.protobuf.kotlin.DslList<webhook_api.v1.WebhookOuterClass.ListDeliveryHistoryResponse.DeliveryHistory, DeliveryHistoryProxy>.add(value: webhook_api.v1.WebhookOuterClass.ListDeliveryHistoryResponse.DeliveryHistory) {
      _builder.addDeliveryHistory(value)
    }
    /**
     * ```
     * List of Delivery History returned.
     * ```
     *
     * `repeated .webhook_api.v1.ListDeliveryHistoryResponse.DeliveryHistory delivery_history = 1;`
     * @param value The deliveryHistory to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("plusAssignDeliveryHistory")
    @Suppress("NOTHING_TO_INLINE")
    public inline operator fun com.google.protobuf.kotlin.DslList<webhook_api.v1.WebhookOuterClass.ListDeliveryHistoryResponse.DeliveryHistory, DeliveryHistoryProxy>.plusAssign(value: webhook_api.v1.WebhookOuterClass.ListDeliveryHistoryResponse.DeliveryHistory) {
      add(value)
    }
    /**
     * ```
     * List of Delivery History returned.
     * ```
     *
     * `repeated .webhook_api.v1.ListDeliveryHistoryResponse.DeliveryHistory delivery_history = 1;`
     * @param values The deliveryHistory to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("addAllDeliveryHistory")
    public fun com.google.protobuf.kotlin.DslList<webhook_api.v1.WebhookOuterClass.ListDeliveryHistoryResponse.DeliveryHistory, DeliveryHistoryProxy>.addAll(values: kotlin.collections.Iterable<webhook_api.v1.WebhookOuterClass.ListDeliveryHistoryResponse.DeliveryHistory>) {
      _builder.addAllDeliveryHistory(values)
    }
    /**
     * ```
     * List of Delivery History returned.
     * ```
     *
     * `repeated .webhook_api.v1.ListDeliveryHistoryResponse.DeliveryHistory delivery_history = 1;`
     * @param values The deliveryHistory to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("plusAssignAllDeliveryHistory")
    @Suppress("NOTHING_TO_INLINE")
    public inline operator fun com.google.protobuf.kotlin.DslList<webhook_api.v1.WebhookOuterClass.ListDeliveryHistoryResponse.DeliveryHistory, DeliveryHistoryProxy>.plusAssign(values: kotlin.collections.Iterable<webhook_api.v1.WebhookOuterClass.ListDeliveryHistoryResponse.DeliveryHistory>) {
      addAll(values)
    }
    /**
     * ```
     * List of Delivery History returned.
     * ```
     *
     * `repeated .webhook_api.v1.ListDeliveryHistoryResponse.DeliveryHistory delivery_history = 1;`
     * @param index The index to set the value at.
     * @param value The deliveryHistory to set.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("setDeliveryHistory")
    public operator fun com.google.protobuf.kotlin.DslList<webhook_api.v1.WebhookOuterClass.ListDeliveryHistoryResponse.DeliveryHistory, DeliveryHistoryProxy>.set(index: kotlin.Int, value: webhook_api.v1.WebhookOuterClass.ListDeliveryHistoryResponse.DeliveryHistory) {
      _builder.setDeliveryHistory(index, value)
    }
    /**
     * ```
     * List of Delivery History returned.
     * ```
     *
     * `repeated .webhook_api.v1.ListDeliveryHistoryResponse.DeliveryHistory delivery_history = 1;`
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("clearDeliveryHistory")
    public fun com.google.protobuf.kotlin.DslList<webhook_api.v1.WebhookOuterClass.ListDeliveryHistoryResponse.DeliveryHistory, DeliveryHistoryProxy>.clear() {
      _builder.clearDeliveryHistory()
    }


    /**
     * ```
     * Total number of owned delivery history returned.
     * ```
     *
     * `uint32 total_owned_delivery_history = 2;`
     */
    public var totalOwnedDeliveryHistory: kotlin.Int
      @JvmName("getTotalOwnedDeliveryHistory")
      get() = _builder.getTotalOwnedDeliveryHistory()
      @JvmName("setTotalOwnedDeliveryHistory")
      set(value) {
        _builder.setTotalOwnedDeliveryHistory(value)
      }
    /**
     * ```
     * Total number of owned delivery history returned.
     * ```
     *
     * `uint32 total_owned_delivery_history = 2;`
     */
    public fun clearTotalOwnedDeliveryHistory() {
      _builder.clearTotalOwnedDeliveryHistory()
    }
  }
  @kotlin.jvm.JvmName("-initializedeliveryHistory")
  public inline fun deliveryHistory(block: webhook_api.v1.ListDeliveryHistoryResponseKt.DeliveryHistoryKt.Dsl.() -> kotlin.Unit): webhook_api.v1.WebhookOuterClass.ListDeliveryHistoryResponse.DeliveryHistory =
    webhook_api.v1.ListDeliveryHistoryResponseKt.DeliveryHistoryKt.Dsl._create(webhook_api.v1.WebhookOuterClass.ListDeliveryHistoryResponse.DeliveryHistory.newBuilder()).apply { block() }._build()
  /**
   * ```
   * [Example]
   * {
   * "id": "ID1",
   * "webhook_id": "WEBHOOK_ID1",
   * "event_id": "EVENT_ID1",
   * "owner_id": "OWNER_ID1",
   * "delivery_stage": 0,
   * "created": "2019-10-12T07:20:50.52Z"
   * }
   * ```
   *
   * Protobuf type `webhook_api.v1.ListDeliveryHistoryResponse.DeliveryHistory`
   */
  public object DeliveryHistoryKt {
    @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
    @com.google.protobuf.kotlin.ProtoDslMarker
    public class Dsl private constructor(
      private val _builder: webhook_api.v1.WebhookOuterClass.ListDeliveryHistoryResponse.DeliveryHistory.Builder
    ) {
      public companion object {
        @kotlin.jvm.JvmSynthetic
        @kotlin.PublishedApi
        internal fun _create(builder: webhook_api.v1.WebhookOuterClass.ListDeliveryHistoryResponse.DeliveryHistory.Builder): Dsl = Dsl(builder)
      }

      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _build(): webhook_api.v1.WebhookOuterClass.ListDeliveryHistoryResponse.DeliveryHistory = _builder.build()

      /**
       * ```
       * Delivery History ID.
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
       * Delivery History ID.
       * ```
       *
       * `string id = 1;`
       */
      public fun clearId() {
        _builder.clearId()
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

      /**
       * ```
       * Event ID.
       * ```
       *
       * `string event_id = 3;`
       */
      public var eventId: kotlin.String
        @JvmName("getEventId")
        get() = _builder.getEventId()
        @JvmName("setEventId")
        set(value) {
          _builder.setEventId(value)
        }
      /**
       * ```
       * Event ID.
       * ```
       *
       * `string event_id = 3;`
       */
      public fun clearEventId() {
        _builder.clearEventId()
      }

      /**
       * ```
       * Owner ID
       * ```
       *
       * `string owner_id = 4;`
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
       * Owner ID
       * ```
       *
       * `string owner_id = 4;`
       */
      public fun clearOwnerId() {
        _builder.clearOwnerId()
      }

      /**
       * ```
       * Delivery Stage.
       * ```
       *
       * `.webhook_api.v1.DeliveryStage delivery_stage = 5;`
       */
      public var deliveryStage: webhook_api.v1.WebhookOuterClass.DeliveryStage
        @JvmName("getDeliveryStage")
        get() = _builder.getDeliveryStage()
        @JvmName("setDeliveryStage")
        set(value) {
          _builder.setDeliveryStage(value)
        }
      public var deliveryStageValue: kotlin.Int
        @JvmName("getDeliveryStageValue")
        get() = _builder.getDeliveryStageValue()
        @JvmName("setDeliveryStageValue")
        set(value) {
          _builder.setDeliveryStageValue(value)
        }
      /**
       * ```
       * Delivery Stage.
       * ```
       *
       * `.webhook_api.v1.DeliveryStage delivery_stage = 5;`
       */
      public fun clearDeliveryStage() {
        _builder.clearDeliveryStage()
      }

      /**
       * ```
       * Created Date. Rfc3339 format.
       * ```
       *
       * `string created = 6;`
       */
      public var created: kotlin.String
        @JvmName("getCreated")
        get() = _builder.getCreated()
        @JvmName("setCreated")
        set(value) {
          _builder.setCreated(value)
        }
      /**
       * ```
       * Created Date. Rfc3339 format.
       * ```
       *
       * `string created = 6;`
       */
      public fun clearCreated() {
        _builder.clearCreated()
      }
    }
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun webhook_api.v1.WebhookOuterClass.ListDeliveryHistoryResponse.copy(block: `webhook_api.v1`.ListDeliveryHistoryResponseKt.Dsl.() -> kotlin.Unit): webhook_api.v1.WebhookOuterClass.ListDeliveryHistoryResponse =
  `webhook_api.v1`.ListDeliveryHistoryResponseKt.Dsl._create(this.toBuilder()).apply { block() }._build()

@kotlin.jvm.JvmSynthetic
public inline fun webhook_api.v1.WebhookOuterClass.ListDeliveryHistoryResponse.DeliveryHistory.copy(block: `webhook_api.v1`.ListDeliveryHistoryResponseKt.DeliveryHistoryKt.Dsl.() -> kotlin.Unit): webhook_api.v1.WebhookOuterClass.ListDeliveryHistoryResponse.DeliveryHistory =
  `webhook_api.v1`.ListDeliveryHistoryResponseKt.DeliveryHistoryKt.Dsl._create(this.toBuilder()).apply { block() }._build()

