#!/bin/bash


PROJECT_ROOT=/workspaces/roverd
TEST_FILES=$PROJECT_ROOT/rovervalidate/src/testfiles


# --- /etc/roverd/rover.yaml ---
mkdir -p /etc/roverd

cp $TEST_FILES/roverd-yaml/valid/common.yaml /etc/roverd/rover.yaml
# cp $TEST_FILES/roverd-yaml/invalid/invalid-name.yaml /etc/roverd/rover.yaml

# --- /home/debix/rover/ ---
mkdir -p /home/debix/rover
chown debix:debix /home/debix/rover

# --- /etc/rover ---
echo "14" > /etc/rover
echo "bunny" >> /etc/rover
echo -n "debix" | sha256sum | cut -d ' ' -f 1 >> /etc/rover
