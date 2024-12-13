# Project

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**author** | Option<**String**> |  | [optional]
**publisher** | Option<**String**> |  | [optional]
**manufacturer** | Option<[**models::OrganizationalEntity**](OrganizationalEntity.md)> |  | [optional]
**supplier** | Option<[**models::OrganizationalEntity**](OrganizationalEntity.md)> |  | [optional]
**group** | Option<**String**> |  | [optional]
**name** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**version** | Option<**String**> |  | [optional]
**classifier** | Option<**String**> |  | [optional]
**cpe** | Option<**String**> |  | [optional]
**purl** | Option<**String**> |  | [optional]
**swid_tag_id** | Option<**String**> |  | [optional]
**direct_dependencies** | Option<**String**> |  | [optional]
**uuid** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**parent** | Option<[**models::Project**](Project.md)> |  | [optional]
**children** | Option<[**Vec<models::Project>**](Project.md)> |  | [optional]
**properties** | Option<[**Vec<models::ProjectProperty>**](ProjectProperty.md)> |  | [optional]
**tags** | Option<[**Vec<models::Tag>**](Tag.md)> |  | [optional]
**last_bom_import** | Option<**String**> |  | [optional]
**last_bom_import_format** | Option<**String**> |  | [optional]
**last_inherited_risk_score** | Option<**f64**> |  | [optional]
**active** | Option<**bool**> |  | [optional]
**external_references** | Option<[**Vec<models::ExternalReference>**](ExternalReference.md)> |  | [optional]
**metadata** | Option<[**models::ProjectMetadata**](ProjectMetadata.md)> |  | [optional]
**versions** | Option<[**Vec<models::ProjectVersion>**](ProjectVersion.md)> |  | [optional]
**metrics** | Option<[**models::ProjectMetrics**](ProjectMetrics.md)> |  | [optional]
**bom_ref** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


