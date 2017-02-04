// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *Play Movies* crate version *1.0.4+20160518*, where *20160518* is the exact revision of the *playmoviespartner:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.4*.
//! 
//! Everything else about the *Play Movies* *v1* API can be found at the
//! [official documentation site](https://developers.google.com/playmoviespartner/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/playmoviespartner1).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.PlayMovies.html) ... 
//! 
//! * accounts
//!  * [*avails get*](struct.AccountAvailGetCall.html), [*avails list*](struct.AccountAvailListCall.html), [*components list*](struct.AccountComponentListCall.html), [*components type get*](struct.AccountComponentTypeGetCall.html), [*experience locales get*](struct.AccountExperienceLocaleGetCall.html), [*experience locales list*](struct.AccountExperienceLocaleListCall.html), [*orders get*](struct.AccountOrderGetCall.html), [*orders list*](struct.AccountOrderListCall.html), [*store infos country get*](struct.AccountStoreInfoCountryGetCall.html) and [*store infos list*](struct.AccountStoreInfoListCall.html)
//! 
//! 
//! 
//! 
//! Not what you are looking for ? Find all other Google APIs in their Rust [documentation index](http://byron.github.io/google-apis-rs).
//! 
//! # Structure of this Library
//! 
//! The API is structured into the following primary items:
//! 
//! * **[Hub](struct.PlayMovies.html)**
//!     * a central object to maintain state and allow accessing all *Activities*
//!     * creates [*Method Builders*](trait.MethodsBuilder.html) which in turn
//!       allow access to individual [*Call Builders*](trait.CallBuilder.html)
//! * **[Resources](trait.Resource.html)**
//!     * primary types that you can apply *Activities* to
//!     * a collection of properties and *Parts*
//!     * **[Parts](trait.Part.html)**
//!         * a collection of properties
//!         * never directly used in *Activities*
//! * **[Activities](trait.CallBuilder.html)**
//!     * operations to apply to *Resources*
//! 
//! All *structures* are marked with applicable traits to further categorize them and ease browsing.
//! 
//! Generally speaking, you can invoke *Activities* like this:
//! 
//! ```Rust,ignore
//! let r = hub.resource().activity(...).doit()
//! ```
//! 
//! Or specifically ...
//! 
//! ```ignore
//! let r = hub.accounts().avails_get(...).doit()
//! ```
//! 
//! The `resource()` and `activity(...)` calls create [builders][builder-pattern]. The second one dealing with `Activities` 
//! supports various methods to configure the impending operation (not shown here). It is made such that all required arguments have to be 
//! specified right away (i.e. `(...)`), whereas all optional ones can be [build up][builder-pattern] as desired.
//! The `doit()` method performs the actual communication with the server and returns the respective result.
//! 
//! # Usage
//! 
//! ## Setting up your Project
//! 
//! To use this library, you would put the following lines into your `Cargo.toml` file:
//! 
//! ```toml
//! [dependencies]
//! google-playmoviespartner1 = "*"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate yup_oauth2 as oauth2;
//! extern crate google_playmoviespartner1 as playmoviespartner1;
//! use playmoviespartner1::{Result, Error};
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use playmoviespartner1::PlayMovies;
//! 
//! // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
//! // `client_secret`, among other things.
//! let secret: ApplicationSecret = Default::default();
//! // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
//! // unless you replace  `None` with the desired Flow.
//! // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
//! // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
//! // retrieve them from storage.
//! let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
//!                               hyper::Client::new(),
//!                               <MemoryStorage as Default>::default(), None);
//! let mut hub = PlayMovies::new(hyper::Client::new(), auth);
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.accounts().avails_get("accountId", "availId")
//!              .doit();
//! 
//! match result {
//!     Err(e) => match e {
//!         // The Error enum provides details about what exactly happened.
//!         // You can also just use its `Debug`, `Display` or `Error` traits
//!          Error::HttpError(_)
//!         |Error::MissingAPIKey
//!         |Error::MissingToken(_)
//!         |Error::Cancelled
//!         |Error::UploadSizeLimitExceeded(_, _)
//!         |Error::Failure(_)
//!         |Error::BadRequest(_)
//!         |Error::FieldClash(_)
//!         |Error::JsonDecodeError(_, _) => println!("{}", e),
//!     },
//!     Ok(res) => println!("Success: {:?}", res),
//! }
//! # }
//! ```
//! ## Handling Errors
//! 
//! All errors produced by the system are provided either as [Result](enum.Result.html) enumeration as return value of 
//! the doit() methods, or handed as possibly intermediate results to either the 
//! [Hub Delegate](trait.Delegate.html), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).
//! 
//! When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
//! makes the system potentially resilient to all kinds of errors.
//! 
//! ## Uploads and Downloads
//! If a method supports downloads, the response body, which is part of the [Result](enum.Result.html), should be
//! read by you to obtain the media.
//! If such a method also supports a [Response Result](trait.ResponseResult.html), it will return that by default.
//! You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
//! this call: `.param("alt", "media")`.
//! 
//! Methods supporting uploads can do so using up to 2 different protocols: 
//! *simple* and *resumable*. The distinctiveness of each is represented by customized 
//! `doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.
//! 
//! ## Customization and Callbacks
//! 
//! You may alter the way an `doit()` method is called by providing a [delegate](trait.Delegate.html) to the 
//! [Method Builder](trait.CallBuilder.html) before making the final `doit()` call. 
//! Respective methods will be called to provide progress information, as well as determine whether the system should 
//! retry on failure.
//! 
//! The [delegate trait](trait.Delegate.html) is default-implemented, allowing you to customize it with minimal effort.
//! 
//! ## Optional Parts in Server-Requests
//! 
//! All structures provided by this library are made to be [enocodable](trait.RequestValue.html) and 
//! [decodable](trait.ResponseResult.html) via *json*. Optionals are used to indicate that partial requests are responses 
//! are valid.
//! Most optionals are are considered [Parts](trait.Part.html) which are identifiable by name, which will be sent to 
//! the server to indicate either the set parts of the request or the desired parts in the response.
//! 
//! ## Builder Arguments
//! 
//! Using [method builders](trait.CallBuilder.html), you are able to prepare an action call by repeatedly calling it's methods.
//! These will always take a single argument, for which the following statements are true.
//! 
//! * [PODs][wiki-pod] are handed by copy
//! * strings are passed as `&str`
//! * [request values](trait.RequestValue.html) are moved
//! 
//! Arguments will always be copied or cloned into the builder, to make them independent of their original life times.
//! 
//! [wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
//! [builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
//! [google-go-api]: https://github.com/google/google-api-go-client
//! 
//! 

// Unused attributes happen thanks to defined, but unused structures
// We don't warn about this, as depending on the API, some data structures or facilities are never used.
// Instead of pre-determining this, we just disable the lint. It's manually tuned to not have any 
// unused imports in fully featured APIs. Same with unused_mut ... .
#![allow(unused_imports, unused_mut, dead_code)]

// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

#[macro_use]
extern crate serde_derive;

extern crate hyper;
extern crate serde;
extern crate serde_json;
extern crate yup_oauth2 as oauth2;
extern crate mime;
extern crate url;

mod cmn;

use std::collections::HashMap;
use std::cell::RefCell;
use std::borrow::BorrowMut;
use std::default::Default;
use std::collections::BTreeMap;
use serde_json as json;
use std::io;
use std::fs;
use std::thread::sleep;
use std::time::Duration;

pub use cmn::{MultiPartReader, ToParts, MethodInfo, Result, Error, CallBuilder, Hub, ReadSeek, Part,
              ResponseResult, RequestValue, NestedType, Delegate, DefaultDelegate, MethodsBuilder,
              Resource, ErrorResponse, remove_json_null_values};


// ##############
// UTILITIES ###
// ############

/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash)]
pub enum Scope {
    /// View the digital assets you publish on Google Play Movies and TV
    PlaymovyPartnerReadonly,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::PlaymovyPartnerReadonly => "https://www.googleapis.com/auth/playmovies_partner.readonly",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::PlaymovyPartnerReadonly
    }
}



// ########
// HUB ###
// ######

/// Central instance to access all PlayMovies related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_playmoviespartner1 as playmoviespartner1;
/// use playmoviespartner1::{Result, Error};
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use playmoviespartner1::PlayMovies;
/// 
/// // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
/// // `client_secret`, among other things.
/// let secret: ApplicationSecret = Default::default();
/// // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
/// // unless you replace  `None` with the desired Flow.
/// // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
/// // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
/// // retrieve them from storage.
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = PlayMovies::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().avails_get("accountId", "availId")
///              .doit();
/// 
/// match result {
///     Err(e) => match e {
///         // The Error enum provides details about what exactly happened.
///         // You can also just use its `Debug`, `Display` or `Error` traits
///          Error::HttpError(_)
///         |Error::MissingAPIKey
///         |Error::MissingToken(_)
///         |Error::Cancelled
///         |Error::UploadSizeLimitExceeded(_, _)
///         |Error::Failure(_)
///         |Error::BadRequest(_)
///         |Error::FieldClash(_)
///         |Error::JsonDecodeError(_, _) => println!("{}", e),
///     },
///     Ok(res) => println!("Success: {:?}", res),
/// }
/// # }
/// ```
pub struct PlayMovies<C, A> {
    client: RefCell<C>,
    auth: RefCell<A>,
    _user_agent: String,
}

impl<'a, C, A> Hub for PlayMovies<C, A> {}

