package server

import (
	"encoding/json"
	"fmt"
	"log"
	"net"
	"net/http"
	"time"
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

var debugPortVoice = "3000"
var debugPortData = "3001"

func CreateServerRandomName() *Server {
	return &Server{Address: "localhost", PortVoice: debugPortVoice, PortData: debugPortData, ServerName: GetRandomServerName()}
}

func CreateServer(serverName string) *Server {
	return &Server{Address: "localhost", PortVoice: debugPortVoice, PortData: debugPortData, ServerName: serverName}
}

func dataServerBaseURL(w http.ResponseWriter, r *http.Request) {
	w.Header().Set("Content-Type", "application/json")

	data := ServerData{
		ServerName: "Flaming Mango",
		Users:      []string{"John", "Jake", "Joe", "James"},
	}

	// Serialize the data to JSON and write it to the response
	err := json.NewEncoder(w).Encode(data)
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

func HostBothServers(server *Server) {
	fmt.Println("Server '" + server.ServerName + "' is Ready")

	HostDataServer(server)
	//go HostVoiceServer(server)

	//keep servers running
	select {
	case <-time.After(1 * time.Hour): // Just a timeout to keep the program running
	}
}
