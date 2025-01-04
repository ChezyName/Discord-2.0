import { ServerInformation } from "./SidePanel";
import { BaseDirectory, create, exists, open, readFile } from '@tauri-apps/plugin-fs';

//THE DEFAULT DATA PORT
const DEFAULT_DATA_PORT = '3001';
const SERVER_LIST_FILE_NAME = "DISCORD2_SERVERS";

function getDataServerFromAddress(address: string): URL{
    if(!address.includes(":")){
        address = address + ":" + DEFAULT_DATA_PORT;
   }

    if(!address.includes("http://")){
        address = "http://" + address;
    }

    return new URL(address);
}

//If File Does Not Exist, Create
async function InitServerFile() {
    const tokenExists = await exists(SERVER_LIST_FILE_NAME, {
        baseDir: BaseDirectory.AppLocalData,
    });

    //Create
    if(!tokenExists){
        await create(SERVER_LIST_FILE_NAME, { baseDir: BaseDirectory.AppLocalData });
    }
}

//Returns List of Severs You Are In
export async function getServerList(): Promise<string[]> {
    await InitServerFile();
    
    //Get File
    const decoder = new TextDecoder();
    
    const data = await readFile(SERVER_LIST_FILE_NAME, {
        baseDir: BaseDirectory.AppLocalData,
    });

    const serverList = decoder.decode(data);

    console.log(serverList);
    
    let serverArray = serverList.split(',');
    console.log(serverArray);

    return serverArray;
}

//Adds to the current array of servers
export async function addServerToList(address: string) {
    await InitServerFile();
    let serverData = await getServerData(address);
    if(serverData != null) {
        let serverArray = await getServerList();
        if(!serverArray.includes(address)){
            serverArray.push(address);

            //write to file
            const encoder = new TextEncoder();
            const data = encoder.encode(serverArray.join(','));
            const file = await open(SERVER_LIST_FILE_NAME, { write: true, baseDir: BaseDirectory.AppLocalData });
            const bytesWritten = await file.write(data);
            await file.close();
        }
    }
}

// Returns Server Data from The Server
export async function getServerData(Address: string) : Promise<ServerInformation|null> {
    if(Address == "") return null;

    let response = await fetch(getDataServerFromAddress(Address));
    console.log(response);

    if(!response.ok) return null;

    let json = await response.json();
    
    let newData: ServerInformation = {
        serverIP: Address,
        serverName: json['server_name'],
        users: json.users,
    }

    return newData;
}