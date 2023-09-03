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

# create answer 1 to question 1
curl --request POST \
POST http://localhost:8000/answer
Accept: application/json
{
	"question_uuid": "{{question_id}}",
	"content": "Content"
}
HTTP 200
[Captures]
answer1_id: jsonpath "$.id"

# create answer 2 to question 1
curl --request POST \
POST http://localhost:8000/answer
Accept: application/json
{
	"question_uuid": "{{question_id}}",
	"content": "Content"
}
HTTP 200
[Captures]
answer2_id: jsonpath "$.id"

# create answer 3 to question 1
curl --request POST \
POST http://localhost:8000/answer
Accept: application/json
{
	"question_uuid": "{{question_id}}",
	"content": "Content"
}
HTTP 200
[Captures]
answer3_id: jsonpath "$.id"

# test that we can get all three answers form question 1
GET http://localhost:8000/answers
Accept: application/json
{
	"question_uuid": "[UUID of a created question]"
}
HTTP 200
[Assert]
jsonpath "$" count 3

[Delete Answers]
DELETE http://localhost:8000/answer
Accept: application/json
{
	"answer_uuid": "[UUID of a created answer]"
}

HTTP 200
