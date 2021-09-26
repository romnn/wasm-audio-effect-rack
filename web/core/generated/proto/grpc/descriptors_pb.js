// source: proto/grpc/descriptors.proto
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

var jspb = require('google-protobuf');
var goog = jspb;
var global = Function('return this')();

goog.exportSymbol('proto.proto.grpc.AudioAnalyzer', null, global);
goog.exportSymbol('proto.proto.grpc.AudioAnalyzerDescriptor', null, global);
goog.exportSymbol('proto.proto.grpc.AudioInputDescriptor', null, global);
goog.exportSymbol('proto.proto.grpc.AudioInputStream', null, global);
goog.exportSymbol('proto.proto.grpc.AudioOutputDescriptor', null, global);
goog.exportSymbol('proto.proto.grpc.AudioOutputStream', null, global);
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
proto.proto.grpc.AudioInputDescriptor = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.proto.grpc.AudioInputDescriptor, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.proto.grpc.AudioInputDescriptor.displayName = 'proto.proto.grpc.AudioInputDescriptor';
}
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
proto.proto.grpc.AudioAnalyzerDescriptor = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.proto.grpc.AudioAnalyzerDescriptor, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.proto.grpc.AudioAnalyzerDescriptor.displayName = 'proto.proto.grpc.AudioAnalyzerDescriptor';
}
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
proto.proto.grpc.AudioOutputDescriptor = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.proto.grpc.AudioOutputDescriptor, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.proto.grpc.AudioOutputDescriptor.displayName = 'proto.proto.grpc.AudioOutputDescriptor';
}
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
proto.proto.grpc.AudioInputStream = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.proto.grpc.AudioInputStream, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.proto.grpc.AudioInputStream.displayName = 'proto.proto.grpc.AudioInputStream';
}
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
proto.proto.grpc.AudioAnalyzer = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.proto.grpc.AudioAnalyzer, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.proto.grpc.AudioAnalyzer.displayName = 'proto.proto.grpc.AudioAnalyzer';
}
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
proto.proto.grpc.AudioOutputStream = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.proto.grpc.AudioOutputStream, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.proto.grpc.AudioOutputStream.displayName = 'proto.proto.grpc.AudioOutputStream';
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
proto.proto.grpc.AudioInputDescriptor.prototype.toObject = function(opt_includeInstance) {
  return proto.proto.grpc.AudioInputDescriptor.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.proto.grpc.AudioInputDescriptor} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.proto.grpc.AudioInputDescriptor.toObject = function(includeInstance, msg) {
  var f, obj = {
    backend: jspb.Message.getFieldWithDefault(msg, 1, ""),
    device: jspb.Message.getFieldWithDefault(msg, 2, ""),
    host: jspb.Message.getFieldWithDefault(msg, 3, "")
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
 * @return {!proto.proto.grpc.AudioInputDescriptor}
 */
proto.proto.grpc.AudioInputDescriptor.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.proto.grpc.AudioInputDescriptor;
  return proto.proto.grpc.AudioInputDescriptor.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.proto.grpc.AudioInputDescriptor} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.proto.grpc.AudioInputDescriptor}
 */
proto.proto.grpc.AudioInputDescriptor.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setBackend(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setDevice(value);
      break;
    case 3:
      var value = /** @type {string} */ (reader.readString());
      msg.setHost(value);
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
proto.proto.grpc.AudioInputDescriptor.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.proto.grpc.AudioInputDescriptor.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.proto.grpc.AudioInputDescriptor} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.proto.grpc.AudioInputDescriptor.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getBackend();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getDevice();
  if (f.length > 0) {
    writer.writeString(
      2,
      f
    );
  }
  f = message.getHost();
  if (f.length > 0) {
    writer.writeString(
      3,
      f
    );
  }
};


/**
 * optional string backend = 1;
 * @return {string}
 */
proto.proto.grpc.AudioInputDescriptor.prototype.getBackend = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.proto.grpc.AudioInputDescriptor} returns this
 */
