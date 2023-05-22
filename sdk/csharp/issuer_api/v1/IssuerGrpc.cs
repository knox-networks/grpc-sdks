// <auto-generated>
//     Generated by the protocol buffer compiler.  DO NOT EDIT!
//     source: issuer_api/v1/issuer.proto
// </auto-generated>
#pragma warning disable 0414, 1591
#region Designer generated code

using grpc = global::Grpc.Core;

namespace IssuerApi.V1 {
  /// <summary>
  /// Financial intermediary with a reserve account with the Authority. A role typically played by commercial banks and
  /// other depository institutions, it can issue, transfer, redeem digital banknotes and request an increase in issuance
  /// limits.  It holds digital banknotes in its own vault.
  /// </summary>
  public static partial class IssuerService
  {
    static readonly string __ServiceName = "issuer_api.v1.IssuerService";

    [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
    static void __Helper_SerializeMessage(global::Google.Protobuf.IMessage message, grpc::SerializationContext context)
    {
      #if !GRPC_DISABLE_PROTOBUF_BUFFER_SERIALIZATION
      if (message is global::Google.Protobuf.IBufferMessage)
      {
        context.SetPayloadLength(message.CalculateSize());
        global::Google.Protobuf.MessageExtensions.WriteTo(message, context.GetBufferWriter());
        context.Complete();
        return;
      }
      #endif
      context.Complete(global::Google.Protobuf.MessageExtensions.ToByteArray(message));
    }

    [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
    static class __Helper_MessageCache<T>
    {
      public static readonly bool IsBufferMessage = global::System.Reflection.IntrospectionExtensions.GetTypeInfo(typeof(global::Google.Protobuf.IBufferMessage)).IsAssignableFrom(typeof(T));
    }

    [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
    static T __Helper_DeserializeMessage<T>(grpc::DeserializationContext context, global::Google.Protobuf.MessageParser<T> parser) where T : global::Google.Protobuf.IMessage<T>
    {
      #if !GRPC_DISABLE_PROTOBUF_BUFFER_SERIALIZATION
      if (__Helper_MessageCache<T>.IsBufferMessage)
      {
        return parser.ParseFrom(context.PayloadAsReadOnlySequence());
      }
      #endif
      return parser.ParseFrom(context.PayloadAsNewBuffer());
    }

    [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
    static readonly grpc::Marshaller<global::IssuerApi.V1.AuthenticateRequest> __Marshaller_issuer_api_v1_AuthenticateRequest = grpc::Marshallers.Create(__Helper_SerializeMessage, context => __Helper_DeserializeMessage(context, global::IssuerApi.V1.AuthenticateRequest.Parser));
    [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
    static readonly grpc::Marshaller<global::IssuerApi.V1.AuthenticateResponse> __Marshaller_issuer_api_v1_AuthenticateResponse = grpc::Marshallers.Create(__Helper_SerializeMessage, context => __Helper_DeserializeMessage(context, global::IssuerApi.V1.AuthenticateResponse.Parser));
    [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
    static readonly grpc::Marshaller<global::IssuerApi.V1.AuthorizeRequest> __Marshaller_issuer_api_v1_AuthorizeRequest = grpc::Marshallers.Create(__Helper_SerializeMessage, context => __Helper_DeserializeMessage(context, global::IssuerApi.V1.AuthorizeRequest.Parser));
    [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
    static readonly grpc::Marshaller<global::IssuerApi.V1.AuthorizeResponse> __Marshaller_issuer_api_v1_AuthorizeResponse = grpc::Marshallers.Create(__Helper_SerializeMessage, context => __Helper_DeserializeMessage(context, global::IssuerApi.V1.AuthorizeResponse.Parser));
    [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
    static readonly grpc::Marshaller<global::IssuerApi.V1.IssueRequest> __Marshaller_issuer_api_v1_IssueRequest = grpc::Marshallers.Create(__Helper_SerializeMessage, context => __Helper_DeserializeMessage(context, global::IssuerApi.V1.IssueRequest.Parser));
    [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
    static readonly grpc::Marshaller<global::IssuerApi.V1.IssueResponse> __Marshaller_issuer_api_v1_IssueResponse = grpc::Marshallers.Create(__Helper_SerializeMessage, context => __Helper_DeserializeMessage(context, global::IssuerApi.V1.IssueResponse.Parser));
    [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
    static readonly grpc::Marshaller<global::IssuerApi.V1.GetRolesRequest> __Marshaller_issuer_api_v1_GetRolesRequest = grpc::Marshallers.Create(__Helper_SerializeMessage, context => __Helper_DeserializeMessage(context, global::IssuerApi.V1.GetRolesRequest.Parser));
    [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
    static readonly grpc::Marshaller<global::IssuerApi.V1.GetRolesResponse> __Marshaller_issuer_api_v1_GetRolesResponse = grpc::Marshallers.Create(__Helper_SerializeMessage, context => __Helper_DeserializeMessage(context, global::IssuerApi.V1.GetRolesResponse.Parser));
    [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
    static readonly grpc::Marshaller<global::IssuerApi.V1.SetRoleRequest> __Marshaller_issuer_api_v1_SetRoleRequest = grpc::Marshallers.Create(__Helper_SerializeMessage, context => __Helper_DeserializeMessage(context, global::IssuerApi.V1.SetRoleRequest.Parser));
    [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
    static readonly grpc::Marshaller<global::IssuerApi.V1.SetRoleResponse> __Marshaller_issuer_api_v1_SetRoleResponse = grpc::Marshallers.Create(__Helper_SerializeMessage, context => __Helper_DeserializeMessage(context, global::IssuerApi.V1.SetRoleResponse.Parser));

    [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
    static readonly grpc::Method<global::IssuerApi.V1.AuthenticateRequest, global::IssuerApi.V1.AuthenticateResponse> __Method_Authenticate = new grpc::Method<global::IssuerApi.V1.AuthenticateRequest, global::IssuerApi.V1.AuthenticateResponse>(
        grpc::MethodType.Unary,
        __ServiceName,
        "Authenticate",
        __Marshaller_issuer_api_v1_AuthenticateRequest,
        __Marshaller_issuer_api_v1_AuthenticateResponse);

    [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
    static readonly grpc::Method<global::IssuerApi.V1.AuthorizeRequest, global::IssuerApi.V1.AuthorizeResponse> __Method_Authorize = new grpc::Method<global::IssuerApi.V1.AuthorizeRequest, global::IssuerApi.V1.AuthorizeResponse>(
        grpc::MethodType.DuplexStreaming,
        __ServiceName,
        "Authorize",
        __Marshaller_issuer_api_v1_AuthorizeRequest,
        __Marshaller_issuer_api_v1_AuthorizeResponse);

    [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
    static readonly grpc::Method<global::IssuerApi.V1.IssueRequest, global::IssuerApi.V1.IssueResponse> __Method_Issue = new grpc::Method<global::IssuerApi.V1.IssueRequest, global::IssuerApi.V1.IssueResponse>(
        grpc::MethodType.ServerStreaming,
        __ServiceName,
        "Issue",
        __Marshaller_issuer_api_v1_IssueRequest,
        __Marshaller_issuer_api_v1_IssueResponse);

    [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
    static readonly grpc::Method<global::IssuerApi.V1.GetRolesRequest, global::IssuerApi.V1.GetRolesResponse> __Method_GetRoles = new grpc::Method<global::IssuerApi.V1.GetRolesRequest, global::IssuerApi.V1.GetRolesResponse>(
        grpc::MethodType.Unary,
        __ServiceName,
        "GetRoles",
        __Marshaller_issuer_api_v1_GetRolesRequest,
        __Marshaller_issuer_api_v1_GetRolesResponse);

    [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
    static readonly grpc::Method<global::IssuerApi.V1.SetRoleRequest, global::IssuerApi.V1.SetRoleResponse> __Method_SetRole = new grpc::Method<global::IssuerApi.V1.SetRoleRequest, global::IssuerApi.V1.SetRoleResponse>(
        grpc::MethodType.Unary,
        __ServiceName,
        "SetRole",
        __Marshaller_issuer_api_v1_SetRoleRequest,
        __Marshaller_issuer_api_v1_SetRoleResponse);

    /// <summary>Service descriptor</summary>
    public static global::Google.Protobuf.Reflection.ServiceDescriptor Descriptor
    {
      get { return global::IssuerApi.V1.IssuerReflection.Descriptor.Services[0]; }
    }

    /// <summary>Base class for server-side implementations of IssuerService</summary>
    [grpc::BindServiceMethod(typeof(IssuerService), "BindService")]
    public abstract partial class IssuerServiceBase
    {
      /// <summary>
      /// Checks that the Verifier is authenticated, then stores and returns the challenge nonce.
      /// </summary>
      /// <param name="request">The request received from the client.</param>
      /// <param name="context">The context of the server-side call handler being invoked.</param>
      /// <returns>The response to send back to the client (wrapped by a task).</returns>
      [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
      public virtual global::System.Threading.Tasks.Task<global::IssuerApi.V1.AuthenticateResponse> Authenticate(global::IssuerApi.V1.AuthenticateRequest request, grpc::ServerCallContext context)
      {
        throw new grpc::RpcException(new grpc::Status(grpc::StatusCode.Unimplemented, ""));
      }

      /// <summary>
      /// Sends a stream of digital banknotes to be authorized. The digital banknote is checked to see if it is transferred
      /// to the new owner, but not yet authorized. IT is then checked for double spends, signed and saved to check for
      /// future double spends. The newly authorized promissory files are returned as a stream.
      /// </summary>
      /// <param name="requestStream">Used for reading requests from the client.</param>
      /// <param name="responseStream">Used for sending responses back to the client.</param>
      /// <param name="context">The context of the server-side call handler being invoked.</param>
      /// <returns>A task indicating completion of the handler.</returns>
      [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
      public virtual global::System.Threading.Tasks.Task Authorize(grpc::IAsyncStreamReader<global::IssuerApi.V1.AuthorizeRequest> requestStream, grpc::IServerStreamWriter<global::IssuerApi.V1.AuthorizeResponse> responseStream, grpc::ServerCallContext context)
      {
        throw new grpc::RpcException(new grpc::Status(grpc::StatusCode.Unimplemented, ""));
      }

      /// <summary>
      /// Issues digital banknotes against the corresponding issuance limit set by an Authority.  The Issuer must collect the
      /// Authenticate challenge signature, the Amount with a total amount, decimal place precision, and a currency code,
      /// along with the verifier of the Authority.
      /// </summary>
      /// <param name="request">The request received from the client.</param>
      /// <param name="responseStream">Used for sending responses back to the client.</param>
      /// <param name="context">The context of the server-side call handler being invoked.</param>
      /// <returns>A task indicating completion of the handler.</returns>
      [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
      public virtual global::System.Threading.Tasks.Task Issue(global::IssuerApi.V1.IssueRequest request, grpc::IServerStreamWriter<global::IssuerApi.V1.IssueResponse> responseStream, grpc::ServerCallContext context)
      {
        throw new grpc::RpcException(new grpc::Status(grpc::StatusCode.Unimplemented, ""));
      }

      /// <summary>
      /// Gets the roles configured for the Issuer- ex Currency limit for issuance.
      /// </summary>
      /// <param name="request">The request received from the client.</param>
      /// <param name="context">The context of the server-side call handler being invoked.</param>
      /// <returns>The response to send back to the client (wrapped by a task).</returns>
      [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
      public virtual global::System.Threading.Tasks.Task<global::IssuerApi.V1.GetRolesResponse> GetRoles(global::IssuerApi.V1.GetRolesRequest request, grpc::ServerCallContext context)
      {
        throw new grpc::RpcException(new grpc::Status(grpc::StatusCode.Unimplemented, ""));
      }

      /// <summary>
      /// Sets the roles configured for the Issuer- Ex List of the currency limits.
      /// </summary>
      /// <param name="request">The request received from the client.</param>
      /// <param name="context">The context of the server-side call handler being invoked.</param>
      /// <returns>The response to send back to the client (wrapped by a task).</returns>
      [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
      public virtual global::System.Threading.Tasks.Task<global::IssuerApi.V1.SetRoleResponse> SetRole(global::IssuerApi.V1.SetRoleRequest request, grpc::ServerCallContext context)
      {
        throw new grpc::RpcException(new grpc::Status(grpc::StatusCode.Unimplemented, ""));
      }

    }

    /// <summary>Client for IssuerService</summary>
    public partial class IssuerServiceClient : grpc::ClientBase<IssuerServiceClient>
    {
      /// <summary>Creates a new client for IssuerService</summary>
      /// <param name="channel">The channel to use to make remote calls.</param>
      [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
      public IssuerServiceClient(grpc::ChannelBase channel) : base(channel)
      {
      }
      /// <summary>Creates a new client for IssuerService that uses a custom <c>CallInvoker</c>.</summary>
      /// <param name="callInvoker">The callInvoker to use to make remote calls.</param>
      [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
      public IssuerServiceClient(grpc::CallInvoker callInvoker) : base(callInvoker)
      {
      }
      /// <summary>Protected parameterless constructor to allow creation of test doubles.</summary>
      [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
      protected IssuerServiceClient() : base()
      {
      }
      /// <summary>Protected constructor to allow creation of configured clients.</summary>
      /// <param name="configuration">The client configuration.</param>
      [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
      protected IssuerServiceClient(ClientBaseConfiguration configuration) : base(configuration)
      {
      }

      /// <summary>
      /// Checks that the Verifier is authenticated, then stores and returns the challenge nonce.
      /// </summary>
      /// <param name="request">The request to send to the server.</param>
      /// <param name="headers">The initial metadata to send with the call. This parameter is optional.</param>
      /// <param name="deadline">An optional deadline for the call. The call will be cancelled if deadline is hit.</param>
      /// <param name="cancellationToken">An optional token for canceling the call.</param>
      /// <returns>The response received from the server.</returns>
      [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
      public virtual global::IssuerApi.V1.AuthenticateResponse Authenticate(global::IssuerApi.V1.AuthenticateRequest request, grpc::Metadata headers = null, global::System.DateTime? deadline = null, global::System.Threading.CancellationToken cancellationToken = default(global::System.Threading.CancellationToken))
      {
        return Authenticate(request, new grpc::CallOptions(headers, deadline, cancellationToken));
      }
      /// <summary>
      /// Checks that the Verifier is authenticated, then stores and returns the challenge nonce.
      /// </summary>
      /// <param name="request">The request to send to the server.</param>
      /// <param name="options">The options for the call.</param>
      /// <returns>The response received from the server.</returns>
      [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
      public virtual global::IssuerApi.V1.AuthenticateResponse Authenticate(global::IssuerApi.V1.AuthenticateRequest request, grpc::CallOptions options)
      {
        return CallInvoker.BlockingUnaryCall(__Method_Authenticate, null, options, request);
      }
      /// <summary>
      /// Checks that the Verifier is authenticated, then stores and returns the challenge nonce.
      /// </summary>
      /// <param name="request">The request to send to the server.</param>
      /// <param name="headers">The initial metadata to send with the call. This parameter is optional.</param>
      /// <param name="deadline">An optional deadline for the call. The call will be cancelled if deadline is hit.</param>
      /// <param name="cancellationToken">An optional token for canceling the call.</param>
      /// <returns>The call object.</returns>
      [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
      public virtual grpc::AsyncUnaryCall<global::IssuerApi.V1.AuthenticateResponse> AuthenticateAsync(global::IssuerApi.V1.AuthenticateRequest request, grpc::Metadata headers = null, global::System.DateTime? deadline = null, global::System.Threading.CancellationToken cancellationToken = default(global::System.Threading.CancellationToken))
      {
        return AuthenticateAsync(request, new grpc::CallOptions(headers, deadline, cancellationToken));
      }
      /// <summary>
      /// Checks that the Verifier is authenticated, then stores and returns the challenge nonce.
      /// </summary>
      /// <param name="request">The request to send to the server.</param>
      /// <param name="options">The options for the call.</param>
      /// <returns>The call object.</returns>
      [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
      public virtual grpc::AsyncUnaryCall<global::IssuerApi.V1.AuthenticateResponse> AuthenticateAsync(global::IssuerApi.V1.AuthenticateRequest request, grpc::CallOptions options)
      {
        return CallInvoker.AsyncUnaryCall(__Method_Authenticate, null, options, request);
      }
      /// <summary>
      /// Sends a stream of digital banknotes to be authorized. The digital banknote is checked to see if it is transferred
      /// to the new owner, but not yet authorized. IT is then checked for double spends, signed and saved to check for
      /// future double spends. The newly authorized promissory files are returned as a stream.
      /// </summary>
      /// <param name="headers">The initial metadata to send with the call. This parameter is optional.</param>
      /// <param name="deadline">An optional deadline for the call. The call will be cancelled if deadline is hit.</param>
      /// <param name="cancellationToken">An optional token for canceling the call.</param>
      /// <returns>The call object.</returns>
      [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
      public virtual grpc::AsyncDuplexStreamingCall<global::IssuerApi.V1.AuthorizeRequest, global::IssuerApi.V1.AuthorizeResponse> Authorize(grpc::Metadata headers = null, global::System.DateTime? deadline = null, global::System.Threading.CancellationToken cancellationToken = default(global::System.Threading.CancellationToken))
      {
        return Authorize(new grpc::CallOptions(headers, deadline, cancellationToken));
      }
      /// <summary>
      /// Sends a stream of digital banknotes to be authorized. The digital banknote is checked to see if it is transferred
      /// to the new owner, but not yet authorized. IT is then checked for double spends, signed and saved to check for
      /// future double spends. The newly authorized promissory files are returned as a stream.
      /// </summary>
      /// <param name="options">The options for the call.</param>
      /// <returns>The call object.</returns>
      [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
      public virtual grpc::AsyncDuplexStreamingCall<global::IssuerApi.V1.AuthorizeRequest, global::IssuerApi.V1.AuthorizeResponse> Authorize(grpc::CallOptions options)
      {
        return CallInvoker.AsyncDuplexStreamingCall(__Method_Authorize, null, options);
      }
      /// <summary>
      /// Issues digital banknotes against the corresponding issuance limit set by an Authority.  The Issuer must collect the
      /// Authenticate challenge signature, the Amount with a total amount, decimal place precision, and a currency code,
      /// along with the verifier of the Authority.
      /// </summary>
      /// <param name="request">The request to send to the server.</param>
      /// <param name="headers">The initial metadata to send with the call. This parameter is optional.</param>
      /// <param name="deadline">An optional deadline for the call. The call will be cancelled if deadline is hit.</param>
      /// <param name="cancellationToken">An optional token for canceling the call.</param>
      /// <returns>The call object.</returns>
      [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
      public virtual grpc::AsyncServerStreamingCall<global::IssuerApi.V1.IssueResponse> Issue(global::IssuerApi.V1.IssueRequest request, grpc::Metadata headers = null, global::System.DateTime? deadline = null, global::System.Threading.CancellationToken cancellationToken = default(global::System.Threading.CancellationToken))
      {
        return Issue(request, new grpc::CallOptions(headers, deadline, cancellationToken));
      }
      /// <summary>
      /// Issues digital banknotes against the corresponding issuance limit set by an Authority.  The Issuer must collect the
      /// Authenticate challenge signature, the Amount with a total amount, decimal place precision, and a currency code,
      /// along with the verifier of the Authority.
      /// </summary>
      /// <param name="request">The request to send to the server.</param>
      /// <param name="options">The options for the call.</param>
      /// <returns>The call object.</returns>
      [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
      public virtual grpc::AsyncServerStreamingCall<global::IssuerApi.V1.IssueResponse> Issue(global::IssuerApi.V1.IssueRequest request, grpc::CallOptions options)
      {
        return CallInvoker.AsyncServerStreamingCall(__Method_Issue, null, options, request);
      }
      /// <summary>
      /// Gets the roles configured for the Issuer- ex Currency limit for issuance.
      /// </summary>
      /// <param name="request">The request to send to the server.</param>
      /// <param name="headers">The initial metadata to send with the call. This parameter is optional.</param>
      /// <param name="deadline">An optional deadline for the call. The call will be cancelled if deadline is hit.</param>
      /// <param name="cancellationToken">An optional token for canceling the call.</param>
      /// <returns>The response received from the server.</returns>
      [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
      public virtual global::IssuerApi.V1.GetRolesResponse GetRoles(global::IssuerApi.V1.GetRolesRequest request, grpc::Metadata headers = null, global::System.DateTime? deadline = null, global::System.Threading.CancellationToken cancellationToken = default(global::System.Threading.CancellationToken))
      {
        return GetRoles(request, new grpc::CallOptions(headers, deadline, cancellationToken));
      }
      /// <summary>
      /// Gets the roles configured for the Issuer- ex Currency limit for issuance.
      /// </summary>
      /// <param name="request">The request to send to the server.</param>
      /// <param name="options">The options for the call.</param>
      /// <returns>The response received from the server.</returns>
      [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
      public virtual global::IssuerApi.V1.GetRolesResponse GetRoles(global::IssuerApi.V1.GetRolesRequest request, grpc::CallOptions options)
      {
        return CallInvoker.BlockingUnaryCall(__Method_GetRoles, null, options, request);
      }
      /// <summary>
      /// Gets the roles configured for the Issuer- ex Currency limit for issuance.
      /// </summary>
      /// <param name="request">The request to send to the server.</param>
      /// <param name="headers">The initial metadata to send with the call. This parameter is optional.</param>
      /// <param name="deadline">An optional deadline for the call. The call will be cancelled if deadline is hit.</param>
      /// <param name="cancellationToken">An optional token for canceling the call.</param>
      /// <returns>The call object.</returns>
      [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
      public virtual grpc::AsyncUnaryCall<global::IssuerApi.V1.GetRolesResponse> GetRolesAsync(global::IssuerApi.V1.GetRolesRequest request, grpc::Metadata headers = null, global::System.DateTime? deadline = null, global::System.Threading.CancellationToken cancellationToken = default(global::System.Threading.CancellationToken))
      {
        return GetRolesAsync(request, new grpc::CallOptions(headers, deadline, cancellationToken));
      }
      /// <summary>
      /// Gets the roles configured for the Issuer- ex Currency limit for issuance.
      /// </summary>
      /// <param name="request">The request to send to the server.</param>
      /// <param name="options">The options for the call.</param>
      /// <returns>The call object.</returns>
      [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
      public virtual grpc::AsyncUnaryCall<global::IssuerApi.V1.GetRolesResponse> GetRolesAsync(global::IssuerApi.V1.GetRolesRequest request, grpc::CallOptions options)
      {
        return CallInvoker.AsyncUnaryCall(__Method_GetRoles, null, options, request);
      }
      /// <summary>
      /// Sets the roles configured for the Issuer- Ex List of the currency limits.
      /// </summary>
      /// <param name="request">The request to send to the server.</param>
      /// <param name="headers">The initial metadata to send with the call. This parameter is optional.</param>
      /// <param name="deadline">An optional deadline for the call. The call will be cancelled if deadline is hit.</param>
      /// <param name="cancellationToken">An optional token for canceling the call.</param>
      /// <returns>The response received from the server.</returns>
      [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
      public virtual global::IssuerApi.V1.SetRoleResponse SetRole(global::IssuerApi.V1.SetRoleRequest request, grpc::Metadata headers = null, global::System.DateTime? deadline = null, global::System.Threading.CancellationToken cancellationToken = default(global::System.Threading.CancellationToken))
      {
        return SetRole(request, new grpc::CallOptions(headers, deadline, cancellationToken));
      }
      /// <summary>
      /// Sets the roles configured for the Issuer- Ex List of the currency limits.
      /// </summary>
      /// <param name="request">The request to send to the server.</param>
      /// <param name="options">The options for the call.</param>
      /// <returns>The response received from the server.</returns>
      [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
      public virtual global::IssuerApi.V1.SetRoleResponse SetRole(global::IssuerApi.V1.SetRoleRequest request, grpc::CallOptions options)
      {
        return CallInvoker.BlockingUnaryCall(__Method_SetRole, null, options, request);
      }
      /// <summary>
      /// Sets the roles configured for the Issuer- Ex List of the currency limits.
      /// </summary>
      /// <param name="request">The request to send to the server.</param>
      /// <param name="headers">The initial metadata to send with the call. This parameter is optional.</param>
      /// <param name="deadline">An optional deadline for the call. The call will be cancelled if deadline is hit.</param>
      /// <param name="cancellationToken">An optional token for canceling the call.</param>
      /// <returns>The call object.</returns>
      [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
      public virtual grpc::AsyncUnaryCall<global::IssuerApi.V1.SetRoleResponse> SetRoleAsync(global::IssuerApi.V1.SetRoleRequest request, grpc::Metadata headers = null, global::System.DateTime? deadline = null, global::System.Threading.CancellationToken cancellationToken = default(global::System.Threading.CancellationToken))
      {
        return SetRoleAsync(request, new grpc::CallOptions(headers, deadline, cancellationToken));
      }
      /// <summary>
      /// Sets the roles configured for the Issuer- Ex List of the currency limits.
      /// </summary>
      /// <param name="request">The request to send to the server.</param>
      /// <param name="options">The options for the call.</param>
      /// <returns>The call object.</returns>
      [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
      public virtual grpc::AsyncUnaryCall<global::IssuerApi.V1.SetRoleResponse> SetRoleAsync(global::IssuerApi.V1.SetRoleRequest request, grpc::CallOptions options)
      {
        return CallInvoker.AsyncUnaryCall(__Method_SetRole, null, options, request);
      }
      /// <summary>Creates a new instance of client from given <c>ClientBaseConfiguration</c>.</summary>
      [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
      protected override IssuerServiceClient NewInstance(ClientBaseConfiguration configuration)
      {
        return new IssuerServiceClient(configuration);
      }
    }

    /// <summary>Creates service definition that can be registered with a server</summary>
    /// <param name="serviceImpl">An object implementing the server-side handling logic.</param>
    [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
    public static grpc::ServerServiceDefinition BindService(IssuerServiceBase serviceImpl)
    {
      return grpc::ServerServiceDefinition.CreateBuilder()
          .AddMethod(__Method_Authenticate, serviceImpl.Authenticate)
          .AddMethod(__Method_Authorize, serviceImpl.Authorize)
          .AddMethod(__Method_Issue, serviceImpl.Issue)
          .AddMethod(__Method_GetRoles, serviceImpl.GetRoles)
          .AddMethod(__Method_SetRole, serviceImpl.SetRole).Build();
    }

    /// <summary>Register service method with a service binder with or without implementation. Useful when customizing the service binding logic.
    /// Note: this method is part of an experimental API that can change or be removed without any prior notice.</summary>
    /// <param name="serviceBinder">Service methods will be bound by calling <c>AddMethod</c> on this object.</param>
    /// <param name="serviceImpl">An object implementing the server-side handling logic.</param>
    [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
    public static void BindService(grpc::ServiceBinderBase serviceBinder, IssuerServiceBase serviceImpl)
    {
      serviceBinder.AddMethod(__Method_Authenticate, serviceImpl == null ? null : new grpc::UnaryServerMethod<global::IssuerApi.V1.AuthenticateRequest, global::IssuerApi.V1.AuthenticateResponse>(serviceImpl.Authenticate));
      serviceBinder.AddMethod(__Method_Authorize, serviceImpl == null ? null : new grpc::DuplexStreamingServerMethod<global::IssuerApi.V1.AuthorizeRequest, global::IssuerApi.V1.AuthorizeResponse>(serviceImpl.Authorize));
      serviceBinder.AddMethod(__Method_Issue, serviceImpl == null ? null : new grpc::ServerStreamingServerMethod<global::IssuerApi.V1.IssueRequest, global::IssuerApi.V1.IssueResponse>(serviceImpl.Issue));
      serviceBinder.AddMethod(__Method_GetRoles, serviceImpl == null ? null : new grpc::UnaryServerMethod<global::IssuerApi.V1.GetRolesRequest, global::IssuerApi.V1.GetRolesResponse>(serviceImpl.GetRoles));
      serviceBinder.AddMethod(__Method_SetRole, serviceImpl == null ? null : new grpc::UnaryServerMethod<global::IssuerApi.V1.SetRoleRequest, global::IssuerApi.V1.SetRoleResponse>(serviceImpl.SetRole));
    }

  }
}
#endregion
