package server

import (
	"encoding/json"
	"log"
	"net"
	"net/http"
	"strings"
	"time"
)

// Server will host both UDP server for Voice Data and TCP Server For Server Data
type VoiceConnection struct {
	Address           string `json:"Address"`
	Name              string `json:"Name"`
	LastSeen          int64  `json:"LastConnected"`
	CanAutoDisconnect bool

	TotalReceivedBytes uint64
	TotalSentBytes     uint64

	LastTotalReceivedBytes uint64
	LastTotalSentBytes     uint64

	ReceivedKBs float64
	SentKBs     float64

	MessagesSent uint64
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

	LastTotalReceivedBytes uint64
	LastTotalSentBytes     uint64

	VoiceLastTotalReceivedBytes uint64
	VoiceLastTotalSentBytes     uint64

	TotalReceivedBytes uint64
	TotalSentBytes     uint64

	TotalReceivedBytesVoice uint64
	TotalSentBytesVoice     uint64

	TotalReceivedBytesMessage uint64
	TotalSentBytesMessage     uint64

	TotalReceivedBytesData uint64
	TotalSentBytesData     uint64

	ReceivedKBs float64
	SentKBs     float64

	VoiceReceivedKBs float64
	VoiceSentKBs     float64
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

type ResponseWriterWithTracking struct {
	http.ResponseWriter
	bytesWritten int
}

func (rw *ResponseWriterWithTracking) Write(p []byte) (int, error) {
	n, err := rw.ResponseWriter.Write(p)
	rw.bytesWritten += n
	return n, err
}

func dataServerBaseURL(w http.ResponseWriter, r *http.Request) {
	trackingWriter := &ResponseWriterWithTracking{ResponseWriter: w}

	w.Header().Set("Access-Control-Allow-Origin", "*")
	w.Header().Set("Content-Type", "application/json")

	users := make([]string, len(server.Connections))
	for i, item := range server.Connections {
		users[i] = item.Name
	}

	var serverData ServerData = ServerData{ServerName: server.ServerName, Users: users}

	// Serialize the data to JSON and write it to the response
	err := json.NewEncoder(trackingWriter).Encode(serverData)
	if err != nil {
		http.Error(w, "Failed to encode JSON", http.StatusInternalServerError)
		return
	}

	// Update the TotalReceivedBytes with the amount of bytes written in the response
	server.TotalSentBytes += uint64(trackingWriter.bytesWritten)
	server.TotalSentBytesData += uint64(trackingWriter.bytesWritten)
}

func HostDataServer(server *Server) {
	// Define the route and associate it with the handler
	http.HandleFunc("/", dataServerBaseURL)
	serverURLFull := server.Address + ":" + server.PortData

	// Start the HTTPS server with SSL certificate and private key
	//fmt.Println("Starting HTTPS server on " + serverURLFull + " - [Messaing Gateway + Data Server]")
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
		//fmt.Println("Error resolving address:", err)
		return
	}

	// Create a UDP connection to listen for incoming data
	conn, err := net.ListenUDP("udp", address)
	if err != nil {
		//fmt.Println("Error listening on address:", err)
		return
	}
	defer conn.Close()

	//fmt.Println("UDP server listening on " + server.Address + ":" + server.PortVoice)

	// Buffer to store incoming data
	buffer := make([]byte, 2048)

	// Read data in a loop
	for {
		n, addr, err := conn.ReadFromUDP(buffer)
		if err != nil {
			//fmt.Println("Error reading from UDP:", err)
			continue
		}

		// Track the total received bytes for the connection
		updateConnectionData(server, addr, uint64(n), 0)
		server.TotalReceivedBytes += uint64(n)

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
				//fmt.Println("New User: '" + username + "' Has Connected on " + addr.String())
			} else {
				server.Connections[index] = NewVC
				//fmt.Println("Returning User: '" + username + "' Has Connected on " + addr.String())
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

			//fmt.Println("User Disconnected: " + server.Connections[index].Name + ".")
			server.Connections[index] = server.Connections[len(server.Connections)-1]
			server.Connections = server.Connections[:len(server.Connections)-1]
		} else {
			// This is Audio Data
			server.TotalReceivedBytesVoice += uint64(n)
			for _, item := range server.Connections {
				// Send Audio Data if NOT self
				if strings.Compare(item.Address, addr.String()) != 0 || debugMode {
					_, err = conn.WriteToUDP(buffer[:n], addr)

					// Track the total sent bytes for the connection
					updateConnectionData(server, addr, 0, uint64(n))

					server.TotalSentBytes += uint64(n)
					server.TotalSentBytesVoice += uint64(n)

					if err != nil {
						//fmt.Println("Error sending voice data to {"+addr.String()+"}, err:", err)
						continue
					} else {
						//fmt.Println("Sending voice data to {" + addr.String() + "}")
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
	}
}

// Helper function to update connection data (sent/received bytes)
func updateConnectionData(server *Server, addr net.Addr, receivedBytes uint64, sentBytes uint64) {
	connIndex, conn := findConnectionByAddress(addr)
	if conn != nil {
		conn.TotalReceivedBytes += receivedBytes
		conn.TotalSentBytes += sentBytes
		server.Connections[connIndex] = *conn
	}
}

// clear userlist if inactive for x seconds
func UserListClearer(timeFrameS int64, server *Server) {
	for {
		//Check Last Seen
		for i, item := range server.Connections {
			scaledTime := item.LastSeen + timeFrameS
			////fmt.Println(scaledTime)
			if scaledTime-time.Now().Unix() <= 0 {
				//remove from connections
				//fmt.Println("Removing: " + server.Connections[i].Name + ".")
				server.Connections[i] = server.Connections[len(server.Connections)-1]
				server.Connections = server.Connections[:len(server.Connections)-1]
			}
		}
	}
}

func UpdateServerStats(server *Server) {
	// Calculate SentKBs and ReceivedKBs based on the total bytes sent and received
	server.SentKBs = float64(server.TotalSentBytes-server.LastTotalSentBytes) / 1024
	server.ReceivedKBs = float64(server.TotalReceivedBytes-server.LastTotalReceivedBytes) / 1024

	// Reset byte counters for the next second
	server.LastTotalSentBytes = server.TotalSentBytes
	server.LastTotalReceivedBytes = server.TotalReceivedBytes

	//---------------------------------------------------------
	//VOICE

	// Calculate SentKBs and ReceivedKBs based on the total bytes sent and received
	server.VoiceSentKBs = float64(server.TotalSentBytesVoice-server.VoiceLastTotalSentBytes) / 1024
	server.VoiceReceivedKBs = float64(server.TotalReceivedBytesVoice-server.VoiceLastTotalReceivedBytes) / 1024

	// Reset byte counters for the next second
	server.LastTotalSentBytes = server.TotalSentBytesVoice
	server.LastTotalReceivedBytes = server.TotalReceivedBytesVoice

	//Update KB/s for Each Voice Connection
	for i, user := range server.Connections {
		server.Connections[i].SentKBs = float64(user.TotalSentBytes-user.LastTotalSentBytes) / 1024
		server.Connections[i].ReceivedKBs = float64(user.TotalReceivedBytes-user.LastTotalReceivedBytes) / 1024

		// Reset byte counters for the next second
		server.Connections[i].LastTotalSentBytes = user.TotalSentBytes
		server.Connections[i].LastTotalReceivedBytes = user.TotalReceivedBytes
	}
}

func HostBothServers(server *Server, isDebug bool) {
	//fmt.Println("Server '" + server.ServerName + "' is Ready")
	debugMode = isDebug

	if isDebug {
		//fmt.Println("	- Additionally, Server is running in DEBUG MODE")
	}

	go launchMessageGateway(server)
	go HostDataServer(server)
	go HostVoiceServer(server)

	go UserListClearer(5, server)

	go runStats(server)

	//keep servers running
	select {}
}