impl<'a, C, A> PlayMovies<C, A>
    where  C: BorrowMut<hyper::Client>, A: oauth2::GetToken {

    pub fn new(client: C, authenticator: A) -> PlayMovies<C, A> {
        PlayMovies {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "google-api-rust-client/1.0.4".to_string(),
        }
    }

    pub fn accounts(&'a self) -> AccountMethods<'a, C, A> {
        AccountMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/1.0.4`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        let prev = self._user_agent.clone();
        self._user_agent = agent_name;
        prev
    }
}


// ############
// SCHEMAS ###
// ##########
/// Response to the 'ListExperienceLocales' method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [experience locales list accounts](struct.AccountExperienceLocaleListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListExperienceLocalesResponse {
    /// See _List methods rules_ for info about this field.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// See _List methods rules_ for more information about this field.
    #[serde(rename="totalSize")]
    pub total_size: Option<i32>,
    /// List of ExperienceLocales that match the request criteria.
    #[serde(rename="experienceLocales")]
    pub experience_locales: Option<Vec<ExperienceLocale>>,
}

impl ResponseResult for ListExperienceLocalesResponse {}


/// Response to the 'ListOrders' method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [orders list accounts](struct.AccountOrderListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListOrdersResponse {
    /// See _List methods rules_ for info about this field.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// See _List methods rules_ for more information about this field.
    #[serde(rename="totalSize")]
    pub total_size: Option<i32>,
    /// List of Orders that match the request criteria.
    pub orders: Option<Vec<Order>>,
}

impl ResponseResult for ListOrdersResponse {}


/// Response to the 'ListStoreInfos' method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [store infos list accounts](struct.AccountStoreInfoListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListStoreInfosResponse {
    /// List of StoreInfos that match the request criteria.
    #[serde(rename="storeInfos")]
    pub store_infos: Option<Vec<StoreInfo>>,
    /// See 'List methods rules' for info about this field.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// See _List methods rules_ for more information about this field.
    #[serde(rename="totalSize")]
    pub total_size: Option<i32>,
}

impl ResponseResult for ListStoreInfosResponse {}


/// A Component is an element (audio, video, subtitle, artwork, trailer, etc...) that is used in a set of ExperienceLocales. A Component is owned by a Studio and managed either by the Studio itself or by one Post-Production House. Each Component is identified by a `component_id` and its `type`.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [components type get accounts](struct.AccountComponentTypeGetCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Component {
    /// High-level status of the Component.
    pub status: Option<String>,
    /// List of Alternative Cut IDs, sometimes available in lieu of the main Edit-level EIDR IDs. This is not an EIDR ID, but a Partner-provided ID. Example: "206346_79838".
    #[serde(rename="altCutIds")]
    pub alt_cut_ids: Option<Vec<String>>,
    /// Processing errors during XML file parsing. Example: 'Invalid input file'
    #[serde(rename="processingErrors")]
    pub processing_errors: Option<Vec<String>>,
    /// Name of the studio that owns the Component.
    #[serde(rename="studioName")]
    pub studio_name: Option<String>,
    /// Detailed status of the Component
    #[serde(rename="statusDetail")]
    pub status_detail: Option<String>,
    /// Notes explaining why a Component has been rejected.
    #[serde(rename="rejectionNote")]
    pub rejection_note: Option<String>,
    /// Timestamp when the Component was approved.
    #[serde(rename="approvedTime")]
    pub approved_time: Option<String>,
    /// List of Title-level EIDR IDs. Example: "10.5240/1489-49A2-3956-4B2D-FE16-5".
    #[serde(rename="titleLevelEidrs")]
    pub title_level_eidrs: Option<Vec<String>>,
    /// IDs internally generated by Google to uniquely identify the ExperienceLocales for which the Component is used. Example: 'KRZiVjY9h7t'
    #[serde(rename="elIds")]
    pub el_ids: Option<Vec<String>>,
    /// Default Edit name, usually in the language of the country of origin. Example: "Googlers, The".
    pub name: Option<String>,
    /// Language of the component, using the "BCP 47" format. Examples: "en", "en-US", "es", "es-419".
    pub language: Option<String>,
    /// Type of the playable unit for which the Component is intended.
    #[serde(rename="playableUnitType")]
    pub playable_unit_type: Option<String>,
    /// Detail about the type of the Component.
    #[serde(rename="componentDetailType")]
    pub component_detail_type: Option<String>,
    /// Timestamp when the Component was received.
    #[serde(rename="receivedTime")]
    pub received_time: Option<String>,
    /// List of custom IDs (defined by the partner) linked to the ExperienceLocale using this Component. Example: "R86241"
    #[serde(rename="customIds")]
    pub custom_ids: Option<Vec<String>>,
    /// Name of the post-production house that manages the Component.
    #[serde(rename="pphName")]
    pub pph_name: Option<String>,
    /// File name of the Component when delivered.
    pub filename: Option<String>,
    /// Component priority, as defined by Google. The higher the value, the higher the priority.
    pub priority: Option<f64>,
    /// List of Edit-level EIDR IDs. Example: "10.5240/1489-49A2-3956-4B2D-FE16-6".
    #[serde(rename="editLevelEidrs")]
    pub edit_level_eidrs: Option<Vec<String>>,
    /// A simpler representation of the priority.
    #[serde(rename="normalizedPriority")]
    pub normalized_priority: Option<String>,
    /// Type of the Component. Example: AUDIO_51
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// ID internally generated by Google to uniquely identify the Component. Example: 'wteyrc_647xc'
    #[serde(rename="componentId")]
    pub component_id: Option<String>,
}

impl ResponseResult for Component {}


/// An Avail describes the Availability Window of a specific Edit in a given country, which means the period Google is allowed to sell or rent the Edit. Avails are exposed in EMA format Version 1.6b (available at http://www.movielabs.com/md/avails/) Studios can see the Avails for the Titles they own. Post-production houses cannot see any Avails.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [avails get accounts](struct.AccountAvailGetCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Avail {
    /// Name of the post-production houses that manage the Avail. Not part of EMA Specs.
    #[serde(rename="pphNames")]
    pub pph_names: Option<Vec<String>>,
    /// Type of transaction.
    #[serde(rename="licenseType")]
    pub license_type: Option<String>,
    /// Manifestation Identifier. This should be the Manifestation Level EIDR. Example: "10.2340/1489-49A2-3956-4B2D-FE16-7"
    #[serde(rename="encodeId")]
    pub encode_id: Option<String>,
    /// Communicating if caption file will be delivered.
    #[serde(rename="captionIncluded")]
    pub caption_included: Option<bool>,
    /// Release date of the Title in earliest released territory. Typically it is just the year, but it is free-form as per EMA spec. Examples: "1979", "Oct 2014"
    #[serde(rename="releaseDate")]
    pub release_date: Option<String>,
    /// Other identifier referring to the episode, as defined by partner. Only available on TV avails. Example: "rs_googlers_s1_3".
    #[serde(rename="episodeAltId")]
    pub episode_alt_id: Option<String>,
    /// OPTIONAL.TV Only. Title used by involved parties to refer to this episode. Only available on TV Avails. Example: "Coding at Google".
    #[serde(rename="episodeTitleInternalAlias")]
    pub episode_title_internal_alias: Option<String>,
    /// ID internally generated by Google to uniquely identify an Avail. Not part of EMA Specs.
    #[serde(rename="availId")]
    pub avail_id: Option<String>,
    /// End of term in YYYY-MM-DD format in the timezone of the country of the Avail. "Open" if no end date is available. Example: "2019-02-17"
    pub end: Option<String>,
    /// Other identifier referring to the Edit, as defined by partner. Example: "GOOGLER_2006"
    #[serde(rename="altId")]
    pub alt_id: Option<String>,
    /// Other identifier referring to the series, as defined by partner. Only available on TV avails. Example: "rs_googlers".
    #[serde(rename="seriesAltId")]
    pub series_alt_id: Option<String>,
    /// Start of term in YYYY-MM-DD format in the timezone of the country of the Avail. Example: "2013-05-14".
    pub start: Option<String>,
    /// Title used by involved parties to refer to this content. Example: "Googlers, The". Only available on Movie Avails.
    #[serde(rename="titleInternalAlias")]
    pub title_internal_alias: Option<String>,
    /// Title used by involved parties to refer to this season. Only available on TV Avails. Example: "Googlers, The".
    #[serde(rename="seasonTitleInternalAlias")]
    pub season_title_internal_alias: Option<String>,
    /// ISO 3166-1 alpha-2 country code for the country or territory of this Avail. For Avails, we use Territory in lieu of Country to comply with EMA specifications. But please note that Territory and Country identify the same thing. Example: "US".
    pub territory: Option<String>,
    /// Edit Identifier. This should be the Edit Level EIDR. Example: "10.2340/1489-49A2-3956-4B2D-FE16-6"
    #[serde(rename="productId")]
    pub product_id: Option<String>,
    /// Spoken language of the intended audience. Language shall be encoded in accordance with RFC 5646. Example: "fr".
    #[serde(rename="storeLanguage")]
    pub store_language: Option<String>,
    /// The number assigned to the season within a series. Only available on TV Avails. Example: "1".
    #[serde(rename="seasonNumber")]
    pub season_number: Option<String>,
    /// The number assigned to the episode within a season. Only available on TV Avails. Example: "3".
    #[serde(rename="episodeNumber")]
    pub episode_number: Option<String>,
    /// Indicates the format profile covered by the transaction.
    #[serde(rename="formatProfile")]
    pub format_profile: Option<String>,
    /// Value to be applied to the pricing type. Example: "4" or "2.99"
    #[serde(rename="priceValue")]
    pub price_value: Option<String>,
    /// Google-generated ID identifying the video linked to this Avail, once delivered. Not part of EMA Specs. Example: 'gtry456_xc'
    #[serde(rename="videoId")]
    pub video_id: Option<String>,
    /// Value representing the rating reason. Rating reasons should be formatted as per [EMA ratings spec](http://www.movielabs.com/md/ratings/) and comma-separated for inclusion of multiple reasons. Example: "L, S, V"
    #[serde(rename="ratingReason")]
    pub rating_reason: Option<String>,
    /// Communicating an exempt category as defined by FCC regulations. It is not required for non-US Avails. Example: "1"
    #[serde(rename="captionExemption")]
    pub caption_exemption: Option<String>,
    /// Work type as enumerated in EMA.
    #[serde(rename="workType")]
    pub work_type: Option<String>,
    /// The name of the studio that owns the Edit referred in the Avail. This is the equivalent of `studio_name` in other resources, but it follows the EMA nomenclature. Example: "Google Films".
    #[serde(rename="displayName")]
    pub display_name: Option<String>,
    /// Other identifier referring to the season, as defined by partner. Only available on TV avails. Example: "rs_googlers_s1".
    #[serde(rename="seasonAltId")]
    pub season_alt_id: Option<String>,
    /// Title Identifier. This should be the Title Level EIDR. Example: "10.5240/1489-49A2-3956-4B2D-FE16-5".
    #[serde(rename="contentId")]
    pub content_id: Option<String>,
    /// Rating system applied to the version of title within territory of Avail. Rating systems should be formatted as per [EMA ratings spec](http://www.movielabs.com/md/ratings/) Example: "MPAA"
    #[serde(rename="ratingSystem")]
    pub rating_system: Option<String>,
    /// Value representing the rating. Ratings should be formatted as per http://www.movielabs.com/md/ratings/ Example: "PG"
    #[serde(rename="ratingValue")]
    pub rating_value: Option<String>,
    /// Title used by involved parties to refer to this series. Only available on TV Avails. Example: "Googlers, The".
    #[serde(rename="seriesTitleInternalAlias")]
    pub series_title_internal_alias: Option<String>,
    /// First date an Edit could be publically announced as becoming available at a specific future date in territory of Avail. *Not* the Avail start date or pre-order start date. Format is YYYY-MM-DD. Only available for pre-orders. Example: "2012-12-10"
    #[serde(rename="suppressionLiftDate")]
    pub suppression_lift_date: Option<String>,
    /// Type of pricing that should be applied to this Avail based on how the partner classify them. Example: "Tier", "WSP", "SRP", or "Category".
    #[serde(rename="priceType")]
    pub price_type: Option<String>,
}

impl ResponseResult for Avail {}


/// An ExperienceLocale tracks the fulfillment of a Title in a country using a specific language, when delivered using component-based delivery. For example, a Title in Switzerland might have 3 ExperienceLocales: they both share the same country ("CH"), but each has different languages ("de", "fr", and "it"). Each ExperienceLocale is uniquely identified by an `el_id`, which is generated by Google. Externally, an ExperienceLocale can also be identified by partners using its EIDR IDs, AltCutID or `custom_id` (when provided).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [experience locales get accounts](struct.AccountExperienceLocaleGetCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExperienceLocale {
    /// Name of the post-production houses that manage the ExperienceLocale.
    #[serde(rename="pphNames")]
    pub pph_names: Option<Vec<String>>,
    /// High-level status of the ExperienceLocale.
    pub status: Option<String>,
    /// Title-level EIDR ID. Example: "10.5240/1489-49A2-3956-4B2D-FE16-5".
    #[serde(rename="titleLevelEidr")]
    pub title_level_eidr: Option<String>,
    /// Trailer ID, as defined by Google, linked to the trailer video in the ExperienceLocale. Example: 'gtry457_tr'.
    #[serde(rename="trailerId")]
    pub trailer_id: Option<String>,
    /// YouTube Channel ID linked to the ExperienceLocale. Example: "UCRG64darCZhb".
    #[serde(rename="channelId")]
    pub channel_id: Option<String>,
    /// Name of the studio that owns the ExperienceLocale.
    #[serde(rename="studioName")]
    pub studio_name: Option<String>,
    /// Video ID, as defined by Google, linked to the feature video in the ExperienceLocale. Example: 'gtry456_xc'.
    #[serde(rename="videoId")]
    pub video_id: Option<String>,
    /// Timestamp when the ExperienceLocale was approved.
    #[serde(rename="approvedTime")]
    pub approved_time: Option<String>,
    /// Edit-level EIDR ID. Example: "10.5240/1489-49A2-3956-4B2D-FE16-6".
    #[serde(rename="editLevelEidr")]
    pub edit_level_eidr: Option<String>,
    /// InventoryID as defined in the EMA specs.
    #[serde(rename="inventoryId")]
    pub inventory_id: Option<String>,
    /// PlayableSequenceID as defined in the EMA specs.
    #[serde(rename="playableSequenceId")]
    pub playable_sequence_id: Option<String>,
    /// Default Edit name, usually in the language of the country of origin. Example: "Googlers, The".
    pub name: Option<String>,
    /// ID internally generated by Google to uniquely identify a ExperienceLocale. Example: 'KRZiVjY9h7t'
    #[serde(rename="elId")]
    pub el_id: Option<String>,
    /// PresentationID as defined in the EMA specs.
    #[serde(rename="presentationId")]
    pub presentation_id: Option<String>,
    /// Timestamp of the earliest start date of the Avails linked to this ExperienceLocale.
    #[serde(rename="earliestAvailStartTime")]
    pub earliest_avail_start_time: Option<String>,
    /// Language of the ExperienceLocale, using the "BCP 47" format. Examples: "en", "en-US", "es", "es-419".
    pub language: Option<String>,
    /// Alternative Cut ID, sometimes available in lieu of the main Edit-level EIDR ID. This is not an EIDR ID, but a Partner-provided ID. Example: "206346_79838".
    #[serde(rename="altCutId")]
    pub alt_cut_id: Option<String>,
    /// List of custom IDs (defined by the partner) linked to this ExperienceLocale. Example: "R86241"
    #[serde(rename="customIds")]
    pub custom_ids: Option<Vec<String>>,
    /// ExperienceLocale priority, as defined by Google. The higher the value, the higher the priority. Example: 90
    pub priority: Option<f64>,
    /// A simpler representation of the priority.
    #[serde(rename="normalizedPriority")]
    pub normalized_priority: Option<String>,
    /// Timestamp when the ExperienceLocale was created.
    #[serde(rename="createdTime")]
    pub created_time: Option<String>,
    /// Country where the ExperienceLocale is available, using the "ISO 3166-1 alpha-2" format. Example: "US".
    pub country: Option<String>,
    /// Type of the Edit linked to the ExperienceLocale.
    #[serde(rename="type")]
    pub type_: Option<String>,
}

impl ResponseResult for ExperienceLocale {}


/// Information about a playable sequence (video) associated with an Edit and available at the Google Play Store. Internally, each StoreInfo is uniquely identified by a `video_id` and `country`. Externally, Title-level EIDR or Edit-level EIDR, if provided, can also be used to identify a specific title or edit in a country.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [store infos country get accounts](struct.AccountStoreInfoCountryGetCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StoreInfo {
    /// Name of the post-production houses that manage the Edit.
    #[serde(rename="pphNames")]
    pub pph_names: Option<Vec<String>>,
    /// Whether the Edit has a EST offer.
    #[serde(rename="hasEstOffer")]
    pub has_est_offer: Option<bool>,
    /// Title-level EIDR ID. Example: "10.5240/1489-49A2-3956-4B2D-FE16-5".
    #[serde(rename="titleLevelEidr")]
    pub title_level_eidr: Option<String>,
    /// Whether the Edit has a SD offer.
    #[serde(rename="hasSdOffer")]
    pub has_sd_offer: Option<bool>,
    /// The number assigned to the season within a show. Only available on TV Edits. Example: "1".
    #[serde(rename="seasonNumber")]
    pub season_number: Option<String>,
    /// Timestamp when the Edit went live on the Store.
    #[serde(rename="liveTime")]
    pub live_time: Option<String>,
    /// Google-generated ID identifying the trailer linked to the Edit. Example: 'bhd_4e_cx'
    #[serde(rename="trailerId")]
    pub trailer_id: Option<String>,
    /// Name of the studio that owns the Edit ordered.
    #[serde(rename="studioName")]
    pub studio_name: Option<String>,
    /// Google-generated ID identifying the video linked to the Edit. Example: 'gtry456_xc'
    #[serde(rename="videoId")]
    pub video_id: Option<String>,
    /// Whether the Edit has a HD offer.
    #[serde(rename="hasHdOffer")]
    pub has_hd_offer: Option<bool>,
    /// Whether the Edit has a VOD offer.
    #[serde(rename="hasVodOffer")]
    pub has_vod_offer: Option<bool>,
    /// Edit-level EIDR ID. Example: "10.5240/1489-49A2-3956-4B2D-FE16-6".
    #[serde(rename="editLevelEidr")]
    pub edit_level_eidr: Option<String>,
    /// Subtitles available for this Edit.
    pub subtitles: Option<Vec<String>>,
    /// The number assigned to the episode within a season. Only available on TV Edits. Example: "1".
    #[serde(rename="episodeNumber")]
    pub episode_number: Option<String>,
    /// Google-generated ID identifying the show linked to the Edit. Only available for TV Edits. Example: 'et2hsue_x'
    #[serde(rename="showId")]
    pub show_id: Option<String>,
    /// Default Edit name, usually in the language of the country of origin. Example: "Googlers, The".
    pub name: Option<String>,
    /// Country where Edit is available in ISO 3166-1 alpha-2 country code. Example: "US".
    pub country: Option<String>,
    /// Google-generated ID identifying the season linked to the Edit. Only available for TV Edits. Example: 'ster23ex'
    #[serde(rename="seasonId")]
    pub season_id: Option<String>,
    /// Knowledge Graph ID associated to this Edit, if available. This ID links the Edit to its knowledge entity, externally accessible at http://freebase.com. In the absense of Title EIDR or Edit EIDR, this ID helps link together multiple Edits across countries. Example: '/m/0ffx29'
    pub mid: Option<String>,
    /// Whether the Edit has a 5.1 channel audio track.
    #[serde(rename="hasAudio51")]
    pub has_audio51: Option<bool>,
    /// Default Season name, usually in the language of the country of origin. Only available for TV Edits Example: "Googlers, The - A Brave New World".
    #[serde(rename="seasonName")]
    pub season_name: Option<String>,
    /// Audio tracks available for this Edit.
    #[serde(rename="audioTracks")]
    pub audio_tracks: Option<Vec<String>>,
    /// Whether the Edit has info cards.
    #[serde(rename="hasInfoCards")]
    pub has_info_cards: Option<bool>,
    /// Edit type, like Movie, Episode or Season.
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// Default Show name, usually in the language of the country of origin. Only available for TV Edits Example: "Googlers, The".
    #[serde(rename="showName")]
    pub show_name: Option<String>,
}

impl ResponseResult for StoreInfo {}


/// Response to the 'ListAvails' method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [avails list accounts](struct.AccountAvailListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListAvailsResponse {
    /// See _List methods rules_ for info about this field.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// See _List methods rules_ for more information about this field.
    #[serde(rename="totalSize")]
    pub total_size: Option<i32>,
    /// List of Avails that match the request criteria.
    pub avails: Option<Vec<Avail>>,
}

impl ResponseResult for ListAvailsResponse {}


/// An Order tracks the fulfillment of an Edit when delivered using the legacy, non-component-based delivery. Each Order is uniquely identified by an `order_id`, which is generated by Google. Externally, Orders can also be identified by partners using its `custom_id` (when provided).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [orders get accounts](struct.AccountOrderGetCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Order {
    /// ID internally generated by Google to uniquely identify an Order. Example: 'abcde12_x'
    #[serde(rename="orderId")]
    pub order_id: Option<String>,
    /// High-level status of the order.
    pub status: Option<String>,
    /// YouTube Channel ID that should be used to fulfill the Order. Example: "UCRG64darCZhb".
    #[serde(rename="channelId")]
    pub channel_id: Option<String>,
    /// Name of the studio that owns the Edit ordered.
    #[serde(rename="studioName")]
    pub studio_name: Option<String>,
    /// Detailed status of the order
    #[serde(rename="statusDetail")]
    pub status_detail: Option<String>,
    /// Field explaining why an Order has been rejected. Example: "Trailer audio is 2ch mono, please re-deliver in stereo".
    #[serde(rename="rejectionNote")]
    pub rejection_note: Option<String>,
    /// Timestamp when the Order was approved.
    #[serde(rename="approvedTime")]
    pub approved_time: Option<String>,
    /// Google-generated ID identifying the video linked to this Order, once delivered. Example: 'gtry456_xc'.
    #[serde(rename="videoId")]
    pub video_id: Option<String>,
    /// Default Episode name, usually in the language of the country of origin. Only available for TV Edits Example: "Googlers, The - Pilot".
    #[serde(rename="episodeName")]
    pub episode_name: Option<String>,
    /// Default Edit name, usually in the language of the country of origin. Example: "Googlers, The".
    pub name: Option<String>,
    /// Default Season name, usually in the language of the country of origin. Only available for TV Edits Example: "Googlers, The - A Brave New World".
    #[serde(rename="seasonName")]
    pub season_name: Option<String>,
    /// Timestamp of the earliest start date of the Avails linked to this Order.
    #[serde(rename="earliestAvailStartTime")]
    pub earliest_avail_start_time: Option<String>,
    /// Countries where the Order is available, using the "ISO 3166-1 alpha-2" format (example: "US").
    pub countries: Option<Vec<String>>,
    /// A simpler representation of the priority.
    #[serde(rename="normalizedPriority")]
    pub normalized_priority: Option<String>,
    /// Timestamp when the Order was fulfilled.
    #[serde(rename="receivedTime")]
    pub received_time: Option<String>,
    /// Timestamp when the Order was created.
    #[serde(rename="orderedTime")]
    pub ordered_time: Option<String>,
    /// Name of the post-production house that manages the Edit ordered.
    #[serde(rename="pphName")]
    pub pph_name: Option<String>,
    /// Order priority, as defined by Google. The higher the value, the higher the priority. Example: 90
    pub priority: Option<f64>,
    /// Legacy Order priority, as defined by Google. Example: 'P0'
    #[serde(rename="legacyPriority")]
    pub legacy_priority: Option<String>,
    /// ID that can be used to externally identify an Order. This ID is provided by partners when submitting the Avails. Example: 'GOOGLER_2006'
    #[serde(rename="customId")]
    pub custom_id: Option<String>,
    /// YouTube Channel Name that should be used to fulfill the Order. Example: "Google_channel".
    #[serde(rename="channelName")]
    pub channel_name: Option<String>,
    /// Type of the Edit linked to the Order.
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// Default Show name, usually in the language of the country of origin. Only available for TV Edits Example: "Googlers, The".
    #[serde(rename="showName")]
    pub show_name: Option<String>,
}

impl ResponseResult for Order {}


/// Response to the 'ListComponents' method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [components list accounts](struct.AccountComponentListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListComponentsResponse {
    /// See _List methods rules_ for info about this field.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// See _List methods rules_ for more information about this field.
    #[serde(rename="totalSize")]
    pub total_size: Option<i32>,
    /// List of Components that match the request criteria.
    pub components: Option<Vec<Component>>,
}

impl ResponseResult for ListComponentsResponse {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *account* resources.
/// It is not used directly, but through the `PlayMovies` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_playmoviespartner1 as playmoviespartner1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use playmoviespartner1::PlayMovies;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = PlayMovies::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `avails_get(...)`, `avails_list(...)`, `components_list(...)`, `components_type_get(...)`, `experience_locales_get(...)`, `experience_locales_list(...)`, `orders_get(...)`, `orders_list(...)`, `store_infos_country_get(...)` and `store_infos_list(...)`
/// // to build up your call.
/// let rb = hub.accounts();
/// # }
/// ```
pub struct AccountMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a PlayMovies<C, A>,
}

impl<'a, C, A> MethodsBuilder for AccountMethods<'a, C, A> {}

impl<'a, C, A> AccountMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// List Orders owned or managed by the partner. See _Authentication and Authorization rules_ and _List methods rules_ for more information about this method.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - REQUIRED. See _General rules_ for more information about this field.
    pub fn orders_list(&self, account_id: &str) -> AccountOrderListCall<'a, C, A> {
        AccountOrderListCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _video_ids: Default::default(),
            _studio_names: Default::default(),
            _status: Default::default(),
            _pph_names: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _name: Default::default(),
            _custom_id: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get a StoreInfo given its video id and country. See _Authentication and Authorization rules_ and _Get methods rules_ for more information about this method.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - REQUIRED. See _General rules_ for more information about this field.
    /// * `videoId` - REQUIRED. Video ID.
    /// * `country` - REQUIRED. Edit country.
    pub fn store_infos_country_get(&self, account_id: &str, video_id: &str, country: &str) -> AccountStoreInfoCountryGetCall<'a, C, A> {
        AccountStoreInfoCountryGetCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _video_id: video_id.to_string(),
            _country: country.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get an Order given its id. See _Authentication and Authorization rules_ and _Get methods rules_ for more information about this method.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - REQUIRED. See _General rules_ for more information about this field.
    /// * `orderId` - REQUIRED. Order ID.
    pub fn orders_get(&self, account_id: &str, order_id: &str) -> AccountOrderGetCall<'a, C, A> {
        AccountOrderGetCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get an Avail given its avail group id and avail id.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - REQUIRED. See _General rules_ for more information about this field.
    /// * `availId` - REQUIRED. Avail ID.
    pub fn avails_get(&self, account_id: &str, avail_id: &str) -> AccountAvailGetCall<'a, C, A> {
        AccountAvailGetCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _avail_id: avail_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List Avails owned or managed by the partner. See _Authentication and Authorization rules_ and _List methods rules_ for more information about this method.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - REQUIRED. See _General rules_ for more information about this field.
    pub fn avails_list(&self, account_id: &str) -> AccountAvailListCall<'a, C, A> {
        AccountAvailListCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _video_ids: Default::default(),
            _title: Default::default(),
            _territories: Default::default(),
            _studio_names: Default::default(),
            _pph_names: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _alt_ids: Default::default(),
            _alt_id: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get an ExperienceLocale given its id. See _Authentication and Authorization rules_ and _Get methods rules_ for more information about this method.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - REQUIRED. See _General rules_ for more information about this field.
    /// * `elId` - REQUIRED. ExperienceLocale ID, as defined by Google.
    pub fn experience_locales_get(&self, account_id: &str, el_id: &str) -> AccountExperienceLocaleGetCall<'a, C, A> {
        AccountExperienceLocaleGetCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _el_id: el_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List StoreInfos owned or managed by the partner. See _Authentication and Authorization rules_ and _List methods rules_ for more information about this method.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - REQUIRED. See _General rules_ for more information about this field.
    pub fn store_infos_list(&self, account_id: &str) -> AccountStoreInfoListCall<'a, C, A> {
        AccountStoreInfoListCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _video_ids: Default::default(),
            _video_id: Default::default(),
            _studio_names: Default::default(),
            _season_ids: Default::default(),
            _pph_names: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _name: Default::default(),
            _mids: Default::default(),
            _countries: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List ExperienceLocales owned or managed by the partner. See _Authentication and Authorization rules_ and _List methods rules_ for more information about this method.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - REQUIRED. See _General rules_ for more information about this field.
    pub fn experience_locales_list(&self, account_id: &str) -> AccountExperienceLocaleListCall<'a, C, A> {
        AccountExperienceLocaleListCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _title_level_eidr: Default::default(),
            _studio_names: Default::default(),
            _status: Default::default(),
            _pph_names: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _edit_level_eidr: Default::default(),
            _custom_id: Default::default(),
            _alt_cut_id: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List Components owned or managed by the partner. See _Authentication and Authorization rules_ and _List methods rules_ for more information about this method.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - REQUIRED. See _General rules_ for more information about this field.
    pub fn components_list(&self, account_id: &str) -> AccountComponentListCall<'a, C, A> {
        AccountComponentListCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _title_level_eidr: Default::default(),
            _studio_names: Default::default(),
            _status: Default::default(),
            _presentation_id: Default::default(),
            _pph_names: Default::default(),
            _playable_sequence_id: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _inventory_id: Default::default(),
            _filename: Default::default(),
            _el_id: Default::default(),
            _edit_level_eidr: Default::default(),
            _custom_id: Default::default(),
            _alt_cut_id: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get a Component given its id.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - REQUIRED. See _General rules_ for more information about this field.
    /// * `componentId` - REQUIRED. Component ID.
    /// * `type` - REQUIRED. Component Type.
    pub fn components_type_get(&self, account_id: &str, component_id: &str, type_: &str) -> AccountComponentTypeGetCall<'a, C, A> {
        AccountComponentTypeGetCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _component_id: component_id.to_string(),
            _type_: type_.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// List Orders owned or managed by the partner. See _Authentication and Authorization rules_ and _List methods rules_ for more information about this method.
///
/// A builder for the *orders.list* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_playmoviespartner1 as playmoviespartner1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use playmoviespartner1::PlayMovies;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = PlayMovies::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().orders_list("accountId")
///              .add_video_ids("accusam")
///              .add_studio_names("takimata")
///              .add_status("justo")
///              .add_pph_names("amet.")
///              .page_token("erat")
///              .page_size(-35)
///              .name("sea")
///              .custom_id("nonumy")
///              .doit();
/// # }
/// ```
pub struct AccountOrderListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a PlayMovies<C, A>,
    _account_id: String,
    _video_ids: Vec<String>,
    _studio_names: Vec<String>,
    _status: Vec<String>,
    _pph_names: Vec<String>,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _name: Option<String>,
    _custom_id: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for AccountOrderListCall<'a, C, A> {}

impl<'a, C, A> AccountOrderListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ListOrdersResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "playmoviespartner.accounts.orders.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((11 + self._additional_params.len()));
        params.push(("accountId", self._account_id.to_string()));
        if self._video_ids.len() > 0 {
            for f in self._video_ids.iter() {
                params.push(("videoIds", f.to_string()));
            }
        }
        if self._studio_names.len() > 0 {
            for f in self._studio_names.iter() {
                params.push(("studioNames", f.to_string()));
            }
        }
        if self._status.len() > 0 {
            for f in self._status.iter() {
                params.push(("status", f.to_string()));
            }
        }
        if self._pph_names.len() > 0 {
            for f in self._pph_names.iter() {
                params.push(("pphNames", f.to_string()));
            }
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        if let Some(value) = self._name {
            params.push(("name", value.to_string()));
        }
        if let Some(value) = self._custom_id {
            params.push(("customId", value.to_string()));
        }
        for &field in ["alt", "accountId", "videoIds", "studioNames", "status", "pphNames", "pageToken", "pageSize", "name", "customId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://playmoviespartner.googleapis.com/v1/accounts/{accountId}/orders".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::PlaymovyPartnerReadonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{accountId}", "accountId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["accountId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }



        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, &url)
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// REQUIRED. See _General rules_ for more information about this field.
    ///
    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn account_id(mut self, new_value: &str) -> AccountOrderListCall<'a, C, A> {
        self._account_id = new_value.to_string();
        self
    }
    /// Filter Orders that match any of the given `video_id`s.
    ///
    /// Append the given value to the *video ids* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_video_ids(mut self, new_value: &str) -> AccountOrderListCall<'a, C, A> {
        self._video_ids.push(new_value.to_string());
        self
    }
    /// See _List methods rules_ for info about this field.
    ///
    /// Append the given value to the *studio names* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_studio_names(mut self, new_value: &str) -> AccountOrderListCall<'a, C, A> {
        self._studio_names.push(new_value.to_string());
        self
    }
    /// Filter Orders that match one of the given status.
    ///
    /// Append the given value to the *status* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_status(mut self, new_value: &str) -> AccountOrderListCall<'a, C, A> {
        self._status.push(new_value.to_string());
        self
    }
    /// See _List methods rules_ for info about this field.
    ///
    /// Append the given value to the *pph names* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_pph_names(mut self, new_value: &str) -> AccountOrderListCall<'a, C, A> {
        self._pph_names.push(new_value.to_string());
        self
    }
    /// See _List methods rules_ for info about this field.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> AccountOrderListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// See _List methods rules_ for info about this field.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> AccountOrderListCall<'a, C, A> {
        self._page_size = Some(new_value);
        self
    }
    /// Filter that matches Orders with a `name`, `show`, `season` or `episode` that contains the given case-insensitive name.
    ///
    /// Sets the *name* query property to the given value.
    pub fn name(mut self, new_value: &str) -> AccountOrderListCall<'a, C, A> {
        self._name = Some(new_value.to_string());
        self
    }
    /// Filter Orders that match a case-insensitive, partner-specific custom id.
    ///
    /// Sets the *custom id* query property to the given value.
    pub fn custom_id(mut self, new_value: &str) -> AccountOrderListCall<'a, C, A> {
        self._custom_id = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> AccountOrderListCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> AccountOrderListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::PlaymovyPartnerReadonly`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> AccountOrderListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// Get a StoreInfo given its video id and country. See _Authentication and Authorization rules_ and _Get methods rules_ for more information about this method.
///
/// A builder for the *storeInfos.country.get* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_playmoviespartner1 as playmoviespartner1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use playmoviespartner1::PlayMovies;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = PlayMovies::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().store_infos_country_get("accountId", "videoId", "country")
///              .doit();
/// # }
/// ```
pub struct AccountStoreInfoCountryGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a PlayMovies<C, A>,
    _account_id: String,
    _video_id: String,
    _country: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for AccountStoreInfoCountryGetCall<'a, C, A> {}

