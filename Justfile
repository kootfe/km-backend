start:
	sudo systemctl start redis
	sudo systemctl start postgresql

sqlx-reset:
	sqlx database drop
	sqlx database create
	sqlx migrate run

auth-test path method token body:
	curl -X {{method}} "http://localhost:8080/{{path}}" \
		-H "Content-Type: application/json" \
		-H "Authorization: Bearer {{token}}" \
		-d '{{body}}'

post-test path body:
	curl -X POST "http://localhost:8080/{{path}}" \
		-H "Content-Type: application/json" \
		-d '{{body}}'
