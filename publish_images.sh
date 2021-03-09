#!/bin/bash

docker login -u harvzor -p $DOCKER_HUB_PASS
docker tag $LOCAL_IMAGE:latest $REMOTE/$LOCAL_IMAGE:latest
docker tag $LOCAL_IMAGE:latest $REMOTE/$LOCAL_IMAGE:$TRAVIS_BUILD_NUMBER
docker push $REMOTE/$LOCAL_IMAGE:latest
docker push $REMOTE/$LOCAL_IMAGE:$TRAVIS_BUILD_NUMBER 