impl<'a, C, A> AccountStoreInfoCountryGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, StoreInfo)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "playmoviespartner.accounts.storeInfos.country.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        params.push(("accountId", self._account_id.to_string()));
        params.push(("videoId", self._video_id.to_string()));
        params.push(("country", self._country.to_string()));
        for &field in ["alt", "accountId", "videoId", "country"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://playmoviespartner.googleapis.com/v1/accounts/{accountId}/storeInfos/{videoId}/country/{country}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::PlaymovyPartnerReadonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{accountId}", "accountId"), ("{videoId}", "videoId"), ("{country}", "country")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(3);
            for param_name in ["country", "videoId", "accountId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }



        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, &url)
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// REQUIRED. See _General rules_ for more information about this field.
    ///
    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn account_id(mut self, new_value: &str) -> AccountStoreInfoCountryGetCall<'a, C, A> {
        self._account_id = new_value.to_string();
        self
    }
    /// REQUIRED. Video ID.
    ///
    /// Sets the *video id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn video_id(mut self, new_value: &str) -> AccountStoreInfoCountryGetCall<'a, C, A> {
        self._video_id = new_value.to_string();
        self
    }
    /// REQUIRED. Edit country.
    ///
    /// Sets the *country* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn country(mut self, new_value: &str) -> AccountStoreInfoCountryGetCall<'a, C, A> {
        self._country = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> AccountStoreInfoCountryGetCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> AccountStoreInfoCountryGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::PlaymovyPartnerReadonly`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> AccountStoreInfoCountryGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// Get an Order given its id. See _Authentication and Authorization rules_ and _Get methods rules_ for more information about this method.
///
/// A builder for the *orders.get* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_playmoviespartner1 as playmoviespartner1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use playmoviespartner1::PlayMovies;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = PlayMovies::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().orders_get("accountId", "orderId")
///              .doit();
/// # }
/// ```
pub struct AccountOrderGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a PlayMovies<C, A>,
    _account_id: String,
    _order_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for AccountOrderGetCall<'a, C, A> {}

