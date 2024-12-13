# NotificationRule

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**enabled** | Option<**bool**> |  | [optional]
**notify_children** | Option<**bool**> |  | [optional]
**log_successful_publish** | Option<**bool**> |  | [optional]
**scope** | **String** |  | 
**notification_level** | Option<**String**> |  | [optional]
**projects** | Option<[**Vec<models::Project>**](Project.md)> |  | [optional]
**teams** | Option<[**Vec<models::Team>**](Team.md)> |  | [optional]
**notify_on** | Option<**Vec<String>**> |  | [optional]
**message** | Option<**String**> |  | [optional]
**publisher** | Option<[**models::NotificationPublisher**](NotificationPublisher.md)> |  | [optional]
**publisher_config** | Option<**String**> |  | [optional]
**uuid** | [**uuid::Uuid**](uuid::Uuid.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


