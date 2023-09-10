# insert question 1
POST http://localhost:8000/question
Accept: application/json
{
	"question": "Question 1",
	"description": "Description"
}
HTTP 200
[Captures]
question_id: jsonpath "$.question_uuid"

# create an answer for question 1
POST http://localhost:8000/answer
Accept: application/json
{
	"question_uuid": "{{question_id}}",
	"content": "Content"
}
HTTP 200
[Asserts]
jsonpath "$.answer_uuid" exists
