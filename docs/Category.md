# Category

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**category_group_id** | **String** |  | 
**name** | **String** |  | 
**hidden** | **bool** | Whether or not the category is hidden | 
**original_category_group_id** | **String** | If category is hidden this is the id of the category group it originally belonged to before it was hidden. | [optional] 
**note** | **String** |  | [optional] 
**budgeted** | **i64** | Budgeted amount in milliunits format | 
**activity** | **i64** | Activity amount in milliunits format | 
**balance** | **i64** | Balance in milliunits format | 
**goal_type** | **String** | The type of goal, if the category has a goal (TB=Target Category Balance, TBD=Target Category Balance by Date, MF=Monthly Funding) | [optional] 
**goal_creation_month** | [***String**](string.md) | The month a goal was created | [optional] 
**goal_target** | **i64** | The goal target amount in milliunits | 
**goal_target_month** | [***String**](string.md) | If the goal type is 'TBD' (Target Category Balance by Date), this is the target month for the goal to be completed | [optional] 
**goal_percentage_complete** | **i32** | The percentage completion of the goal | [optional] 
**deleted** | **bool** | Whether or not the category has been deleted.  Deleted categories will only be included in delta requests. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


