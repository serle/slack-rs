# insert question 1
POST http://localhost:8000/question
Accept: application/json
{
	"title": "Question 1",
	"description": "Description"
}
HTTP 200
[Asserts]
jsonpath "$.id" exists
[Captures]
question_id: jsonpath "$.id"

# create an answer for question 1
curl --request POST \
POST http://localhost:8000/answer
Accept: application/json
{
	"question_uuid": "{{question_id}}",
	"content": "Content"
}
HTTP 200