proto.proto.grpc.AudioInputDescriptor.prototype.setBackend = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional string device = 2;
 * @return {string}
 */
proto.proto.grpc.AudioInputDescriptor.prototype.getDevice = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.proto.grpc.AudioInputDescriptor} returns this
 */
proto.proto.grpc.AudioInputDescriptor.prototype.setDevice = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};


/**
 * optional string host = 3;
 * @return {string}
 */
proto.proto.grpc.AudioInputDescriptor.prototype.getHost = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 3, ""));
};


/**
 * @param {string} value
 * @return {!proto.proto.grpc.AudioInputDescriptor} returns this
 */
proto.proto.grpc.AudioInputDescriptor.prototype.setHost = function(value) {
  return jspb.Message.setProto3StringField(this, 3, value);
};





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
proto.proto.grpc.AudioAnalyzerDescriptor.prototype.toObject = function(opt_includeInstance) {
  return proto.proto.grpc.AudioAnalyzerDescriptor.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.proto.grpc.AudioAnalyzerDescriptor} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.proto.grpc.AudioAnalyzerDescriptor.toObject = function(includeInstance, msg) {
  var f, obj = {
    name: jspb.Message.getFieldWithDefault(msg, 1, ""),
    input: (f = msg.getInput()) && proto.proto.grpc.AudioInputDescriptor.toObject(includeInstance, f)
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
 * @return {!proto.proto.grpc.AudioAnalyzerDescriptor}
 */
proto.proto.grpc.AudioAnalyzerDescriptor.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.proto.grpc.AudioAnalyzerDescriptor;
  return proto.proto.grpc.AudioAnalyzerDescriptor.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.proto.grpc.AudioAnalyzerDescriptor} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.proto.grpc.AudioAnalyzerDescriptor}
 */
proto.proto.grpc.AudioAnalyzerDescriptor.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setName(value);
      break;
    case 10:
      var value = new proto.proto.grpc.AudioInputDescriptor;
      reader.readMessage(value,proto.proto.grpc.AudioInputDescriptor.deserializeBinaryFromReader);
      msg.setInput(value);
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
proto.proto.grpc.AudioAnalyzerDescriptor.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.proto.grpc.AudioAnalyzerDescriptor.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.proto.grpc.AudioAnalyzerDescriptor} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.proto.grpc.AudioAnalyzerDescriptor.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getName();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getInput();
  if (f != null) {
    writer.writeMessage(
      10,
      f,
      proto.proto.grpc.AudioInputDescriptor.serializeBinaryToWriter
    );
  }
};


/**
 * optional string name = 1;
 * @return {string}
 */
proto.proto.grpc.AudioAnalyzerDescriptor.prototype.getName = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.proto.grpc.AudioAnalyzerDescriptor} returns this
 */
proto.proto.grpc.AudioAnalyzerDescriptor.prototype.setName = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional AudioInputDescriptor input = 10;
 * @return {?proto.proto.grpc.AudioInputDescriptor}
 */
proto.proto.grpc.AudioAnalyzerDescriptor.prototype.getInput = function() {
  return /** @type{?proto.proto.grpc.AudioInputDescriptor} */ (
    jspb.Message.getWrapperField(this, proto.proto.grpc.AudioInputDescriptor, 10));
};


