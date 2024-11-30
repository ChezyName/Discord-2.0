package main

import (
	"DiscordServer/server"
)

func main() {
	_server := server.CreateServer("Flaming Mango")
	server.HostBothServers(_server, true)
}
