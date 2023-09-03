#!/bin/bash
set -e

migration=$1

rm ./migrations/*_${migration}_*
