package server

import (
	"encoding/json"
	"fmt"
	"log"
	"net"
	"net/http"
	"strings"
	"time"
)

// Server will host both UDP server for Voice Data and TCP Server For Server Data
type VoiceConnection struct {
	Address  net.Addr `json:"Address"`
	Name     string   `json:"Name"`
	LastSeen int64    `json:"LastConnected"`
}

type ServerData struct {
	ServerName string            `json:"server_name"`
	Users      []VoiceConnection `json:"users"`
}

type Server struct {
	Address     string
	PortVoice   string
	PortData    string
	Connections []VoiceConnection
	ServerName  string
}

var server Server
var debugMode = false

var debugPortVoice = "3000"
var debugPortData = "3001"

func CreateServerRandomName() *Server {
	server = Server{Address: "localhost", PortVoice: debugPortVoice, PortData: debugPortData, ServerName: GetRandomServerName()}
	return &server
}

func CreateServer(serverName string) *Server {
	server = Server{Address: "localhost", PortVoice: debugPortVoice, PortData: debugPortData, ServerName: serverName, Connections: make([]VoiceConnection, 0)}
	return &server
}

func dataServerBaseURL(w http.ResponseWriter, r *http.Request) {
	w.Header().Set("Access-Control-Allow-Origin", "*")
	w.Header().Set("Content-Type", "application/json")

	users := make([]string, len(server.Connections))
	for i, item := range server.Connections {
		users[i] = item.Name
	}

	var serverData ServerData = ServerData{ServerName: server.ServerName, Users: server.Connections}

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

func findConnectionByAddress(addr net.Addr) (int, *VoiceConnection) {
	for i, item := range server.Connections {
		if strings.Compare(item.Address.String(), addr.String()) == 0 {
			return i, &item
		}
	}

	return -1, nil
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
		fmt.Println(data)

		//Check if User is sending thier username
		if strings.Contains(data, "username:") {
			username := strings.Replace(data, "username:", "", 1)

			//Check if user already exists, if not store it part of the list.
			var index = -1
			for i, item := range server.Connections {
				if strings.Compare(item.Address.String(), addr.String()) == 0 {
					index = i
					break
				}
			}

			var NewVC VoiceConnection = VoiceConnection{
				Address:  addr,
				Name:     username,
				LastSeen: time.Now().Unix(),
			}

			if index == -1 {
				//Create new User
				server.Connections = append(server.Connections, NewVC)
				fmt.Println("New User: '" + username + "' Has Connected on " + addr.String())
			} else {
				server.Connections[index] = NewVC
				fmt.Println("Returning User: '" + username + "' Has Connected on " + addr.String())
			}
		} else {
			//This is Audio Data
			for _, item := range server.Connections {
				//send Audio Data if NOT self
				if strings.Compare(item.Address.String(), addr.String()) != 0 || debugMode {
					_, err = conn.WriteToUDP(buffer, addr)
					if err != nil {
						fmt.Println("Error sending voice data to {"+addr.String()+"}, err:", err)
						continue
					} else {
						fmt.Println("Sending voice data to {" + addr.String() + "}")
					}
				}
			}

			//Most likely in Database, change Time
			connIndex, conn := findConnectionByAddress(addr)
			if conn != nil {
				conn.LastSeen = time.Now().Unix()
				server.Connections[connIndex] = *conn
			}
		}
	}
}

// clear userlist if inactive for x seconds
func UserListClearer(timeFrameS int64, server *Server) {
	for {
		//Check Last Seen
		for i, item := range server.Connections {
			scaledTime := item.LastSeen + timeFrameS
			fmt.Println(scaledTime)
			if scaledTime-time.Now().Unix() <= 0 {
				//remove from connections
				server.Connections[i] = server.Connections[len(server.Connections)-1]
				server.Connections = server.Connections[:len(server.Connections)-1]
			}
		}
	}
}

func HostBothServers(server *Server, isDebug bool) {
	fmt.Println("Server '" + server.ServerName + "' is Ready")
	debugMode = isDebug

	go HostDataServer(server)
	go HostVoiceServer(server)
	go UserListClearer(2, server)

	//keep servers running
	select {}
}
