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

goog.provide('proto.webhook_api.v1.CreateContractReactionPayload');

goog.require('jspb.BinaryReader');
goog.require('jspb.BinaryWriter');
goog.require('jspb.Message');
goog.require('proto.common.Commitment');
goog.require('proto.common.Condition');

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
proto.webhook_api.v1.CreateContractReactionPayload = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, proto.webhook_api.v1.CreateContractReactionPayload.repeatedFields_, null);
};
goog.inherits(proto.webhook_api.v1.CreateContractReactionPayload, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.webhook_api.v1.CreateContractReactionPayload.displayName = 'proto.webhook_api.v1.CreateContractReactionPayload';
}

/**
 * List of repeated fields within this message type.
 * @private {!Array<number>}
 * @const
 */
proto.webhook_api.v1.CreateContractReactionPayload.repeatedFields_ = [2,3];



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
proto.webhook_api.v1.CreateContractReactionPayload.prototype.toObject = function(opt_includeInstance) {
  return proto.webhook_api.v1.CreateContractReactionPayload.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.webhook_api.v1.CreateContractReactionPayload} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.webhook_api.v1.CreateContractReactionPayload.toObject = function(includeInstance, msg) {
  var f, obj = {
    senderWalletId: jspb.Message.getFieldWithDefault(msg, 1, ""),
    commitmentsList: jspb.Message.toObjectList(msg.getCommitmentsList(),
    proto.common.Commitment.toObject, includeInstance),
    conditionsList: jspb.Message.toObjectList(msg.getConditionsList(),
    proto.common.Condition.toObject, includeInstance),
    timeoutSecs: jspb.Message.getFieldWithDefault(msg, 4, 0),
    memo: jspb.Message.getFieldWithDefault(msg, 5, "")
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
 * @return {!proto.webhook_api.v1.CreateContractReactionPayload}
 */
proto.webhook_api.v1.CreateContractReactionPayload.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.webhook_api.v1.CreateContractReactionPayload;
  return proto.webhook_api.v1.CreateContractReactionPayload.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.webhook_api.v1.CreateContractReactionPayload} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.webhook_api.v1.CreateContractReactionPayload}
 */
proto.webhook_api.v1.CreateContractReactionPayload.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setSenderWalletId(value);
      break;
    case 2:
      var value = new proto.common.Commitment;
      reader.readMessage(value,proto.common.Commitment.deserializeBinaryFromReader);
      msg.addCommitments(value);
      break;
    case 3:
      var value = new proto.common.Condition;
      reader.readMessage(value,proto.common.Condition.deserializeBinaryFromReader);
      msg.addConditions(value);
      break;
    case 4:
      var value = /** @type {number} */ (reader.readUint32());
      msg.setTimeoutSecs(value);
      break;
    case 5:
      var value = /** @type {string} */ (reader.readString());
      msg.setMemo(value);
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
proto.webhook_api.v1.CreateContractReactionPayload.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.webhook_api.v1.CreateContractReactionPayload.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.webhook_api.v1.CreateContractReactionPayload} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.webhook_api.v1.CreateContractReactionPayload.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getSenderWalletId();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getCommitmentsList();
  if (f.length > 0) {
    writer.writeRepeatedMessage(
      2,
      f,
      proto.common.Commitment.serializeBinaryToWriter
    );
  }
  f = message.getConditionsList();
  if (f.length > 0) {
    writer.writeRepeatedMessage(
      3,
      f,
      proto.common.Condition.serializeBinaryToWriter
    );
  }
  f = message.getTimeoutSecs();
  if (f !== 0) {
    writer.writeUint32(
      4,
      f
    );
  }
  f = message.getMemo();
  if (f.length > 0) {
    writer.writeString(
      5,
      f
    );
  }
};


/**
 * optional string sender_wallet_id = 1;
 * @return {string}
 */
proto.webhook_api.v1.CreateContractReactionPayload.prototype.getSenderWalletId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.webhook_api.v1.CreateContractReactionPayload} returns this
 */
