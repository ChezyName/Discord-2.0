import { ServerInformation } from "./SidePanel";
import { BaseDirectory, create, exists, writeFile, readFile } from '@tauri-apps/plugin-fs';

//THE DEFAULT DATA PORT
//const DEFAULT_DATA_PORT = '3000';
const SERVER_LIST_FILE_NAME = "DISCORD2_SERVERS";

function getDataServerFromAddress(address: string): URL{
    if(address == "") return new URL("");

    if(!address.includes("http://")){
        address = "http://" + address;
    }

    return new URL(address);
}

export function getMessageGatewayFromAddress(address: string): URL {
    return new URL(address) //getDataServerFromAddress(address);
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
    return ['localhost:7777']
    await InitServerFile();
    
    //Get File
    const decoder = new TextDecoder();
    
    const data = await readFile(SERVER_LIST_FILE_NAME, {
        baseDir: BaseDirectory.AppLocalData,
    });

    const serverList = decoder.decode(data);
    
    let serverArray = serverList.split(',');

    return serverArray;
}

//Adds to the current array of servers
export async function addServerToList(address: string) {
    await InitServerFile();
    let addressFixed = address.replace("http://","").replace("https://","");
    let serverData = await getServerData(addressFixed);
    if(serverData != null) {
        let serverArray = await getServerList();
        if(!serverArray.includes(address)){
            serverArray.push(address);

            //write to file
            const encoder = new TextEncoder();
            const data = encoder.encode(serverArray.join(','));
            await writeFile(SERVER_LIST_FILE_NAME, data, {
                baseDir: BaseDirectory.AppLocalData,
            });
        }
    }
}

//Remove
export async function removeFromServerList(address: string) {
    await InitServerFile();
    console.log("Attempt to Remove: ", address)
    let serverArray = await getServerList();

    if(serverArray.includes(address)){
        console.log("Removing: ", address, serverArray)
        serverArray.splice(serverArray.indexOf(address), 1);

        //write to file
        const encoder = new TextEncoder();
        const data = encoder.encode(serverArray.join(','));
        await writeFile(SERVER_LIST_FILE_NAME, data, {
            baseDir: BaseDirectory.AppLocalData,
        });
    }
    else { console.log("Server Removal Incomplete - 404 Server Not Found"); }
}

// Returns Server Data from The Server
export async function getServerData(Address: string) : Promise<ServerInformation|null> {
    if(Address == "") return null;

    try {
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
    } catch (e) {
        return null
    }
}

export function getDisplayName(): string {
    return localStorage.getItem('displayName') || '';
}

/**
 * given parent and item (child), returns true or false based on if item is scroll on exactly its position
 * or when .scrollIntoView({ behavior: "instant" }) is used
 * 
 * @param parent parent of item
 * @param item item itself
 */
export function isScrolledOnElement(item: HTMLDivElement | null): boolean {
    if(item == null) return false
    return item.getBoundingClientRect().bottom <= window.innerHeight
}

/**
 * Given a URL, openes it in a new tab / window.
 * @param url 
 */
export const openInNewTab = (url:string) => {
    const newWindow = window.open(url, '_blank', 'noopener,noreferrer')
    if (newWindow) newWindow.opener = null
}