#! /bin/bash

separator="********************************************************************"

# Ensure this script is in the root

# Starting the collector
echo $separator
echo "starting the collector"
echo $separator
`pypy3 backend/collector/main.py`

# Starting the emitter
echo $separator
echo "starting the emitter"
echo $separator
cd backend/emitter  
cargo run
cd ..
cd ..

# Starting the dashboard
echo $separator
echo "starting the dashboard"
echo $separator

