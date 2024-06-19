// source: registry_api/v1/registry.proto
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

goog.provide('proto.registry_api.v1.ResolutionMetadataDidUrl');

goog.require('jspb.BinaryReader');
goog.require('jspb.BinaryWriter');
goog.require('jspb.Message');

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
proto.registry_api.v1.ResolutionMetadataDidUrl = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.registry_api.v1.ResolutionMetadataDidUrl, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.registry_api.v1.ResolutionMetadataDidUrl.displayName = 'proto.registry_api.v1.ResolutionMetadataDidUrl';
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
proto.registry_api.v1.ResolutionMetadataDidUrl.prototype.toObject = function(opt_includeInstance) {
  return proto.registry_api.v1.ResolutionMetadataDidUrl.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.registry_api.v1.ResolutionMetadataDidUrl} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.registry_api.v1.ResolutionMetadataDidUrl.toObject = function(includeInstance, msg) {
  var f, obj = {
    did: jspb.Message.getFieldWithDefault(msg, 1, ""),
    methodname: jspb.Message.getFieldWithDefault(msg, 2, ""),
    methodspecificid: jspb.Message.getFieldWithDefault(msg, 3, "")
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
 * @return {!proto.registry_api.v1.ResolutionMetadataDidUrl}
 */
proto.registry_api.v1.ResolutionMetadataDidUrl.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.registry_api.v1.ResolutionMetadataDidUrl;
  return proto.registry_api.v1.ResolutionMetadataDidUrl.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.registry_api.v1.ResolutionMetadataDidUrl} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.registry_api.v1.ResolutionMetadataDidUrl}
 */
proto.registry_api.v1.ResolutionMetadataDidUrl.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setDid(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setMethodname(value);
      break;
    case 3:
      var value = /** @type {string} */ (reader.readString());
      msg.setMethodspecificid(value);
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
proto.registry_api.v1.ResolutionMetadataDidUrl.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.registry_api.v1.ResolutionMetadataDidUrl.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.registry_api.v1.ResolutionMetadataDidUrl} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.registry_api.v1.ResolutionMetadataDidUrl.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getDid();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getMethodname();
  if (f.length > 0) {
    writer.writeString(
      2,
      f
    );
  }
  f = message.getMethodspecificid();
  if (f.length > 0) {
    writer.writeString(
      3,
      f
    );
  }
};


/**
 * optional string did = 1;
 * @return {string}
 */
proto.registry_api.v1.ResolutionMetadataDidUrl.prototype.getDid = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.registry_api.v1.ResolutionMetadataDidUrl} returns this
 */
proto.registry_api.v1.ResolutionMetadataDidUrl.prototype.setDid = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional string methodName = 2;
 * @return {string}
 */
proto.registry_api.v1.ResolutionMetadataDidUrl.prototype.getMethodname = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.registry_api.v1.ResolutionMetadataDidUrl} returns this
 */
proto.registry_api.v1.ResolutionMetadataDidUrl.prototype.setMethodname = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};


/**
 * optional string methodSpecificId = 3;
 * @return {string}
 */
proto.registry_api.v1.ResolutionMetadataDidUrl.prototype.getMethodspecificid = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 3, ""));
};


/**
 * @param {string} value
 * @return {!proto.registry_api.v1.ResolutionMetadataDidUrl} returns this
 */
proto.registry_api.v1.ResolutionMetadataDidUrl.prototype.setMethodspecificid = function(value) {
  return jspb.Message.setProto3StringField(this, 3, value);
};