impl<'a, C, A> AccountOrderGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Order)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "playmoviespartner.accounts.orders.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("accountId", self._account_id.to_string()));
        params.push(("orderId", self._order_id.to_string()));
        for &field in ["alt", "accountId", "orderId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://playmoviespartner.googleapis.com/v1/accounts/{accountId}/orders/{orderId}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::PlaymovyPartnerReadonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{accountId}", "accountId"), ("{orderId}", "orderId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["orderId", "accountId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }



        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, &url)
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// REQUIRED. See _General rules_ for more information about this field.
    ///
    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn account_id(mut self, new_value: &str) -> AccountOrderGetCall<'a, C, A> {
        self._account_id = new_value.to_string();
        self
    }
    /// REQUIRED. Order ID.
    ///
    /// Sets the *order id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn order_id(mut self, new_value: &str) -> AccountOrderGetCall<'a, C, A> {
        self._order_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> AccountOrderGetCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> AccountOrderGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::PlaymovyPartnerReadonly`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> AccountOrderGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// Get an Avail given its avail group id and avail id.
///
/// A builder for the *avails.get* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_playmoviespartner1 as playmoviespartner1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use playmoviespartner1::PlayMovies;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = PlayMovies::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().avails_get("accountId", "availId")
///              .doit();
/// # }
/// ```
pub struct AccountAvailGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a PlayMovies<C, A>,
    _account_id: String,
    _avail_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for AccountAvailGetCall<'a, C, A> {}

