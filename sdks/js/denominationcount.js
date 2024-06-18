// source: treasury/v1/treasury.proto
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

goog.provide('proto.treasury.DenominationCount');

goog.require('jspb.BinaryReader');
goog.require('jspb.BinaryWriter');
goog.require('jspb.Message');
goog.require('proto.common.Amount');

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
proto.treasury.DenominationCount = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.treasury.DenominationCount, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.treasury.DenominationCount.displayName = 'proto.treasury.DenominationCount';
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
proto.treasury.DenominationCount.prototype.toObject = function(opt_includeInstance) {
  return proto.treasury.DenominationCount.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.treasury.DenominationCount} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.treasury.DenominationCount.toObject = function(includeInstance, msg) {
  var f, obj = {
    denomination: (f = msg.getDenomination()) && proto.common.Amount.toObject(includeInstance, f),
    count: jspb.Message.getFieldWithDefault(msg, 2, 0),
    total: (f = msg.getTotal()) && proto.common.Amount.toObject(includeInstance, f)
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
 * @return {!proto.treasury.DenominationCount}
 */
proto.treasury.DenominationCount.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.treasury.DenominationCount;
  return proto.treasury.DenominationCount.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.treasury.DenominationCount} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.treasury.DenominationCount}
 */
proto.treasury.DenominationCount.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.common.Amount;
      reader.readMessage(value,proto.common.Amount.deserializeBinaryFromReader);
      msg.setDenomination(value);
      break;
    case 2:
      var value = /** @type {number} */ (reader.readUint32());
      msg.setCount(value);
      break;
    case 3:
      var value = new proto.common.Amount;
      reader.readMessage(value,proto.common.Amount.deserializeBinaryFromReader);
      msg.setTotal(value);
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
proto.treasury.DenominationCount.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.treasury.DenominationCount.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.treasury.DenominationCount} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.treasury.DenominationCount.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getDenomination();
  if (f != null) {
    writer.writeMessage(
      1,
      f,
      proto.common.Amount.serializeBinaryToWriter
    );
  }
  f = message.getCount();
  if (f !== 0) {
    writer.writeUint32(
      2,
      f
    );
  }
  f = message.getTotal();
  if (f != null) {
    writer.writeMessage(
      3,
      f,
      proto.common.Amount.serializeBinaryToWriter
    );
  }
};


/**
 * optional common.Amount denomination = 1;
 * @return {?proto.common.Amount}
 */
proto.treasury.DenominationCount.prototype.getDenomination = function() {
  return /** @type{?proto.common.Amount} */ (
    jspb.Message.getWrapperField(this, proto.common.Amount, 1));
};


/**
 * @param {?proto.common.Amount|undefined} value
 * @return {!proto.treasury.DenominationCount} returns this
*/
proto.treasury.DenominationCount.prototype.setDenomination = function(value) {
  return jspb.Message.setWrapperField(this, 1, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.treasury.DenominationCount} returns this
 */
proto.treasury.DenominationCount.prototype.clearDenomination = function() {
  return this.setDenomination(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.treasury.DenominationCount.prototype.hasDenomination = function() {
  return jspb.Message.getField(this, 1) != null;
};


/**
 * optional uint32 count = 2;
 * @return {number}
 */
proto.treasury.DenominationCount.prototype.getCount = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 2, 0));
};


/**
 * @param {number} value
 * @return {!proto.treasury.DenominationCount} returns this
 */
proto.treasury.DenominationCount.prototype.setCount = function(value) {
  return jspb.Message.setProto3IntField(this, 2, value);
};


/**
 * optional common.Amount total = 3;
 * @return {?proto.common.Amount}
 */
proto.treasury.DenominationCount.prototype.getTotal = function() {
  return /** @type{?proto.common.Amount} */ (
    jspb.Message.getWrapperField(this, proto.common.Amount, 3));
};


/**
 * @param {?proto.common.Amount|undefined} value
 * @return {!proto.treasury.DenominationCount} returns this
*/
proto.treasury.DenominationCount.prototype.setTotal = function(value) {
  return jspb.Message.setWrapperField(this, 3, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.treasury.DenominationCount} returns this
 */
proto.treasury.DenominationCount.prototype.clearTotal = function() {
  return this.setTotal(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.treasury.DenominationCount.prototype.hasTotal = function() {
  return jspb.Message.getField(this, 3) != null;
};


