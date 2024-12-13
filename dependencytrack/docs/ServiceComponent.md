# ServiceComponent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**provider** | Option<[**models::OrganizationalEntity**](OrganizationalEntity.md)> |  | [optional]
**group** | Option<**String**> |  | [optional]
**name** | Option<**String**> |  | [optional]
**version** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**endpoints** | Option<**Vec<String>**> |  | [optional]
**authenticated** | Option<**bool**> |  | [optional]
**crosses_trust_boundary** | Option<**bool**> |  | [optional]
**data** | Option<[**Vec<models::DataClassification>**](DataClassification.md)> |  | [optional]
**external_references** | Option<[**Vec<models::ExternalReference>**](ExternalReference.md)> |  | [optional]
**parent** | Option<[**models::ServiceComponent**](ServiceComponent.md)> |  | [optional]
**children** | Option<[**Vec<models::ServiceComponent>**](ServiceComponent.md)> |  | [optional]
**vulnerabilities** | Option<[**Vec<models::Vulnerability>**](Vulnerability.md)> |  | [optional]
**project** | [**models::Project**](Project.md) |  | 
**last_inherited_risk_score** | Option<**f64**> |  | [optional]
**notes** | Option<**String**> |  | [optional]
**uuid** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**bom_ref** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


