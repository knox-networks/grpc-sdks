// source: webhook_api/v1/webhook.proto
/**
 * @fileoverview
 * @enhanceable
 * @suppress {missingRequire} reports error on implicit type usages.
 * @suppress {messageConventions} JS Compiler reports an error if a variable or
 *     field starts with 'MSG_' and isn't a translatable message.
 * @public
 */
// GENERATED CODE -- DO NOT EDIT!
/* eslint-disable */
// @ts-nocheck

goog.provide('proto.webhook_api.v1.Webhook');

goog.require('jspb.BinaryReader');
goog.require('jspb.BinaryWriter');
goog.require('jspb.Message');

goog.forwardDeclare('proto.webhook_api.v1.DeliveryMethodType');
goog.forwardDeclare('proto.webhook_api.v1.EventType');
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.webhook_api.v1.Webhook = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, proto.webhook_api.v1.Webhook.repeatedFields_, null);
};
goog.inherits(proto.webhook_api.v1.Webhook, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.webhook_api.v1.Webhook.displayName = 'proto.webhook_api.v1.Webhook';
}

/**
 * List of repeated fields within this message type.
 * @private {!Array<number>}
 * @const
 */
proto.webhook_api.v1.Webhook.repeatedFields_ = [5];



if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.webhook_api.v1.Webhook.prototype.toObject = function(opt_includeInstance) {
  return proto.webhook_api.v1.Webhook.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.webhook_api.v1.Webhook} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.webhook_api.v1.Webhook.toObject = function(includeInstance, msg) {
  var f, obj = {
    id: jspb.Message.getFieldWithDefault(msg, 1, ""),
    ownerId: jspb.Message.getFieldWithDefault(msg, 2, ""),
    name: jspb.Message.getFieldWithDefault(msg, 3, ""),
    webhookUrl: jspb.Message.getFieldWithDefault(msg, 4, ""),
    supportedEventsList: (f = jspb.Message.getRepeatedField(msg, 5)) == null ? undefined : f,
    deliveryMethod: jspb.Message.getFieldWithDefault(msg, 6, 0)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.webhook_api.v1.Webhook}
 */
proto.webhook_api.v1.Webhook.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.webhook_api.v1.Webhook;
  return proto.webhook_api.v1.Webhook.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.webhook_api.v1.Webhook} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.webhook_api.v1.Webhook}
 */
proto.webhook_api.v1.Webhook.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setId(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setOwnerId(value);
      break;
    case 3:
      var value = /** @type {string} */ (reader.readString());
      msg.setName(value);
      break;
    case 4:
      var value = /** @type {string} */ (reader.readString());
      msg.setWebhookUrl(value);
      break;
    case 5:
      var values = /** @type {!Array<!proto.webhook_api.v1.EventType>} */ (reader.isDelimited() ? reader.readPackedEnum() : [reader.readEnum()]);
      for (var i = 0; i < values.length; i++) {
        msg.addSupportedEvents(values[i]);
      }
      break;
    case 6:
      var value = /** @type {!proto.webhook_api.v1.DeliveryMethodType} */ (reader.readEnum());
      msg.setDeliveryMethod(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.webhook_api.v1.Webhook.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.webhook_api.v1.Webhook.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.webhook_api.v1.Webhook} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.webhook_api.v1.Webhook.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getId();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getOwnerId();
  if (f.length > 0) {
    writer.writeString(
      2,
      f
    );
  }
  f = message.getName();
  if (f.length > 0) {
    writer.writeString(
      3,
      f
    );
  }
  f = message.getWebhookUrl();
  if (f.length > 0) {
    writer.writeString(
      4,
      f
    );
  }
  f = message.getSupportedEventsList();
  if (f.length > 0) {
    writer.writePackedEnum(
      5,
      f
    );
  }
  f = message.getDeliveryMethod();
  if (f !== 0.0) {
    writer.writeEnum(
      6,
      f
    );
  }
};


/**
 * optional string id = 1;
 * @return {string}
 */
proto.webhook_api.v1.Webhook.prototype.getId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.webhook_api.v1.Webhook} returns this
 */
proto.webhook_api.v1.Webhook.prototype.setId = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional string owner_id = 2;
 * @return {string}
 */
proto.webhook_api.v1.Webhook.prototype.getOwnerId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.webhook_api.v1.Webhook} returns this
 */
proto.webhook_api.v1.Webhook.prototype.setOwnerId = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};


/**
 * optional string name = 3;
 * @return {string}
 */
proto.webhook_api.v1.Webhook.prototype.getName = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 3, ""));
};


/**
 * @param {string} value
 * @return {!proto.webhook_api.v1.Webhook} returns this
 */
proto.webhook_api.v1.Webhook.prototype.setName = function(value) {
  return jspb.Message.setProto3StringField(this, 3, value);
};


/**
 * optional string webhook_url = 4;
 * @return {string}
 */
proto.webhook_api.v1.Webhook.prototype.getWebhookUrl = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 4, ""));
};


/**
 * @param {string} value
 * @return {!proto.webhook_api.v1.Webhook} returns this
 */
proto.webhook_api.v1.Webhook.prototype.setWebhookUrl = function(value) {
  return jspb.Message.setProto3StringField(this, 4, value);
};


/**
 * repeated EventType supported_events = 5;
 * @return {!Array<!proto.webhook_api.v1.EventType>}
 */
proto.webhook_api.v1.Webhook.prototype.getSupportedEventsList = function() {
  return /** @type {!Array<!proto.webhook_api.v1.EventType>} */ (jspb.Message.getRepeatedField(this, 5));
};


/**
 * @param {!Array<!proto.webhook_api.v1.EventType>} value
 * @return {!proto.webhook_api.v1.Webhook} returns this
 */
proto.webhook_api.v1.Webhook.prototype.setSupportedEventsList = function(value) {
  return jspb.Message.setField(this, 5, value || []);
};


/**
 * @param {!proto.webhook_api.v1.EventType} value
 * @param {number=} opt_index
 * @return {!proto.webhook_api.v1.Webhook} returns this
 */
proto.webhook_api.v1.Webhook.prototype.addSupportedEvents = function(value, opt_index) {
  return jspb.Message.addToRepeatedField(this, 5, value, opt_index);
};


/**
 * Clears the list making it empty but non-null.
 * @return {!proto.webhook_api.v1.Webhook} returns this
 */
proto.webhook_api.v1.Webhook.prototype.clearSupportedEventsList = function() {
  return this.setSupportedEventsList([]);
};


/**
 * optional DeliveryMethodType delivery_method = 6;
 * @return {!proto.webhook_api.v1.DeliveryMethodType}
 */
proto.webhook_api.v1.Webhook.prototype.getDeliveryMethod = function() {
  return /** @type {!proto.webhook_api.v1.DeliveryMethodType} */ (jspb.Message.getFieldWithDefault(this, 6, 0));
};


/**
 * @param {!proto.webhook_api.v1.DeliveryMethodType} value
 * @return {!proto.webhook_api.v1.Webhook} returns this
 */
proto.webhook_api.v1.Webhook.prototype.setDeliveryMethod = function(value) {
  return jspb.Message.setProto3EnumField(this, 6, value);
};

