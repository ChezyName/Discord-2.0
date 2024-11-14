package server

import (
	"fmt"
	"log"
	"net"
	"time"
)

// Server will host both UDP server for Voice Data and TCP Server For Server Data
type Server struct {
	Address   string
	PortVoice string
	PortData  string
}

func CreateServer() *Server {
	return &Server{Address: "127.0.0.1", PortVoice: "3000", PortData: "3001"}
}

func HostDataServer(server *Server) {
	// Resolve TCP address
	tcpAddr, err := net.ResolveTCPAddr("tcp", server.Address+":"+server.PortData)
	if err != nil {
		log.Fatal("Error resolving Data Server address:", err)
		return
	}

	// Start listening for TCP connections
	tcpListener, err := net.ListenTCP("tcp", tcpAddr)
	if err != nil {
		log.Fatal("Error starting Data Server:", err)
		return
	}
	defer tcpListener.Close()

	fmt.Println("Data Server started on: " + server.Address + ":" + server.PortData)

	/*
		for {
			// Accept new TCP connection
			conn, err := tcpListener.AcceptTCP()
			if err != nil {
				fmt.Println("Error accepting TCP connection:", err)
				continue
			}

			// Handle the connection in a new goroutine
			go handleTCPConnection(conn)
		}
	*/
}

func HostVoiceServer(server *Server) {
	// Resolve UDP address
	udpAddr, err := net.ResolveUDPAddr("udp", server.Address+":"+server.PortVoice)
	if err != nil {
		log.Fatal("Error resolving Voice address:", err)
		return
	}

	// Start listening for UDP connections
	udpConn, err := net.ListenUDP("udp", udpAddr)
	if err != nil {
		log.Fatal("Error starting Voice server:", err)
		return
	}
	defer udpConn.Close()

	fmt.Println("Voice Server started on: " + server.Address + ":" + server.PortVoice)

	/* DATA RECEVING CODE
	buffer := make([]byte, 1024)
	for {
		// Read data from UDP connection
		n, addr, err := udpConn.ReadFromUDP(buffer)
		if err != nil {
			fmt.Println("Error reading UDP message:", err)
			continue
		}

		// Print the received message
		fmt.Printf("Received from %s: %s\n", addr, string(buffer[:n]))

		// Send a response back
		response := "Hello from UDP server"
		_, err = udpConn.WriteToUDP([]byte(response), addr)
		if err != nil {
			fmt.Println("Error sending UDP response:", err)
		}
	}
	*/
}

func HostBothServers(server *Server) {
	go HostDataServer(server)
	go HostVoiceServer(server)

	//keep servers running
	select {
	case <-time.After(1 * time.Hour): // Just a timeout to keep the program running
	}
}
