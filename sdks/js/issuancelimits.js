// source: issuer_api/v1/issuer.proto
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

goog.provide('proto.issuer_api.v1.IssuanceLimits');

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
proto.issuer_api.v1.IssuanceLimits = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, proto.issuer_api.v1.IssuanceLimits.repeatedFields_, null);
};
goog.inherits(proto.issuer_api.v1.IssuanceLimits, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.issuer_api.v1.IssuanceLimits.displayName = 'proto.issuer_api.v1.IssuanceLimits';
}

/**
 * List of repeated fields within this message type.
 * @private {!Array<number>}
 * @const
 */
proto.issuer_api.v1.IssuanceLimits.repeatedFields_ = [2];



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
proto.issuer_api.v1.IssuanceLimits.prototype.toObject = function(opt_includeInstance) {
  return proto.issuer_api.v1.IssuanceLimits.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.issuer_api.v1.IssuanceLimits} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.issuer_api.v1.IssuanceLimits.toObject = function(includeInstance, msg) {
  var f, obj = {
    innerList: jspb.Message.toObjectList(msg.getInnerList(),
    proto.common.Amount.toObject, includeInstance)
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
 * @return {!proto.issuer_api.v1.IssuanceLimits}
 */
proto.issuer_api.v1.IssuanceLimits.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.issuer_api.v1.IssuanceLimits;
  return proto.issuer_api.v1.IssuanceLimits.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.issuer_api.v1.IssuanceLimits} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.issuer_api.v1.IssuanceLimits}
 */
proto.issuer_api.v1.IssuanceLimits.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 2:
      var value = new proto.common.Amount;
      reader.readMessage(value,proto.common.Amount.deserializeBinaryFromReader);
      msg.addInner(value);
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
proto.issuer_api.v1.IssuanceLimits.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.issuer_api.v1.IssuanceLimits.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.issuer_api.v1.IssuanceLimits} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.issuer_api.v1.IssuanceLimits.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getInnerList();
  if (f.length > 0) {
    writer.writeRepeatedMessage(
      2,
      f,
      proto.common.Amount.serializeBinaryToWriter
    );
  }
};


/**
 * repeated common.Amount inner = 2;
 * @return {!Array<!proto.common.Amount>}
 */
proto.issuer_api.v1.IssuanceLimits.prototype.getInnerList = function() {
  return /** @type{!Array<!proto.common.Amount>} */ (
    jspb.Message.getRepeatedWrapperField(this, proto.common.Amount, 2));
};


/**
 * @param {!Array<!proto.common.Amount>} value
 * @return {!proto.issuer_api.v1.IssuanceLimits} returns this
*/
proto.issuer_api.v1.IssuanceLimits.prototype.setInnerList = function(value) {
  return jspb.Message.setRepeatedWrapperField(this, 2, value);
};


/**
 * @param {!proto.common.Amount=} opt_value
 * @param {number=} opt_index
 * @return {!proto.common.Amount}
 */
proto.issuer_api.v1.IssuanceLimits.prototype.addInner = function(opt_value, opt_index) {
  return jspb.Message.addToRepeatedWrapperField(this, 2, opt_value, proto.common.Amount, opt_index);
};


/**
 * Clears the list making it empty but non-null.
 * @return {!proto.issuer_api.v1.IssuanceLimits} returns this
 */
proto.issuer_api.v1.IssuanceLimits.prototype.clearInnerList = function() {
  return this.setInnerList([]);
};


