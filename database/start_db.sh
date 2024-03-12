#!/bin/bash

docker run --rm --network host \
    --name test-db \
    --health-cmd="mysqladmin -p0000 ping --silent" \
    -v "$(pwd):/taiyaki" -w /taiyaki \
    -eMYSQL_ROOT_PASSWORD=0000 \
    -d mysql

printf "Waiting on mysql...";

MYSQL_STATUS=$(docker inspect -f {{.State.Health.Status}} test-db)

until [ "$MYSQL_STATUS" == "healthy" ]; do
    printf ".";
    sleep 1;
    MYSQL_STATUS=$(docker inspect -f {{.State.Health.Status}} test-db)
done;

printf "\n";
echo "mysql is up and running!";

docker exec test-db /bin/bash -c "mysql -p0000 < create_db.sql"
docker exec test-db /bin/bash -c "mysql -p0000 < init_data.sql"