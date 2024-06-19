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

goog.provide('proto.registry_api.v1.ResolveRepresentationResponse');

goog.require('jspb.BinaryReader');
goog.require('jspb.BinaryWriter');
goog.require('jspb.Message');
goog.require('proto.registry_api.v1.DidDocumentMetadata');
goog.require('proto.registry_api.v1.ResolutionMetadata');

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
proto.registry_api.v1.ResolveRepresentationResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.registry_api.v1.ResolveRepresentationResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.registry_api.v1.ResolveRepresentationResponse.displayName = 'proto.registry_api.v1.ResolveRepresentationResponse';
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
proto.registry_api.v1.ResolveRepresentationResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.registry_api.v1.ResolveRepresentationResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.registry_api.v1.ResolveRepresentationResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.registry_api.v1.ResolveRepresentationResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    diddocumentstream: jspb.Message.getFieldWithDefault(msg, 1, ""),
    didresolutionmetadata: (f = msg.getDidresolutionmetadata()) && proto.registry_api.v1.ResolutionMetadata.toObject(includeInstance, f),
    diddocumentmetadata: (f = msg.getDiddocumentmetadata()) && proto.registry_api.v1.DidDocumentMetadata.toObject(includeInstance, f)
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
 * @return {!proto.registry_api.v1.ResolveRepresentationResponse}
 */
proto.registry_api.v1.ResolveRepresentationResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.registry_api.v1.ResolveRepresentationResponse;
  return proto.registry_api.v1.ResolveRepresentationResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.registry_api.v1.ResolveRepresentationResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.registry_api.v1.ResolveRepresentationResponse}
 */
proto.registry_api.v1.ResolveRepresentationResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setDiddocumentstream(value);
      break;
    case 2:
      var value = new proto.registry_api.v1.ResolutionMetadata;
      reader.readMessage(value,proto.registry_api.v1.ResolutionMetadata.deserializeBinaryFromReader);
      msg.setDidresolutionmetadata(value);
      break;
    case 3:
      var value = new proto.registry_api.v1.DidDocumentMetadata;
      reader.readMessage(value,proto.registry_api.v1.DidDocumentMetadata.deserializeBinaryFromReader);
      msg.setDiddocumentmetadata(value);
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
proto.registry_api.v1.ResolveRepresentationResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.registry_api.v1.ResolveRepresentationResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.registry_api.v1.ResolveRepresentationResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.registry_api.v1.ResolveRepresentationResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getDiddocumentstream();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getDidresolutionmetadata();
  if (f != null) {
    writer.writeMessage(
      2,
      f,
      proto.registry_api.v1.ResolutionMetadata.serializeBinaryToWriter
    );
  }
  f = message.getDiddocumentmetadata();
  if (f != null) {
    writer.writeMessage(
      3,
      f,
      proto.registry_api.v1.DidDocumentMetadata.serializeBinaryToWriter
    );
  }
};


/**
 * optional string didDocumentStream = 1;
 * @return {string}
 */
proto.registry_api.v1.ResolveRepresentationResponse.prototype.getDiddocumentstream = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.registry_api.v1.ResolveRepresentationResponse} returns this
 */
proto.registry_api.v1.ResolveRepresentationResponse.prototype.setDiddocumentstream = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional ResolutionMetadata didResolutionMetadata = 2;
 * @return {?proto.registry_api.v1.ResolutionMetadata}
 */
proto.registry_api.v1.ResolveRepresentationResponse.prototype.getDidresolutionmetadata = function() {
  return /** @type{?proto.registry_api.v1.ResolutionMetadata} */ (
    jspb.Message.getWrapperField(this, proto.registry_api.v1.ResolutionMetadata, 2));
};


/**
 * @param {?proto.registry_api.v1.ResolutionMetadata|undefined} value
 * @return {!proto.registry_api.v1.ResolveRepresentationResponse} returns this
*/
proto.registry_api.v1.ResolveRepresentationResponse.prototype.setDidresolutionmetadata = function(value) {
  return jspb.Message.setWrapperField(this, 2, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.registry_api.v1.ResolveRepresentationResponse} returns this
 */
proto.registry_api.v1.ResolveRepresentationResponse.prototype.clearDidresolutionmetadata = function() {
  return this.setDidresolutionmetadata(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.registry_api.v1.ResolveRepresentationResponse.prototype.hasDidresolutionmetadata = function() {
  return jspb.Message.getField(this, 2) != null;
};


/**
 * optional DidDocumentMetadata didDocumentMetadata = 3;
 * @return {?proto.registry_api.v1.DidDocumentMetadata}
 */
proto.registry_api.v1.ResolveRepresentationResponse.prototype.getDiddocumentmetadata = function() {
  return /** @type{?proto.registry_api.v1.DidDocumentMetadata} */ (
    jspb.Message.getWrapperField(this, proto.registry_api.v1.DidDocumentMetadata, 3));
};


/**
 * @param {?proto.registry_api.v1.DidDocumentMetadata|undefined} value
 * @return {!proto.registry_api.v1.ResolveRepresentationResponse} returns this
*/
proto.registry_api.v1.ResolveRepresentationResponse.prototype.setDiddocumentmetadata = function(value) {
  return jspb.Message.setWrapperField(this, 3, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.registry_api.v1.ResolveRepresentationResponse} returns this
 */
proto.registry_api.v1.ResolveRepresentationResponse.prototype.clearDiddocumentmetadata = function() {
  return this.setDiddocumentmetadata(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.registry_api.v1.ResolveRepresentationResponse.prototype.hasDiddocumentmetadata = function() {
  return jspb.Message.getField(this, 3) != null;
};

