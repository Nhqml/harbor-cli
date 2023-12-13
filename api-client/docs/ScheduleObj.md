# ScheduleObj

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | Option<**String**> | The schedule type. The valid values are 'Hourly', 'Daily', 'Weekly', 'Custom', 'Manual', 'None' and 'Schedule'. 'Manual' means to trigger it right away, 'Schedule' means to trigger it by a specified cron schedule and  'None' means to cancel the schedule.  | [optional]
**cron** | Option<**String**> | A cron expression, a time-based job scheduler. | [optional]
**next_scheduled_time** | Option<**String**> | The next time to schedule to run the job. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


