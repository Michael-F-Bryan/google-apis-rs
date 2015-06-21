initSidebarItems({"fn":[["remove_json_null_values",""]],"trait":[["CallBuilder","Identifies types which represent builders for a particular resource method"],["Delegate","A trait specifying functionality to help controlling any request performed by the API. The trait has a conservative default implementation."],["Hub","Identifies the Hub. There is only one per library, this trait is supposed to make intended use more explicit. The hub allows to access all resource methods more easily."],["MethodsBuilder","Identifies types for building methods of a particular resource type"],["NestedType","Identifies types which are only used by other types internally. They have no special meaning, this trait just marks them for completeness."],["Part","Identifies types which are only used as part of other types, which usually are carrying the `Resource` trait."],["ReadSeek","A utility to specify reader types which provide seeking capabilities too"],["RequestValue","Identifies types which are used in API requests."],["Resource","Identifies types which can be inserted and deleted. Types with this trait are most commonly used by clients of this API."],["ResponseResult","Identifies types which are used in API responses."],["ToParts","A trait for all types that can convert themselves into a *parts* string"]],"struct":[["Account","There is no detailed description."],["AccountAdclientListCall","List all ad clients in this Ad Exchange account."],["AccountAlertListCall","List the alerts for this Ad Exchange account."],["AccountCustomchannelGetCall","Get the specified custom channel from the specified ad client."],["AccountCustomchannelListCall","List all custom channels in the specified ad client for this Ad Exchange account."],["AccountGetCall","Get information about the selected Ad Exchange account."],["AccountListCall","List all accounts available to this Ad Exchange account."],["AccountMetadataDimensionListCall","List the metadata for the dimensions available to this AdExchange account."],["AccountMetadataMetricListCall","List the metadata for the metrics available to this AdExchange account."],["AccountMethods","A builder providing access to all methods supported on *account* resources. It is not used directly, but through the `AdExchangeSeller` hub."],["AccountPreferreddealGetCall","Get information about the selected Ad Exchange Preferred Deal."],["AccountPreferreddealListCall","List the preferred deals for this Ad Exchange account."],["AccountReportGenerateCall","Generate an Ad Exchange report based on the report request sent in the query parameters. Returns the result as JSON; to retrieve output in CSV format specify \"alt=csv\" as a query parameter."],["AccountReportSavedGenerateCall","Generate an Ad Exchange report based on the saved report ID sent in the query parameters."],["AccountReportSavedListCall","List all saved reports in this Ad Exchange account."],["AccountUrlchannelListCall","List all URL channels in the specified ad client for this Ad Exchange account."],["Accounts","There is no detailed description."],["AdClient","There is no detailed description."],["AdClients","There is no detailed description."],["AdExchangeSeller","Central instance to access all AdExchangeSeller related resource activities"],["Alert","There is no detailed description."],["Alerts","There is no detailed description."],["CustomChannel","There is no detailed description."],["CustomChannelTargetingInfo","The targeting information of this custom channel, if activated."],["CustomChannels","There is no detailed description."],["DefaultDelegate","A delegate with a conservative default implementation, which is used if no other delegate is set."],["ErrorResponse","A utility to represent detailed errors we might see in case there are BadRequests. The latter happen if the sent parameters or request structures are unsound"],["Metadata","There is no detailed description."],["MethodInfo","Contains information about an API request."],["MultiPartReader","Provides a `Read` interface that converts multiple parts into the protocol identified by RFC2387. **Note**: This implementation is just as rich as it needs to be to perform uploads to google APIs, and might not be a fully-featured implementation."],["PreferredDeal","There is no detailed description."],["PreferredDeals","There is no detailed description."],["Report","There is no detailed description."],["ReportHeaders","The header information of the columns requested in the report. This is a list of headers; one for each dimension in the request, followed by one for each metric in the request."],["ReportingMetadataEntry","There is no detailed description."],["SavedReport","There is no detailed description."],["SavedReports","There is no detailed description."],["UrlChannel","There is no detailed description."],["UrlChannels","There is no detailed description."]],"enum":[["Error",""],["Scope","Identifies the an OAuth2 authorization scope. A scope is needed when requesting an authorization token."]],"type":[["Result","A universal result type used as return for all calls."]]});