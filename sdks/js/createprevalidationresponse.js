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

goog.provide('proto.webhook_api.v1.CreatePrevalidationResponse');

goog.require('jspb.BinaryReader');
goog.require('jspb.BinaryWriter');
goog.require('jspb.Message');
goog.require('proto.webhook_api.v1.Prevalidation');

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
proto.webhook_api.v1.CreatePrevalidationResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.webhook_api.v1.CreatePrevalidationResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.webhook_api.v1.CreatePrevalidationResponse.displayName = 'proto.webhook_api.v1.CreatePrevalidationResponse';
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
proto.webhook_api.v1.CreatePrevalidationResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.webhook_api.v1.CreatePrevalidationResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.webhook_api.v1.CreatePrevalidationResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.webhook_api.v1.CreatePrevalidationResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    prevalidation: (f = msg.getPrevalidation()) && proto.webhook_api.v1.Prevalidation.toObject(includeInstance, f)
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
 * @return {!proto.webhook_api.v1.CreatePrevalidationResponse}
 */
proto.webhook_api.v1.CreatePrevalidationResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.webhook_api.v1.CreatePrevalidationResponse;
  return proto.webhook_api.v1.CreatePrevalidationResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.webhook_api.v1.CreatePrevalidationResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.webhook_api.v1.CreatePrevalidationResponse}
 */
proto.webhook_api.v1.CreatePrevalidationResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.webhook_api.v1.Prevalidation;
      reader.readMessage(value,proto.webhook_api.v1.Prevalidation.deserializeBinaryFromReader);
      msg.setPrevalidation(value);
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
proto.webhook_api.v1.CreatePrevalidationResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.webhook_api.v1.CreatePrevalidationResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.webhook_api.v1.CreatePrevalidationResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.webhook_api.v1.CreatePrevalidationResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getPrevalidation();
  if (f != null) {
    writer.writeMessage(
      1,
      f,
      proto.webhook_api.v1.Prevalidation.serializeBinaryToWriter
    );
  }
};


/**
 * optional Prevalidation prevalidation = 1;
 * @return {?proto.webhook_api.v1.Prevalidation}
 */
proto.webhook_api.v1.CreatePrevalidationResponse.prototype.getPrevalidation = function() {
  return /** @type{?proto.webhook_api.v1.Prevalidation} */ (
    jspb.Message.getWrapperField(this, proto.webhook_api.v1.Prevalidation, 1));
};


/**
 * @param {?proto.webhook_api.v1.Prevalidation|undefined} value
 * @return {!proto.webhook_api.v1.CreatePrevalidationResponse} returns this
*/
proto.webhook_api.v1.CreatePrevalidationResponse.prototype.setPrevalidation = function(value) {
  return jspb.Message.setWrapperField(this, 1, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.webhook_api.v1.CreatePrevalidationResponse} returns this
 */
proto.webhook_api.v1.CreatePrevalidationResponse.prototype.clearPrevalidation = function() {
  return this.setPrevalidation(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.webhook_api.v1.CreatePrevalidationResponse.prototype.hasPrevalidation = function() {
  return jspb.Message.getField(this, 1) != null;
};


