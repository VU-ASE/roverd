#!/bin/bash

# How to zip
# cd imaging
# zip -r ../imaging.zip bin/imaging service.yaml

DIR=circular_pipeline

curl -u debix:debix \
  -X POST \
  -H "Content-Type: multipart/form-data" \
  -F "content=@/workspaces/roverd/roverd/examples/$DIR/actuator.zip" \
  http://localhost/upload

curl -u debix:debix \
  -X POST \
  -H "Content-Type: multipart/form-data" \
  -F "content=@/workspaces/roverd/roverd/examples/$DIR/controller.zip" \
  http://localhost/upload

curl -u debix:debix \
  -X POST \
  -H "Content-Type: multipart/form-data" \
  -F "content=@/workspaces/roverd/roverd/examples/$DIR/imaging.zip" \
  http://localhost/upload



echo
