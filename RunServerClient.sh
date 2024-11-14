#!/bin/bash
$echo "Launching Server"
cd Server
go run main.go
cd ../

$echo "Launching Client"
cd Client
npm install
npm run tdev
cd ../

$echo Done