impl<'a, C, A> AccountAvailGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Avail)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "playmoviespartner.accounts.avails.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("accountId", self._account_id.to_string()));
        params.push(("availId", self._avail_id.to_string()));
        for &field in ["alt", "accountId", "availId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://playmoviespartner.googleapis.com/v1/accounts/{accountId}/avails/{availId}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::PlaymovyPartnerReadonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{accountId}", "accountId"), ("{availId}", "availId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["availId", "accountId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }



        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, &url)
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// REQUIRED. See _General rules_ for more information about this field.
    ///
    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn account_id(mut self, new_value: &str) -> AccountAvailGetCall<'a, C, A> {
        self._account_id = new_value.to_string();
        self
    }
    /// REQUIRED. Avail ID.
    ///
    /// Sets the *avail id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn avail_id(mut self, new_value: &str) -> AccountAvailGetCall<'a, C, A> {
        self._avail_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> AccountAvailGetCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> AccountAvailGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::PlaymovyPartnerReadonly`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> AccountAvailGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// List Avails owned or managed by the partner. See _Authentication and Authorization rules_ and _List methods rules_ for more information about this method.
///
/// A builder for the *avails.list* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_playmoviespartner1 as playmoviespartner1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use playmoviespartner1::PlayMovies;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = PlayMovies::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().avails_list("accountId")
///              .add_video_ids("et")
///              .title("et")
///              .add_territories("diam")
///              .add_studio_names("ipsum")
///              .add_pph_names("Lorem")
///              .page_token("et")
///              .page_size(-70)
///              .add_alt_ids("aliquyam")
///              .alt_id("sea")
///              .doit();
/// # }
/// ```
pub struct AccountAvailListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a PlayMovies<C, A>,
    _account_id: String,
    _video_ids: Vec<String>,
    _title: Option<String>,
    _territories: Vec<String>,
    _studio_names: Vec<String>,
    _pph_names: Vec<String>,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _alt_ids: Vec<String>,
    _alt_id: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for AccountAvailListCall<'a, C, A> {}

