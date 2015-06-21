initSidebarItems({"enum":[["Error",""],["GeoJsonGeometry","There is no detailed description."],["MapItem","There is no detailed description."],["Scope","Identifies the an OAuth2 authorization scope. A scope is needed when requesting an authorization token."]],"type":[["Result","A universal result type used as return for all calls."]],"trait":[["CallBuilder","Identifies types which represent builders for a particular resource method"],["Delegate","A trait specifying functionality to help controlling any request performed by the API. The trait has a conservative default implementation."],["Hub","Identifies the Hub. There is only one per library, this trait is supposed to make intended use more explicit. The hub allows to access all resource methods more easily."],["MethodsBuilder","Identifies types for building methods of a particular resource type"],["NestedType","Identifies types which are only used by other types internally. They have no special meaning, this trait just marks them for completeness."],["Part","Identifies types which are only used as part of other types, which usually are carrying the `Resource` trait."],["ReadSeek","A utility to specify reader types which provide seeking capabilities too"],["RequestValue","Identifies types which are used in API requests."],["Resource","Identifies types which can be inserted and deleted. Types with this trait are most commonly used by clients of this API."],["ResponseResult","Identifies types which are used in API responses."],["ToParts","A trait for all types that can convert themselves into a *parts* string"]],"struct":[["AcquisitionTime","Acquisition time represents acquired time of a raster."],["Asset","An asset is any Google Maps Engine resource that has a globally unique ID. Assets include maps, layers, vector tables, raster collections, and rasters. Projects and features are not considered assets."],["AssetGetCall","Return metadata for a particular asset."],["AssetListCall","Return all assets readable by the current user."],["AssetMethods","A builder providing access to all methods supported on *asset* resources. It is not used directly, but through the `MapsEngine` hub."],["AssetParentListCall","Return all parent ids of the specified asset."],["AssetPermissionListCall","Return all of the permissions for the specified asset."],["AssetsListResponse","The response returned by a call to resources.List."],["Border","Border in line style. Both color and width are required."],["Color","Basic color used in styling."],["Datasource","There is no detailed description."],["Datasources","There is no detailed description."],["DefaultDelegate","A delegate with a conservative default implementation, which is used if no other delegate is set."],["DisplayRule","A display rule of the vector style."],["ErrorResponse","A utility to represent detailed errors we might see in case there are BadRequests. The latter happen if the sent parameters or request structures are unsound"],["Feature","A feature within a table."],["FeatureInfo","A feature info contains information about individual feature."],["FeaturesBatchDeleteRequest","The request sent to features.BatchDelete."],["FeaturesBatchInsertRequest","The request sent to features.Insert."],["FeaturesBatchPatchRequest","The request sent to features.BatchPatch."],["FeaturesListResponse","The response returned by a call to features.List."],["File","A single File, which is a component of an Asset."],["Filter","Conditions for filtering features."],["GeoJsonGeometryCollection","A heterogenous collection of GeoJsonGeometry objects."],["GeoJsonLineString","There is no detailed description."],["GeoJsonMultiLineString","Multi Line String"],["GeoJsonMultiPoint","There is no detailed description."],["GeoJsonMultiPolygon","There is no detailed description."],["GeoJsonPoint","There is no detailed description."],["GeoJsonPolygon","There is no detailed description."],["GeoJsonPosition","A position represents a geographical position as an array containing a longitude and a latitude, and optionally an altitude, in that order. All Geometry objects make use of positions to represent geometries as nested arrays. The structure of the array is governed by the type of the geometry."],["GeoJsonProperties","The properties associated with a feature."],["Icon","An icon is a user-uploaded image that can be used to style point geometries."],["IconStyle","Style for icon, this is part of point style."],["IconsListResponse","The response returned by a call to icons.List."],["LabelStyle","Text label style."],["LatLngBox","A rectangular geographic bounds."],["Layer","A Layer combines multiple datasources, with styling information, for presentation on a map."],["LayerCancelProcessingCall","Cancel processing on a layer asset."],["LayerCreateCall","Create a layer asset."],["LayerDeleteCall","Delete a layer."],["LayerGetCall","Return metadata for a particular layer."],["LayerGetPublishedCall","Return the published metadata for a particular layer."],["LayerListCall","Return all layers readable by the current user."],["LayerListPublishedCall","Return all published layers readable by the current user."],["LayerMethods","A builder providing access to all methods supported on *layer* resources. It is not used directly, but through the `MapsEngine` hub."],["LayerParentListCall","Return all parent ids of the specified layer."],["LayerPatchCall","Mutate a layer asset."],["LayerPermissionBatchDeleteCall","Remove permission entries from an already existing asset."],["LayerPermissionBatchUpdateCall","Add or update permission entries to an already existing asset."],["LayerPermissionListCall","Return all of the permissions for the specified asset."],["LayerProcesCall","Process a layer asset."],["LayerPublishCall","Publish a layer asset."],["LayerUnpublishCall","Unpublish a layer asset."],["LayersListResponse","The response returned by a call to layers.List. Note: The list response does not include all the fields available in a layer. Refer to the layer resource description for details of the fields that are not included. You'll need to send a get request to retrieve the additional fields for each layer."],["LineStyle","Style for lines."],["LineStyleStroke","Stroke of the line."],["Map","A Map is a collection of Layers, optionally contained within folders."],["MapContents","There is no detailed description."],["MapCreateCall","Create a map asset."],["MapDeleteCall","Delete a map."],["MapFolder","There is no detailed description."],["MapGetCall","Return metadata for a particular map."],["MapGetPublishedCall","Return the published metadata for a particular map."],["MapKmlLink","There is no detailed description."],["MapLayer","There is no detailed description."],["MapListCall","Return all maps readable by the current user."],["MapListPublishedCall","Return all published maps readable by the current user."],["MapMethods","A builder providing access to all methods supported on *map* resources. It is not used directly, but through the `MapsEngine` hub."],["MapPatchCall","Mutate a map asset."],["MapPermissionBatchDeleteCall","Remove permission entries from an already existing asset."],["MapPermissionBatchUpdateCall","Add or update permission entries to an already existing asset."],["MapPermissionListCall","Return all of the permissions for the specified asset."],["MapPublishCall","Publish a map asset."],["MapUnpublishCall","Unpublish a map asset."],["MapsEngine","Central instance to access all MapsEngine related resource activities"],["MapsListResponse","The response returned by a call to maps.List."],["MethodInfo","Contains information about an API request."],["MultiPartReader","Provides a `Read` interface that converts multiple parts into the protocol identified by RFC2387. **Note**: This implementation is just as rich as it needs to be to perform uploads to google APIs, and might not be a fully-featured implementation."],["Parent","A list of the parents of an asset."],["ParentsListResponse","The response returned by a call to parents.List."],["Permission","A permission defines the user or group that has access to an asset, and the type of access they have."],["PermissionsBatchDeleteRequest","The request sent to mapsengine.permissions.batchDelete."],["PermissionsBatchDeleteResponse","The response returned by a call to mapsengine.permissions.batchDelete."],["PermissionsBatchUpdateRequest","The request sent to mapsengine.permissions.batchUpdate."],["PermissionsBatchUpdateResponse","The response returned by a call to mapsengine.permissions.batchUpdate."],["PermissionsListResponse","There is no detailed description."],["PointStyle","Style for points."],["PolygonStyle","Style for polygons."],["ProcessResponse","The response returned by a call to any asset's Process method."],["Project","A Maps Engine project groups a collection of resources."],["ProjectIconCreateCall","Create an icon."],["ProjectIconGetCall","Return an icon or its associated metadata"],["ProjectIconListCall","Return all icons in the current project"],["ProjectListCall","Return all projects readable by the current user."],["ProjectMethods","A builder providing access to all methods supported on *project* resources. It is not used directly, but through the `MapsEngine` hub."],["ProjectsListResponse","The response returned by a call to projects.List."],["PublishResponse","The response returned by a call to any asset's Publish method."],["PublishedLayer","The published version of a layer."],["PublishedLayersListResponse","The response returned by a call to layers.List.published."],["PublishedMap","The published version of a map asset."],["PublishedMapsListResponse","The response returned by a call to maps.List.published."],["Raster","A geo-referenced raster."],["RasterCollection","A raster collection groups multiple Raster resources for inclusion in a Layer."],["RasterCollectionCancelProcessingCall","Cancel processing on a raster collection asset."],["RasterCollectionCreateCall","Create a raster collection asset."],["RasterCollectionDeleteCall","Delete a raster collection."],["RasterCollectionGetCall","Return metadata for a particular raster collection."],["RasterCollectionListCall","Return all raster collections readable by the current user."],["RasterCollectionMethods","A builder providing access to all methods supported on *rasterCollection* resources. It is not used directly, but through the `MapsEngine` hub."],["RasterCollectionParentListCall","Return all parent ids of the specified raster collection."],["RasterCollectionPatchCall","Mutate a raster collection asset."],["RasterCollectionPermissionBatchDeleteCall","Remove permission entries from an already existing asset."],["RasterCollectionPermissionBatchUpdateCall","Add or update permission entries to an already existing asset."],["RasterCollectionPermissionListCall","Return all of the permissions for the specified asset."],["RasterCollectionProcesCall","Process a raster collection asset."],["RasterCollectionRasterBatchDeleteCall","Remove rasters from an existing raster collection."],["RasterCollectionRasterBatchInsertCall","Add rasters to an existing raster collection. Rasters must be successfully processed in order to be added to a raster collection."],["RasterCollectionRasterListCall","Return all rasters within a raster collection."],["RasterCollectionsListResponse","The response returned by a call to raster_collections.List. Note: The list response does not include all the fields available in a raster collection. Refer to the RasterCollection resource description for details of the fields that are not included. You'll need to send a get request to retrieve the additional fields for each raster collection."],["RasterCollectionsRaster","A raster resource."],["RasterCollectionsRasterBatchDeleteRequest","The request sent to rasterCollections.Rasters.BatchDelete."],["RasterCollectionsRastersBatchDeleteResponse","The response returned by a call to rasterCollections.rasters.batchDelete."],["RasterCollectionsRastersBatchInsertRequest","The request sent to rasterCollections.Rasters.BatchInsert."],["RasterCollectionsRastersBatchInsertResponse","The response returned by a call to rasterCollections.rasters.batchInsert."],["RasterCollectionsRastersListResponse","The response returned by a call to rasterCollections.rasters.List."],["RasterDeleteCall","Delete a raster."],["RasterFileInsertCall","Upload a file to a raster asset."],["RasterGetCall","Return metadata for a single raster."],["RasterListCall","Return all rasters readable by the current user."],["RasterMethods","A builder providing access to all methods supported on *raster* resources. It is not used directly, but through the `MapsEngine` hub."],["RasterParentListCall","Return all parent ids of the specified rasters."],["RasterPatchCall","Mutate a raster asset."],["RasterPermissionBatchDeleteCall","Remove permission entries from an already existing asset."],["RasterPermissionBatchUpdateCall","Add or update permission entries to an already existing asset."],["RasterPermissionListCall","Return all of the permissions for the specified asset."],["RasterProcesCall","Process a raster asset."],["RasterUploadCall","Create a skeleton raster asset for upload."],["RastersListResponse","The response returned by a call to rasters.List."],["ScaledShape","Parameters for styling points as scaled shapes."],["ScalingFunction","Parameters for scaling scaled shapes."],["Schema","A schema indicating the properties which may be associated with features within a Table, and the types of those properties."],["SizeRange","Scaled shape size range in pixels. For circles, size corresponds to diameter."],["Table","A collection of geographic features, and associated metadata."],["TableColumn","There is no detailed description."],["TableCreateCall","Create a table asset."],["TableDeleteCall","Delete a table."],["TableFeatureBatchDeleteCall","Delete all features matching the given IDs."],["TableFeatureBatchInsertCall","Append features to an existing table."],["TableFeatureBatchPatchCall","Update the supplied features."],["TableFeatureGetCall","Return a single feature, given its ID."],["TableFeatureListCall","Return all features readable by the current user."],["TableFileInsertCall","Upload a file to a placeholder table asset. See Table Upload in the Developer's Guide for more information. Supported file types are listed in the Supported data formats and limits article of the Google Maps Engine help center."],["TableGetCall","Return metadata for a particular table, including the schema."],["TableListCall","Return all tables readable by the current user."],["TableMethods","A builder providing access to all methods supported on *table* resources. It is not used directly, but through the `MapsEngine` hub."],["TableParentListCall","Return all parent ids of the specified table."],["TablePatchCall","Mutate a table asset."],["TablePermissionBatchDeleteCall","Remove permission entries from an already existing asset."],["TablePermissionBatchUpdateCall","Add or update permission entries to an already existing asset."],["TablePermissionListCall","Return all of the permissions for the specified asset."],["TableProcesCall","Process a table asset."],["TableUploadCall","Create a placeholder table asset to which table files can be uploaded. Once the placeholder has been created, files are uploaded to the https://www.googleapis.com/upload/mapsengine/v1/tables/table_id/files endpoint. See Table Upload in the Developer's Guide or Table.files: insert in the reference documentation for more information."],["TablesListResponse","The response returned by a call to tables.List. Note: The list response does not include all the fields available in a table. Refer to the table resource description for details of the fields that are not included. You'll need to send a get request to retrieve the additional fields for each table."],["Tags","There is no detailed description."],["ValueRange","Range of values used for scaling shapes. The min/max values will be drawn as shapes with the min/max size."],["VectorStyle","A vector style contains styling information for vector layer."],["ZoomLevels","Zoom level range. Zoom levels are restricted between 0 and 24, inclusive."]],"fn":[["remove_json_null_values",""]]});