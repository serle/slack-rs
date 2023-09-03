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
question1_id: jsonpath "$.id"

# insert question 2
POST http://localhost:8000/question
Accept: application/json
{
	"title": "Question 2",
	"description": "Description"
}
HTTP 200
[Asserts]
jsonpath "$.id" exists
[Captures]
question2_id: jsonpath "$.id"

# insert question 3
POST http://localhost:8000/question
Accept: application/json
{
	"title": "Question 3",
	"description": "Description"
}
HTTP 200
[Asserts]
jsonpath "$.id" exists
[Captures]
question3_id: jsonpath "$.id"

# test get questions
[Get Questions]
GET http://localhost:8000/questions
Accept: application/json
HTTP 200
[Asserts]
jsonpath "$" count == 3







[Create Answer]
curl --request POST \
POST http://localhost:8000/answer
Accept: application/json
{
	"question_uuid": "[UUID of a created question]",
	"content": "Content"
}

HTTP 200

[Get Answers]
curl --request GET \
GET http://localhost:8000/answers
Accept: application/json
{
	"question_uuid": "[UUID of a created question]"
}

HTTP 200

[Delete Answers]
DELETE http://localhost:8000/answer
Accept: application/json
{
	"answer_uuid": "[UUID of a created answer]"
}

HTTP 200
