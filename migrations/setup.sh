source .env

node src/cars.js down $TEST

node src/cars.js up $TEST

echo "schema up"

if [ "$SEED" = "yes" ]; then

    echo "seeding"

fi

echo "all done"