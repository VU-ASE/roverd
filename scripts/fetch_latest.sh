#!/bin/bash


ROVER_IP=192.168.0.107
ROVER_PW=alan

function check_roverd {
    status=`curl -s $ROVER_IP/status | jq -r '.status'`

    if [ "$status" != "operational" ] ; then
        echo Error: roverd not operational;
        exit 1;
    fi
}

function get_service {
    curl -u debix:debix -X 'POST' "http://192.168.0.112/update"
}

check_roverd

get_service imaging
get_service controller
get_service actuator
get_service transceiver
