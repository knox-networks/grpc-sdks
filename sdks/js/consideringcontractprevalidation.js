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

goog.provide('proto.webhook_api.v1.ConsideringContractPrevalidation');
goog.provide('proto.webhook_api.v1.ConsideringContractPrevalidation.ValidationOutcome');

goog.require('jspb.BinaryReader');
goog.require('jspb.BinaryWriter');
goog.require('jspb.Message');
goog.require('proto.webhook_api.v1.ValidationCondition');

goog.forwardDeclare('proto.common.LogicalOperator');
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
proto.webhook_api.v1.ConsideringContractPrevalidation = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, proto.webhook_api.v1.ConsideringContractPrevalidation.repeatedFields_, null);
};
goog.inherits(proto.webhook_api.v1.ConsideringContractPrevalidation, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.webhook_api.v1.ConsideringContractPrevalidation.displayName = 'proto.webhook_api.v1.ConsideringContractPrevalidation';
}

/**
 * List of repeated fields within this message type.
 * @private {!Array<number>}
 * @const
 */
proto.webhook_api.v1.ConsideringContractPrevalidation.repeatedFields_ = [1];



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
proto.webhook_api.v1.ConsideringContractPrevalidation.prototype.toObject = function(opt_includeInstance) {
  return proto.webhook_api.v1.ConsideringContractPrevalidation.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.webhook_api.v1.ConsideringContractPrevalidation} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.webhook_api.v1.ConsideringContractPrevalidation.toObject = function(includeInstance, msg) {
  var f, obj = {
    conditionsList: jspb.Message.toObjectList(msg.getConditionsList(),
    proto.webhook_api.v1.ValidationCondition.toObject, includeInstance),
    outcome: jspb.Message.getFieldWithDefault(msg, 2, 0),
    logicalBase: jspb.Message.getFieldWithDefault(msg, 3, 0)
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
 * @return {!proto.webhook_api.v1.ConsideringContractPrevalidation}
 */
proto.webhook_api.v1.ConsideringContractPrevalidation.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.webhook_api.v1.ConsideringContractPrevalidation;
  return proto.webhook_api.v1.ConsideringContractPrevalidation.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.webhook_api.v1.ConsideringContractPrevalidation} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.webhook_api.v1.ConsideringContractPrevalidation}
 */
proto.webhook_api.v1.ConsideringContractPrevalidation.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.webhook_api.v1.ValidationCondition;
      reader.readMessage(value,proto.webhook_api.v1.ValidationCondition.deserializeBinaryFromReader);
      msg.addConditions(value);
      break;
    case 2:
      var value = /** @type {!proto.webhook_api.v1.ConsideringContractPrevalidation.ValidationOutcome} */ (reader.readEnum());
      msg.setOutcome(value);
      break;
    case 3:
      var value = /** @type {!proto.common.LogicalOperator} */ (reader.readEnum());
      msg.setLogicalBase(value);
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
proto.webhook_api.v1.ConsideringContractPrevalidation.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.webhook_api.v1.ConsideringContractPrevalidation.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.webhook_api.v1.ConsideringContractPrevalidation} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.webhook_api.v1.ConsideringContractPrevalidation.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getConditionsList();
  if (f.length > 0) {
    writer.writeRepeatedMessage(
      1,
      f,
      proto.webhook_api.v1.ValidationCondition.serializeBinaryToWriter
    );
  }
  f = message.getOutcome();
  if (f !== 0.0) {
    writer.writeEnum(
      2,
      f
    );
  }
  f = message.getLogicalBase();
  if (f !== 0.0) {
    writer.writeEnum(
      3,
      f
    );
  }
};


/**
 * @enum {number}
 */
proto.webhook_api.v1.ConsideringContractPrevalidation.ValidationOutcome = {
  CONSIDERING_CONTRACT_OUTCOME_UNSPECIFIED: 0,
  CONSIDERING_CONTRACT_OUTCOME_ACCEPTED: 1,
  CONSIDERING_CONTRACT_OUTCOME_REJECTED: 2
};

/**
 * repeated ValidationCondition conditions = 1;
 * @return {!Array<!proto.webhook_api.v1.ValidationCondition>}
 */
proto.webhook_api.v1.ConsideringContractPrevalidation.prototype.getConditionsList = function() {
  return /** @type{!Array<!proto.webhook_api.v1.ValidationCondition>} */ (
    jspb.Message.getRepeatedWrapperField(this, proto.webhook_api.v1.ValidationCondition, 1));
};


/**
 * @param {!Array<!proto.webhook_api.v1.ValidationCondition>} value
 * @return {!proto.webhook_api.v1.ConsideringContractPrevalidation} returns this
*/
proto.webhook_api.v1.ConsideringContractPrevalidation.prototype.setConditionsList = function(value) {
  return jspb.Message.setRepeatedWrapperField(this, 1, value);
};


/**
 * @param {!proto.webhook_api.v1.ValidationCondition=} opt_value
 * @param {number=} opt_index
 * @return {!proto.webhook_api.v1.ValidationCondition}
 */
proto.webhook_api.v1.ConsideringContractPrevalidation.prototype.addConditions = function(opt_value, opt_index) {
  return jspb.Message.addToRepeatedWrapperField(this, 1, opt_value, proto.webhook_api.v1.ValidationCondition, opt_index);
};


/**
 * Clears the list making it empty but non-null.
 * @return {!proto.webhook_api.v1.ConsideringContractPrevalidation} returns this
 */
proto.webhook_api.v1.ConsideringContractPrevalidation.prototype.clearConditionsList = function() {
  return this.setConditionsList([]);
};


/**
 * optional ValidationOutcome outcome = 2;
 * @return {!proto.webhook_api.v1.ConsideringContractPrevalidation.ValidationOutcome}
 */
proto.webhook_api.v1.ConsideringContractPrevalidation.prototype.getOutcome = function() {
  return /** @type {!proto.webhook_api.v1.ConsideringContractPrevalidation.ValidationOutcome} */ (jspb.Message.getFieldWithDefault(this, 2, 0));
};


/**
 * @param {!proto.webhook_api.v1.ConsideringContractPrevalidation.ValidationOutcome} value
 * @return {!proto.webhook_api.v1.ConsideringContractPrevalidation} returns this
 */
proto.webhook_api.v1.ConsideringContractPrevalidation.prototype.setOutcome = function(value) {
  return jspb.Message.setProto3EnumField(this, 2, value);
};


/**
 * optional common.LogicalOperator logical_base = 3;
 * @return {!proto.common.LogicalOperator}
 */
proto.webhook_api.v1.ConsideringContractPrevalidation.prototype.getLogicalBase = function() {
  return /** @type {!proto.common.LogicalOperator} */ (jspb.Message.getFieldWithDefault(this, 3, 0));
};


/**
 * @param {!proto.common.LogicalOperator} value
 * @return {!proto.webhook_api.v1.ConsideringContractPrevalidation} returns this
 */
proto.webhook_api.v1.ConsideringContractPrevalidation.prototype.setLogicalBase = function(value) {
  return jspb.Message.setProto3EnumField(this, 3, value);
};