impl<'a, C, A> AccountAvailListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ListAvailsResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "playmoviespartner.accounts.avails.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((12 + self._additional_params.len()));
        params.push(("accountId", self._account_id.to_string()));
        if self._video_ids.len() > 0 {
            for f in self._video_ids.iter() {
                params.push(("videoIds", f.to_string()));
            }
        }
        if let Some(value) = self._title {
            params.push(("title", value.to_string()));
        }
        if self._territories.len() > 0 {
            for f in self._territories.iter() {
                params.push(("territories", f.to_string()));
            }
        }
        if self._studio_names.len() > 0 {
            for f in self._studio_names.iter() {
                params.push(("studioNames", f.to_string()));
            }
        }
        if self._pph_names.len() > 0 {
            for f in self._pph_names.iter() {
                params.push(("pphNames", f.to_string()));
            }
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        if self._alt_ids.len() > 0 {
            for f in self._alt_ids.iter() {
                params.push(("altIds", f.to_string()));
            }
        }
        if let Some(value) = self._alt_id {
            params.push(("altId", value.to_string()));
        }
        for &field in ["alt", "accountId", "videoIds", "title", "territories", "studioNames", "pphNames", "pageToken", "pageSize", "altIds", "altId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://playmoviespartner.googleapis.com/v1/accounts/{accountId}/avails".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::PlaymovyPartnerReadonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{accountId}", "accountId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["accountId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }



        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, &url)
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// REQUIRED. See _General rules_ for more information about this field.
    ///
    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn account_id(mut self, new_value: &str) -> AccountAvailListCall<'a, C, A> {
        self._account_id = new_value.to_string();
        self
    }
    /// Filter Avails that match any of the given `video_id`s.
    ///
    /// Append the given value to the *video ids* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_video_ids(mut self, new_value: &str) -> AccountAvailListCall<'a, C, A> {
        self._video_ids.push(new_value.to_string());
        self
    }
    /// Filter that matches Avails with a `title_internal_alias`, `series_title_internal_alias`, `season_title_internal_alias`, or `episode_title_internal_alias` that contains the given case-insensitive title.
    ///
    /// Sets the *title* query property to the given value.
    pub fn title(mut self, new_value: &str) -> AccountAvailListCall<'a, C, A> {
        self._title = Some(new_value.to_string());
        self
    }
    /// Filter Avails that match (case-insensitive) any of the given country codes, using the "ISO 3166-1 alpha-2" format (examples: "US", "us", "Us").
    ///
    /// Append the given value to the *territories* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_territories(mut self, new_value: &str) -> AccountAvailListCall<'a, C, A> {
        self._territories.push(new_value.to_string());
        self
    }
    /// See _List methods rules_ for info about this field.
    ///
    /// Append the given value to the *studio names* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_studio_names(mut self, new_value: &str) -> AccountAvailListCall<'a, C, A> {
        self._studio_names.push(new_value.to_string());
        self
    }
    /// See _List methods rules_ for info about this field.
    ///
    /// Append the given value to the *pph names* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_pph_names(mut self, new_value: &str) -> AccountAvailListCall<'a, C, A> {
        self._pph_names.push(new_value.to_string());
        self
    }
    /// See _List methods rules_ for info about this field.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> AccountAvailListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// See _List methods rules_ for info about this field.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> AccountAvailListCall<'a, C, A> {
        self._page_size = Some(new_value);
        self
    }
    /// Filter Avails that match (case-insensitive) any of the given partner-specific custom ids.
    ///
    /// Append the given value to the *alt ids* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_alt_ids(mut self, new_value: &str) -> AccountAvailListCall<'a, C, A> {
        self._alt_ids.push(new_value.to_string());
        self
    }
    /// Filter Avails that match a case-insensitive, partner-specific custom id. NOTE: this field is deprecated and will be removed on V2; `alt_ids` should be used instead.
    ///
    /// Sets the *alt id* query property to the given value.
    pub fn alt_id(mut self, new_value: &str) -> AccountAvailListCall<'a, C, A> {
        self._alt_id = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> AccountAvailListCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> AccountAvailListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::PlaymovyPartnerReadonly`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> AccountAvailListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// Get an ExperienceLocale given its id. See _Authentication and Authorization rules_ and _Get methods rules_ for more information about this method.
///
/// A builder for the *experienceLocales.get* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_playmoviespartner1 as playmoviespartner1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use playmoviespartner1::PlayMovies;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = PlayMovies::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().experience_locales_get("accountId", "elId")
///              .doit();
/// # }
/// ```
pub struct AccountExperienceLocaleGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a PlayMovies<C, A>,
    _account_id: String,
    _el_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for AccountExperienceLocaleGetCall<'a, C, A> {}

impl<'a, C, A> AccountExperienceLocaleGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ExperienceLocale)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "playmoviespartner.accounts.experienceLocales.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("accountId", self._account_id.to_string()));
        params.push(("elId", self._el_id.to_string()));
        for &field in ["alt", "accountId", "elId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://playmoviespartner.googleapis.com/v1/accounts/{accountId}/experienceLocales/{elId}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::PlaymovyPartnerReadonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{accountId}", "accountId"), ("{elId}", "elId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["elId", "accountId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }



        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, &url)
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// REQUIRED. See _General rules_ for more information about this field.
    ///
    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn account_id(mut self, new_value: &str) -> AccountExperienceLocaleGetCall<'a, C, A> {
        self._account_id = new_value.to_string();
        self
    }
    /// REQUIRED. ExperienceLocale ID, as defined by Google.
    ///
    /// Sets the *el id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn el_id(mut self, new_value: &str) -> AccountExperienceLocaleGetCall<'a, C, A> {
        self._el_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> AccountExperienceLocaleGetCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> AccountExperienceLocaleGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::PlaymovyPartnerReadonly`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> AccountExperienceLocaleGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// List StoreInfos owned or managed by the partner. See _Authentication and Authorization rules_ and _List methods rules_ for more information about this method.
///
/// A builder for the *storeInfos.list* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_playmoviespartner1 as playmoviespartner1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use playmoviespartner1::PlayMovies;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = PlayMovies::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().store_infos_list("accountId")
///              .add_video_ids("sadipscing")
///              .video_id("dolor")
///              .add_studio_names("eirmod")
///              .add_season_ids("elitr")
///              .add_pph_names("amet")
///              .page_token("no")
///              .page_size(-36)
///              .name("eirmod")
///              .add_mids("dolore")
///              .add_countries("invidunt")
///              .doit();
/// # }
/// ```
pub struct AccountStoreInfoListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a PlayMovies<C, A>,
    _account_id: String,
    _video_ids: Vec<String>,
    _video_id: Option<String>,
    _studio_names: Vec<String>,
    _season_ids: Vec<String>,
    _pph_names: Vec<String>,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _name: Option<String>,
    _mids: Vec<String>,
    _countries: Vec<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for AccountStoreInfoListCall<'a, C, A> {}

