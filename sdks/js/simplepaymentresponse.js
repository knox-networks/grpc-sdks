// source: common/v1/packet.proto
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

goog.provide('proto.common.SimplePaymentResponse');

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
proto.common.SimplePaymentResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, proto.common.SimplePaymentResponse.repeatedFields_, null);
};
goog.inherits(proto.common.SimplePaymentResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.common.SimplePaymentResponse.displayName = 'proto.common.SimplePaymentResponse';
}

/**
 * List of repeated fields within this message type.
 * @private {!Array<number>}
 * @const
 */
proto.common.SimplePaymentResponse.repeatedFields_ = [2];



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
proto.common.SimplePaymentResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.common.SimplePaymentResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.common.SimplePaymentResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.common.SimplePaymentResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    receivedAmount: (f = msg.getReceivedAmount()) && proto.common.Amount.toObject(includeInstance, f),
    receivedPromissoriesList: (f = jspb.Message.getRepeatedField(msg, 2)) == null ? undefined : f
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
 * @return {!proto.common.SimplePaymentResponse}
 */
proto.common.SimplePaymentResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.common.SimplePaymentResponse;
  return proto.common.SimplePaymentResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.common.SimplePaymentResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.common.SimplePaymentResponse}
 */
proto.common.SimplePaymentResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.common.Amount;
      reader.readMessage(value,proto.common.Amount.deserializeBinaryFromReader);
      msg.setReceivedAmount(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.addReceivedPromissories(value);
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
proto.common.SimplePaymentResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.common.SimplePaymentResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.common.SimplePaymentResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.common.SimplePaymentResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getReceivedAmount();
  if (f != null) {
    writer.writeMessage(
      1,
      f,
      proto.common.Amount.serializeBinaryToWriter
    );
  }
  f = message.getReceivedPromissoriesList();
  if (f.length > 0) {
    writer.writeRepeatedString(
      2,
      f
    );
  }
};


/**
 * optional Amount received_amount = 1;
 * @return {?proto.common.Amount}
 */
proto.common.SimplePaymentResponse.prototype.getReceivedAmount = function() {
  return /** @type{?proto.common.Amount} */ (
    jspb.Message.getWrapperField(this, proto.common.Amount, 1));
};


/**
 * @param {?proto.common.Amount|undefined} value
 * @return {!proto.common.SimplePaymentResponse} returns this
*/
proto.common.SimplePaymentResponse.prototype.setReceivedAmount = function(value) {
  return jspb.Message.setWrapperField(this, 1, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.common.SimplePaymentResponse} returns this
 */
proto.common.SimplePaymentResponse.prototype.clearReceivedAmount = function() {
  return this.setReceivedAmount(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.common.SimplePaymentResponse.prototype.hasReceivedAmount = function() {
  return jspb.Message.getField(this, 1) != null;
};


/**
 * repeated string received_promissories = 2;
 * @return {!Array<string>}
 */
proto.common.SimplePaymentResponse.prototype.getReceivedPromissoriesList = function() {
  return /** @type {!Array<string>} */ (jspb.Message.getRepeatedField(this, 2));
};


/**
 * @param {!Array<string>} value
 * @return {!proto.common.SimplePaymentResponse} returns this
 */
proto.common.SimplePaymentResponse.prototype.setReceivedPromissoriesList = function(value) {
  return jspb.Message.setField(this, 2, value || []);
};


/**
 * @param {string} value
 * @param {number=} opt_index
 * @return {!proto.common.SimplePaymentResponse} returns this
 */
proto.common.SimplePaymentResponse.prototype.addReceivedPromissories = function(value, opt_index) {
  return jspb.Message.addToRepeatedField(this, 2, value, opt_index);
};


/**
 * Clears the list making it empty but non-null.
 * @return {!proto.common.SimplePaymentResponse} returns this
 */
proto.common.SimplePaymentResponse.prototype.clearReceivedPromissoriesList = function() {
  return this.setReceivedPromissoriesList([]);
};


