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
