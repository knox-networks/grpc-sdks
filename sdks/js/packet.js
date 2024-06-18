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

goog.provide('proto.common.Packet');

goog.require('jspb.BinaryReader');
goog.require('jspb.BinaryWriter');
goog.require('jspb.Message');
goog.require('proto.common.DynamicVerifier');
goog.require('proto.common.TransactionHeader');

goog.forwardDeclare('proto.common.PacketType');
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
proto.common.Packet = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.common.Packet, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.common.Packet.displayName = 'proto.common.Packet';
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
proto.common.Packet.prototype.toObject = function(opt_includeInstance) {
  return proto.common.Packet.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.common.Packet} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.common.Packet.toObject = function(includeInstance, msg) {
  var f, obj = {
    packetType: jspb.Message.getFieldWithDefault(msg, 1, 0),
    data: msg.getData_asB64(),
    sender: (f = msg.getSender()) && proto.common.DynamicVerifier.toObject(includeInstance, f),
    recipient: (f = msg.getRecipient()) && proto.common.DynamicVerifier.toObject(includeInstance, f),
    nonce: msg.getNonce_asB64(),
    transactionHeader: (f = msg.getTransactionHeader()) && proto.common.TransactionHeader.toObject(includeInstance, f)
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
 * @return {!proto.common.Packet}
 */
proto.common.Packet.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.common.Packet;
  return proto.common.Packet.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.common.Packet} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.common.Packet}
 */
proto.common.Packet.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {!proto.common.PacketType} */ (reader.readEnum());
      msg.setPacketType(value);
      break;
    case 2:
      var value = /** @type {!Uint8Array} */ (reader.readBytes());
      msg.setData(value);
      break;
    case 4:
      var value = new proto.common.DynamicVerifier;
      reader.readMessage(value,proto.common.DynamicVerifier.deserializeBinaryFromReader);
      msg.setSender(value);
      break;
    case 5:
      var value = new proto.common.DynamicVerifier;
      reader.readMessage(value,proto.common.DynamicVerifier.deserializeBinaryFromReader);
      msg.setRecipient(value);
      break;
    case 6:
      var value = /** @type {!Uint8Array} */ (reader.readBytes());
      msg.setNonce(value);
      break;
    case 7:
      var value = new proto.common.TransactionHeader;
      reader.readMessage(value,proto.common.TransactionHeader.deserializeBinaryFromReader);
      msg.setTransactionHeader(value);
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
proto.common.Packet.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.common.Packet.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.common.Packet} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.common.Packet.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getPacketType();
  if (f !== 0.0) {
    writer.writeEnum(
      1,
      f
    );
  }
  f = message.getData_asU8();
  if (f.length > 0) {
    writer.writeBytes(
      2,
      f
    );
  }
  f = message.getSender();
  if (f != null) {
    writer.writeMessage(
      4,
      f,
      proto.common.DynamicVerifier.serializeBinaryToWriter
    );
  }
  f = message.getRecipient();
  if (f != null) {
    writer.writeMessage(
      5,
      f,
      proto.common.DynamicVerifier.serializeBinaryToWriter
    );
  }
  f = message.getNonce_asU8();
  if (f.length > 0) {
    writer.writeBytes(
      6,
      f
    );
  }
  f = message.getTransactionHeader();
  if (f != null) {
    writer.writeMessage(
      7,
      f,
      proto.common.TransactionHeader.serializeBinaryToWriter
    );
  }
};


/**
 * optional PacketType packet_type = 1;
 * @return {!proto.common.PacketType}
 */
proto.common.Packet.prototype.getPacketType = function() {
  return /** @type {!proto.common.PacketType} */ (jspb.Message.getFieldWithDefault(this, 1, 0));
};


/**
 * @param {!proto.common.PacketType} value
 * @return {!proto.common.Packet} returns this
 */
proto.common.Packet.prototype.setPacketType = function(value) {
  return jspb.Message.setProto3EnumField(this, 1, value);
};


/**
 * optional bytes data = 2;
 * @return {string}
 */
proto.common.Packet.prototype.getData = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * optional bytes data = 2;
 * This is a type-conversion wrapper around `getData()`
 * @return {string}
 */
proto.common.Packet.prototype.getData_asB64 = function() {
  return /** @type {string} */ (jspb.Message.bytesAsB64(
      this.getData()));
};


