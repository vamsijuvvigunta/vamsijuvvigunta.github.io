import axios from "axios";


export async function OneShotMsg(promptStr: string, modeStr: string) : Promise<string|null> {
    
    // test
    try {
        const jrpc_url = "/api/rpc";        
        const response = await axios.post(jrpc_url, {
                jsonrpc: '2.0',
                id: 1,
                method: 'llm-worker/one_shot_msg',
                params: {
                    data : {
                        prompt : promptStr,
                        mode : modeStr
                    }
                }                
            }
        )
          
        if (response) {
            // The JRPC payload is in the response.data.result
            // Which itself has a data.result field.            
            let jrpcResponse = response.data.result;
            console.log("OneShotMsg succeeded");
            console.log(`Response : ${JSON.stringify(jrpcResponse.data.response)}`)
            return jrpcResponse.data.response;
        } 
    } catch(err)
    {
        console.error(err)
        return "Error from OneShotMsg";
    }    

    return null;
}
