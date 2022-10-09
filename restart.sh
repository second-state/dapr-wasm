#!/usr/bin/env bash

dapr list

pushd image-api-classify
nohup ./run_classify.sh > nohup.log & 
popd

pushd image-api-grayscale
nohup ./run_grayscale.sh > nohup.log &
popd