impl<'a, C, A> AccountStoreInfoListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ListStoreInfosResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "playmoviespartner.accounts.storeInfos.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((13 + self._additional_params.len()));
        params.push(("accountId", self._account_id.to_string()));
        if self._video_ids.len() > 0 {
            for f in self._video_ids.iter() {
                params.push(("videoIds", f.to_string()));
            }
        }
        if let Some(value) = self._video_id {
            params.push(("videoId", value.to_string()));
        }
        if self._studio_names.len() > 0 {
            for f in self._studio_names.iter() {
                params.push(("studioNames", f.to_string()));
            }
        }
        if self._season_ids.len() > 0 {
            for f in self._season_ids.iter() {
                params.push(("seasonIds", f.to_string()));
            }
        }
        if self._pph_names.len() > 0 {
            for f in self._pph_names.iter() {
                params.push(("pphNames", f.to_string()));
            }
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        if let Some(value) = self._name {
            params.push(("name", value.to_string()));
        }
        if self._mids.len() > 0 {
            for f in self._mids.iter() {
                params.push(("mids", f.to_string()));
            }
        }
        if self._countries.len() > 0 {
            for f in self._countries.iter() {
                params.push(("countries", f.to_string()));
            }
        }
        for &field in ["alt", "accountId", "videoIds", "videoId", "studioNames", "seasonIds", "pphNames", "pageToken", "pageSize", "name", "mids", "countries"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://playmoviespartner.googleapis.com/v1/accounts/{accountId}/storeInfos".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::PlaymovyPartnerReadonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{accountId}", "accountId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["accountId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }



        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, &url)
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// REQUIRED. See _General rules_ for more information about this field.
    ///
    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn account_id(mut self, new_value: &str) -> AccountStoreInfoListCall<'a, C, A> {
        self._account_id = new_value.to_string();
        self
    }
    /// Filter StoreInfos that match any of the given `video_id`s.
    ///
    /// Append the given value to the *video ids* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_video_ids(mut self, new_value: &str) -> AccountStoreInfoListCall<'a, C, A> {
        self._video_ids.push(new_value.to_string());
        self
    }
    /// Filter StoreInfos that match a given `video_id`. NOTE: this field is deprecated and will be removed on V2; `video_ids` should be used instead.
    ///
    /// Sets the *video id* query property to the given value.
    pub fn video_id(mut self, new_value: &str) -> AccountStoreInfoListCall<'a, C, A> {
        self._video_id = Some(new_value.to_string());
        self
    }
    /// See _List methods rules_ for info about this field.
    ///
    /// Append the given value to the *studio names* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_studio_names(mut self, new_value: &str) -> AccountStoreInfoListCall<'a, C, A> {
        self._studio_names.push(new_value.to_string());
        self
    }
    /// Filter StoreInfos that match any of the given `season_id`s.
    ///
    /// Append the given value to the *season ids* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_season_ids(mut self, new_value: &str) -> AccountStoreInfoListCall<'a, C, A> {
        self._season_ids.push(new_value.to_string());
        self
    }
    /// See _List methods rules_ for info about this field.
    ///
    /// Append the given value to the *pph names* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_pph_names(mut self, new_value: &str) -> AccountStoreInfoListCall<'a, C, A> {
        self._pph_names.push(new_value.to_string());
        self
    }
    /// See _List methods rules_ for info about this field.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> AccountStoreInfoListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// See _List methods rules_ for info about this field.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> AccountStoreInfoListCall<'a, C, A> {
        self._page_size = Some(new_value);
        self
    }
    /// Filter that matches StoreInfos with a `name` or `show_name` that contains the given case-insensitive name.
    ///
    /// Sets the *name* query property to the given value.
    pub fn name(mut self, new_value: &str) -> AccountStoreInfoListCall<'a, C, A> {
        self._name = Some(new_value.to_string());
        self
    }
    /// Filter StoreInfos that match any of the given `mid`s.
    ///
    /// Append the given value to the *mids* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_mids(mut self, new_value: &str) -> AccountStoreInfoListCall<'a, C, A> {
        self._mids.push(new_value.to_string());
        self
    }
    /// Filter StoreInfos that match (case-insensitive) any of the given country codes, using the "ISO 3166-1 alpha-2" format (examples: "US", "us", "Us").
    ///
    /// Append the given value to the *countries* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_countries(mut self, new_value: &str) -> AccountStoreInfoListCall<'a, C, A> {
        self._countries.push(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> AccountStoreInfoListCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> AccountStoreInfoListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::PlaymovyPartnerReadonly`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> AccountStoreInfoListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// List ExperienceLocales owned or managed by the partner. See _Authentication and Authorization rules_ and _List methods rules_ for more information about this method.
///
/// A builder for the *experienceLocales.list* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_playmoviespartner1 as playmoviespartner1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use playmoviespartner1::PlayMovies;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = PlayMovies::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().experience_locales_list("accountId")
///              .title_level_eidr("accusam")
///              .add_studio_names("Lorem")
///              .add_status("sea")
///              .add_pph_names("et")
///              .page_token("duo")
///              .page_size(-21)
///              .edit_level_eidr("eirmod")
///              .custom_id("sanctus")
///              .alt_cut_id("et")
///              .doit();
/// # }
/// ```
pub struct AccountExperienceLocaleListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a PlayMovies<C, A>,
    _account_id: String,
    _title_level_eidr: Option<String>,
    _studio_names: Vec<String>,
    _status: Vec<String>,
    _pph_names: Vec<String>,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _edit_level_eidr: Option<String>,
    _custom_id: Option<String>,
    _alt_cut_id: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for AccountExperienceLocaleListCall<'a, C, A> {}

impl<'a, C, A> AccountExperienceLocaleListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ListExperienceLocalesResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "playmoviespartner.accounts.experienceLocales.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((12 + self._additional_params.len()));
        params.push(("accountId", self._account_id.to_string()));
        if let Some(value) = self._title_level_eidr {
            params.push(("titleLevelEidr", value.to_string()));
        }
        if self._studio_names.len() > 0 {
            for f in self._studio_names.iter() {
                params.push(("studioNames", f.to_string()));
            }
        }
        if self._status.len() > 0 {
            for f in self._status.iter() {
                params.push(("status", f.to_string()));
            }
        }
        if self._pph_names.len() > 0 {
            for f in self._pph_names.iter() {
                params.push(("pphNames", f.to_string()));
            }
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        if let Some(value) = self._edit_level_eidr {
            params.push(("editLevelEidr", value.to_string()));
        }
        if let Some(value) = self._custom_id {
            params.push(("customId", value.to_string()));
        }
        if let Some(value) = self._alt_cut_id {
            params.push(("altCutId", value.to_string()));
        }
        for &field in ["alt", "accountId", "titleLevelEidr", "studioNames", "status", "pphNames", "pageToken", "pageSize", "editLevelEidr", "customId", "altCutId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://playmoviespartner.googleapis.com/v1/accounts/{accountId}/experienceLocales".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::PlaymovyPartnerReadonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{accountId}", "accountId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["accountId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }



        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, &url)
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// REQUIRED. See _General rules_ for more information about this field.
    ///
    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn account_id(mut self, new_value: &str) -> AccountExperienceLocaleListCall<'a, C, A> {
        self._account_id = new_value.to_string();
        self
    }
    /// Filter ExperienceLocales that match a given title-level EIDR.
    ///
    /// Sets the *title level eidr* query property to the given value.
    pub fn title_level_eidr(mut self, new_value: &str) -> AccountExperienceLocaleListCall<'a, C, A> {
        self._title_level_eidr = Some(new_value.to_string());
        self
    }
    /// See _List methods rules_ for info about this field.
    ///
    /// Append the given value to the *studio names* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_studio_names(mut self, new_value: &str) -> AccountExperienceLocaleListCall<'a, C, A> {
        self._studio_names.push(new_value.to_string());
        self
    }
    /// Filter ExperienceLocales that match one of the given status.
    ///
    /// Append the given value to the *status* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_status(mut self, new_value: &str) -> AccountExperienceLocaleListCall<'a, C, A> {
        self._status.push(new_value.to_string());
        self
    }
    /// See _List methods rules_ for info about this field.
    ///
    /// Append the given value to the *pph names* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_pph_names(mut self, new_value: &str) -> AccountExperienceLocaleListCall<'a, C, A> {
        self._pph_names.push(new_value.to_string());
        self
    }
    /// See _List methods rules_ for info about this field.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> AccountExperienceLocaleListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// See _List methods rules_ for info about this field.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> AccountExperienceLocaleListCall<'a, C, A> {
        self._page_size = Some(new_value);
        self
    }
    /// Filter ExperienceLocales that match a given edit-level EIDR.
    ///
    /// Sets the *edit level eidr* query property to the given value.
    pub fn edit_level_eidr(mut self, new_value: &str) -> AccountExperienceLocaleListCall<'a, C, A> {
        self._edit_level_eidr = Some(new_value.to_string());
        self
    }
    /// Filter ExperienceLocales that match a case-insensitive, partner-specific custom id.
    ///
    /// Sets the *custom id* query property to the given value.
    pub fn custom_id(mut self, new_value: &str) -> AccountExperienceLocaleListCall<'a, C, A> {
        self._custom_id = Some(new_value.to_string());
        self
    }
    /// Filter ExperienceLocales that match a case-insensitive, partner-specific Alternative Cut ID.
    ///
    /// Sets the *alt cut id* query property to the given value.
    pub fn alt_cut_id(mut self, new_value: &str) -> AccountExperienceLocaleListCall<'a, C, A> {
        self._alt_cut_id = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> AccountExperienceLocaleListCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> AccountExperienceLocaleListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::PlaymovyPartnerReadonly`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> AccountExperienceLocaleListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// List Components owned or managed by the partner. See _Authentication and Authorization rules_ and _List methods rules_ for more information about this method.
///
/// A builder for the *components.list* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_playmoviespartner1 as playmoviespartner1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use playmoviespartner1::PlayMovies;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = PlayMovies::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().components_list("accountId")
///              .title_level_eidr("et")
///              .add_studio_names("consetetur")
///              .add_status("ut")
///              .presentation_id("ea")
///              .add_pph_names("sed")
///              .playable_sequence_id("dolor")
///              .page_token("dolor")
///              .page_size(-48)
///              .inventory_id("et")
///              .filename("consetetur")
///              .el_id("amet.")
///              .edit_level_eidr("voluptua.")
///              .custom_id("Lorem")
///              .alt_cut_id("gubergren")
///              .doit();
/// # }
/// ```
pub struct AccountComponentListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a PlayMovies<C, A>,
    _account_id: String,
    _title_level_eidr: Option<String>,
    _studio_names: Vec<String>,
    _status: Vec<String>,
    _presentation_id: Option<String>,
    _pph_names: Vec<String>,
    _playable_sequence_id: Option<String>,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _inventory_id: Option<String>,
    _filename: Option<String>,
    _el_id: Option<String>,
    _edit_level_eidr: Option<String>,
    _custom_id: Option<String>,
    _alt_cut_id: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for AccountComponentListCall<'a, C, A> {}