proto.webhook_api.v1.CreateContractReactionPayload.prototype.setSenderWalletId = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * repeated common.Commitment commitments = 2;
 * @return {!Array<!proto.common.Commitment>}
 */
proto.webhook_api.v1.CreateContractReactionPayload.prototype.getCommitmentsList = function() {
  return /** @type{!Array<!proto.common.Commitment>} */ (
    jspb.Message.getRepeatedWrapperField(this, proto.common.Commitment, 2));
};


/**
 * @param {!Array<!proto.common.Commitment>} value
 * @return {!proto.webhook_api.v1.CreateContractReactionPayload} returns this
*/
proto.webhook_api.v1.CreateContractReactionPayload.prototype.setCommitmentsList = function(value) {
  return jspb.Message.setRepeatedWrapperField(this, 2, value);
};


/**
 * @param {!proto.common.Commitment=} opt_value
 * @param {number=} opt_index
 * @return {!proto.common.Commitment}
 */
proto.webhook_api.v1.CreateContractReactionPayload.prototype.addCommitments = function(opt_value, opt_index) {
  return jspb.Message.addToRepeatedWrapperField(this, 2, opt_value, proto.common.Commitment, opt_index);
};


/**
 * Clears the list making it empty but non-null.
 * @return {!proto.webhook_api.v1.CreateContractReactionPayload} returns this
 */
proto.webhook_api.v1.CreateContractReactionPayload.prototype.clearCommitmentsList = function() {
  return this.setCommitmentsList([]);
};


/**
 * repeated common.Condition conditions = 3;
 * @return {!Array<!proto.common.Condition>}
 */
proto.webhook_api.v1.CreateContractReactionPayload.prototype.getConditionsList = function() {
  return /** @type{!Array<!proto.common.Condition>} */ (
    jspb.Message.getRepeatedWrapperField(this, proto.common.Condition, 3));
};


/**
 * @param {!Array<!proto.common.Condition>} value
 * @return {!proto.webhook_api.v1.CreateContractReactionPayload} returns this
*/
proto.webhook_api.v1.CreateContractReactionPayload.prototype.setConditionsList = function(value) {
  return jspb.Message.setRepeatedWrapperField(this, 3, value);
};


/**
 * @param {!proto.common.Condition=} opt_value
 * @param {number=} opt_index
 * @return {!proto.common.Condition}
 */
proto.webhook_api.v1.CreateContractReactionPayload.prototype.addConditions = function(opt_value, opt_index) {
  return jspb.Message.addToRepeatedWrapperField(this, 3, opt_value, proto.common.Condition, opt_index);
};


/**
 * Clears the list making it empty but non-null.
 * @return {!proto.webhook_api.v1.CreateContractReactionPayload} returns this
 */
proto.webhook_api.v1.CreateContractReactionPayload.prototype.clearConditionsList = function() {
  return this.setConditionsList([]);
};


/**
 * optional uint32 timeout_secs = 4;
 * @return {number}
 */
proto.webhook_api.v1.CreateContractReactionPayload.prototype.getTimeoutSecs = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 4, 0));
};


/**
 * @param {number} value
 * @return {!proto.webhook_api.v1.CreateContractReactionPayload} returns this
 */
proto.webhook_api.v1.CreateContractReactionPayload.prototype.setTimeoutSecs = function(value) {
  return jspb.Message.setProto3IntField(this, 4, value);
};


/**
 * optional string memo = 5;
 * @return {string}
 */
proto.webhook_api.v1.CreateContractReactionPayload.prototype.getMemo = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 5, ""));
};


/**
 * @param {string} value
 * @return {!proto.webhook_api.v1.CreateContractReactionPayload} returns this
 */
proto.webhook_api.v1.CreateContractReactionPayload.prototype.setMemo = function(value) {
  return jspb.Message.setProto3StringField(this, 5, value);
};


