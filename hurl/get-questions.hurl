# insert question 1
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
question1_id: jsonpath "$.question_uuid"

# insert question 2
POST http://localhost:8000/question
Accept: application/json
{
	"question": "Question 2",
	"description": "Description"
}
HTTP 200
[Asserts]
jsonpath "$.question_uuid" exists
[Captures]
question2_id: jsonpath "$.question_uuid"

# insert question 3
POST http://localhost:8000/question
Accept: application/json
{
	"question": "Question 3",
	"description": "Description"
}
HTTP 200
[Asserts]
jsonpath "$.question_uuid" exists
[Captures]
question3_id: jsonpath "$.question_uuid"

# test get questions
GET http://localhost:8000/questions
Accept: application/json
HTTP 200
[Asserts]
jsonpath "$" count == 3