# \MailboxApi

All URIs are relative to *https://https://publicapi.sbanken.no/apibeta*

Method | HTTP request | Description
------------- | ------------- | -------------
[**mailbox_count_archive_message**](MailboxApi.md#mailbox_count_archive_message) | **GET** /api/v2/Mailbox/archive/count | Count the number of messages in your archive
[**mailbox_count_inbox_message**](MailboxApi.md#mailbox_count_inbox_message) | **GET** /api/v2/Mailbox/inbox/count | Count the number of messages in your inbox
[**mailbox_delete_archive**](MailboxApi.md#mailbox_delete_archive) | **DELETE** /api/v2/Mailbox/archive/{id} | Delete a message from your archive
[**mailbox_delete_inbox**](MailboxApi.md#mailbox_delete_inbox) | **DELETE** /api/v2/Mailbox/inbox/{id} | Delete a message from your inbox
[**mailbox_list_archive**](MailboxApi.md#mailbox_list_archive) | **GET** /api/v2/Mailbox/archive | List the messages in your archive
[**mailbox_list_inbox**](MailboxApi.md#mailbox_list_inbox) | **GET** /api/v2/Mailbox/inbox | List the messages in your inbox
[**mailbox_move_to_archive**](MailboxApi.md#mailbox_move_to_archive) | **PUT** /api/v2/Mailbox/inbox/{id}/move | Move a message from your inbox to your archive
[**mailbox_move_to_inbox**](MailboxApi.md#mailbox_move_to_inbox) | **PUT** /api/v2/Mailbox/archive/{id}/move | Move a message from your archive to your inbox 
[**mailbox_read_archive**](MailboxApi.md#mailbox_read_archive) | **GET** /api/v2/Mailbox/archive/{id} | Read a message from your archive.
[**mailbox_read_archive_attachment**](MailboxApi.md#mailbox_read_archive_attachment) | **GET** /api/v2/Mailbox/archive/{messageId}/attachment/{attachmentId} | Read the attachment of an archive message.
[**mailbox_read_inbox**](MailboxApi.md#mailbox_read_inbox) | **GET** /api/v2/Mailbox/inbox/{id} | Read a message from your inbox
[**mailbox_read_inbox_attachment**](MailboxApi.md#mailbox_read_inbox_attachment) | **GET** /api/v2/Mailbox/inbox/{messageId}/attachment/{attachmentId} | Read the attachment of an inbox message.
[**mailbox_set_read_status_archive**](MailboxApi.md#mailbox_set_read_status_archive) | **PUT** /api/v2/Mailbox/archive/{id}/readstatus | Sets the read/unread status of a message in your archive to the indicated value
[**mailbox_set_read_status_inbox**](MailboxApi.md#mailbox_set_read_status_inbox) | **PUT** /api/v2/Mailbox/inbox/{id}/readstatus | Sets the read/unread status of a message in your inbox to the indicated value



## mailbox_count_archive_message

> i32 mailbox_count_archive_message(status_filter, start_date, end_date)
Count the number of messages in your archive

StatusFilter=Read is currently not supported. It is only included for future implementation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**status_filter** | Option<**String**> | Optional. Only count messages with this status. If set to `null` messages are not filtered by status. Default value is `null`. |  |
**start_date** | Option<**String**> | Optional. Only count messages created on or after this date. If set to `null` the filter is not applied. Default value is `null`. |  |
**end_date** | Option<**String**> | Optional. Only count messages created before or on this date. If set to `null` the filter is not applied. Default value is `null`. |  |

### Return type

**i32**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mailbox_count_inbox_message

> i32 mailbox_count_inbox_message(status_filter, start_date, end_date)
Count the number of messages in your inbox

StatusFilter=Read is currently not supported. It is only included for future implementation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**status_filter** | Option<**String**> | Optional. Only count messages with this status. If set to `null` messages are not filtered by status. Default value is `null`. |  |
**start_date** | Option<**String**> | Optional. Only count messages created on or after this date. If set to `null` the filter is not applied. Default value is `null`. |  |
**end_date** | Option<**String**> | Optional. Only count messages created before or on this date. If set to `null` the filter is not applied. Default value is `null`. |  |

### Return type

**i32**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mailbox_delete_archive

> std::path::PathBuf mailbox_delete_archive(id)
Delete a message from your archive

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The archive message id | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mailbox_delete_inbox

> std::path::PathBuf mailbox_delete_inbox(id)
Delete a message from your inbox

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The messageId | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mailbox_list_archive

> crate::models::ListResultOfMessageListItem mailbox_list_archive(index, length, status_filter, start_date, end_date)
List the messages in your archive

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | Option<**i32**> | Optional. The starting index of the message list to read, starting at 0 for the first page. Default value is 0. |  |[default to 0]
**length** | Option<**i32**> | Optional. The length of the page, minimum 1. Default value is 100. |  |[default to 100]
**status_filter** | Option<**String**> | Optional. Only list messages with this status. If set to `null` messages are not filtered by status. Default value is `null`. |  |
**start_date** | Option<**String**> | Optional. Only list messages created on or after this date. If set to `null` the filter is not applied. Default value is `null`. |  |
**end_date** | Option<**String**> | Optional. Only list messages created before or on this date. If set to `null` the filter is not applied. Default value is `null`. |  |

### Return type

[**crate::models::ListResultOfMessageListItem**](ListResultOfMessageListItem.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mailbox_list_inbox

> crate::models::ListResultOfMessageListItem mailbox_list_inbox(index, length, status_filter, start_date, end_date)
List the messages in your inbox

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | Option<**i32**> | Optional. The starting index of the message list to read, starting at 0 for the first page. Default value is 0. |  |[default to 0]
**length** | Option<**i32**> | The length of the page, minimum 1. |  |[default to 100]
**status_filter** | Option<**String**> | Optional. Only list messages with this status. If set to `null` messages are not filtered by status. Default value is `null`. |  |
**start_date** | Option<**String**> | Optional. Only list messages created on or after this date. If set to `null` the filter is not applied. Default value is `null`. |  |
**end_date** | Option<**String**> | Optional. Only list messages created before or on this date. If set to `null` the filter is not applied. Default value is `null`. |  |

### Return type

[**crate::models::ListResultOfMessageListItem**](ListResultOfMessageListItem.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mailbox_move_to_archive

> i64 mailbox_move_to_archive(id)
Move a message from your inbox to your archive

The moved message is assigned a archive message id that is distinct from its inbox message id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The inbox message id | [required] |

### Return type

**i64**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mailbox_move_to_inbox

> i64 mailbox_move_to_inbox(id)
Move a message from your archive to your inbox 

This operation is currently not supported and is only included for future implementation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The archive message id | [required] |

### Return type

**i64**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mailbox_read_archive

> crate::models::MessageItem mailbox_read_archive(id)
Read a message from your archive.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The archive message Id | [required] |

### Return type

[**crate::models::MessageItem**](MessageItem.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mailbox_read_archive_attachment

> crate::models::MailboxAttachment mailbox_read_archive_attachment(message_id, attachment_id)
Read the attachment of an archive message.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**message_id** | **i64** | The archive message Id | [required] |
**attachment_id** | **i32** | The attachment Id | [required] |

### Return type

[**crate::models::MailboxAttachment**](MailboxAttachment.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mailbox_read_inbox

> crate::models::MessageItem mailbox_read_inbox(id)
Read a message from your inbox

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The inbox message Id | [required] |

### Return type

[**crate::models::MessageItem**](MessageItem.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mailbox_read_inbox_attachment

> crate::models::MailboxAttachment mailbox_read_inbox_attachment(message_id, attachment_id)
Read the attachment of an inbox message.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**message_id** | **i64** | The inbox message Id | [required] |
**attachment_id** | **i32** | The attachment Id | [required] |

### Return type

[**crate::models::MailboxAttachment**](MailboxAttachment.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mailbox_set_read_status_archive

> std::path::PathBuf mailbox_set_read_status_archive(id, value)
Sets the read/unread status of a message in your archive to the indicated value

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The archive message Id | [required] |
**value** | [**SetReadStatusRequest**](SetReadStatusRequest.md) | A request object that describes the desired settings to apply. | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, text/json, application/_*+json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mailbox_set_read_status_inbox

> std::path::PathBuf mailbox_set_read_status_inbox(id, value)
Sets the read/unread status of a message in your inbox to the indicated value

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The inbox message Id | [required] |
**value** | [**SetReadStatusRequest**](SetReadStatusRequest.md) | A request object that describes the desired settings to apply. | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, text/json, application/_*+json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

