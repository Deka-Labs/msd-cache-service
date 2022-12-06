#!/bin/sh

if [ -z "${DEV_SERVER}" ]
then 
    target/release/msd-cache-service
else 
    cargo watch -x run
fi

exit 0
