# insert a question
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

# create answer to the question
curl --request POST \
POST http://localhost:8000/answer
Accept: application/json
{
	"question_uuid": "{{question_id}}",
	"content": "Content"
}
HTTP 200
[Captures]
answer_id: jsonpath "$.id"

# delete the answer for the question
DELETE http://localhost:8000/answer
Accept: application/json
{
	"answer_uuid": "{{answer_id}}"
}
HTTP 200
