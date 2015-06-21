initSidebarItems({"type":[["Result","A universal result type used as return for all calls."]],"trait":[["CallBuilder","Identifies types which represent builders for a particular resource method"],["Delegate","A trait specifying functionality to help controlling any request performed by the API. The trait has a conservative default implementation."],["Hub","Identifies the Hub. There is only one per library, this trait is supposed to make intended use more explicit. The hub allows to access all resource methods more easily."],["MethodsBuilder","Identifies types for building methods of a particular resource type"],["NestedType","Identifies types which are only used by other types internally. They have no special meaning, this trait just marks them for completeness."],["Part","Identifies types which are only used as part of other types, which usually are carrying the `Resource` trait."],["ReadSeek","A utility to specify reader types which provide seeking capabilities too"],["RequestValue","Identifies types which are used in API requests."],["Resource","Identifies types which can be inserted and deleted. Types with this trait are most commonly used by clients of this API."],["ResponseResult","Identifies types which are used in API responses."],["ToParts","A trait for all types that can convert themselves into a *parts* string"]],"enum":[["Error",""],["Scope","Identifies the an OAuth2 authorization scope. A scope is needed when requesting an authorization token."]],"fn":[["remove_json_null_values",""]],"struct":[["AggregatedStats","There is no detailed description."],["AggregatedStatsReply","There is no detailed description."],["Cloudlatencytest","Central instance to access all Cloudlatencytest related resource activities"],["DefaultDelegate","A delegate with a conservative default implementation, which is used if no other delegate is set."],["DoubleValue","There is no detailed description."],["ErrorResponse","A utility to represent detailed errors we might see in case there are BadRequests. The latter happen if the sent parameters or request structures are unsound"],["IntValue","There is no detailed description."],["MethodInfo","Contains information about an API request."],["MultiPartReader","Provides a `Read` interface that converts multiple parts into the protocol identified by RFC2387. **Note**: This implementation is just as rich as it needs to be to perform uploads to google APIs, and might not be a fully-featured implementation."],["Stats","There is no detailed description."],["StatsReply","There is no detailed description."],["StatscollectionMethods","A builder providing access to all methods supported on *statscollection* resources. It is not used directly, but through the `Cloudlatencytest` hub."],["StatscollectionUpdateaggregatedstatCall","RPC to update the new TCP stats."],["StatscollectionUpdatestatCall","RPC to update the new TCP stats."],["StringValue","There is no detailed description."]]});