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

goog.provide('proto.webhook_api.v1.CreatePrevalidationRequest');
goog.provide('proto.webhook_api.v1.CreatePrevalidationRequest.TriggerCase');

goog.require('jspb.BinaryReader');
goog.require('jspb.BinaryWriter');
goog.require('jspb.Message');
goog.require('proto.webhook_api.v1.ConsideringContractPrevalidation');

goog.forwardDeclare('proto.webhook_api.v1.PrevalidationTriggerType');
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
proto.webhook_api.v1.CreatePrevalidationRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, proto.webhook_api.v1.CreatePrevalidationRequest.oneofGroups_);
};
goog.inherits(proto.webhook_api.v1.CreatePrevalidationRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.webhook_api.v1.CreatePrevalidationRequest.displayName = 'proto.webhook_api.v1.CreatePrevalidationRequest';
}

/**
 * Oneof group definitions for this message. Each group defines the field
 * numbers belonging to that group. When of these fields' value is set, all
 * other fields in the group are cleared. During deserialization, if multiple
 * fields are encountered for a group, only the last value seen will be kept.
 * @private {!Array<!Array<number>>}
 * @const
 */
proto.webhook_api.v1.CreatePrevalidationRequest.oneofGroups_ = [[4]];

/**
 * @enum {number}
 */
proto.webhook_api.v1.CreatePrevalidationRequest.TriggerCase = {
  TRIGGER_NOT_SET: 0,
  CONSIDERING_CONTRACT_PREVALIDATION: 4
};

/**
 * @return {proto.webhook_api.v1.CreatePrevalidationRequest.TriggerCase}
 */
proto.webhook_api.v1.CreatePrevalidationRequest.prototype.getTriggerCase = function() {
  return /** @type {proto.webhook_api.v1.CreatePrevalidationRequest.TriggerCase} */(jspb.Message.computeOneofCase(this, proto.webhook_api.v1.CreatePrevalidationRequest.oneofGroups_[0]));
};



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
proto.webhook_api.v1.CreatePrevalidationRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.webhook_api.v1.CreatePrevalidationRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.webhook_api.v1.CreatePrevalidationRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.webhook_api.v1.CreatePrevalidationRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    apiKey: jspb.Message.getFieldWithDefault(msg, 1, ""),
    name: jspb.Message.getFieldWithDefault(msg, 2, ""),
    triggerType: jspb.Message.getFieldWithDefault(msg, 3, 0),
    consideringContractPrevalidation: (f = msg.getConsideringContractPrevalidation()) && proto.webhook_api.v1.ConsideringContractPrevalidation.toObject(includeInstance, f)
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
 * @return {!proto.webhook_api.v1.CreatePrevalidationRequest}
 */
proto.webhook_api.v1.CreatePrevalidationRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.webhook_api.v1.CreatePrevalidationRequest;
  return proto.webhook_api.v1.CreatePrevalidationRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.webhook_api.v1.CreatePrevalidationRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.webhook_api.v1.CreatePrevalidationRequest}
 */
proto.webhook_api.v1.CreatePrevalidationRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setApiKey(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setName(value);
      break;
    case 3:
      var value = /** @type {!proto.webhook_api.v1.PrevalidationTriggerType} */ (reader.readEnum());
      msg.setTriggerType(value);
      break;
    case 4:
      var value = new proto.webhook_api.v1.ConsideringContractPrevalidation;
      reader.readMessage(value,proto.webhook_api.v1.ConsideringContractPrevalidation.deserializeBinaryFromReader);
      msg.setConsideringContractPrevalidation(value);
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
proto.webhook_api.v1.CreatePrevalidationRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.webhook_api.v1.CreatePrevalidationRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.webhook_api.v1.CreatePrevalidationRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.webhook_api.v1.CreatePrevalidationRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getApiKey();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getName();
  if (f.length > 0) {
    writer.writeString(
      2,
      f
    );
  }
  f = message.getTriggerType();
  if (f !== 0.0) {
    writer.writeEnum(
      3,
      f
    );
  }
  f = message.getConsideringContractPrevalidation();
  if (f != null) {
    writer.writeMessage(
      4,
      f,
      proto.webhook_api.v1.ConsideringContractPrevalidation.serializeBinaryToWriter
    );
  }
};


/**
 * optional string api_key = 1;
 * @return {string}
 */
proto.webhook_api.v1.CreatePrevalidationRequest.prototype.getApiKey = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.webhook_api.v1.CreatePrevalidationRequest} returns this
 */
proto.webhook_api.v1.CreatePrevalidationRequest.prototype.setApiKey = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional string name = 2;
 * @return {string}
 */
proto.webhook_api.v1.CreatePrevalidationRequest.prototype.getName = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.webhook_api.v1.CreatePrevalidationRequest} returns this
 */
proto.webhook_api.v1.CreatePrevalidationRequest.prototype.setName = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};


/**
 * optional PrevalidationTriggerType trigger_type = 3;
 * @return {!proto.webhook_api.v1.PrevalidationTriggerType}
 */
proto.webhook_api.v1.CreatePrevalidationRequest.prototype.getTriggerType = function() {
  return /** @type {!proto.webhook_api.v1.PrevalidationTriggerType} */ (jspb.Message.getFieldWithDefault(this, 3, 0));
};


/**
 * @param {!proto.webhook_api.v1.PrevalidationTriggerType} value
 * @return {!proto.webhook_api.v1.CreatePrevalidationRequest} returns this
 */
proto.webhook_api.v1.CreatePrevalidationRequest.prototype.setTriggerType = function(value) {
  return jspb.Message.setProto3EnumField(this, 3, value);
};


/**
 * optional ConsideringContractPrevalidation considering_contract_prevalidation = 4;
 * @return {?proto.webhook_api.v1.ConsideringContractPrevalidation}
 */
proto.webhook_api.v1.CreatePrevalidationRequest.prototype.getConsideringContractPrevalidation = function() {
  return /** @type{?proto.webhook_api.v1.ConsideringContractPrevalidation} */ (
    jspb.Message.getWrapperField(this, proto.webhook_api.v1.ConsideringContractPrevalidation, 4));
};


/**
 * @param {?proto.webhook_api.v1.ConsideringContractPrevalidation|undefined} value
 * @return {!proto.webhook_api.v1.CreatePrevalidationRequest} returns this
*/
proto.webhook_api.v1.CreatePrevalidationRequest.prototype.setConsideringContractPrevalidation = function(value) {
  return jspb.Message.setOneofWrapperField(this, 4, proto.webhook_api.v1.CreatePrevalidationRequest.oneofGroups_[0], value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.webhook_api.v1.CreatePrevalidationRequest} returns this
 */
proto.webhook_api.v1.CreatePrevalidationRequest.prototype.clearConsideringContractPrevalidation = function() {
  return this.setConsideringContractPrevalidation(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.webhook_api.v1.CreatePrevalidationRequest.prototype.hasConsideringContractPrevalidation = function() {
  return jspb.Message.getField(this, 4) != null;
};

