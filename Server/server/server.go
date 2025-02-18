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
	Address            string `json:"Address"`
	Name               string `json:"Name"`
	LastSeen           int64  `json:"LastConnected"`
	CanAutoDisconnect  bool
	TotalReceivedBytes uint64  `json:"TotalReceivedBytes"`
	TotalSentBytes     uint64  `json:"TotalSentBytes"`
	ReceivedKBs        float64 `json:"ReceivedKBs"`
	SentKBs            float64 `json:"SentKBs"`
	MessagesSent       uint64  `json:"MessagesSent"`
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
var debugMode = false

const DefaultPort = "3000"

var Port = DefaultPort

func CreateServerRandomName(port string) *Server {
	server = Server{Address: "localhost", PortVoice: port, PortData: port, ServerName: GetRandomServerName()}
	return &server
}

func CreateServer(serverName string, port string) *Server {
	server = Server{Address: "localhost", PortVoice: port, PortData: port, ServerName: serverName, Connections: make([]VoiceConnection, 0)}
	return &server
}

func dataServerBaseURL(w http.ResponseWriter, r *http.Request) {
	w.Header().Set("Access-Control-Allow-Origin", "*")
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
	fmt.Println("Starting HTTPS server on " + serverURLFull + " - [Messaing Gateway + Data Server]")
	err := http.ListenAndServe(serverURLFull, nil)
	if err != nil {
		log.Fatalf("Server failed to start: %v", err)
	}
}

func findConnectionByAddress(addr net.Addr) (int, *VoiceConnection) {
	for i, item := range server.Connections {
		if strings.Compare(item.Address, addr.String()) == 0 {
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
	buffer := make([]byte, 2048)

	// Track start time to calculate KB/s
	startTime := time.Now()

	var totalReceivedBytes uint64
	var totalSentBytes uint64

	// Read data in a loop
	for {
		n, addr, err := conn.ReadFromUDP(buffer)
		if err != nil {
			fmt.Println("Error reading from UDP:", err)
			continue
		}

		// Track the total received bytes for the connection
		updateConnectionData(server, addr, uint64(n), 0)

		// Check if User is sending their username
		if strings.Contains(string(buffer[:n]), "username:") {
			username := strings.Replace(string(buffer[:n]), "username:", "", 1)

			// Check if user already exists, if not store it as part of the list.
			var index = -1
			for i, item := range server.Connections {
				if strings.Compare(item.Address, addr.String()) == 0 {
					index = i
					break
				}
			}

			var NewVC VoiceConnection = VoiceConnection{
				Address:           addr.String(),
				Name:              username,
				LastSeen:          time.Now().Unix(),
				CanAutoDisconnect: true,
			}

			if index == -1 {
				// Create new User
				server.Connections = append(server.Connections, NewVC)
				fmt.Println("New User: '" + username + "' Has Connected on " + addr.String())
			} else {
				server.Connections[index] = NewVC
				fmt.Println("Returning User: '" + username + "' Has Connected on " + addr.String())
			}
		} else if strings.Contains(string(buffer[:n]), "hb") {
			// Heartbeat: Update last seen time
			var index = -1
			for i, item := range server.Connections {
				if strings.Compare(item.Address, addr.String()) == 0 {
					index = i
					break
				}
			}

			if index == -1 {
				return
			}

			server.Connections[index].LastSeen = time.Now().Unix()
		} else if strings.Contains(string(buffer[:n]), "disconnect") {
			// User is leaving the server
			var index = -1
			for i, item := range server.Connections {
				if strings.Compare(item.Address, addr.String()) == 0 {
					index = i
					break
				}
			}

			if index == -1 {
				return
			}

			fmt.Println("User Disconnected: " + server.Connections[index].Name + ".")
			server.Connections[index] = server.Connections[len(server.Connections)-1]
			server.Connections = server.Connections[:len(server.Connections)-1]
		} else {
			// This is Audio Data
			for _, item := range server.Connections {
				// Send Audio Data if NOT self
				if strings.Compare(item.Address, addr.String()) != 0 {
					_, err = conn.WriteToUDP(buffer[:n], addr)

					// Track the total sent bytes for the connection
					updateConnectionData(server, addr, 0, uint64(n))

					if err != nil {
						fmt.Println("Error sending voice data to {"+addr.String()+"}, err:", err)
						continue
					} else {
						fmt.Println("Sending voice data to {" + addr.String() + "}")
					}
				}
			}

			// Update the "LastSeen" time for the connection
			connIndex, conn := findConnectionByAddress(addr)
			if conn != nil {
				conn.LastSeen = time.Now().Unix()
				server.Connections[connIndex] = *conn
			}
		}

		_, conn := findConnectionByAddress(addr)
		// Calculate elapsed time and KB/s
		elapsedTime := time.Since(startTime)
		// If more than 1 second has passed, calculate and print KB/s
		if elapsedTime >= time.Second {
			// Calculate KB/s for both received and sent data
			recvKBps := float64(totalReceivedBytes) / 1024 / elapsedTime.Seconds()
			sentKBps := float64(totalSentBytes) / 1024 / elapsedTime.Seconds()

			// Print network stats per second
			conn.ReceivedKBs = recvKBps
			conn.SentKBs = sentKBps

			// Reset stats for next second
			totalReceivedBytes = 0
			totalSentBytes = 0
			startTime = time.Now()
		}
	}
}

// Helper function to update connection data (sent/received bytes)
func updateConnectionData(server *Server, addr net.Addr, receivedBytes uint64, sentBytes uint64) {
	// Find connection by address
	for i, conn := range server.Connections {
		if strings.Compare(conn.Address, addr.String()) == 0 {
			// Update received and sent bytes for this connection
			server.Connections[i].TotalReceivedBytes += receivedBytes
			server.Connections[i].TotalSentBytes += sentBytes
			return
		}
	}
}

// clear userlist if inactive for x seconds
func UserListClearer(timeFrameS int64, server *Server) {
	for {
		//Check Last Seen
		for i, item := range server.Connections {
			if !server.Connections[i].CanAutoDisconnect {
				continue
			}

			scaledTime := item.LastSeen + timeFrameS
			//fmt.Println(scaledTime)
			if scaledTime-time.Now().Unix() <= 0 {
				//remove from connections
				fmt.Println("Removing: " + server.Connections[i].Name + ".")
				server.Connections[i] = server.Connections[len(server.Connections)-1]
				server.Connections = server.Connections[:len(server.Connections)-1]
			}
		}
	}
}

func HostBothServers(server *Server, isDebug bool) {
	fmt.Println("Server '" + server.ServerName + "' is Ready")
	debugMode = isDebug

	if isDebug {
		fmt.Println("	- Additionally, Server is running in DEBUG MODE")
	}

	go runStats(server)
	go launchMessageGateway(server)
	go HostDataServer(server)
	go HostVoiceServer(server)
	go UserListClearer(5, server)

	//keep servers running
	select {}
}
