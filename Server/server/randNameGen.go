package server

import (
	"math/rand"
	"time"
)

// Declare global variables using 'var'
var ServerNameA = []string{
	"Blazing", "Majestic", "Silent", "Mysterious", "Fierce", "Glowing", "Ancient", "Savage", "Charming", "Fearless",
	"Graceful", "Luminous", "Eerie", "Wild", "Mystic", "Vibrant", "Unyielding", "Benevolent", "Dazzling", "Arcane",
	"Swift", "Glacial", "Invincible", "Harmonic", "Enduring", "Solemn", "Radiating", "Gallant", "Blissful", "Resolute",
	"Steadfast", "Thundering", "Ethereal", "Seraphic", "Dynamic", "Whispering", "Enchanting", "Resilient", "Formidable", "Transcendent",
	"Regal", "Dominant", "Sublime", "Spectral", "Shimmering", "Roaring", "Merciful", "Ruthless", "Elegant", "Timeless",
}

var ServerNameB = []string{
	"Phoenix", "Dragon", "Crystal", "Mountain", "Forest", "Cloud", "River", "Ember", "Storm", "Flame",
	"Haven", "Galaxy", "Beacon", "Citadel", "Throne", "Crown", "Legion", "Guardian", "Temple", "Horizon",
	"Sentinel", "Rift", "Eclipse", "Comet", "Sapphire", "Obelisk", "Titan", "Griffin", "Sanctuary", "Harbinger",
	"Oracle", "Canyon", "Phantom", "Echo", "Warden", "Pinnacle", "Spire", "Nebula", "Voyager", "Frost",
	"Chimera", "Basilisk", "Inferno", "Meadow", "Valley", "Star", "Sphere", "Blizzard", "Serpent", "Tundra",
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