/**
 * optional bytes data = 2;
 * Note that Uint8Array is not supported on all browsers.
 * @see http://caniuse.com/Uint8Array
 * This is a type-conversion wrapper around `getData()`
 * @return {!Uint8Array}
 */
proto.common.Packet.prototype.getData_asU8 = function() {
  return /** @type {!Uint8Array} */ (jspb.Message.bytesAsU8(
      this.getData()));
};


/**
 * @param {!(string|Uint8Array)} value
 * @return {!proto.common.Packet} returns this
 */
proto.common.Packet.prototype.setData = function(value) {
  return jspb.Message.setProto3BytesField(this, 2, value);
};


/**
 * optional DynamicVerifier sender = 4;
 * @return {?proto.common.DynamicVerifier}
 */
proto.common.Packet.prototype.getSender = function() {
  return /** @type{?proto.common.DynamicVerifier} */ (
    jspb.Message.getWrapperField(this, proto.common.DynamicVerifier, 4));
};


/**
 * @param {?proto.common.DynamicVerifier|undefined} value
 * @return {!proto.common.Packet} returns this
*/
proto.common.Packet.prototype.setSender = function(value) {
  return jspb.Message.setWrapperField(this, 4, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.common.Packet} returns this
 */
proto.common.Packet.prototype.clearSender = function() {
  return this.setSender(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.common.Packet.prototype.hasSender = function() {
  return jspb.Message.getField(this, 4) != null;
};


/**
 * optional DynamicVerifier recipient = 5;
 * @return {?proto.common.DynamicVerifier}
 */
proto.common.Packet.prototype.getRecipient = function() {
  return /** @type{?proto.common.DynamicVerifier} */ (
    jspb.Message.getWrapperField(this, proto.common.DynamicVerifier, 5));
};


/**
 * @param {?proto.common.DynamicVerifier|undefined} value
 * @return {!proto.common.Packet} returns this
*/
proto.common.Packet.prototype.setRecipient = function(value) {
  return jspb.Message.setWrapperField(this, 5, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.common.Packet} returns this
 */
proto.common.Packet.prototype.clearRecipient = function() {
  return this.setRecipient(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.common.Packet.prototype.hasRecipient = function() {
  return jspb.Message.getField(this, 5) != null;
};


/**
 * optional bytes nonce = 6;
 * @return {string}
 */
proto.common.Packet.prototype.getNonce = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 6, ""));
};


/**
 * optional bytes nonce = 6;
 * This is a type-conversion wrapper around `getNonce()`
 * @return {string}
 */
proto.common.Packet.prototype.getNonce_asB64 = function() {
  return /** @type {string} */ (jspb.Message.bytesAsB64(
      this.getNonce()));
};


/**
 * optional bytes nonce = 6;
 * Note that Uint8Array is not supported on all browsers.
 * @see http://caniuse.com/Uint8Array
 * This is a type-conversion wrapper around `getNonce()`
 * @return {!Uint8Array}
 */
proto.common.Packet.prototype.getNonce_asU8 = function() {
  return /** @type {!Uint8Array} */ (jspb.Message.bytesAsU8(
      this.getNonce()));
};


/**
 * @param {!(string|Uint8Array)} value
 * @return {!proto.common.Packet} returns this
 */
proto.common.Packet.prototype.setNonce = function(value) {
  return jspb.Message.setProto3BytesField(this, 6, value);
};


/**
 * optional TransactionHeader transaction_header = 7;
 * @return {?proto.common.TransactionHeader}
 */
proto.common.Packet.prototype.getTransactionHeader = function() {
  return /** @type{?proto.common.TransactionHeader} */ (
    jspb.Message.getWrapperField(this, proto.common.TransactionHeader, 7));
};


/**
 * @param {?proto.common.TransactionHeader|undefined} value
 * @return {!proto.common.Packet} returns this
*/
proto.common.Packet.prototype.setTransactionHeader = function(value) {
  return jspb.Message.setWrapperField(this, 7, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.common.Packet} returns this
 */
proto.common.Packet.prototype.clearTransactionHeader = function() {
  return this.setTransactionHeader(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.common.Packet.prototype.hasTransactionHeader = function() {
  return jspb.Message.getField(this, 7) != null;
};


