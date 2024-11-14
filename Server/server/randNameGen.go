package server

import (
	"math/rand"
	"time"
)

// Declare global variables using 'var'
var ServerNameA = []string{
	"flaming", "mysterious", "bizarre", "shiny", "silent", "brave", "ancient", "savage", "glistening", "majestic",
}

var ServerNameB = []string{
	"mango", "mountain", "wizard", "ocean", "forest", "cloud", "phoenix", "dragon", "star", "crystal",
}

func init() {
	// Initialize random seed
	rand.Seed(time.Now().UnixNano())
}

func GetRandomServerName() string {
	// Get a random adjective and a random noun
	adjective := ServerNameA[rand.Intn(len(ServerNameA))]
	noun := ServerNameB[rand.Intn(len(ServerNameB))]

	// Return the random server name
	return adjective + " " + noun
}