impl<'a, C, A> AccountComponentListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ListComponentsResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "playmoviespartner.accounts.components.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((17 + self._additional_params.len()));
        params.push(("accountId", self._account_id.to_string()));
        if let Some(value) = self._title_level_eidr {
            params.push(("titleLevelEidr", value.to_string()));
        }
        if self._studio_names.len() > 0 {
            for f in self._studio_names.iter() {
                params.push(("studioNames", f.to_string()));
            }
        }
        if self._status.len() > 0 {
            for f in self._status.iter() {
                params.push(("status", f.to_string()));
            }
        }
        if let Some(value) = self._presentation_id {
            params.push(("presentationId", value.to_string()));
        }
        if self._pph_names.len() > 0 {
            for f in self._pph_names.iter() {
                params.push(("pphNames", f.to_string()));
            }
        }
        if let Some(value) = self._playable_sequence_id {
            params.push(("playableSequenceId", value.to_string()));
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        if let Some(value) = self._inventory_id {
            params.push(("inventoryId", value.to_string()));
        }
        if let Some(value) = self._filename {
            params.push(("filename", value.to_string()));
        }
        if let Some(value) = self._el_id {
            params.push(("elId", value.to_string()));
        }
        if let Some(value) = self._edit_level_eidr {
            params.push(("editLevelEidr", value.to_string()));
        }
        if let Some(value) = self._custom_id {
            params.push(("customId", value.to_string()));
        }
        if let Some(value) = self._alt_cut_id {
            params.push(("altCutId", value.to_string()));
        }
        for &field in ["alt", "accountId", "titleLevelEidr", "studioNames", "status", "presentationId", "pphNames", "playableSequenceId", "pageToken", "pageSize", "inventoryId", "filename", "elId", "editLevelEidr", "customId", "altCutId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://playmoviespartner.googleapis.com/v1/accounts/{accountId}/components".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::PlaymovyPartnerReadonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{accountId}", "accountId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["accountId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }



        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, &url)
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// REQUIRED. See _General rules_ for more information about this field.
    ///
    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn account_id(mut self, new_value: &str) -> AccountComponentListCall<'a, C, A> {
        self._account_id = new_value.to_string();
        self
    }
    /// Filter Components that match a given title-level EIDR.
    ///
    /// Sets the *title level eidr* query property to the given value.
    pub fn title_level_eidr(mut self, new_value: &str) -> AccountComponentListCall<'a, C, A> {
        self._title_level_eidr = Some(new_value.to_string());
        self
    }
    /// See _List methods rules_ for info about this field.
    ///
    /// Append the given value to the *studio names* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_studio_names(mut self, new_value: &str) -> AccountComponentListCall<'a, C, A> {
        self._studio_names.push(new_value.to_string());
        self
    }
    /// Filter Components that match one of the given status.
    ///
    /// Append the given value to the *status* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_status(mut self, new_value: &str) -> AccountComponentListCall<'a, C, A> {
        self._status.push(new_value.to_string());
        self
    }
    /// PresentationID available in Common Manifest.
    ///
    /// Sets the *presentation id* query property to the given value.
    pub fn presentation_id(mut self, new_value: &str) -> AccountComponentListCall<'a, C, A> {
        self._presentation_id = Some(new_value.to_string());
        self
    }
    /// See _List methods rules_ for info about this field.
    ///
    /// Append the given value to the *pph names* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_pph_names(mut self, new_value: &str) -> AccountComponentListCall<'a, C, A> {
        self._pph_names.push(new_value.to_string());
        self
    }
    /// PlayableSequenceID available in Common Manifest.
    ///
    /// Sets the *playable sequence id* query property to the given value.
    pub fn playable_sequence_id(mut self, new_value: &str) -> AccountComponentListCall<'a, C, A> {
        self._playable_sequence_id = Some(new_value.to_string());
        self
    }
    /// See _List methods rules_ for info about this field.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> AccountComponentListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// See _List methods rules_ for info about this field.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> AccountComponentListCall<'a, C, A> {
        self._page_size = Some(new_value);
        self
    }
    /// InventoryID available in Common Manifest.
    ///
    /// Sets the *inventory id* query property to the given value.
    pub fn inventory_id(mut self, new_value: &str) -> AccountComponentListCall<'a, C, A> {
        self._inventory_id = Some(new_value.to_string());
        self
    }
    /// Filter Components that match a case-insensitive substring of the physical name of the delivered file.
    ///
    /// Sets the *filename* query property to the given value.
    pub fn filename(mut self, new_value: &str) -> AccountComponentListCall<'a, C, A> {
        self._filename = Some(new_value.to_string());
        self
    }
    /// Experience ID, as defined by Google.
    ///
    /// Sets the *el id* query property to the given value.
    pub fn el_id(mut self, new_value: &str) -> AccountComponentListCall<'a, C, A> {
        self._el_id = Some(new_value.to_string());
        self
    }
    /// Filter Components that match a given edit-level EIDR.
    ///
    /// Sets the *edit level eidr* query property to the given value.
    pub fn edit_level_eidr(mut self, new_value: &str) -> AccountComponentListCall<'a, C, A> {
        self._edit_level_eidr = Some(new_value.to_string());
        self
    }
    /// Filter Components that match a case-insensitive partner-specific custom id.
    ///
    /// Sets the *custom id* query property to the given value.
    pub fn custom_id(mut self, new_value: &str) -> AccountComponentListCall<'a, C, A> {
        self._custom_id = Some(new_value.to_string());
        self
    }
    /// Filter Components that match a case-insensitive, partner-specific Alternative Cut ID.
    ///
    /// Sets the *alt cut id* query property to the given value.
    pub fn alt_cut_id(mut self, new_value: &str) -> AccountComponentListCall<'a, C, A> {
        self._alt_cut_id = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> AccountComponentListCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> AccountComponentListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::PlaymovyPartnerReadonly`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> AccountComponentListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// Get a Component given its id.
///
/// A builder for the *components.type.get* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_playmoviespartner1 as playmoviespartner1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use playmoviespartner1::PlayMovies;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = PlayMovies::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().components_type_get("accountId", "componentId", "type")
///              .doit();
/// # }
/// ```
pub struct AccountComponentTypeGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a PlayMovies<C, A>,
    _account_id: String,
    _component_id: String,
    _type_: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for AccountComponentTypeGetCall<'a, C, A> {}

impl<'a, C, A> AccountComponentTypeGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Component)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "playmoviespartner.accounts.components.type.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        params.push(("accountId", self._account_id.to_string()));
        params.push(("componentId", self._component_id.to_string()));
        params.push(("type", self._type_.to_string()));
        for &field in ["alt", "accountId", "componentId", "type"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://playmoviespartner.googleapis.com/v1/accounts/{accountId}/components/{componentId}/type/{type}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::PlaymovyPartnerReadonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{accountId}", "accountId"), ("{componentId}", "componentId"), ("{type}", "type")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(3);
            for param_name in ["type", "componentId", "accountId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }



        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, &url)
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// REQUIRED. See _General rules_ for more information about this field.
    ///
    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn account_id(mut self, new_value: &str) -> AccountComponentTypeGetCall<'a, C, A> {
        self._account_id = new_value.to_string();
        self
    }
    /// REQUIRED. Component ID.
    ///
    /// Sets the *component id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn component_id(mut self, new_value: &str) -> AccountComponentTypeGetCall<'a, C, A> {
        self._component_id = new_value.to_string();
        self
    }
    /// REQUIRED. Component Type.
    ///
    /// Sets the *type* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn type_(mut self, new_value: &str) -> AccountComponentTypeGetCall<'a, C, A> {
        self._type_ = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> AccountComponentTypeGetCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> AccountComponentTypeGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::PlaymovyPartnerReadonly`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> AccountComponentTypeGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


