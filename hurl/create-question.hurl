# create a question
POST http://localhost:8000/question
Accept: application/json
{
	"question": "Title",
	"description": "Description"
}

HTTP 200
[Asserts]
jsonpath "$.question_uuid" exists

