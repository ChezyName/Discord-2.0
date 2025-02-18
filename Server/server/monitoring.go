package server

import (
	"fmt"
	"os"
	"os/exec"
	"runtime"
	"time"

	"github.com/shirou/gopsutil/net"
	"github.com/shirou/gopsutil/v3/process"
)

// ClearConsole clears the terminal screen before printing new stats
func ClearConsole() {
	switch runtime.GOOS {
	case "windows":
		cmd := exec.Command("cmd", "/c", "cls") // Windows clear command
		cmd.Stdout = os.Stdout
		cmd.Run()
	default:
		fmt.Print("\033[H\033[2J") // ANSI escape codes for Unix/macOS
	}
}

func runStats(server *Server) {
	// Get current process
	pid := int32(os.Getpid())
	proc, err := process.NewProcess(pid)
	if err != nil {
		fmt.Println("Error getting process:", err)
		return
	}

	// Get initial network stats
	prevNetStats, _ := net.IOCounters(false)
	totalSent := prevNetStats[0].BytesSent
	totalRecv := prevNetStats[0].BytesRecv

	for {
		ClearConsole() // Clear screen before printing stats

		// Get CPU usage
		cpuPercent, _ := proc.CPUPercent()

		// Get Memory usage
		memInfo, _ := proc.MemoryInfo()

		// Get Disk I/O usage
		ioCounters, _ := proc.IOCounters()

		// Get current network stats
		currentNetStats, _ := net.IOCounters(false)

		// Calculate network usage since last check
		sentBytes := currentNetStats[0].BytesSent - prevNetStats[0].BytesSent
		recvBytes := currentNetStats[0].BytesRecv - prevNetStats[0].BytesRecv

		// Update total bytes
		totalSent = currentNetStats[0].BytesSent
		totalRecv = currentNetStats[0].BytesRecv

		// Update previous network stats
		prevNetStats = currentNetStats

		fmt.Printf("---- Server Info ----\n")
		fmt.Printf("Name: %s\n", server.ServerName)
		fmt.Printf("Port: %s\n", server.PortVoice)

		// Print Stats
		fmt.Printf("\n---- Server Stats ----\n")
		fmt.Printf("PID: %d\n", pid)
		fmt.Printf("CPU Usage: %.2f%%\n", cpuPercent)
		fmt.Printf("Memory Usage: %.2f MB\n", float64(memInfo.RSS)/1e6)
		fmt.Printf("Disk Read: %.2f MB | Disk Write: %.2f MB\n", float64(ioCounters.ReadBytes)/1e6, float64(ioCounters.WriteBytes)/1e6)
		fmt.Printf("Network Speed: Sent: %.2f KB/s | Received: %.2f KB/s\n", float64(sentBytes)/1e3, float64(recvBytes)/1e3)
		fmt.Printf("Total Network: Sent: %.2f MB | Received: %.2f MB\n", float64(totalSent)/1e6, float64(totalRecv)/1e6)

		fmt.Printf("\n---- User Stats ----\n")
		for _, user := range server.Connections {
			fmt.Printf("%s | %s\n", user.Name, user.Address)
			fmt.Printf("-	Messages Sent: %d\n", user.MessagesSent)

			fmt.Printf("-	Network Speed: Sent: %.2f KB/s | Received: %.2f KB/s\n",
				user.SentKBs, user.ReceivedKBs)

			fmt.Printf("-	Total Network: Sent: %.2f MB | Received: %.2f MB\n",
				float64(user.TotalSentBytes)/1e6, float64(user.TotalReceivedBytes)/1e6)
		}

		time.Sleep(1 * time.Second)
	}
}
