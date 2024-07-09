single_threaded_tests_db_up:
	docker run --name single_threaded_tests_db -p 5432:5432 \
		-e POSTGRES_USER=user -e POSTGRES_PASSWORD=pass -e POSTGRES_DB=test \
		-d postgres

single_threaded_tests_db_down:
	docker container stop $(docker container ls -q --filter name=single_threaded_tests_db)

multi_threaded_tests_db_up:
	docker run --name multi_threaded_tests_db -p 5432:5432 \
	-e POSTGRES_USER=user -e POSTGRES_PASSWORD=pass \
	-d postgres

multi_threaded_tests_db_down:
	docker container stop $(docker container ls -q --filter name=multi_threaded_tests_db)

sqlx_multi_threaded_tests_db_up:
	docker run --name sqlx_multi_threaded_tests_db -p 5432:5432 \
	-e POSTGRES_USER=user -e POSTGRES_PASSWORD=pass -e POSTGRES_DB=test \
	-d postgres

sqlx_multi_threaded_tests_db_down:
	docker container stop $(docker container ls -q --filter name=sqlx_multi_threaded_tests_db)
