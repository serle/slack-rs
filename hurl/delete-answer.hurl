# insert a question
POST http://localhost:8000/question
Accept: application/json
{
	"question": "Question 1",
	"description": "Description"
}
HTTP 200
[Captures]
question_id: jsonpath "$.question_uuid"

# create answer to the question
POST http://localhost:8000/answer
Accept: application/json
{
	"question_uuid": "{{question_id}}",
	"content": "Content"
}
HTTP 200
[Captures]
answer_id: jsonpath "$.answer_uuid"

# delete the answer for the question
DELETE http://localhost:8000/answer
Accept: application/json
{
	"answer_uuid": "{{answer_id}}"
}
HTTP 200