/**
 * @param {?proto.proto.grpc.AudioInputDescriptor|undefined} value
 * @return {!proto.proto.grpc.AudioAnalyzerDescriptor} returns this
*/
proto.proto.grpc.AudioAnalyzerDescriptor.prototype.setInput = function(value) {
  return jspb.Message.setWrapperField(this, 10, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.proto.grpc.AudioAnalyzerDescriptor} returns this
 */
proto.proto.grpc.AudioAnalyzerDescriptor.prototype.clearInput = function() {
  return this.setInput(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.proto.grpc.AudioAnalyzerDescriptor.prototype.hasInput = function() {
  return jspb.Message.getField(this, 10) != null;
};





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
proto.proto.grpc.AudioOutputDescriptor.prototype.toObject = function(opt_includeInstance) {
  return proto.proto.grpc.AudioOutputDescriptor.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.proto.grpc.AudioOutputDescriptor} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.proto.grpc.AudioOutputDescriptor.toObject = function(includeInstance, msg) {
  var f, obj = {
    backend: jspb.Message.getFieldWithDefault(msg, 1, ""),
    device: jspb.Message.getFieldWithDefault(msg, 2, ""),
    host: jspb.Message.getFieldWithDefault(msg, 3, ""),
    input: (f = msg.getInput()) && proto.proto.grpc.AudioInputDescriptor.toObject(includeInstance, f)
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
 * @return {!proto.proto.grpc.AudioOutputDescriptor}
 */
proto.proto.grpc.AudioOutputDescriptor.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.proto.grpc.AudioOutputDescriptor;
  return proto.proto.grpc.AudioOutputDescriptor.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.proto.grpc.AudioOutputDescriptor} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.proto.grpc.AudioOutputDescriptor}
 */
proto.proto.grpc.AudioOutputDescriptor.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setBackend(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setDevice(value);
      break;
    case 3:
      var value = /** @type {string} */ (reader.readString());
      msg.setHost(value);
      break;
    case 10:
      var value = new proto.proto.grpc.AudioInputDescriptor;
      reader.readMessage(value,proto.proto.grpc.AudioInputDescriptor.deserializeBinaryFromReader);
      msg.setInput(value);
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
proto.proto.grpc.AudioOutputDescriptor.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.proto.grpc.AudioOutputDescriptor.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.proto.grpc.AudioOutputDescriptor} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.proto.grpc.AudioOutputDescriptor.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getBackend();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getDevice();
  if (f.length > 0) {
    writer.writeString(
      2,
      f
    );
  }
  f = message.getHost();
  if (f.length > 0) {
    writer.writeString(
      3,
      f
    );
  }
  f = message.getInput();
  if (f != null) {
    writer.writeMessage(
      10,
      f,
      proto.proto.grpc.AudioInputDescriptor.serializeBinaryToWriter
    );
  }
};


/**
 * optional string backend = 1;
 * @return {string}
 */
proto.proto.grpc.AudioOutputDescriptor.prototype.getBackend = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.proto.grpc.AudioOutputDescriptor} returns this
 */
proto.proto.grpc.AudioOutputDescriptor.prototype.setBackend = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional string device = 2;
 * @return {string}
 */
proto.proto.grpc.AudioOutputDescriptor.prototype.getDevice = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.proto.grpc.AudioOutputDescriptor} returns this
 */
proto.proto.grpc.AudioOutputDescriptor.prototype.setDevice = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};


/**
 * optional string host = 3;
 * @return {string}
 */
proto.proto.grpc.AudioOutputDescriptor.prototype.getHost = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 3, ""));
};


/**
 * @param {string} value
 * @return {!proto.proto.grpc.AudioOutputDescriptor} returns this
 */
proto.proto.grpc.AudioOutputDescriptor.prototype.setHost = function(value) {
  return jspb.Message.setProto3StringField(this, 3, value);
};


/**
 * optional AudioInputDescriptor input = 10;
 * @return {?proto.proto.grpc.AudioInputDescriptor}
 */
proto.proto.grpc.AudioOutputDescriptor.prototype.getInput = function() {
  return /** @type{?proto.proto.grpc.AudioInputDescriptor} */ (
    jspb.Message.getWrapperField(this, proto.proto.grpc.AudioInputDescriptor, 10));
};


