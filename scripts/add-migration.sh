#!/bin/bash
set -e

now=$(date +%Y%m%d)
migration=$1
up_filename=${now}_${migration}_up.sql
echo "-- Add up migration script here" > ./migrations/${up_filename}
down_filename=${now}_${migration}_down.sql
echo "-- Add down migration script here" > ./migrations/${down_filename}
