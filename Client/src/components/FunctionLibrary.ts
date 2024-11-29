import { ServerInformation } from "./SidePanel";

//THE DEFAULT DATA PORT
const DataPort = '3001';

function getDataServerFromAddress(address: string): URL{
    if(!address.includes("http://")){
        address = "http://" + address;
    }

    if(!address.includes(":")){
         address = address + ":" + DataPort;
    }

    return new URL(address);
}

//Returns List of Severs You Are In
export function getServerList(): string[] {
    return ['localhost']
}

// Returns Server Data from The Server
export async function getServerData(Address: string) : Promise<ServerInformation> {
    let response = await fetch(getDataServerFromAddress(Address));
    console.log(response);
    let json = await response.json();
    
    let newData: ServerInformation = {
        serverIP: Address,
        serverName: json['server_name'],
        users: json.users,
    }

    return newData;
}