/**
 * @param {?proto.proto.grpc.AudioInputDescriptor|undefined} value
 * @return {!proto.proto.grpc.AudioOutputDescriptor} returns this
*/
proto.proto.grpc.AudioOutputDescriptor.prototype.setInput = function(value) {
  return jspb.Message.setWrapperField(this, 10, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.proto.grpc.AudioOutputDescriptor} returns this
 */
proto.proto.grpc.AudioOutputDescriptor.prototype.clearInput = function() {
  return this.setInput(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.proto.grpc.AudioOutputDescriptor.prototype.hasInput = function() {
  return jspb.Message.getField(this, 10) != null;
};





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
proto.proto.grpc.AudioInputStream.prototype.toObject = function(opt_includeInstance) {
  return proto.proto.grpc.AudioInputStream.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.proto.grpc.AudioInputStream} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.proto.grpc.AudioInputStream.toObject = function(includeInstance, msg) {
  var f, obj = {
    descriptor: (f = msg.getDescriptor()) && proto.proto.grpc.AudioInputDescriptor.toObject(includeInstance, f)
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
 * @return {!proto.proto.grpc.AudioInputStream}
 */
proto.proto.grpc.AudioInputStream.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.proto.grpc.AudioInputStream;
  return proto.proto.grpc.AudioInputStream.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.proto.grpc.AudioInputStream} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.proto.grpc.AudioInputStream}
 */
proto.proto.grpc.AudioInputStream.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.proto.grpc.AudioInputDescriptor;
      reader.readMessage(value,proto.proto.grpc.AudioInputDescriptor.deserializeBinaryFromReader);
      msg.setDescriptor(value);
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
proto.proto.grpc.AudioInputStream.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.proto.grpc.AudioInputStream.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.proto.grpc.AudioInputStream} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.proto.grpc.AudioInputStream.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getDescriptor();
  if (f != null) {
    writer.writeMessage(
      1,
      f,
      proto.proto.grpc.AudioInputDescriptor.serializeBinaryToWriter
    );
  }
};


/**
 * optional AudioInputDescriptor descriptor = 1;
 * @return {?proto.proto.grpc.AudioInputDescriptor}
 */
proto.proto.grpc.AudioInputStream.prototype.getDescriptor = function() {
  return /** @type{?proto.proto.grpc.AudioInputDescriptor} */ (
    jspb.Message.getWrapperField(this, proto.proto.grpc.AudioInputDescriptor, 1));
};


/**
 * @param {?proto.proto.grpc.AudioInputDescriptor|undefined} value
 * @return {!proto.proto.grpc.AudioInputStream} returns this
*/
proto.proto.grpc.AudioInputStream.prototype.setDescriptor = function(value) {
  return jspb.Message.setWrapperField(this, 1, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.proto.grpc.AudioInputStream} returns this
 */
proto.proto.grpc.AudioInputStream.prototype.clearDescriptor = function() {
  return this.setDescriptor(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.proto.grpc.AudioInputStream.prototype.hasDescriptor = function() {
  return jspb.Message.getField(this, 1) != null;
};





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
proto.proto.grpc.AudioAnalyzer.prototype.toObject = function(opt_includeInstance) {
  return proto.proto.grpc.AudioAnalyzer.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.proto.grpc.AudioAnalyzer} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.proto.grpc.AudioAnalyzer.toObject = function(includeInstance, msg) {
  var f, obj = {
    descriptor: (f = msg.getDescriptor()) && proto.proto.grpc.AudioAnalyzerDescriptor.toObject(includeInstance, f)
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
 * @return {!proto.proto.grpc.AudioAnalyzer}
 */
proto.proto.grpc.AudioAnalyzer.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.proto.grpc.AudioAnalyzer;
  return proto.proto.grpc.AudioAnalyzer.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.proto.grpc.AudioAnalyzer} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.proto.grpc.AudioAnalyzer}
 */
proto.proto.grpc.AudioAnalyzer.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.proto.grpc.AudioAnalyzerDescriptor;
      reader.readMessage(value,proto.proto.grpc.AudioAnalyzerDescriptor.deserializeBinaryFromReader);
      msg.setDescriptor(value);
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
proto.proto.grpc.AudioAnalyzer.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.proto.grpc.AudioAnalyzer.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.proto.grpc.AudioAnalyzer} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.proto.grpc.AudioAnalyzer.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getDescriptor();
  if (f != null) {
    writer.writeMessage(
      1,
      f,
      proto.proto.grpc.AudioAnalyzerDescriptor.serializeBinaryToWriter
    );
  }
};


/**
 * optional AudioAnalyzerDescriptor descriptor = 1;
 * @return {?proto.proto.grpc.AudioAnalyzerDescriptor}
 */
proto.proto.grpc.AudioAnalyzer.prototype.getDescriptor = function() {
  return /** @type{?proto.proto.grpc.AudioAnalyzerDescriptor} */ (
    jspb.Message.getWrapperField(this, proto.proto.grpc.AudioAnalyzerDescriptor, 1));
};


/**
 * @param {?proto.proto.grpc.AudioAnalyzerDescriptor|undefined} value
 * @return {!proto.proto.grpc.AudioAnalyzer} returns this
*/
proto.proto.grpc.AudioAnalyzer.prototype.setDescriptor = function(value) {
  return jspb.Message.setWrapperField(this, 1, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.proto.grpc.AudioAnalyzer} returns this
 */
proto.proto.grpc.AudioAnalyzer.prototype.clearDescriptor = function() {
  return this.setDescriptor(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.proto.grpc.AudioAnalyzer.prototype.hasDescriptor = function() {
  return jspb.Message.getField(this, 1) != null;
};





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
proto.proto.grpc.AudioOutputStream.prototype.toObject = function(opt_includeInstance) {
  return proto.proto.grpc.AudioOutputStream.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.proto.grpc.AudioOutputStream} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.proto.grpc.AudioOutputStream.toObject = function(includeInstance, msg) {
  var f, obj = {
    descriptor: (f = msg.getDescriptor()) && proto.proto.grpc.AudioOutputDescriptor.toObject(includeInstance, f)
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
 * @return {!proto.proto.grpc.AudioOutputStream}
 */
proto.proto.grpc.AudioOutputStream.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.proto.grpc.AudioOutputStream;
  return proto.proto.grpc.AudioOutputStream.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.proto.grpc.AudioOutputStream} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.proto.grpc.AudioOutputStream}
 */
proto.proto.grpc.AudioOutputStream.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.proto.grpc.AudioOutputDescriptor;
      reader.readMessage(value,proto.proto.grpc.AudioOutputDescriptor.deserializeBinaryFromReader);
      msg.setDescriptor(value);
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
proto.proto.grpc.AudioOutputStream.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.proto.grpc.AudioOutputStream.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.proto.grpc.AudioOutputStream} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.proto.grpc.AudioOutputStream.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getDescriptor();
  if (f != null) {
    writer.writeMessage(
      1,
      f,
      proto.proto.grpc.AudioOutputDescriptor.serializeBinaryToWriter
    );
  }
};


/**
 * optional AudioOutputDescriptor descriptor = 1;
 * @return {?proto.proto.grpc.AudioOutputDescriptor}
 */
proto.proto.grpc.AudioOutputStream.prototype.getDescriptor = function() {
  return /** @type{?proto.proto.grpc.AudioOutputDescriptor} */ (
    jspb.Message.getWrapperField(this, proto.proto.grpc.AudioOutputDescriptor, 1));
};


/**
 * @param {?proto.proto.grpc.AudioOutputDescriptor|undefined} value
 * @return {!proto.proto.grpc.AudioOutputStream} returns this
*/
proto.proto.grpc.AudioOutputStream.prototype.setDescriptor = function(value) {
  return jspb.Message.setWrapperField(this, 1, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.proto.grpc.AudioOutputStream} returns this
 */
proto.proto.grpc.AudioOutputStream.prototype.clearDescriptor = function() {
  return this.setDescriptor(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.proto.grpc.AudioOutputStream.prototype.hasDescriptor = function() {
  return jspb.Message.getField(this, 1) != null;
};


goog.object.extend(exports, proto.proto.grpc);
