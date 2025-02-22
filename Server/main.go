package main

import (
	"DiscordServer/server"
	"flag"
	"fmt"
)

func main() {
	name := flag.String("name", server.GetRandomServerName(), "Name of the server")
	debug := flag.Bool("debug", false, "Enable debug mode")

	port := flag.String("port", server.DefaultPort, "Change the port of the Server || Default 3000")

	flag.Parse()

	fmt.Println("Prepairing Server....")
	fmt.Printf("Server Name: %s\n", *name)
	fmt.Printf("Port: %v\n", *port)
	fmt.Printf("Debug Mode: %v\n", *debug)
	fmt.Println("----------------------------------")

	_server := server.CreateServer(*name, *port)
	server.HostBothServers(_server, *debug)
}
