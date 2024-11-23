package server

import (
	"encoding/json"
	"fmt"
	"log"
	"net"
	"net/http"
	"strings"
)

// Server will host both UDP server for Voice Data and TCP Server For Server Data
type VoiceConnection struct {
	Address net.Addr
	Name    string
}

type ServerData struct {
	ServerName string   `json:"server_name"`
	Users      []string `json:"users"`
}

type Server struct {
	Address     string
	PortVoice   string
	PortData    string
	Connections []VoiceConnection
	ServerName  string
}

var server Server

var debugPortVoice = "3000"
var debugPortData = "3001"

func CreateServerRandomName() *Server {
	server = Server{Address: "localhost", PortVoice: debugPortVoice, PortData: debugPortData, ServerName: GetRandomServerName()}
	return &server
}

func CreateServer(serverName string) *Server {
	server = Server{Address: "localhost", PortVoice: debugPortVoice, PortData: debugPortData, ServerName: serverName}
	return &server
}

func dataServerBaseURL(w http.ResponseWriter, r *http.Request) {
	w.Header().Set("Content-Type", "application/json")

	users := make([]string, len(server.Connections))
	for i, item := range server.Connections {
		users[i] = item.Name
	}

	var serverData ServerData = ServerData{ServerName: server.ServerName, Users: users}

	// Serialize the data to JSON and write it to the response
	err := json.NewEncoder(w).Encode(serverData)
	if err != nil {
		http.Error(w, "Failed to encode JSON", http.StatusInternalServerError)
		return
	}
}

func HostDataServer(server *Server) {
	// Define the route and associate it with the handler
	http.HandleFunc("/", dataServerBaseURL)
	serverURLFull := server.Address + ":" + server.PortData

	// Start the HTTPS server with SSL certificate and private key
	fmt.Println("Starting HTTPS server on " + serverURLFull)
	err := http.ListenAndServe(serverURLFull, nil)
	if err != nil {
		log.Fatalf("Server failed to start: %v", err)
	}
}

func HostVoiceServer(server *Server) {
	// Create a UDP address to listen on (e.g., port 8080)
	address, err := net.ResolveUDPAddr("udp", server.Address+":"+server.PortVoice)
	if err != nil {
		fmt.Println("Error resolving address:", err)
		return
	}

	// Create a UDP connection to listen for incoming data
	conn, err := net.ListenUDP("udp", address)
	if err != nil {
		fmt.Println("Error listening on address:", err)
		return
	}
	defer conn.Close()

	fmt.Println("UDP server listening on " + server.Address + ":" + server.PortVoice)

	// Buffer to store incoming data
	buffer := make([]byte, 1024)

	// Read data in a loop
	for {
		n, addr, err := conn.ReadFromUDP(buffer)
		if err != nil {
			fmt.Println("Error reading from UDP:", err)
			continue
		}

		// Print the received data
		//fmt.Printf("Received %d bytes from %s: %s\n", n, addr, string(buffer[:n]))
		data := string(buffer[:n])

		//Check if User is sending thier username
		if strings.Contains(data, "username:") {
			username := strings.Replace(data, "username:", "", 1)
			fmt.Println("New User: '" + username + "' Has Connected on " + addr.String())

			//Check if user already exists, if not store it part of the list.
			var index = -1
			for i, item := range server.Connections {
				if strings.Compare(item.Address.String(), addr.String()) == 0 {
					index = i
					break
				}
			}

			var NewVC VoiceConnection = VoiceConnection{
				Address: addr,
				Name:    username,
			}

			if index == -1 {
				//Create new User
				server.Connections = append(server.Connections, NewVC)
			} else {
				server.Connections[index] = NewVC
			}
		} else {
			//This is Audio Data
		}
	}
}

func HostBothServers(server *Server) {
	fmt.Println("Server '" + server.ServerName + "' is Ready")

	go HostDataServer(server)
	go HostVoiceServer(server)

	//keep servers running
	select {}
}
