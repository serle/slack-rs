# create a question
POST http://localhost:8000/question
Accept: application/json
{
	"question": "Title",
	"description": "Description"
}
HTTP 200
[Captures]
question_id: jsonpath "$.question_uuid"
[Asserts]
jsonpath "$.question_uuid" exists

# deleting the above question
DELETE http://localhost:8000/question
Accept: application/json
{
	"question_uuid": "{{question_id}}"
}
HTTP 200
