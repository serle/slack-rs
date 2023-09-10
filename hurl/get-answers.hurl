# insert a question
POST http://localhost:8000/question
Accept: application/json
{
	"question": "Question 1",
	"description": "Description"
}
HTTP 200
[Asserts]
jsonpath "$.question_uuid" exists
[Captures]
question_id: jsonpath "$.question_uuid"

# create answer 1 to question 1
POST http://localhost:8000/answer
Accept: application/json
{
	"question_uuid": "{{question_id}}",
	"content": "Content"
}
HTTP 200
[Captures]
answer1_id: jsonpath "$.answer_uuid"
[Asserts]
jsonpath "$.answer_uuid" exists

# create answer 2 to question 1
POST http://localhost:8000/answer
Accept: application/json
{
	"question_uuid": "{{question_id}}",
	"content": "Content"
}
HTTP 200
[Captures]
answer2_id: jsonpath "$.answer_uuid"
[Asserts]
jsonpath "$.answer_uuid" exists

# create answer 3 to question 1
POST http://localhost:8000/answer
Accept: application/json
{
	"question_uuid": "{{question_id}}",
	"content": "Content"
}
HTTP 200
[Captures]
answer3_id: jsonpath "$.answer_uuid"
[Asserts]
jsonpath "$.answer_uuid" exists


# test that we can get all three answers form question 1
GET http://localhost:8000/answers
Accept: application/json
{
	"question_uuid": "{{question_id}}"
}
HTTP 200
[Asserts]
jsonpath "$" count == 3

# delete all three answers
DELETE http://localhost:8000/answer
Accept: application/json
{
	"answer_uuid": "{{answer1_id}}"
}
HTTP 200

DELETE http://localhost:8000/answer
Accept: application/json
{
	"answer_uuid": "{{answer2_id}}"
}
HTTP 200

DELETE http://localhost:8000/answer
Accept: application/json
{
	"answer_uuid": "{{answer3_id}}"
}
HTTP 200
