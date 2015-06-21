initSidebarItems({"trait":[["CallBuilder","Identifies types which represent builders for a particular resource method"],["Delegate","A trait specifying functionality to help controlling any request performed by the API. The trait has a conservative default implementation."],["Hub","Identifies the Hub. There is only one per library, this trait is supposed to make intended use more explicit. The hub allows to access all resource methods more easily."],["MethodsBuilder","Identifies types for building methods of a particular resource type"],["NestedType","Identifies types which are only used by other types internally. They have no special meaning, this trait just marks them for completeness."],["Part","Identifies types which are only used as part of other types, which usually are carrying the `Resource` trait."],["ReadSeek","A utility to specify reader types which provide seeking capabilities too"],["RequestValue","Identifies types which are used in API requests."],["Resource","Identifies types which can be inserted and deleted. Types with this trait are most commonly used by clients of this API."],["ResponseResult","Identifies types which are used in API responses."],["ToParts","A trait for all types that can convert themselves into a *parts* string"]],"fn":[["remove_json_null_values",""]],"struct":[["CreateAuthUriResponse","Response of creating the IDP authentication URL."],["DefaultDelegate","A delegate with a conservative default implementation, which is used if no other delegate is set."],["DeleteAccountResponse","Respone of deleting account."],["DownloadAccountResponse","Respone of downloading accounts in batch."],["ErrorResponse","A utility to represent detailed errors we might see in case there are BadRequests. The latter happen if the sent parameters or request structures are unsound"],["GetAccountInfoResponse","Response of getting account information."],["GetOobConfirmationCodeResponse","Response of getting a code for user confirmation (reset password, change email etc.)."],["GetRecaptchaParamResponse","Response of getting recaptcha param."],["IdentityToolkit","Central instance to access all IdentityToolkit related resource activities"],["IdentitytoolkitRelyingpartyCreateAuthUriRequest","Request to get the IDP authentication URL."],["IdentitytoolkitRelyingpartyDeleteAccountRequest","Request to delete account."],["IdentitytoolkitRelyingpartyDownloadAccountRequest","Request to download user account in batch."],["IdentitytoolkitRelyingpartyGetAccountInfoRequest","Request to get the account information."],["IdentitytoolkitRelyingpartyGetPublicKeysResponse","Respone of getting public keys."],["IdentitytoolkitRelyingpartyResetPasswordRequest","Request to reset the password."],["IdentitytoolkitRelyingpartySetAccountInfoRequest","Request to set the account information."],["IdentitytoolkitRelyingpartyUploadAccountRequest","Request to upload user account in batch."],["IdentitytoolkitRelyingpartyVerifyAssertionRequest","Request to verify the IDP assertion."],["IdentitytoolkitRelyingpartyVerifyPasswordRequest","Request to verify the password."],["MethodInfo","Contains information about an API request."],["MultiPartReader","Provides a `Read` interface that converts multiple parts into the protocol identified by RFC2387. **Note**: This implementation is just as rich as it needs to be to perform uploads to google APIs, and might not be a fully-featured implementation."],["Relyingparty","Request of getting a code for user confirmation (reset password, change email etc.)"],["RelyingpartyCreateAuthUriCall","Creates the URI used by the IdP to authenticate the user."],["RelyingpartyDeleteAccountCall","Delete user account."],["RelyingpartyDownloadAccountCall","Batch download user accounts."],["RelyingpartyGetAccountInfoCall","Returns the account info."],["RelyingpartyGetOobConfirmationCodeCall","Get a code for user action confirmation."],["RelyingpartyGetPublicKeyCall","Get token signing public key."],["RelyingpartyGetRecaptchaParamCall","Get recaptcha secure param."],["RelyingpartyMethods","A builder providing access to all methods supported on *relyingparty* resources. It is not used directly, but through the `IdentityToolkit` hub."],["RelyingpartyResetPasswordCall","Reset password for a user."],["RelyingpartySetAccountInfoCall","Set account info for a user."],["RelyingpartyUploadAccountCall","Batch upload existing user accounts."],["RelyingpartyVerifyAssertionCall","Verifies the assertion returned by the IdP."],["RelyingpartyVerifyPasswordCall","Verifies the user entered password."],["ResetPasswordResponse","Response of resetting the password."],["SetAccountInfoResponse","Respone of setting the account information."],["SetAccountInfoResponseProviderUserInfo","The user's profiles at the associated IdPs."],["UploadAccountResponse","Respone of uploading accounts in batch."],["UploadAccountResponseError","The error encountered while processing the account info."],["UserInfo","Template for an individual account info."],["UserInfoProviderUserInfo","The IDP of the user."],["VerifyAssertionResponse","Response of verifying the IDP assertion."],["VerifyPasswordResponse","Request of verifying the password."]],"enum":[["Error",""]],"type":[["Result","A universal result type used as return for all calls."]]});