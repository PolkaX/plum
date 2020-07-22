// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

/// The Drand gRPC interface.
#[allow(dead_code)]
#[rustfmt::skip]
mod protos {
    pub mod drand {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Empty {
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Identity {
            #[prost(string, tag="1")]
            pub address: std::string::String,
            #[prost(bytes, tag="2")]
            pub key: std::vec::Vec<u8>,
            #[prost(bool, tag="3")]
            pub tls: bool,
        }
        /// Node holds the information related to a server in a group that forms a drand
        /// network
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Node {
            #[prost(message, optional, tag="1")]
            pub public: ::std::option::Option<Identity>,
            #[prost(uint32, tag="2")]
            pub index: u32,
        }
        /// GroupPacket represents a group that is running a drand network (or is in the
        /// process of creating one or performing a resharing).
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct GroupPacket {
            #[prost(message, repeated, tag="1")]
            pub nodes: ::std::vec::Vec<Node>,
            #[prost(uint32, tag="2")]
            pub threshold: u32,
            /// period in seconds
            #[prost(uint32, tag="3")]
            pub period: u32,
            #[prost(uint64, tag="4")]
            pub genesis_time: u64,
            #[prost(uint64, tag="5")]
            pub transition_time: u64,
            #[prost(bytes, tag="6")]
            pub genesis_seed: std::vec::Vec<u8>,
            #[prost(bytes, repeated, tag="7")]
            pub dist_key: ::std::vec::Vec<std::vec::Vec<u8>>,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct GroupRequest {
        }
        /// PublicRandRequest requests a public random value that has been generated in a
        /// unbiasable way and verifiable.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct PublicRandRequest {
            /// round uniquely identifies a beacon. If round == 0 (or unspecified), then
            /// the response will contain the last.
            #[prost(uint64, tag="1")]
            pub round: u64,
        }
        /// PublicRandResponse holds a signature which is the random value. It can be
        /// verified thanks to the distributed public key of the nodes that have ran the
        /// DKG protocol and is unbiasable. The randomness can be verified using the BLS
        /// verification routine with the message "round || previous_rand".
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct PublicRandResponse {
            #[prost(uint64, tag="1")]
            pub round: u64,
            #[prost(bytes, tag="2")]
            pub signature: std::vec::Vec<u8>,
            #[prost(bytes, tag="3")]
            pub previous_signature: std::vec::Vec<u8>,
            /// randomness is simply there to demonstrate - it is the hash of the
            /// signature. It should be computed locally.
            #[prost(bytes, tag="4")]
            pub randomness: std::vec::Vec<u8>,
        }
        /// PrivateRandRequest is the message to send when requesting a private random
        /// value.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct PrivateRandRequest {
            /// Request is the ECIES encryption of an ephemereal public key towards which
            /// to encrypt the private randomness. The format of the bytes is denoted by
            /// the ECIES encryption used by drand.
            #[prost(bytes, tag="1")]
            pub request: std::vec::Vec<u8>,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct PrivateRandResponse {
            /// Responses is the ECIES encryption of the private randomness using the
            /// ephemereal public key sent in the request.  The format of the bytes is
            /// denoted by the ECIES  encryption used by drand.
            #[prost(bytes, tag="1")]
            pub response: std::vec::Vec<u8>,
        }
        /// DistKeyRequest requests the distributed public key used during the randomness generation process
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct DistKeyRequest {
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct DistKeyResponse {
            #[prost(bytes, tag="2")]
            pub key: std::vec::Vec<u8>,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct HomeRequest {
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct HomeResponse {
            #[prost(string, tag="1")]
            pub status: std::string::String,
        }
        const METHOD_PUBLIC_PUBLIC_RAND: ::grpcio::Method<PublicRandRequest, PublicRandResponse> = ::grpcio::Method{ty: ::grpcio::MethodType::Unary, name: "/drand.Public/PublicRand", req_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, };
        const METHOD_PUBLIC_PUBLIC_RAND_STREAM: ::grpcio::Method<PublicRandRequest, PublicRandResponse> = ::grpcio::Method{ty: ::grpcio::MethodType::ServerStreaming, name: "/drand.Public/PublicRandStream", req_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, };
        const METHOD_PUBLIC_PRIVATE_RAND: ::grpcio::Method<PrivateRandRequest, PrivateRandResponse> = ::grpcio::Method{ty: ::grpcio::MethodType::Unary, name: "/drand.Public/PrivateRand", req_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, };
        const METHOD_PUBLIC_GROUP: ::grpcio::Method<GroupRequest, GroupPacket> = ::grpcio::Method{ty: ::grpcio::MethodType::Unary, name: "/drand.Public/Group", req_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, };
        const METHOD_PUBLIC_DIST_KEY: ::grpcio::Method<DistKeyRequest, DistKeyResponse> = ::grpcio::Method{ty: ::grpcio::MethodType::Unary, name: "/drand.Public/DistKey", req_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, };
        const METHOD_PUBLIC_HOME: ::grpcio::Method<HomeRequest, HomeResponse> = ::grpcio::Method{ty: ::grpcio::MethodType::Unary, name: "/drand.Public/Home", req_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, };
        #[derive(Clone)]
        pub struct PublicClient { client: ::grpcio::Client }
        impl PublicClient {
            pub fn new(channel: ::grpcio::Channel) -> Self { PublicClient { client: ::grpcio::Client::new(channel) }}
            pub fn public_rand_opt(&self, req: &PublicRandRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<PublicRandResponse,> { self.client.unary_call(&METHOD_PUBLIC_PUBLIC_RAND, req, opt) }
            pub fn public_rand(&self, req: &PublicRandRequest) -> ::grpcio::Result<PublicRandResponse,> { self.public_rand_opt(req, ::grpcio::CallOption::default()) }
            pub fn public_rand_async_opt(&self, req: &PublicRandRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<PublicRandResponse>,> { self.client.unary_call_async(&METHOD_PUBLIC_PUBLIC_RAND, req, opt) }
            pub fn public_rand_async(&self, req: &PublicRandRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<PublicRandResponse>,> { self.public_rand_async_opt(req, ::grpcio::CallOption::default()) }
            pub fn public_rand_stream_opt(&self, req: &PublicRandRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<PublicRandResponse>,> { self.client.server_streaming(&METHOD_PUBLIC_PUBLIC_RAND_STREAM, req, opt) }
            pub fn public_rand_stream(&self, req: &PublicRandRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<PublicRandResponse>,> { self.public_rand_stream_opt(req, ::grpcio::CallOption::default()) }
            pub fn private_rand_opt(&self, req: &PrivateRandRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<PrivateRandResponse,> { self.client.unary_call(&METHOD_PUBLIC_PRIVATE_RAND, req, opt) }
            pub fn private_rand(&self, req: &PrivateRandRequest) -> ::grpcio::Result<PrivateRandResponse,> { self.private_rand_opt(req, ::grpcio::CallOption::default()) }
            pub fn private_rand_async_opt(&self, req: &PrivateRandRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<PrivateRandResponse>,> { self.client.unary_call_async(&METHOD_PUBLIC_PRIVATE_RAND, req, opt) }
            pub fn private_rand_async(&self, req: &PrivateRandRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<PrivateRandResponse>,> { self.private_rand_async_opt(req, ::grpcio::CallOption::default()) }
            pub fn group_opt(&self, req: &GroupRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<GroupPacket,> { self.client.unary_call(&METHOD_PUBLIC_GROUP, req, opt) }
            pub fn group(&self, req: &GroupRequest) -> ::grpcio::Result<GroupPacket,> { self.group_opt(req, ::grpcio::CallOption::default()) }
            pub fn group_async_opt(&self, req: &GroupRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<GroupPacket>,> { self.client.unary_call_async(&METHOD_PUBLIC_GROUP, req, opt) }
            pub fn group_async(&self, req: &GroupRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<GroupPacket>,> { self.group_async_opt(req, ::grpcio::CallOption::default()) }
            pub fn dist_key_opt(&self, req: &DistKeyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<DistKeyResponse,> { self.client.unary_call(&METHOD_PUBLIC_DIST_KEY, req, opt) }
            pub fn dist_key(&self, req: &DistKeyRequest) -> ::grpcio::Result<DistKeyResponse,> { self.dist_key_opt(req, ::grpcio::CallOption::default()) }
            pub fn dist_key_async_opt(&self, req: &DistKeyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<DistKeyResponse>,> { self.client.unary_call_async(&METHOD_PUBLIC_DIST_KEY, req, opt) }
            pub fn dist_key_async(&self, req: &DistKeyRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<DistKeyResponse>,> { self.dist_key_async_opt(req, ::grpcio::CallOption::default()) }
            pub fn home_opt(&self, req: &HomeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<HomeResponse,> { self.client.unary_call(&METHOD_PUBLIC_HOME, req, opt) }
            pub fn home(&self, req: &HomeRequest) -> ::grpcio::Result<HomeResponse,> { self.home_opt(req, ::grpcio::CallOption::default()) }
            pub fn home_async_opt(&self, req: &HomeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<HomeResponse>,> { self.client.unary_call_async(&METHOD_PUBLIC_HOME, req, opt) }
            pub fn home_async(&self, req: &HomeRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<HomeResponse>,> { self.home_async_opt(req, ::grpcio::CallOption::default()) }
            pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Output = ()> + Send + 'static {self.client.spawn(f)}
        }
        pub trait Public {
            fn public_rand(&mut self, ctx: ::grpcio::RpcContext, req: PublicRandRequest, sink: ::grpcio::UnarySink<PublicRandResponse>);
            fn public_rand_stream(&mut self, ctx: ::grpcio::RpcContext, req: PublicRandRequest, sink: ::grpcio::ServerStreamingSink<PublicRandResponse>);
            fn private_rand(&mut self, ctx: ::grpcio::RpcContext, req: PrivateRandRequest, sink: ::grpcio::UnarySink<PrivateRandResponse>);
            fn group(&mut self, ctx: ::grpcio::RpcContext, req: GroupRequest, sink: ::grpcio::UnarySink<GroupPacket>);
            fn dist_key(&mut self, ctx: ::grpcio::RpcContext, req: DistKeyRequest, sink: ::grpcio::UnarySink<DistKeyResponse>);
            fn home(&mut self, ctx: ::grpcio::RpcContext, req: HomeRequest, sink: ::grpcio::UnarySink<HomeResponse>);
        }
        pub fn create_public<S: Public + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
            let mut builder = ::grpcio::ServiceBuilder::new();
            let mut instance = s.clone();
            builder = builder.add_unary_handler(&METHOD_PUBLIC_PUBLIC_RAND, move |ctx, req, resp| instance.public_rand(ctx, req, resp));
            let mut instance = s.clone();
            builder = builder.add_server_streaming_handler(&METHOD_PUBLIC_PUBLIC_RAND_STREAM, move |ctx, req, resp| instance.public_rand_stream(ctx, req, resp));
            let mut instance = s.clone();
            builder = builder.add_unary_handler(&METHOD_PUBLIC_PRIVATE_RAND, move |ctx, req, resp| instance.private_rand(ctx, req, resp));
            let mut instance = s.clone();
            builder = builder.add_unary_handler(&METHOD_PUBLIC_GROUP, move |ctx, req, resp| instance.group(ctx, req, resp));
            let mut instance = s.clone();
            builder = builder.add_unary_handler(&METHOD_PUBLIC_DIST_KEY, move |ctx, req, resp| instance.dist_key(ctx, req, resp));
            let mut instance = s;
            builder = builder.add_unary_handler(&METHOD_PUBLIC_HOME, move |ctx, req, resp| instance.home(ctx, req, resp));
            builder.build()
        }

        // Generated file, please don't edit manually.

        impl Empty {
            pub fn new_() -> Empty { ::std::default::Default::default() }
            #[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
        }
        impl ::protobuf::Clear for Empty {fn clear(&mut self) { ::prost::Message::clear(self); }
        }
        impl ::protobuf::Message for Empty {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
            fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
            fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
            fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
            fn new() -> Self { Self::default() }
            fn default_instance() -> &'static Empty {
                ::lazy_static::lazy_static! {
            static ref INSTANCE: Empty = Empty::default();
        }
                &*INSTANCE
            }
            fn is_initialized(&self) -> bool { true }
            fn write_to_with_cached_sizes(&self, _os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> { unimplemented!(); }
            fn merge_from(&mut self, _is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> { unimplemented!(); }
            fn get_unknown_fields(&self) -> &::protobuf::UnknownFields { unimplemented!(); }
            fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields { unimplemented!(); }
            fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
                let mut buf = Vec::new();
                if ::prost::Message::encode(self, &mut buf).is_err() {
                    return Err(::protobuf::ProtobufError::WireError(::protobuf::error::WireError::Other));
                }
                Ok(buf)
            }
            fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
                if ::prost::Message::merge(self, bytes).is_err() {
                    return Err(::protobuf::ProtobufError::WireError(::protobuf::error::WireError::Other));
                }
                Ok(())
            }
        }
        impl Identity {
            pub fn new_() -> Identity { ::std::default::Default::default() }
            #[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
            #[inline] pub fn clear_address(&mut self) { self.address.clear(); }
            #[inline] pub fn set_address(&mut self, v: std :: string :: String) { self.address = v; }
            #[inline] pub fn get_address(&self) -> &str { &self.address }
            #[inline] pub fn mut_address(&mut self) -> &mut std :: string :: String { &mut self.address }
            #[inline] pub fn take_address(&mut self) -> std :: string :: String { ::std::mem::replace(&mut self.address, ::std::string::String::new()) }
            #[inline] pub fn clear_key(&mut self) { self.key.clear(); }
            #[inline] pub fn set_key(&mut self, v: std :: vec :: Vec < u8 >) { self.key = v; }
            #[inline] pub fn get_key(&self) -> &[u8] { &self.key }
            #[inline] pub fn mut_key(&mut self) -> &mut std :: vec :: Vec < u8 > { &mut self.key }
            #[inline] pub fn take_key(&mut self) -> std :: vec :: Vec < u8 > { ::std::mem::replace(&mut self.key, ::std::vec::Vec::new()) }
            #[inline] pub fn clear_tls(&mut self) { self.tls = false }
            #[inline] pub fn set_tls(&mut self, v: bool) { self.tls = v; }
            #[inline] pub fn get_tls(&self) -> bool { self.tls }
        }
        impl ::protobuf::Clear for Identity {fn clear(&mut self) { ::prost::Message::clear(self); }
        }
        impl ::protobuf::Message for Identity {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
            fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
            fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
            fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
            fn new() -> Self { Self::default() }
            fn default_instance() -> &'static Identity {
                ::lazy_static::lazy_static! {
            static ref INSTANCE: Identity = Identity::default();
        }
                &*INSTANCE
            }
            fn is_initialized(&self) -> bool { true }
            fn write_to_with_cached_sizes(&self, _os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> { unimplemented!(); }
            fn merge_from(&mut self, _is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> { unimplemented!(); }
            fn get_unknown_fields(&self) -> &::protobuf::UnknownFields { unimplemented!(); }
            fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields { unimplemented!(); }
            fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
                let mut buf = Vec::new();
                if ::prost::Message::encode(self, &mut buf).is_err() {
                    return Err(::protobuf::ProtobufError::WireError(::protobuf::error::WireError::Other));
                }
                Ok(buf)
            }
            fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
                if ::prost::Message::merge(self, bytes).is_err() {
                    return Err(::protobuf::ProtobufError::WireError(::protobuf::error::WireError::Other));
                }
                Ok(())
            }
        }
        impl Node {
            pub fn new_() -> Node { ::std::default::Default::default() }
            #[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
            #[inline] pub fn has_public(&self) -> bool { self.public.is_some() }
            #[inline] pub fn clear_public(&mut self) { self.public = ::std::option::Option::None }
            #[inline] pub fn set_public(&mut self, v: Identity) { self.public = ::std::option::Option::Some(v); }
            #[inline] pub fn get_public(&self) -> &Identity { match self.public.as_ref() {
                Some(v) => v,
                None => Identity::default_ref(),
            } }
            #[inline] pub fn mut_public(&mut self) -> &mut Identity { if self.public.is_none() {
                self.public = ::std::option::Option::Some(Identity::default());
            }
                self.public.as_mut().unwrap() }
            #[inline] pub fn take_public(&mut self) -> Identity { self.public.take().unwrap_or_else(Identity::default) }
            #[inline] pub fn clear_index(&mut self) { self.index = 0 }
            #[inline] pub fn set_index(&mut self, v: u32) { self.index = v; }
            #[inline] pub fn get_index(&self) -> u32 { self.index }
        }
        impl ::protobuf::Clear for Node {fn clear(&mut self) { ::prost::Message::clear(self); }
        }
        impl ::protobuf::Message for Node {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
            fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
            fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
            fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
            fn new() -> Self { Self::default() }
            fn default_instance() -> &'static Node {
                ::lazy_static::lazy_static! {
            static ref INSTANCE: Node = Node::default();
        }
                &*INSTANCE
            }
            fn is_initialized(&self) -> bool { true }
            fn write_to_with_cached_sizes(&self, _os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> { unimplemented!(); }
            fn merge_from(&mut self, _is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> { unimplemented!(); }
            fn get_unknown_fields(&self) -> &::protobuf::UnknownFields { unimplemented!(); }
            fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields { unimplemented!(); }
            fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
                let mut buf = Vec::new();
                if ::prost::Message::encode(self, &mut buf).is_err() {
                    return Err(::protobuf::ProtobufError::WireError(::protobuf::error::WireError::Other));
                }
                Ok(buf)
            }
            fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
                if ::prost::Message::merge(self, bytes).is_err() {
                    return Err(::protobuf::ProtobufError::WireError(::protobuf::error::WireError::Other));
                }
                Ok(())
            }
        }
        impl GroupPacket {
            pub fn new_() -> GroupPacket { ::std::default::Default::default() }
            #[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
            #[inline] pub fn clear_nodes(&mut self) { self.nodes.clear(); }
            #[inline] pub fn set_nodes(&mut self, v: ::std::vec::Vec<Node>) { self.nodes = v; }
            #[inline] pub fn get_nodes(&self) -> &[Node] { &self.nodes }
            #[inline] pub fn mut_nodes(&mut self) -> &mut ::std::vec::Vec<Node> { &mut self.nodes }
            #[inline] pub fn take_nodes(&mut self) -> ::std::vec::Vec<Node> { ::std::mem::replace(&mut self.nodes, ::std::vec::Vec::new()) }
            #[inline] pub fn clear_threshold(&mut self) { self.threshold = 0 }
            #[inline] pub fn set_threshold(&mut self, v: u32) { self.threshold = v; }
            #[inline] pub fn get_threshold(&self) -> u32 { self.threshold }
            #[inline] pub fn clear_period(&mut self) { self.period = 0 }
            #[inline] pub fn set_period(&mut self, v: u32) { self.period = v; }
            #[inline] pub fn get_period(&self) -> u32 { self.period }
            #[inline] pub fn clear_genesis_time(&mut self) { self.genesis_time = 0 }
            #[inline] pub fn set_genesis_time(&mut self, v: u64) { self.genesis_time = v; }
            #[inline] pub fn get_genesis_time(&self) -> u64 { self.genesis_time }
            #[inline] pub fn clear_transition_time(&mut self) { self.transition_time = 0 }
            #[inline] pub fn set_transition_time(&mut self, v: u64) { self.transition_time = v; }
            #[inline] pub fn get_transition_time(&self) -> u64 { self.transition_time }
            #[inline] pub fn clear_genesis_seed(&mut self) { self.genesis_seed.clear(); }
            #[inline] pub fn set_genesis_seed(&mut self, v: std :: vec :: Vec < u8 >) { self.genesis_seed = v; }
            #[inline] pub fn get_genesis_seed(&self) -> &[u8] { &self.genesis_seed }
            #[inline] pub fn mut_genesis_seed(&mut self) -> &mut std :: vec :: Vec < u8 > { &mut self.genesis_seed }
            #[inline] pub fn take_genesis_seed(&mut self) -> std :: vec :: Vec < u8 > { ::std::mem::replace(&mut self.genesis_seed, ::std::vec::Vec::new()) }
            #[inline] pub fn clear_dist_key(&mut self) { self.dist_key.clear(); }
            #[inline] pub fn set_dist_key(&mut self, v: ::std::vec::Vec<std :: vec :: Vec < u8 >>) { self.dist_key = v; }
            #[inline] pub fn get_dist_key(&self) -> &[std :: vec :: Vec < u8 >] { &self.dist_key }
            #[inline] pub fn mut_dist_key(&mut self) -> &mut ::std::vec::Vec<std :: vec :: Vec < u8 >> { &mut self.dist_key }
            #[inline] pub fn take_dist_key(&mut self) -> ::std::vec::Vec<std :: vec :: Vec < u8 >> { ::std::mem::replace(&mut self.dist_key, ::std::vec::Vec::new()) }
        }
        impl ::protobuf::Clear for GroupPacket {fn clear(&mut self) { ::prost::Message::clear(self); }
        }
        impl ::protobuf::Message for GroupPacket {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
            fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
            fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
            fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
            fn new() -> Self { Self::default() }
            fn default_instance() -> &'static GroupPacket {
                ::lazy_static::lazy_static! {
            static ref INSTANCE: GroupPacket = GroupPacket::default();
        }
                &*INSTANCE
            }
            fn is_initialized(&self) -> bool { true }
            fn write_to_with_cached_sizes(&self, _os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> { unimplemented!(); }
            fn merge_from(&mut self, _is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> { unimplemented!(); }
            fn get_unknown_fields(&self) -> &::protobuf::UnknownFields { unimplemented!(); }
            fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields { unimplemented!(); }
            fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
                let mut buf = Vec::new();
                if ::prost::Message::encode(self, &mut buf).is_err() {
                    return Err(::protobuf::ProtobufError::WireError(::protobuf::error::WireError::Other));
                }
                Ok(buf)
            }
            fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
                if ::prost::Message::merge(self, bytes).is_err() {
                    return Err(::protobuf::ProtobufError::WireError(::protobuf::error::WireError::Other));
                }
                Ok(())
            }
        }
        impl GroupRequest {
            pub fn new_() -> GroupRequest { ::std::default::Default::default() }
            #[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
        }
        impl ::protobuf::Clear for GroupRequest {fn clear(&mut self) { ::prost::Message::clear(self); }
        }
        impl ::protobuf::Message for GroupRequest {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
            fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
            fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
            fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
            fn new() -> Self { Self::default() }
            fn default_instance() -> &'static GroupRequest {
                ::lazy_static::lazy_static! {
            static ref INSTANCE: GroupRequest = GroupRequest::default();
        }
                &*INSTANCE
            }
            fn is_initialized(&self) -> bool { true }
            fn write_to_with_cached_sizes(&self, _os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> { unimplemented!(); }
            fn merge_from(&mut self, _is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> { unimplemented!(); }
            fn get_unknown_fields(&self) -> &::protobuf::UnknownFields { unimplemented!(); }
            fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields { unimplemented!(); }
            fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
                let mut buf = Vec::new();
                if ::prost::Message::encode(self, &mut buf).is_err() {
                    return Err(::protobuf::ProtobufError::WireError(::protobuf::error::WireError::Other));
                }
                Ok(buf)
            }
            fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
                if ::prost::Message::merge(self, bytes).is_err() {
                    return Err(::protobuf::ProtobufError::WireError(::protobuf::error::WireError::Other));
                }
                Ok(())
            }
        }
        impl PublicRandRequest {
            pub fn new_() -> PublicRandRequest { ::std::default::Default::default() }
            #[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
            #[inline] pub fn clear_round(&mut self) { self.round = 0 }
            #[inline] pub fn set_round(&mut self, v: u64) { self.round = v; }
            #[inline] pub fn get_round(&self) -> u64 { self.round }
        }
        impl ::protobuf::Clear for PublicRandRequest {fn clear(&mut self) { ::prost::Message::clear(self); }
        }
        impl ::protobuf::Message for PublicRandRequest {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
            fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
            fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
            fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
            fn new() -> Self { Self::default() }
            fn default_instance() -> &'static PublicRandRequest {
                ::lazy_static::lazy_static! {
            static ref INSTANCE: PublicRandRequest = PublicRandRequest::default();
        }
                &*INSTANCE
            }
            fn is_initialized(&self) -> bool { true }
            fn write_to_with_cached_sizes(&self, _os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> { unimplemented!(); }
            fn merge_from(&mut self, _is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> { unimplemented!(); }
            fn get_unknown_fields(&self) -> &::protobuf::UnknownFields { unimplemented!(); }
            fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields { unimplemented!(); }
            fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
                let mut buf = Vec::new();
                if ::prost::Message::encode(self, &mut buf).is_err() {
                    return Err(::protobuf::ProtobufError::WireError(::protobuf::error::WireError::Other));
                }
                Ok(buf)
            }
            fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
                if ::prost::Message::merge(self, bytes).is_err() {
                    return Err(::protobuf::ProtobufError::WireError(::protobuf::error::WireError::Other));
                }
                Ok(())
            }
        }
        impl PublicRandResponse {
            pub fn new_() -> PublicRandResponse { ::std::default::Default::default() }
            #[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
            #[inline] pub fn clear_round(&mut self) { self.round = 0 }
            #[inline] pub fn set_round(&mut self, v: u64) { self.round = v; }
            #[inline] pub fn get_round(&self) -> u64 { self.round }
            #[inline] pub fn clear_signature(&mut self) { self.signature.clear(); }
            #[inline] pub fn set_signature(&mut self, v: std :: vec :: Vec < u8 >) { self.signature = v; }
            #[inline] pub fn get_signature(&self) -> &[u8] { &self.signature }
            #[inline] pub fn mut_signature(&mut self) -> &mut std :: vec :: Vec < u8 > { &mut self.signature }
            #[inline] pub fn take_signature(&mut self) -> std :: vec :: Vec < u8 > { ::std::mem::replace(&mut self.signature, ::std::vec::Vec::new()) }
            #[inline] pub fn clear_previous_signature(&mut self) { self.previous_signature.clear(); }
            #[inline] pub fn set_previous_signature(&mut self, v: std :: vec :: Vec < u8 >) { self.previous_signature = v; }
            #[inline] pub fn get_previous_signature(&self) -> &[u8] { &self.previous_signature }
            #[inline] pub fn mut_previous_signature(&mut self) -> &mut std :: vec :: Vec < u8 > { &mut self.previous_signature }
            #[inline] pub fn take_previous_signature(&mut self) -> std :: vec :: Vec < u8 > { ::std::mem::replace(&mut self.previous_signature, ::std::vec::Vec::new()) }
            #[inline] pub fn clear_randomness(&mut self) { self.randomness.clear(); }
            #[inline] pub fn set_randomness(&mut self, v: std :: vec :: Vec < u8 >) { self.randomness = v; }
            #[inline] pub fn get_randomness(&self) -> &[u8] { &self.randomness }
            #[inline] pub fn mut_randomness(&mut self) -> &mut std :: vec :: Vec < u8 > { &mut self.randomness }
            #[inline] pub fn take_randomness(&mut self) -> std :: vec :: Vec < u8 > { ::std::mem::replace(&mut self.randomness, ::std::vec::Vec::new()) }
        }
        impl ::protobuf::Clear for PublicRandResponse {fn clear(&mut self) { ::prost::Message::clear(self); }
        }
        impl ::protobuf::Message for PublicRandResponse {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
            fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
            fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
            fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
            fn new() -> Self { Self::default() }
            fn default_instance() -> &'static PublicRandResponse {
                ::lazy_static::lazy_static! {
            static ref INSTANCE: PublicRandResponse = PublicRandResponse::default();
        }
                &*INSTANCE
            }
            fn is_initialized(&self) -> bool { true }
            fn write_to_with_cached_sizes(&self, _os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> { unimplemented!(); }
            fn merge_from(&mut self, _is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> { unimplemented!(); }
            fn get_unknown_fields(&self) -> &::protobuf::UnknownFields { unimplemented!(); }
            fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields { unimplemented!(); }
            fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
                let mut buf = Vec::new();
                if ::prost::Message::encode(self, &mut buf).is_err() {
                    return Err(::protobuf::ProtobufError::WireError(::protobuf::error::WireError::Other));
                }
                Ok(buf)
            }
            fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
                if ::prost::Message::merge(self, bytes).is_err() {
                    return Err(::protobuf::ProtobufError::WireError(::protobuf::error::WireError::Other));
                }
                Ok(())
            }
        }
        impl PrivateRandRequest {
            pub fn new_() -> PrivateRandRequest { ::std::default::Default::default() }
            #[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
            #[inline] pub fn clear_request(&mut self) { self.request.clear(); }
            #[inline] pub fn set_request(&mut self, v: std :: vec :: Vec < u8 >) { self.request = v; }
            #[inline] pub fn get_request(&self) -> &[u8] { &self.request }
            #[inline] pub fn mut_request(&mut self) -> &mut std :: vec :: Vec < u8 > { &mut self.request }
            #[inline] pub fn take_request(&mut self) -> std :: vec :: Vec < u8 > { ::std::mem::replace(&mut self.request, ::std::vec::Vec::new()) }
        }
        impl ::protobuf::Clear for PrivateRandRequest {fn clear(&mut self) { ::prost::Message::clear(self); }
        }
        impl ::protobuf::Message for PrivateRandRequest {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
            fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
            fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
            fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
            fn new() -> Self { Self::default() }
            fn default_instance() -> &'static PrivateRandRequest {
                ::lazy_static::lazy_static! {
            static ref INSTANCE: PrivateRandRequest = PrivateRandRequest::default();
        }
                &*INSTANCE
            }
            fn is_initialized(&self) -> bool { true }
            fn write_to_with_cached_sizes(&self, _os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> { unimplemented!(); }
            fn merge_from(&mut self, _is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> { unimplemented!(); }
            fn get_unknown_fields(&self) -> &::protobuf::UnknownFields { unimplemented!(); }
            fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields { unimplemented!(); }
            fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
                let mut buf = Vec::new();
                if ::prost::Message::encode(self, &mut buf).is_err() {
                    return Err(::protobuf::ProtobufError::WireError(::protobuf::error::WireError::Other));
                }
                Ok(buf)
            }
            fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
                if ::prost::Message::merge(self, bytes).is_err() {
                    return Err(::protobuf::ProtobufError::WireError(::protobuf::error::WireError::Other));
                }
                Ok(())
            }
        }
        impl PrivateRandResponse {
            pub fn new_() -> PrivateRandResponse { ::std::default::Default::default() }
            #[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
            #[inline] pub fn clear_response(&mut self) { self.response.clear(); }
            #[inline] pub fn set_response(&mut self, v: std :: vec :: Vec < u8 >) { self.response = v; }
            #[inline] pub fn get_response(&self) -> &[u8] { &self.response }
            #[inline] pub fn mut_response(&mut self) -> &mut std :: vec :: Vec < u8 > { &mut self.response }
            #[inline] pub fn take_response(&mut self) -> std :: vec :: Vec < u8 > { ::std::mem::replace(&mut self.response, ::std::vec::Vec::new()) }
        }
        impl ::protobuf::Clear for PrivateRandResponse {fn clear(&mut self) { ::prost::Message::clear(self); }
        }
        impl ::protobuf::Message for PrivateRandResponse {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
            fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
            fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
            fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
            fn new() -> Self { Self::default() }
            fn default_instance() -> &'static PrivateRandResponse {
                ::lazy_static::lazy_static! {
            static ref INSTANCE: PrivateRandResponse = PrivateRandResponse::default();
        }
                &*INSTANCE
            }
            fn is_initialized(&self) -> bool { true }
            fn write_to_with_cached_sizes(&self, _os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> { unimplemented!(); }
            fn merge_from(&mut self, _is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> { unimplemented!(); }
            fn get_unknown_fields(&self) -> &::protobuf::UnknownFields { unimplemented!(); }
            fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields { unimplemented!(); }
            fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
                let mut buf = Vec::new();
                if ::prost::Message::encode(self, &mut buf).is_err() {
                    return Err(::protobuf::ProtobufError::WireError(::protobuf::error::WireError::Other));
                }
                Ok(buf)
            }
            fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
                if ::prost::Message::merge(self, bytes).is_err() {
                    return Err(::protobuf::ProtobufError::WireError(::protobuf::error::WireError::Other));
                }
                Ok(())
            }
        }
        impl DistKeyRequest {
            pub fn new_() -> DistKeyRequest { ::std::default::Default::default() }
            #[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
        }
        impl ::protobuf::Clear for DistKeyRequest {fn clear(&mut self) { ::prost::Message::clear(self); }
        }
        impl ::protobuf::Message for DistKeyRequest {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
            fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
            fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
            fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
            fn new() -> Self { Self::default() }
            fn default_instance() -> &'static DistKeyRequest {
                ::lazy_static::lazy_static! {
            static ref INSTANCE: DistKeyRequest = DistKeyRequest::default();
        }
                &*INSTANCE
            }
            fn is_initialized(&self) -> bool { true }
            fn write_to_with_cached_sizes(&self, _os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> { unimplemented!(); }
            fn merge_from(&mut self, _is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> { unimplemented!(); }
            fn get_unknown_fields(&self) -> &::protobuf::UnknownFields { unimplemented!(); }
            fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields { unimplemented!(); }
            fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
                let mut buf = Vec::new();
                if ::prost::Message::encode(self, &mut buf).is_err() {
                    return Err(::protobuf::ProtobufError::WireError(::protobuf::error::WireError::Other));
                }
                Ok(buf)
            }
            fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
                if ::prost::Message::merge(self, bytes).is_err() {
                    return Err(::protobuf::ProtobufError::WireError(::protobuf::error::WireError::Other));
                }
                Ok(())
            }
        }
        impl DistKeyResponse {
            pub fn new_() -> DistKeyResponse { ::std::default::Default::default() }
            #[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
            #[inline] pub fn clear_key(&mut self) { self.key.clear(); }
            #[inline] pub fn set_key(&mut self, v: std :: vec :: Vec < u8 >) { self.key = v; }
            #[inline] pub fn get_key(&self) -> &[u8] { &self.key }
            #[inline] pub fn mut_key(&mut self) -> &mut std :: vec :: Vec < u8 > { &mut self.key }
            #[inline] pub fn take_key(&mut self) -> std :: vec :: Vec < u8 > { ::std::mem::replace(&mut self.key, ::std::vec::Vec::new()) }
        }
        impl ::protobuf::Clear for DistKeyResponse {fn clear(&mut self) { ::prost::Message::clear(self); }
        }
        impl ::protobuf::Message for DistKeyResponse {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
            fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
            fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
            fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
            fn new() -> Self { Self::default() }
            fn default_instance() -> &'static DistKeyResponse {
                ::lazy_static::lazy_static! {
            static ref INSTANCE: DistKeyResponse = DistKeyResponse::default();
        }
                &*INSTANCE
            }
            fn is_initialized(&self) -> bool { true }
            fn write_to_with_cached_sizes(&self, _os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> { unimplemented!(); }
            fn merge_from(&mut self, _is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> { unimplemented!(); }
            fn get_unknown_fields(&self) -> &::protobuf::UnknownFields { unimplemented!(); }
            fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields { unimplemented!(); }
            fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
                let mut buf = Vec::new();
                if ::prost::Message::encode(self, &mut buf).is_err() {
                    return Err(::protobuf::ProtobufError::WireError(::protobuf::error::WireError::Other));
                }
                Ok(buf)
            }
            fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
                if ::prost::Message::merge(self, bytes).is_err() {
                    return Err(::protobuf::ProtobufError::WireError(::protobuf::error::WireError::Other));
                }
                Ok(())
            }
        }
        impl HomeRequest {
            pub fn new_() -> HomeRequest { ::std::default::Default::default() }
            #[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
        }
        impl ::protobuf::Clear for HomeRequest {fn clear(&mut self) { ::prost::Message::clear(self); }
        }
        impl ::protobuf::Message for HomeRequest {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
            fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
            fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
            fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
            fn new() -> Self { Self::default() }
            fn default_instance() -> &'static HomeRequest {
                ::lazy_static::lazy_static! {
            static ref INSTANCE: HomeRequest = HomeRequest::default();
        }
                &*INSTANCE
            }
            fn is_initialized(&self) -> bool { true }
            fn write_to_with_cached_sizes(&self, _os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> { unimplemented!(); }
            fn merge_from(&mut self, _is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> { unimplemented!(); }
            fn get_unknown_fields(&self) -> &::protobuf::UnknownFields { unimplemented!(); }
            fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields { unimplemented!(); }
            fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
                let mut buf = Vec::new();
                if ::prost::Message::encode(self, &mut buf).is_err() {
                    return Err(::protobuf::ProtobufError::WireError(::protobuf::error::WireError::Other));
                }
                Ok(buf)
            }
            fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
                if ::prost::Message::merge(self, bytes).is_err() {
                    return Err(::protobuf::ProtobufError::WireError(::protobuf::error::WireError::Other));
                }
                Ok(())
            }
        }
        impl HomeResponse {
            pub fn new_() -> HomeResponse { ::std::default::Default::default() }
            #[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
            #[inline] pub fn clear_status(&mut self) { self.status.clear(); }
            #[inline] pub fn set_status(&mut self, v: std :: string :: String) { self.status = v; }
            #[inline] pub fn get_status(&self) -> &str { &self.status }
            #[inline] pub fn mut_status(&mut self) -> &mut std :: string :: String { &mut self.status }
            #[inline] pub fn take_status(&mut self) -> std :: string :: String { ::std::mem::replace(&mut self.status, ::std::string::String::new()) }
        }
        impl ::protobuf::Clear for HomeResponse {fn clear(&mut self) { ::prost::Message::clear(self); }
        }
        impl ::protobuf::Message for HomeResponse {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
            fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
            fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
            fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
            fn new() -> Self { Self::default() }
            fn default_instance() -> &'static HomeResponse {
                ::lazy_static::lazy_static! {
            static ref INSTANCE: HomeResponse = HomeResponse::default();
        }
                &*INSTANCE
            }
            fn is_initialized(&self) -> bool { true }
            fn write_to_with_cached_sizes(&self, _os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> { unimplemented!(); }
            fn merge_from(&mut self, _is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> { unimplemented!(); }
            fn get_unknown_fields(&self) -> &::protobuf::UnknownFields { unimplemented!(); }
            fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields { unimplemented!(); }
            fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
                let mut buf = Vec::new();
                if ::prost::Message::encode(self, &mut buf).is_err() {
                    return Err(::protobuf::ProtobufError::WireError(::protobuf::error::WireError::Other));
                }
                Ok(buf)
            }
            fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
                if ::prost::Message::merge(self, bytes).is_err() {
                    return Err(::protobuf::ProtobufError::WireError(::protobuf::error::WireError::Other));
                }
                Ok(())
            }
        }
    }

    // include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));
}

pub use self::protos::*;
