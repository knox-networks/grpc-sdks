// source: common/v1/amqp.proto
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

goog.provide('proto.common.ReceiveTerminationData');

goog.require('jspb.BinaryReader');
goog.require('jspb.BinaryWriter');
goog.require('jspb.Message');

goog.forwardDeclare('proto.common.TerminationResult');
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
proto.common.ReceiveTerminationData = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.common.ReceiveTerminationData, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.common.ReceiveTerminationData.displayName = 'proto.common.ReceiveTerminationData';
}



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
proto.common.ReceiveTerminationData.prototype.toObject = function(opt_includeInstance) {
  return proto.common.ReceiveTerminationData.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.common.ReceiveTerminationData} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.common.ReceiveTerminationData.toObject = function(includeInstance, msg) {
  var f, obj = {
    participant: jspb.Message.getFieldWithDefault(msg, 1, ""),
    contractId: jspb.Message.getFieldWithDefault(msg, 2, ""),
    result: jspb.Message.getFieldWithDefault(msg, 3, 0)
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
 * @return {!proto.common.ReceiveTerminationData}
 */
proto.common.ReceiveTerminationData.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.common.ReceiveTerminationData;
  return proto.common.ReceiveTerminationData.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.common.ReceiveTerminationData} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.common.ReceiveTerminationData}
 */
proto.common.ReceiveTerminationData.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setParticipant(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setContractId(value);
      break;
    case 3:
      var value = /** @type {!proto.common.TerminationResult} */ (reader.readEnum());
      msg.setResult(value);
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
proto.common.ReceiveTerminationData.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.common.ReceiveTerminationData.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.common.ReceiveTerminationData} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.common.ReceiveTerminationData.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getParticipant();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getContractId();
  if (f.length > 0) {
    writer.writeString(
      2,
      f
    );
  }
  f = message.getResult();
  if (f !== 0.0) {
    writer.writeEnum(
      3,
      f
    );
  }
};


/**
 * optional string participant = 1;
 * @return {string}
 */
proto.common.ReceiveTerminationData.prototype.getParticipant = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.common.ReceiveTerminationData} returns this
 */
proto.common.ReceiveTerminationData.prototype.setParticipant = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional string contract_id = 2;
 * @return {string}
 */
proto.common.ReceiveTerminationData.prototype.getContractId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.common.ReceiveTerminationData} returns this
 */
proto.common.ReceiveTerminationData.prototype.setContractId = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};


/**
 * optional TerminationResult result = 3;
 * @return {!proto.common.TerminationResult}
 */
proto.common.ReceiveTerminationData.prototype.getResult = function() {
  return /** @type {!proto.common.TerminationResult} */ (jspb.Message.getFieldWithDefault(this, 3, 0));
};


/**
 * @param {!proto.common.TerminationResult} value
 * @return {!proto.common.ReceiveTerminationData} returns this
 */
proto.common.ReceiveTerminationData.prototype.setResult = function(value) {
  return jspb.Message.setProto3EnumField(this, 3, value);
};


