package main

import (
	"DiscordServer/server"
)

func main() {
	_server := server.CreateServer()
	server.HostBothServers(_server)
}
