# Team

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**uuid** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**name** | Option<**String**> |  | [optional]
**api_keys** | Option<[**Vec<models::ApiKey>**](ApiKey.md)> |  | [optional]
**ldap_users** | Option<[**Vec<models::LdapUser>**](LdapUser.md)> |  | [optional]
**managed_users** | Option<[**Vec<models::ManagedUser>**](ManagedUser.md)> |  | [optional]
**oidc_users** | Option<[**Vec<models::OidcUser>**](OidcUser.md)> |  | [optional]
**mapped_ldap_groups** | Option<[**Vec<models::MappedLdapGroup>**](MappedLdapGroup.md)> |  | [optional]
**mapped_oidc_groups** | Option<[**Vec<models::MappedOidcGroup>**](MappedOidcGroup.md)> |  | [optional]
**permissions** | Option<[**Vec<models::Permission>**](Permission.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


