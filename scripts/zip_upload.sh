#!/bin/bash

curl -u debix:debix \
  -X POST \
  -H "Content-Type: multipart/form-data" \
  -F "content=@/workspaces/roverd/roverd/examples/actuator.zip" \
  http://localhost/upload

curl -u debix:debix \
  -X POST \
  -H "Content-Type: multipart/form-data" \
  -F "content=@/workspaces/roverd/roverd/examples/controller.zip" \
  http://localhost/upload

curl -u debix:debix \
  -X POST \
  -H "Content-Type: multipart/form-data" \
  -F "content=@/workspaces/roverd/roverd/examples/imaging.zip" \
  http://localhost/upload

echo
