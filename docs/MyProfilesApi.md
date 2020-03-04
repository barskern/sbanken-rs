# \MyProfilesApi

All URIs are relative to *https://api.sbanken.no*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_contact_info**](MyProfilesApi.md#get_contact_info) | **GET** /exec.customers/api/v1/MyProfiles/contactinformation | 
[**get_investment_accounts**](MyProfilesApi.md#get_investment_accounts) | **GET** /exec.customers/api/v1/MyProfiles/investmentaccounts | 
[**get_loans**](MyProfilesApi.md#get_loans) | **GET** /exec.customers/api/v1/MyProfiles/loans | 
[**get_profile**](MyProfilesApi.md#get_profile) | **GET** /exec.customers/api/v1/MyProfiles/profile | 
[**get_profile_accounts**](MyProfilesApi.md#get_profile_accounts) | **GET** /exec.customers/api/v1/MyProfiles/accounts | 
[**update_profile**](MyProfilesApi.md#update_profile) | **POST** /exec.customers/api/v1/MyProfiles/profile | 



## get_contact_info

> crate::models::ItemResultCustomerV1 get_contact_info(customer_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** |  | [required] |

### Return type

[**crate::models::ItemResultCustomerV1**](ItemResult.Customer.v1.md)

### Authorization

[sbanken](../README.md#sbanken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_investment_accounts

> crate::models::ListResultInvestmentAccountItemV1 get_investment_accounts(customer_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** |  | [required] |

### Return type

[**crate::models::ListResultInvestmentAccountItemV1**](ListResult.InvestmentAccountItem.v1.md)

### Authorization

[sbanken](../README.md#sbanken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_loans

> crate::models::ListResultLoanAccountV1 get_loans(customer_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** |  | [required] |

### Return type

[**crate::models::ListResultLoanAccountV1**](ListResult.LoanAccount.v1.md)

### Authorization

[sbanken](../README.md#sbanken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_profile

> crate::models::ItemResultMyProfileV1 get_profile(customer_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** |  | [required] |

### Return type

[**crate::models::ItemResultMyProfileV1**](ItemResult.MyProfile.v1.md)

### Authorization

[sbanken](../README.md#sbanken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_profile_accounts

> crate::models::ListResultAccountV1 get_profile_accounts(customer_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** |  | [required] |

### Return type

[**crate::models::ListResultAccountV1**](ListResult.Account.v1.md)

### Authorization

[sbanken](../README.md#sbanken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_profile

> crate::models::NoResult update_profile(customer_id, relationship_status, home_type, highest_complete_education, work_status, main_employer, has_children, number_of_children_below18_at_home, monthly_child_support_expenses, monthly_child_care_expenses, years_in_current_position, salary_frequency, monthly_rent, monthly_rental_revenues, yearly_income, salary_date, is_car_owner)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** |  | [required] |
**relationship_status** | Option<[**crate::models::RelationshipStatusV1**](.md)> |  |  |
**home_type** | Option<[**crate::models::HomeTypeV1**](.md)> |  |  |
**highest_complete_education** | Option<[**crate::models::EducationLevelV1**](.md)> |  |  |
**work_status** | Option<**String**> |  |  |
**main_employer** | Option<**String**> |  |  |
**has_children** | Option<**bool**> |  |  |
**number_of_children_below18_at_home** | Option<**i32**> |  |  |
**monthly_child_support_expenses** | Option<**i32**> |  |  |
**monthly_child_care_expenses** | Option<**i32**> |  |  |
**years_in_current_position** | Option<**i32**> |  |  |
**salary_frequency** | Option<[**crate::models::SalaryFrequencyV1**](.md)> |  |  |
**monthly_rent** | Option<**i32**> |  |  |
**monthly_rental_revenues** | Option<**i32**> |  |  |
**yearly_income** | Option<**i32**> |  |  |
**salary_date** | Option<**i32**> |  |  |
**is_car_owner** | Option<**bool**> |  |  |

### Return type

[**crate::models::NoResult**](NoResult..md)

### Authorization

[sbanken](../README.md#sbanken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

