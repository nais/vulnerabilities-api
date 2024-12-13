# Component

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**author** | Option<**String**> |  | [optional]
**publisher** | Option<**String**> |  | [optional]
**supplier** | Option<[**models::OrganizationalEntity**](OrganizationalEntity.md)> |  | [optional]
**group** | Option<**String**> |  | [optional]
**name** | Option<**String**> |  | [optional]
**version** | Option<**String**> |  | [optional]
**classifier** | Option<**String**> |  | [optional]
**filename** | Option<**String**> |  | [optional]
**extension** | Option<**String**> |  | [optional]
**md5** | Option<**String**> |  | [optional]
**sha1** | Option<**String**> |  | [optional]
**sha256** | Option<**String**> |  | [optional]
**sha384** | Option<**String**> |  | [optional]
**sha512** | Option<**String**> |  | [optional]
**sha3_256** | Option<**String**> |  | [optional]
**sha3_384** | Option<**String**> |  | [optional]
**sha3_512** | Option<**String**> |  | [optional]
**blake2b_256** | Option<**String**> |  | [optional]
**blake2b_384** | Option<**String**> |  | [optional]
**blake2b_512** | Option<**String**> |  | [optional]
**blake3** | Option<**String**> |  | [optional]
**cpe** | Option<**String**> |  | [optional]
**purl** | Option<**String**> |  | [optional]
**purl_coordinates** | Option<**String**> |  | [optional][readonly]
**swid_tag_id** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**copyright** | Option<**String**> |  | [optional]
**license** | Option<**String**> |  | [optional]
**license_expression** | Option<**String**> |  | [optional]
**license_url** | Option<**String**> |  | [optional]
**resolved_license** | Option<[**models::License**](License.md)> |  | [optional]
**direct_dependencies** | Option<**String**> |  | [optional]
**external_references** | Option<[**Vec<models::ExternalReference>**](ExternalReference.md)> |  | [optional]
**parent** | Option<[**models::Component**](Component.md)> |  | [optional]
**children** | Option<[**Vec<models::Component>**](Component.md)> |  | [optional]
**properties** | Option<[**Vec<models::ComponentProperty>**](ComponentProperty.md)> |  | [optional]
**vulnerabilities** | Option<[**Vec<models::Vulnerability>**](Vulnerability.md)> |  | [optional]
**project** | [**models::Project**](Project.md) |  | 
**last_inherited_risk_score** | Option<**f64**> |  | [optional]
**notes** | Option<**String**> |  | [optional]
**uuid** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**metrics** | Option<[**models::DependencyMetrics**](DependencyMetrics.md)> |  | [optional]
**repository_meta** | Option<[**models::RepositoryMetaComponent**](RepositoryMetaComponent.md)> |  | [optional]
**dependency_graph** | Option<**Vec<String>**> |  | [optional]
**expand_dependency_graph** | Option<**bool**> |  | [optional]
**is_internal** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


