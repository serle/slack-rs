# create a question
POST http://localhost:8000/question
Accept: application/json
{
	"title": "Title",
	"description": "Description"
}
HTTP 200
[Captures]
question_id: jsonpath "$.id"

# deleting the above question
[Delete Question]
DELETE http://localhost:8000/question
Accept: application/json
{
	"question_uuid": "{{question_id}}"
}
HTTP 200
